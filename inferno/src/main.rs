use tracing::{debug, error, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    skyfall::run().await?;
    
    Ok(())
}
