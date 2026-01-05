use serde::{Deserialize, Serialize};

/// Wire protocol message format for gossipsub communication
/// Supports both data messages and control messages for comprehensive testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WireMessage {
    /// Data message representing honest peer traffic
    Good { 
        seq: u64,           // Sequence number for replay attack prevention
        payload: Vec<u8>,   // Message content
    },

    /// Control message for protocol management and extensions
    Control { 
        kind: u8,           // Control message type identifier
        data: Vec<u8>,      // Control message payload
    },
}

/// Encode a wire message to bytes for network transmission
/// Uses bincode for efficient binary serialization
pub fn encode(msg: &WireMessage) -> Vec<u8> {
    bincode::serialize(msg).expect("bincode serialize should not fail")
}

/// Decode bytes into a wire message
/// Returns error if the data is malformed, corrupted, or doesn't match expected format
pub fn decode(bytes: &[u8]) -> anyhow::Result<WireMessage> {
    Ok(bincode::deserialize(bytes)?)
}
