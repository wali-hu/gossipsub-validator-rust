# gossipsub-score-sim

Mini take-home: implement an application-level message validator + per-peer scoring/quarantine
on top of Rust libp2p Gossipsub.

This implementation configures Gossipsub in "manual validation" mode where inbound messages are *not* automatically forwarded, and the node must call `report_message_validation_result(message_id, propagation_source, acceptance)` to Accept/Reject/Ignore each message.

## Features Implemented

### Core Objectives ✅

1. **Validator**
   - Decode messages using bincode/serde
   - Enforce max message size limits
   - **Additional validation rules:**
     - Empty payload rejection
     - Sequence number validation (prevents replay/out-of-order attacks)
     - Message deduplication using content hashing
   - Returns: Accept / Reject / Ignore with reasoning

2. **Rate limiting + backpressure**
   - Per-peer token bucket rate limiting (5 tokens/sec, 10 token capacity)
   - Bounded dedupe cache (max 10,000 entries with LRU eviction)
   - Bounded peer tracking (max 1,000 peers)
   - Demonstrates spam nodes don't blow up memory/CPU

3. **Peer scoring + quarantine**
   - Application-level scoring separate from Gossipsub's internal score
   - **Scoring system:**
     - Valid messages: +0.1 points
     - Rejected messages: -2 to -5 points (based on severity)
     - Rate limit violations: -10 points
     - Replay attacks: -3 points
   - Quarantine threshold: -50 points
   - Quarantined peers have messages ignored

4. **Simulation**
   - Spawns N peers in-process with bootstrap connectivity
   - Clear outcome reporting: honest vs spam message statistics
   - Comprehensive summary report with success metrics

### Bonus Task Implemented ✅

**Content-addressed message IDs**
- Message IDs use SHA256 hash of message content
- Includes domain separation (`gossipsub-msg:` prefix)
- Includes topic hash for additional security
- **Tradeoffs vs sequence-based IDs:**
  - ✅ Prevents duplicate message propagation across restarts
  - ✅ Content-based deduplication works across peers
  - ❌ Slightly higher CPU cost for hashing
  - ❌ No natural ordering for debugging

## How to run

```bash
# Basic simulation
RUST_LOG=info cargo run --release

# Custom parameters
RUST_LOG=info cargo run --release -- \
  --peers 10 \
  --bad-peers 3 \
  --duration-secs 30 \
  --publish-per-sec 5 \
  --spam-per-sec 50 \
  --max-message-bytes 16384

# With debug logging to see validation decisions
RUST_LOG=debug cargo run --release -- --peers 6 --bad-peers 2 --duration-secs 15
```

## Run tests

```bash
# Unit and property tests
cargo test

# Property tests only
cargo test --test validator_prop
```

## Architecture

### Validation Pipeline
1. **Size check** - Fast rejection of oversized messages
2. **Quarantine check** - Skip processing for quarantined peers  
3. **Rate limiting** - Token bucket per peer
4. **Decode validation** - Ensure message can be deserialized
5. **Content validation** - Empty payload, sequence number, deduplication
6. **Scoring update** - Apply score delta and check quarantine threshold

### Memory Management
- **Bounded peer tracking**: Max 1,000 peers with LRU eviction
- **Bounded dedupe cache**: Max 10,000 message hashes with FIFO eviction
- **Token bucket limits**: Fixed capacity per peer
- **No unbounded data structures**

### Peer Scoring System
```
Initial score: 0.0
Valid message: +0.1
Empty payload: -2.0
Decode error: -3.0
Replay/old seq: -3.0
Oversize message: -5.0
Rate limit violation: -10.0
Quarantine threshold: -50.0
```

## Expected Output

```
=== SIMULATION SUMMARY ===
Total Peers: 8 (Honest: 6, Bad: 2)
Total Messages: 2847
  - Accepted: 1456 (51.1%)
  - Rejected: 1203 (42.3%)
  - Ignored: 188 (6.6%)
Honest Message Success Rate: 94.2%
Quarantined Peers: 2
Outcome: SUCCESS: Honest messages delivered, spam mostly rejected
========================
```

## Implementation Highlights

- **Correct manual validation**: Every message gets exactly one Accept/Reject/Ignore report
- **Minimal, focused validation rules** with comprehensive tests
- **Bounded resource usage** under spam attacks
- **Coherent scoring and quarantine behavior**
- **Useful metrics and summary reporting**
- **Content-addressed message IDs** with domain separation
