use bincode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WireMessage {
    Good { seq: u64, payload: Vec<u8> },
    Bad,
}

pub fn encode(msg: &WireMessage) -> Vec<u8> {
    bincode::serialize(msg).expect("encode")
}

pub fn decode(bytes: &[u8]) -> Result<WireMessage, bincode::Error> {
    bincode::deserialize(bytes)
}
