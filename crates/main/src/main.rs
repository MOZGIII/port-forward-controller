//! Main entrypoint.

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Hello, world!");
}
