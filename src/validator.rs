use std::collections::{HashMap, VecDeque};
use std::time::{Instant};
use libp2p::gossipsub::MessageAcceptance;
use libp2p::PeerId;
use sha2::{Digest, Sha256};

use crate::codec::{decode, WireMessage};

// Configuration constants for validator behavior
const MAX_PEERS: usize = 1000;
const MAX_DEDUPE_SIZE: usize = 10000;
const TOKEN_BUCKET_CAPACITY: u32 = 20;
const TOKEN_REFILL_RATE: f64 = 15.0; // tokens per second
const QUARANTINE_THRESHOLD: f64 = -100.0;

#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    pub max_message_bytes: usize,
}

#[derive(Debug)]
pub struct Decision {
    pub acceptance: MessageAcceptance,
    pub reason: &'static str,
    pub score_delta: f64,
}

/// Token bucket implementation for rate limiting per peer
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new() -> Self {
        Self {
            tokens: TOKEN_BUCKET_CAPACITY as f64,
            last_refill: Instant::now(),
        }
    }

    /// Attempt to consume one token, returns true if successful
    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// Refill tokens based on elapsed time since last refill
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * TOKEN_REFILL_RATE).min(TOKEN_BUCKET_CAPACITY as f64);
        self.last_refill = now;
    }
}

/// Per-peer state tracking for scoring and rate limiting
struct PeerState {
    score: f64,
    bucket: TokenBucket,
    last_seq: Option<u64>,
    quarantined: bool,
}

impl Default for PeerState {
    fn default() -> Self {
        Self {
            score: 0.0,
            bucket: TokenBucket::new(),
            last_seq: None,
            quarantined: false,
        }
    }
}

/// Main validator that implements message validation with peer scoring
pub struct Validator {
    cfg: ValidatorConfig,
    peers: HashMap<PeerId, PeerState>,
    dedupe_cache: VecDeque<[u8; 32]>, // SHA256 hashes for deduplication
    dedupe_set: std::collections::HashSet<[u8; 32]>,
}

impl Validator {
    pub fn new(cfg: ValidatorConfig) -> Self {
        Self {
            cfg,
            peers: HashMap::new(),
            dedupe_cache: VecDeque::new(),
            dedupe_set: std::collections::HashSet::new(),
        }
    }

    /// Main validation function that processes incoming messages
    pub fn validate(&mut self, from: &PeerId, bytes: &[u8]) -> Decision {
        // First check: message size validation (most efficient check)
        if bytes.len() > self.cfg.max_message_bytes {
            self.update_peer_score(from, -5.0);
            return Decision {
                acceptance: MessageAcceptance::Reject,
                reason: "oversize",
                score_delta: -5.0,
            };
        }

        // Second check: peer quarantine status
        if self.is_quarantined(from) {
            return Decision {
                acceptance: MessageAcceptance::Ignore,
                reason: "quarantined",
                score_delta: 0.0,
            };
        }

        // Third check: rate limiting using token bucket algorithm
        if !self.check_rate_limit(from) {
            let current_score = self.get_peer_score(from);
            // Apply lighter penalty to peers with better reputation
            let penalty = if current_score < -10.0 { -15.0 } else { -1.0 };
            self.update_peer_score(from, penalty);
            return Decision {
                acceptance: MessageAcceptance::Reject,
                reason: "rate_limited",
                score_delta: penalty,
            };
        }

        // Fourth check: message decoding validation
        let msg = match decode(bytes) {
            Ok(msg) => msg,
            Err(_) => {
                self.update_peer_score(from, -15.0);
                return Decision {
                    acceptance: MessageAcceptance::Reject,
                    reason: "decode_error",
                    score_delta: -15.0,
                };
            }
        };

        // Fifth check: content-specific validation based on message type
        match msg {
            WireMessage::Good { seq, payload } => {
                // Validate payload is not empty (common spam indicator)
                if payload.is_empty() {
                    self.update_peer_score(from, -20.0);
                    return Decision {
                        acceptance: MessageAcceptance::Reject,
                        reason: "empty_payload",
                        score_delta: -20.0,
                    };
                }

                // Validate sequence number to prevent replay attacks
                if let Some(last_seq) = self.get_peer_last_seq(from) {
                    if seq <= last_seq {
                        self.update_peer_score(from, -25.0);
                        return Decision {
                            acceptance: MessageAcceptance::Reject,
                            reason: "replay_or_old_seq",
                            score_delta: -25.0,
                        };
                    }
                }
                self.update_peer_last_seq(from, seq);

                // Check for duplicate messages using content-addressed hashing
                let msg_hash = self.hash_message(bytes);
                if self.is_duplicate(&msg_hash) {
                    self.update_peer_score(from, -0.5);
                    return Decision {
                        acceptance: MessageAcceptance::Ignore,
                        reason: "duplicate",
                        score_delta: -0.5,
                    };
                }
                self.add_to_dedupe(msg_hash);

                // Message passed all validation checks - reward the peer
                self.update_peer_score(from, 10.0);
                Decision {
                    acceptance: MessageAcceptance::Accept,
                    reason: "ok",
                    score_delta: 10.0,
                }
            }
            WireMessage::Control { .. } => {
                // Control messages get basic validation only
                Decision {
                    acceptance: MessageAcceptance::Accept,
                    reason: "ok_control",
                    score_delta: 0.0,
                }
            }
        }
    }

    pub fn get_peer_score(&self, peer: &PeerId) -> f64 {
        self.peers.get(peer).map(|p| p.score).unwrap_or(0.0)
    }

    pub fn get_quarantined_count(&self) -> usize {
        self.peers.values().filter(|p| p.quarantined).count()
    }

    /// Check if peer can send a message based on rate limiting
    fn check_rate_limit(&mut self, peer: &PeerId) -> bool {
        self.ensure_peer_exists(peer);
        self.peers.get_mut(peer).unwrap().bucket.try_consume()
    }

    /// Update peer score and manage quarantine status
    fn update_peer_score(&mut self, peer: &PeerId, delta: f64) {
        self.ensure_peer_exists(peer);
        let peer_state = self.peers.get_mut(peer).unwrap();
        peer_state.score += delta;
        
        // Update quarantine status based on current score
        if peer_state.score <= QUARANTINE_THRESHOLD {
            peer_state.quarantined = true;
        } else if peer_state.score > -50.0 {
            // Allow recovery when score improves significantly
            peer_state.quarantined = false;
        }
    }

    fn is_quarantined(&self, peer: &PeerId) -> bool {
        self.peers.get(peer).map(|p| p.quarantined).unwrap_or(false)
    }

    fn get_peer_last_seq(&self, peer: &PeerId) -> Option<u64> {
        self.peers.get(peer).and_then(|p| p.last_seq)
    }

    fn update_peer_last_seq(&mut self, peer: &PeerId, seq: u64) {
        self.ensure_peer_exists(peer);
        self.peers.get_mut(peer).unwrap().last_seq = Some(seq);
    }

    /// Ensure peer exists in tracking map with bounded memory management
    fn ensure_peer_exists(&mut self, peer: &PeerId) {
        if self.peers.len() >= MAX_PEERS {
            // Remove oldest peer to maintain bounded memory usage
            if let Some(oldest_peer) = self.peers.keys().next().cloned() {
                self.peers.remove(&oldest_peer);
            }
        }
        
        self.peers.entry(*peer).or_insert_with(PeerState::default);
    }

    /// Generate content-addressed message ID using SHA256 with domain separation
    fn hash_message(&self, bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"gossipsub-msg:");  // Domain separation
        hasher.update(bytes);
        hasher.finalize().into()
    }

    fn is_duplicate(&self, hash: &[u8; 32]) -> bool {
        self.dedupe_set.contains(hash)
    }

    /// Add message hash to deduplication cache with LRU eviction
    fn add_to_dedupe(&mut self, hash: [u8; 32]) {
        if self.dedupe_cache.len() >= MAX_DEDUPE_SIZE {
            if let Some(old_hash) = self.dedupe_cache.pop_front() {
                self.dedupe_set.remove(&old_hash);
            }
        }
        self.dedupe_cache.push_back(hash);
        self.dedupe_set.insert(hash);
    }
}
