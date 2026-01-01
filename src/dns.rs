use crate::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use trust_dns_proto::{
    op::{Message, MessageType, OpCode, ResponseCode},
};

/// Run the DNS server on port 53
pub async fn run_dns_server(state: AppState) -> anyhow::Result<()> {
    let socket = Arc::new(UdpSocket::bind(&state.config.dns_bind).await?);
    tracing::info!("DNS server listening on {}", state.config.dns_bind);

    let mut buf = [0u8; 512]; // DNS packets are max 512 bytes

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let data = buf[..len].to_vec(); // Clone the data

        // Handle the query in a separate task to avoid blocking
        let socket_clone = Arc::clone(&socket);
        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_dns_query(&socket_clone, &data, addr, state_clone).await {
                tracing::error!("Error handling DNS query: {}", e);
            }
        });
    }
}

/// Handle a single DNS query
async fn handle_dns_query(
    socket: &UdpSocket,
    data: &[u8],
    addr: SocketAddr,
    state: AppState,
) -> anyhow::Result<()> {
    // Parse the DNS message
    let request = Message::from_vec(data)?;

    // Only handle standard queries
    if request.message_type() != MessageType::Query || request.op_code() != OpCode::Query {
        return Ok(());
    }

    // Get the first query (DNS supports multiple queries per message)
    let query = match request.queries().first() {
        Some(q) => q,
        None => return Ok(()),
    };

    let domain = query.name().to_string().trim_end_matches('.').to_string();

    // Check if domain should be blocked
    let should_block = state.blocklist.is_blocked(&domain);

    // Update statistics
    {
        let mut stats = state.stats.lock().await;
        if should_block {
            stats.record_blocked(&domain);
            tracing::debug!("Blocked query for domain: {}", domain);
        } else {
            stats.record_allowed();
            tracing::debug!("Allowed query for domain: {}", domain);
        }
    }

    if should_block {
        // Return NXDOMAIN for blocked domains
        let response = create_nxdomain_response(&request);
        let response_bytes = response.to_vec()?;
        socket.send_to(&response_bytes, addr).await?;
    } else {
        // Forward to upstream DNS
        forward_to_upstream(socket, data, addr, &state.config.upstream_dns).await?;
    }

    Ok(())
}

/// Create a NXDOMAIN response for blocked domains
fn create_nxdomain_response(request: &Message) -> Message {
    let mut response = Message::new();
    response.set_id(request.id());
    response.set_message_type(MessageType::Response);
    response.set_op_code(OpCode::Query);
    response.set_authoritative(false);
    response.set_truncated(false);
    response.set_recursion_desired(request.recursion_desired());
    response.set_recursion_available(true);
    response.set_response_code(ResponseCode::NXDomain);

    // Copy the original query
    if let Some(query) = request.queries().first() {
        response.add_query(query.clone());
    }

    response
}

/// Forward query to upstream DNS server
async fn forward_to_upstream(
    socket: &UdpSocket,
    query_data: &[u8],
    client_addr: SocketAddr,
    upstream_addr: &str,
) -> anyhow::Result<()> {
    // Create upstream socket
    let upstream_socket = UdpSocket::bind("0.0.0.0:0").await?;
    upstream_socket.connect(upstream_addr).await?;

    // Send query to upstream
    upstream_socket.send(query_data).await?;

    // Receive response
    let mut buf = [0u8; 512];
    let len = upstream_socket.recv(&mut buf).await?;
    let response_data = &buf[..len];

    // Send response back to client
    socket.send_to(response_data, client_addr).await?;

    Ok(())
}