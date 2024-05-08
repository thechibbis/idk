#[tokio::main]
async fn main() -> anyhow::Result<()> {
    skyfall::application::App::run().await?;

    Ok(())
}
