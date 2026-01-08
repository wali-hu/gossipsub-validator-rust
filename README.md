# Gossipsub Validator + Peer Scoring Simulation

A Rust implementation of an application-level message validator with per-peer scoring and quarantine behavior on top of libp2p Gossipsub.

## Overview

This project implements a mini gossipsub simulation that demonstrates:
- Manual message validation (Accept/Reject/Ignore)
- Per-peer application scoring
- Quarantine behavior for malicious peers
- Rate limiting and bounded resource usage
- Multi-peer in-process simulation

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        Simulation                           │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │
│  │ Node 0  │  │ Node 1  │  │ Node 2  │  │ Node N  │        │
│  │ (bad)   │  │ (bad)   │  │ (honest)│  │ (honest)│        │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────┬────┘        │
│       │            │            │            │              │
│       └────────────┴─────┬──────┴────────────┘              │
│                          │                                  │
│                   Gossipsub Mesh                            │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Validator (`src/validator.rs`)

The validator processes every incoming message and returns a `Decision`:

```rust
pub struct Decision {
    pub acceptance: MessageAcceptance,  // Accept, Reject, or Ignore
    pub reason: &'static str,           // Why this decision was made
    pub score_delta: f64,               // Score change for the peer
}
```

**Validation Rules:**

| Rule | Verdict | Score Delta | Description |
|------|---------|-------------|-------------|
| Oversize | Reject | -60 | Message exceeds `max_message_bytes` |
| Decode error | Reject | -30 | Cannot deserialize as `WireMessage` |
| Empty payload | Reject | -30 | `WireMessage::Good` with empty payload |
| Malicious marker | Reject | -80 | `WireMessage::Bad` variant |
| Rate limited | Reject | -5 | Peer exceeded token bucket rate |
| Duplicate | Ignore | 0 | Already seen (content-addressed dedupe) |
| Replay/old seq | Ignore | 0 | Sequence number not increasing for author |
| Forwarder quarantined | Ignore | 0 | Forwarder is in quarantine |
| Valid | Accept | 0 | Passed all checks |

**Key Design Decision - Author vs Forwarder:**

```
In Gossipsub:
- message.source = original author (who created the message)
- propagation_source = forwarder (who sent it to us)

We use:
- Author for: sequence/replay checks, content violations
- Forwarder for: rate limiting, quarantine checks
- Ignore (not Reject) for: replay/stale messages (don't punish forwarders)
```

This is critical because in a mesh topology, the same message may arrive from multiple forwarders. Punishing forwarders for author violations would collapse the network.

### 2. Peer Scoring

**Application-Level Scoring (`validator.rs`):**

- Each peer has a score starting at 0
- Violations decrease score (penalties in table above)
- Repeated offences escalate: `effective_delta = base_delta * (1 + 0.5 * (offences - 1))`
- Quarantine threshold: -25 (peer is ignored when score drops below this)

**Gossipsub-Level Scoring (`behaviour.rs`):**

```rust
let mut params = PeerScoreParams::default();
params.app_specific_weight = 5.0;  // App scores have strong influence
params.ip_colocation_factor_threshold = 1_000_000.0;  // Disabled for localhost

let thresholds = PeerScoreThresholds {
    gossip_threshold: -15.0,
    publish_threshold: -40.0,
    graylist_threshold: -80.0,
    ..Default::default()
};

gossipsub.with_peer_score(params, thresholds)?;
```

### 3. Rate Limiting (`validator.rs`)

Token bucket per peer:
- Capacity: 100 tokens
- Refill rate: 50 tokens/second
- Each message consumes 1 token
- Exceeding limit → Reject with small penalty (-5)

### 4. Bounded Resources

| Resource | Bound | Eviction Policy |
|----------|-------|-----------------|
| Dedupe cache | 10,000 entries | FIFO (oldest removed) |
| Peer state map | 1,000 peers | Remove oldest entry |

### 5. Message Format (`src/codec.rs`)

```rust
pub enum WireMessage {
    Good { seq: u64, payload: Vec<u8> },  // Normal message
    Bad,                                    // Malicious marker (for testing)
}
```

Serialization: bincode

### 6. Content-Addressed Message IDs (`src/behaviour.rs`)

```rust
let message_id_fn = |message: &gossipsub::Message| {
    let mut hasher = Sha256::new();
    hasher.update(b"gossipsub-v1.1:");  // Domain separation
    hasher.update(&message.data);
    gossipsub::MessageId::from(hex::encode(hasher.finalize()))
};
```

This ensures duplicate messages (same content) get the same ID regardless of sender.

## Simulation (`src/sim.rs`)

### Topology

- All nodes dial node 0 (bootstrap)
- Additional cross-connections for better mesh: `if (i + j) % 3 == 0`
- 3 seconds wait for mesh formation before publishing

### Bad Peer Behavior

Bad peers (first N nodes where N = `--bad-peers`) send:
- Random junk (decode errors)
- Oversize payloads
- Empty payloads
- `WireMessage::Bad` markers

Each bad message is unique (random nonce) to bypass gossipsub's internal dedupe.

### Metrics Tracking

Per-node counters:
- `accepted`, `rejected`, `ignored` - total message counts
- `honest_accepted`, `honest_rejected` - messages from honest authors
- `quarantined_peers` - peers in quarantine

**Honest Success Rate** = `honest_accepted / (honest_accepted + honest_rejected)`

This metric tracks whether honest messages get delivered, regardless of spam.

## Running the Simulation

```bash
# Basic run
RUST_LOG=info cargo run --release -- \
  --peers 10 \
  --bad-peers 2 \
  --duration-secs 20 \
  --publish-per-sec 5 \
  --spam-per-sec 50 \
  --max-message-bytes 16384

# Baseline test (no attackers)
RUST_LOG=warn cargo run --release -- --peers 6 --bad-peers 0 --duration-secs 10

# Spam test
RUST_LOG=warn cargo run --release -- --peers 8 --bad-peers 2 --duration-secs 15 --spam-per-sec 50
```

### CLI Options

| Option | Default | Description |
|--------|---------|-------------|
| `--peers` | 8 | Total peers in simulation |
| `--bad-peers` | 2 | First N peers are attackers |
| `--duration-secs` | 20 | Simulation duration |
| `--publish-per-sec` | 5 | Honest publish rate per peer |
| `--spam-per-sec` | 50 | Bad peer spam rate |
| `--max-message-bytes` | 16384 | Max allowed message size |
| `--seed` | 1337 | RNG seed for reproducibility |

## Expected Results

### Baseline (bad_peers = 0)

```
=== SIMULATION SUMMARY ===
Total Peers: 6 (Honest: 6, Bad: 0)
Total Messages: 900
  - Accepted: 900 (100.0%)
  - Rejected: 0 (0.0%)
  - Ignored: 0 (0.0%)
Honest Message Success Rate: 100.0%
Quarantined Peers: 0
========================
```

✅ No quarantines, no rejects, 100% delivery

### Spam Run (bad_peers = 2)

```
=== SIMULATION SUMMARY ===
Total Peers: 8 (Honest: 6, Bad: 2)
Total Messages: 730
  - Accepted: 720 (98.6%)
  - Rejected: 10 (1.4%)
  - Ignored: 0 (0.0%)
Honest Message Success Rate: 100.0%
Quarantined Peers: 10
========================
```

✅ Bad peers quarantined, honest delivery maintained at 100%

## File Structure

```
src/
├── main.rs        # Entry point
├── cli.rs         # Command-line argument parsing
├── sim.rs         # Simulation orchestration
├── p2p.rs         # Node spawning, swarm event loop
├── behaviour.rs   # Gossipsub config with peer scoring
├── validator.rs   # Message validation + app scoring
├── codec.rs       # WireMessage serialization
├── metrics.rs     # Counter structs
└── lib.rs         # Library exports

tests/
└── validator_prop.rs  # Property-based tests
```

## Key Implementation Details

### Why Ignore Instead of Reject for Replay?

```rust
// In validator.rs
if seq <= last {
    return Decision {
        acceptance: MessageAcceptance::Ignore,  // NOT Reject
        reason: "replay_or_old_seq",
        score_delta: 0.0,  // No penalty
    };
}
```

`Reject` punishes the `propagation_source` (forwarder) in gossipsub. If we reject replays, we'd punish honest forwarders who happen to forward a message we've already seen from another path. `Ignore` silently drops without penalty.

### Why Separate Author and Forwarder Tracking?

```rust
pub fn validate(&mut self, propagation_source: &PeerId, author: Option<&PeerId>, bytes: &[u8]) -> Decision {
    // Forwarder quarantine check
    if self.is_quarantined(propagation_source) { ... }
    
    // Rate limit on forwarder
    if !self.peers.get_mut(propagation_source).unwrap().bucket.try_consume(1) { ... }
    
    // Content violations blame author
    let target = author.unwrap_or(propagation_source);
    self.record_offence_and_update(target, base_delta);
}
```

- **Forwarder** controls how fast messages arrive → rate limiting
- **Author** controls message content → content violations

### Gossipsub Peer Scoring Integration

```rust
// In p2p.rs, after validation
if let Some(new_score) = validator.get_app_score_option(&target) {
    swarm.behaviour_mut().gossipsub.set_application_score(&target, new_score);
}
```

This syncs our application scores to gossipsub's internal scoring, affecting mesh membership decisions.

## Limitations & Future Work

1. **Localhost only** - IP colocation penalties disabled; real deployment needs tuning
2. **No persistence** - Scores reset on restart
3. **Simple topology** - Star + random connections; real networks are more complex
4. **No eclipse detection** - Bonus task not implemented

## Dependencies

- `libp2p 0.56` - Networking (gossipsub, tcp, noise, yamux)
- `tokio` - Async runtime
- `serde/bincode` - Serialization
- `sha2` - Content hashing
- `tracing` - Logging

## License

MIT
