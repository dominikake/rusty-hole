use rusty_hole::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
