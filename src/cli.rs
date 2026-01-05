use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "gossipsub-score-sim", about = "Mini gossipsub validator + peer scoring simulation")]
pub struct Cli {
    /// Total peers in the sim (includes bad peers).
    #[arg(long, default_value_t = 8)]
    pub peers: usize,

    /// First N peers are attackers/spammers.
    #[arg(long, default_value_t = 2)]
    pub bad_peers: usize,

    #[arg(long, default_value_t = 20)]
    pub duration_secs: u64,

    /// Honest publish rate (per peer).
    #[arg(long, default_value_t = 5)]
    pub publish_per_sec: u64,

    /// Bad publish rate (per peer).
    #[arg(long, default_value_t = 50)]
    pub spam_per_sec: u64,

    #[arg(long, default_value = "frost-sim/coordination/1")]
    pub topic: String,

    #[arg(long, default_value_t = 1337)]
    pub seed: u64,

    #[arg(long, default_value_t = 16 * 1024)]
    pub max_message_bytes: usize,
}
