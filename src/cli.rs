use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, default_value_t = 10)]
    pub peers: usize,

    #[arg(long, default_value_t = 2)]
    pub bad_peers: usize,

    #[arg(long, default_value_t = 20)]
    pub duration_secs: u64,

    #[arg(long, default_value_t = 5)]
    pub publish_per_sec: u32,

    #[arg(long, default_value_t = 50)]
    pub spam_per_sec: u32,

    #[arg(long, default_value_t = 16384)]
    pub max_message_bytes: usize,

    #[arg(long, default_value_t = 0)]
    pub seed: u64,
}
