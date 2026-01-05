use std::collections::{HashMap, VecDeque};
use std::time::{Instant};
use libp2p::gossipsub::MessageAcceptance;
use libp2p::PeerId;
use sha2::{Digest, Sha256};

use crate::codec::{decode, WireMessage};

const MAX_PEERS: usize = 1000;
const MAX_DEDUPE_SIZE: usize = 10000;
const TOKEN_BUCKET_CAPACITY: u32 = 10;
const TOKEN_REFILL_RATE: f64 = 5.0; // tokens per second
const QUARANTINE_THRESHOLD: f64 = -50.0;

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

    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * TOKEN_REFILL_RATE).min(TOKEN_BUCKET_CAPACITY as f64);
        self.last_refill = now;
    }
}

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

pub struct Validator {
    cfg: ValidatorConfig,
    peers: HashMap<PeerId, PeerState>,
    dedupe_cache: VecDeque<[u8; 32]>, // SHA256 hashes
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

    pub fn validate(&mut self, from: &PeerId, bytes: &[u8]) -> Decision {
        // 1) Size check (cheap)
        if bytes.len() > self.cfg.max_message_bytes {
            self.update_peer_score(from, -5.0);
            return Decision {
                acceptance: MessageAcceptance::Reject,
                reason: "oversize",
                score_delta: -5.0,
            };
        }

        // 2) Check if peer is quarantined
        if self.is_quarantined(from) {
            return Decision {
                acceptance: MessageAcceptance::Ignore,
                reason: "quarantined",
                score_delta: 0.0,
            };
        }

        // 3) Rate limiting check
        if !self.check_rate_limit(from) {
            self.update_peer_score(from, -10.0);
            return Decision {
                acceptance: MessageAcceptance::Reject,
                reason: "rate_limited",
                score_delta: -10.0,
            };
        }

        // 4) Decode check
        let msg = match decode(bytes) {
            Ok(msg) => msg,
            Err(_) => {
                self.update_peer_score(from, -3.0);
                return Decision {
                    acceptance: MessageAcceptance::Reject,
                    reason: "decode_error",
                    score_delta: -3.0,
                };
            }
        };

        // 5) Content validation
        match msg {
            WireMessage::Good { seq, payload } => {
                // Rule 1: Empty payload check
                if payload.is_empty() {
                    self.update_peer_score(from, -2.0);
                    return Decision {
                        acceptance: MessageAcceptance::Reject,
                        reason: "empty_payload",
                        score_delta: -2.0,
                    };
                }

                // Rule 2: Sequence number validation (prevent replay/out-of-order)
                if let Some(last_seq) = self.get_peer_last_seq(from) {
                    if seq <= last_seq {
                        self.update_peer_score(from, -3.0);
                        return Decision {
                            acceptance: MessageAcceptance::Reject,
                            reason: "replay_or_old_seq",
                            score_delta: -3.0,
                        };
                    }
                }
                self.update_peer_last_seq(from, seq);

                // Rule 3: Deduplication check
                let msg_hash = self.hash_message(bytes);
                if self.is_duplicate(&msg_hash) {
                    self.update_peer_score(from, -1.0);
                    return Decision {
                        acceptance: MessageAcceptance::Ignore,
                        reason: "duplicate",
                        score_delta: -1.0,
                    };
                }
                self.add_to_dedupe(msg_hash);

                // Valid message
                self.update_peer_score(from, 0.1);
                Decision {
                    acceptance: MessageAcceptance::Accept,
                    reason: "ok",
                    score_delta: 0.1,
                }
            }
            WireMessage::Control { .. } => {
                // Basic control message validation
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

    fn check_rate_limit(&mut self, peer: &PeerId) -> bool {
        self.ensure_peer_exists(peer);
        self.peers.get_mut(peer).unwrap().bucket.try_consume()
    }

    fn update_peer_score(&mut self, peer: &PeerId, delta: f64) {
        self.ensure_peer_exists(peer);
        let peer_state = self.peers.get_mut(peer).unwrap();
        peer_state.score += delta;
        
        // Check for quarantine
        if peer_state.score <= QUARANTINE_THRESHOLD && !peer_state.quarantined {
            peer_state.quarantined = true;
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

    fn ensure_peer_exists(&mut self, peer: &PeerId) {
        if self.peers.len() >= MAX_PEERS {
            // Remove oldest peer to maintain bounded memory
            if let Some(oldest_peer) = self.peers.keys().next().cloned() {
                self.peers.remove(&oldest_peer);
            }
        }
        
        self.peers.entry(*peer).or_insert_with(PeerState::default);
    }

    fn hash_message(&self, bytes: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finalize().into()
    }

    fn is_duplicate(&self, hash: &[u8; 32]) -> bool {
        self.dedupe_set.contains(hash)
    }

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
