#[tokio::main]
async fn main() -> anyhow::Result<()> {
    booqer_cli::run().await
}
