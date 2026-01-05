#[derive(Debug, Default, Clone)]
pub struct Counters {
    pub accepted: u64,
    pub rejected: u64,
    pub ignored: u64,
    pub quarantined_peers: u64,
}

#[derive(Debug, Clone)]
pub struct PeerMetrics {
    pub score: f64,
    pub messages_sent: u64,
    pub messages_rejected: u64,
}
