use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WireMessage {
    /// "Honest" traffic.
    Good { seq: u64, payload: Vec<u8> },

    /// Optional extension point for more message types.
    Control { kind: u8, data: Vec<u8> },
}

pub fn encode(msg: &WireMessage) -> Vec<u8> {
    bincode::serialize(msg).expect("bincode serialize should not fail")
}

pub fn decode(bytes: &[u8]) -> anyhow::Result<WireMessage> {
    Ok(bincode::deserialize(bytes)?)
}
