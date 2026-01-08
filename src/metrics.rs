use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Counters {
    pub accepted: u64,
    pub rejected: u64,
    pub ignored: u64,
}
