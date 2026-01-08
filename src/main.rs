use clap::Parser;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cli = gossipsub_score_sim::cli::Cli::parse();
    gossipsub_score_sim::sim::run(cli).await
}
