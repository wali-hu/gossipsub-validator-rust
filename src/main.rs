mod cli;
mod sim;
mod codec;
mod validator;
mod behaviour;
mod p2p;
mod metrics;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = cli::Cli::parse();
    sim::run(cli).await
}
