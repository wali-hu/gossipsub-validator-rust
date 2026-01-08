// --- constants / structs (replace existing constants) ---
const MAX_DEDUPE_SIZE: usize = 10_000;
// Keep generous token bucket capacity so honest bursts are fine
const TOKEN_BUCKET_CAPACITY: u32 = 100;
const TOKEN_REFILL_RATE: f64 = 50.0; // tokens per second
// Lower quarantine threshold so attackers are removed faster
const QUARANTINE_THRESHOLD: f64 = -120.0;

use std::collections::{HashMap, VecDeque, HashSet};
use std::time::Instant;
use libp2p::gossipsub::MessageAcceptance;
use libp2p::PeerId;
use sha2::{Digest, Sha256};

use crate::codec::{decode, WireMessage};

const MAX_PEERS: usize = 1000;

#[derive(Debug, Clone)]
pub struct ValidatorConfig {
    pub max_message_bytes: usize,
}

#[derive(Debug, Clone)]
struct TokenBucket {
    capacity: u32,
    tokens: f64,
    last: Instant,
}

impl TokenBucket {
    fn new() -> Self {
        Self {
            capacity: TOKEN_BUCKET_CAPACITY,
            tokens: TOKEN_BUCKET_CAPACITY as f64,
            last: Instant::now(),
        }
    }

    fn try_consume(&mut self, amount: u32) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last).as_secs_f64();
        self.last = now;
        self.tokens += elapsed * TOKEN_REFILL_RATE;
        if self.tokens > self.capacity as f64 {
            self.tokens = self.capacity as f64;
        }
        if self.tokens >= amount as f64 {
            self.tokens -= amount as f64;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Decision {
    pub acceptance: MessageAcceptance,
    pub reason: &'static str,
    pub score_delta: f64,
}

// --- Validator struct now includes offences map ---
pub struct Validator {
    cfg: ValidatorConfig,
    peers: HashMap<PeerId, PeerState>,
    // small bounded dedupe
    dedupe_cache: VecDeque<[u8; 32]>,
    dedupe_set: HashSet<[u8; 32]>,
    // offences counts per forwarder (escalate repeated malicious events)
    offences: HashMap<PeerId, u32>,
}

impl Validator {
    pub fn new(cfg: ValidatorConfig) -> Self {
        Self {
            cfg,
            peers: HashMap::new(),
            dedupe_cache: VecDeque::new(),
            dedupe_set: HashSet::new(),
            offences: HashMap::new(),
        }
    }

    /// Validate a message. `author` is the original message publisher (message.source),
    /// `forwarder` is the propagation source (the peer that forwarded the message to us).
    pub fn validate(&mut self, author: &PeerId, forwarder: &PeerId, bytes: &[u8]) -> Decision {
        // Helpful debug: record incoming validation attempt
        tracing::debug!(%author, %forwarder, len = bytes.len(), "validate called");
        // If forwarder quarantined, silently ignore
        if self.is_quarantined(forwarder) {
            return Decision {
                acceptance: MessageAcceptance::Ignore,
                reason: "forwarder_quarantined",
                score_delta: 0.0,
            };
        }

        // Oversize check
        if bytes.len() > self.cfg.max_message_bytes {
            // stronger penalty for oversize
            let base = -60.0;
            self.record_offence_and_update(forwarder, base);
            return Decision {
                acceptance: MessageAcceptance::Reject,
                reason: "oversize",
                score_delta: base,
            };
        }

        // Rate limit check on forwarder
        self.ensure_peer_exists(forwarder);
        if !self.peers.get_mut(forwarder).unwrap().bucket.try_consume(1) {
            // gentle penalty for short bursts; don't kill honest forwarders
            let base = -5.0;
            self.record_offence_and_update(forwarder, base);
            return Decision {
                acceptance: MessageAcceptance::Reject,
                reason: "rate_limited",
                score_delta: base,
            };
        }

        // Decode
        let msg = match decode(bytes) {
            Ok(m) => m,
            Err(_) => {
                // decode failures may indicate malice from forwarder — medium penalty
                let base = -20.0;
                self.record_offence_and_update(forwarder, base);
                return Decision {
                    acceptance: MessageAcceptance::Reject,
                    reason: "decode_error",
                    score_delta: base,
                };
            }
        };

        // Deduplicate by content hash
        let mut hasher = Sha256::new();
        hasher.update(b"gossipsub-v1.1:");
        hasher.update(bytes);
        let hash = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash);
        if self.is_dupe(&key) {
            // dedupe -> ignore (no penalty)
            return Decision {
                acceptance: MessageAcceptance::Ignore,
                reason: "duplicate",
                score_delta: 0.0,
            };
        }
        // add to dedupe cache
        self.add_to_dedupe(key);

        // Content-specific checks
        match msg {
            WireMessage::Good { seq, payload } => {
                if payload.is_empty() {
                    let base = -30.0;
                    self.record_offence_and_update(forwarder, base);
                    return Decision {
                        acceptance: MessageAcceptance::Reject,
                        reason: "empty_payload",
                        score_delta: base,
                    };
                }

                // Replay/sequence validation keyed by *author*
                let last = self.get_peer_last_seq(author).unwrap_or(0);
                if seq <= last {
                    // leave as IGNORE so forwarders are not punished for possible retransmits
                    return Decision {
                        acceptance: MessageAcceptance::Ignore,
                        reason: "replay_or_old_seq",
                        score_delta: 0.0,
                    };
                }
                // Update last seq for author
                self.update_peer_last_seq(author, seq);

                // Accept valid message
                return Decision {
                    acceptance: MessageAcceptance::Accept,
                    reason: "ok",
                    score_delta: 0.0,
                };
            }
            WireMessage::Bad => {
                // clearly malicious payload — escalate heavily
                let base = -80.0;
                self.record_offence_and_update(forwarder, base);
                return Decision {
                    acceptance: MessageAcceptance::Reject,
                    reason: "malicious_payload",
                    score_delta: base,
                };
            }
        }
    }

    pub fn get_peer_score(&self, peer: &PeerId) -> f64 {
        self.peers.get(peer).map(|p| p.score).unwrap_or(0.0)
    }

    pub fn get_quarantined_count(&self) -> usize {
        self.peers.values().filter(|p| p.quarantined).count()
    }

    pub fn dump_peer_states(&self) -> Vec<(libp2p::PeerId, f64, bool)> {
        self.peers.iter().map(|(p,s)| (*p, s.score, s.quarantined)).collect()
    }

    fn update_peer_score(&mut self, peer: &PeerId, delta: f64) {
        self.ensure_peer_exists(peer);
        let state = self.peers.get_mut(peer).unwrap();
        state.score += delta;
        let was_quarantined = state.quarantined;
        state.quarantined = state.score <= QUARANTINE_THRESHOLD;

        // Log score updates and transitions so we can debug why peers are quarantined
        tracing::info!(peer = %peer, new_score = state.score, delta = delta, quarantined = state.quarantined, "peer score updated");
        if !was_quarantined && state.quarantined {
            tracing::warn!(peer = %peer, new_score = state.score, "peer entered quarantine");
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
        if let Some(state) = self.peers.get_mut(peer) {
            state.last_seq = Some(seq);
        }
    }

    fn ensure_peer_exists(&mut self, peer: &PeerId) {
        if self.peers.len() >= MAX_PEERS {
            // remove a random/first key to bound memory
            if let Some(old) = self.peers.keys().next().cloned() {
                self.peers.remove(&old);
            }
        }
        self.peers.entry(*peer).or_insert_with(PeerState::default);
    }

    fn is_dupe(&self, hash: &[u8; 32]) -> bool {
        self.dedupe_set.contains(hash)
    }

    fn add_to_dedupe(&mut self, hash: [u8; 32]) {
        if self.dedupe_cache.len() >= MAX_DEDUPE_SIZE {
            if let Some(old) = self.dedupe_cache.pop_front() {
                self.dedupe_set.remove(&old);
            }
        }
        self.dedupe_cache.push_back(hash);
        self.dedupe_set.insert(hash);
    }

    // increments offences count, computes scaled delta, updates score and returns the effective delta
    fn record_offence_and_update(&mut self, peer: &PeerId, base_delta: f64) -> f64 {
        // increment offence count
        let count = self.offences.entry(*peer).or_insert(0);
        *count += 1;
        let count_val = *count;
        // scaling factor (each extra offence increases delta by 25%)
        let scale = 1.0 + ((count_val as f64 - 1.0) * 0.25).max(0.0);
        let effective_delta = base_delta * scale;
        self.update_peer_score(peer, effective_delta);
        tracing::info!(peer = %peer, offences = count_val, base = base_delta, effective = effective_delta, "offence recorded and score updated");
        // if offences exceed 4, immediately quarantine
        if count_val > 4 {
            if let Some(s) = self.peers.get_mut(peer) {
                s.quarantined = true;
                tracing::warn!(peer = %peer, score = s.score, "peer forced into quarantine due to repeated offences");
            }
        }
        effective_delta
    }

    fn get_offence_count(&self, peer: &PeerId) -> u32 {
        *self.offences.get(peer).unwrap_or(&0)
    }
}
