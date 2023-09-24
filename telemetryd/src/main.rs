fn init_tracing() {
    use tracing_subscriber::{filter::Targets, fmt, prelude::*, Registry};

    let filter: Targets = std::env::var("RUST_LOG")
        .as_deref()
        .unwrap_or("info")
        .parse::<Targets>()
        .unwrap();

    Registry::default()
        .with(fmt::layer().with_target(true))
        .with(filter)
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    telemetryd::cli::parse().run().await;
}
