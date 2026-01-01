# RustyHole - Lightweight Network Ad Blocker

A lightweight, open-source DNS sinkhole written in Rust, inspired by Pi-hole. RustyHole acts as a network-wide ad blocker by intercepting DNS queries and blocking requests to known ad/malware domains.

## Features

- **DNS Sinkhole**: Intercepts DNS queries and blocks ad/malware domains
- **Blocklist Management**: Downloads and parses blocklists from URLs (StevenBlack format)
- **Web Dashboard**: Simple HTTP dashboard for monitoring stats
- **Async Performance**: Built with Tokio for high concurrent query handling
- **Memory Efficient**: Uses HashSet for O(1) domain lookups
- **Configurable**: TOML-based configuration for easy customization

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Client DNS    │───▶│  RustyHole DNS   │───▶│  Upstream DNS   │
│     Query       │    │    Server        │    │  (8.8.8.8)      │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │   Blocklist      │
                       │   Checker        │
                       └──────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │   Web Dashboard  │
                       │   (Stats/Monitor)│
                       └──────────────────┘
```

## Requirements

- Rust 1.70+
- Root privileges (for DNS port 53 binding)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rusty-hole.git
cd rusty-hole
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

Edit `config.toml`:

```toml
# Upstream DNS server to forward legitimate queries to
upstream_dns = "8.8.8.8:53"

# List of URLs to fetch blocklists from
blocklist_urls = [
    "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
]

# Domains to whitelist (overrides blocklist)
whitelist = []

# Port for the web dashboard
web_port = 8080

# DNS server bind address (requires root privileges)
dns_bind = "0.0.0.0:53"
```

## Usage

1. **Configure Network DNS**: Set your router's DNS server to point to this machine's IP address.

2. **Run RustyHole**:
```bash
sudo ./target/release/rusty-hole
```

The DNS server will bind to port 53 and the web dashboard will be available at http://localhost:8080.

## Web Dashboard

- **Stats API**: `GET /stats` - Returns JSON with query statistics
- **Dashboard**: `GET /dashboard` - HTML interface showing current stats

## Router Setup

### DHCP Reservation (Recommended)
1. Access your router's admin panel
2. Find DHCP settings
3. Create a static IP reservation for this machine
4. Set the router's DNS server to the reserved IP

### Manual DNS Configuration
Alternatively, configure each device to use this machine's IP as DNS server.

## Blocklist Sources

RustyHole supports any hosts-file format blocklist. Popular sources:

- StevenBlack hosts: `https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts`
- AdAway: Various mirrors available
- Custom lists in hosts format

## Security Notes

- The web dashboard currently has no authentication
- Only bind the dashboard to localhost in production
- Run with minimal privileges where possible

## Performance

- Memory usage: ~50-100MB depending on blocklist size
- Handles thousands of concurrent queries
- Fast O(1) domain lookups using HashSet

## Differences from Pi-hole

- Written in Rust for memory safety and performance
- Simpler architecture, no gravity update mechanism
- No built-in DHCP server
- Minimal dependencies
- No auto-updates (rebuild required)

## Development

```bash
# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Lint code
cargo clippy
```

## Troubleshooting

### Permission Denied (Port 53)
Run with sudo:
```bash
sudo cargo run --release
```

### High Memory Usage
Reduce blocklist URLs or switch to smaller lists.

### No Blocking
Check:
- Router DNS points to RustyHole IP
- Blocklist URLs are accessible
- Logs show blocked queries

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Submit a pull request