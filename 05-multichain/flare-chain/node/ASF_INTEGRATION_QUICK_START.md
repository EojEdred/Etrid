# ASF Consensus Integration - Quick Start Guide

## Overview

This guide helps developers integrate and use the ASF consensus RPC and telemetry features in FlareChain.

---

## Using ASF RPC Endpoints

### 1. Query Finality Level

**Get the finality level of any block:**

```bash
# Using curl
curl -H "Content-Type: application/json" \
     -d '{
       "id":1,
       "jsonrpc":"2.0",
       "method":"asf_getFinalityLevel",
       "params":["0x1234..."]
     }' \
     http://localhost:9944

# Response:
{
  "jsonrpc": "2.0",
  "result": {
    "block_hash": "0x1234...",
    "block_number": 12345,
    "level": 3,
    "level_name": "Strong",
    "certificate_count": 75,
    "is_finalized": true,
    "time_to_finalization_ms": 12000
  },
  "id": 1
}
```

**Using polkadot.js:**

```javascript
const api = await ApiPromise.create({ provider: wsProvider });
const blockHash = '0x1234...';
const finalityInfo = await api.rpc.asf.getFinalityLevel(blockHash);

console.log('Finality Level:', finalityInfo.level_name);
console.log('Is Finalized:', finalityInfo.is_finalized);
console.log('Certificates:', finalityInfo.certificate_count);
```

### 2. Query Validator Set

**Get all active validators:**

```bash
curl -H "Content-Type: application/json" \
     -d '{
       "id":1,
       "jsonrpc":"2.0",
       "method":"asf_getValidatorSet",
       "params":[]
     }' \
     http://localhost:9944
```

**Using polkadot.js:**

```javascript
const validators = await api.rpc.asf.getValidatorSet();
validators.forEach(v => {
  console.log(`Validator: ${v.validator_id}`);
  console.log(`  Stake: ${v.stake}`);
  console.log(`  Active: ${v.is_active}`);
  console.log(`  Role: ${v.role || 'None'}`);
});
```

### 3. Submit a Vote

**External validators can submit votes:**

```javascript
// Sign vote with your validator key
const vote = {
  block_hash: blockHash,
  block_number: blockNumber,
  phase: 'Prepare', // or PreCommit, Commit, Decide
  validator: validatorId,
  stake_weight: stake,
  epoch: currentEpoch,
  timestamp: Date.now(),
};

// Encode and sign
const voteHex = u8aToHex(vote.toU8a());
const signatureHex = u8aToHex(validatorPair.sign(voteHex));

// Submit via RPC
const result = await api.rpc.asf.submitVote(voteHex, signatureHex);
console.log('Vote submitted:', result.success);
console.log('Message:', result.message);
```

### 4. Get Block Status

**Get comprehensive consensus data for a block:**

```javascript
const status = await api.rpc.asf.getBlockStatus(blockHash);

console.log('Block:', status.block_number);
console.log('Finality:', status.finality_level.level_name);
console.log('Certificates by Phase:');
console.log('  Prepare:', status.certificates_by_phase.prepare);
console.log('  PreCommit:', status.certificates_by_phase.pre_commit);
console.log('  Commit:', status.certificates_by_phase.commit);
console.log('  Decide:', status.certificates_by_phase.decide);
console.log('Total Votes:', status.total_votes);
```

### 5. Get Committee Information

```javascript
const committee = await api.rpc.asf.getCommitteeInfo();

console.log('Epoch:', committee.epoch);
console.log('Committee Size:', committee.size);
console.log('Total Stake:', committee.total_stake);
console.log('BFT Threshold:', committee.bft_threshold, 'validators');
console.log('Stake Threshold:', committee.stake_threshold);
```

### 6. Query Slashing History

```javascript
// Get last 10 slashing events
const history = await api.rpc.asf.getSlashingHistory(10);

history.forEach(event => {
  console.log(`Validator ${event.validator_id} slashed:`);
  console.log(`  Severity: ${event.severity}`);
  console.log(`  Reason: ${event.reason}`);
  console.log(`  Amount: ${event.amount_slashed}`);
  console.log(`  Excluded: ${event.excluded}`);
});
```

---

## Telemetry Integration

### Adding Telemetry to Your Code

**Import the module:**

```rust
use crate::asf_telemetry;
```

**Emit vote events:**

```rust
// When receiving a vote
asf_telemetry::telemetry_vote_received(&telemetry, &vote);

// When submitting a vote
asf_telemetry::telemetry_vote_submitted(&telemetry, &vote);
```

**Emit certificate events:**

```rust
// When generating a certificate
asf_telemetry::telemetry_certificate_generated(&telemetry, &certificate);

// When broadcasting a certificate
asf_telemetry::telemetry_certificate_broadcast(&telemetry, &certificate);
```

**Emit finality events:**

```rust
// When finality level changes
asf_telemetry::telemetry_finality_level_changed(
    &telemetry,
    &block_hash,
    block_number,
    FinalityLevel::Moderate,  // old level
    FinalityLevel::Strong,     // new level
    75                         // certificate count
);

// When block reaches finality
asf_telemetry::telemetry_finality_reached(
    &telemetry,
    &block_hash,
    block_number,
    FinalityLevel::Strong,
    Some(12000)  // time to finalization in ms
);
```

**Emit slashing events:**

```rust
// When slashing occurs
asf_telemetry::telemetry_slashing_event(
    &telemetry,
    &validator_id,
    "Critical",              // severity
    "ConflictingVotes",      // reason
    10_000_000_000_000_000_000_000,  // amount slashed
    epoch,
    true                     // excluded from committee
);

// When Byzantine behavior detected
asf_telemetry::telemetry_byzantine_detected(
    &telemetry,
    &validator_id,
    "Double voting",
    3,                       // incident count
    epoch
);
```

**Emit committee events:**

```rust
// When committee rotates
asf_telemetry::telemetry_committee_rotated(
    &telemetry,
    new_epoch,
    21,                      // committee size
    1_344_000_000_000_000_000_000_000  // total stake
);

// When epoch transitions
asf_telemetry::telemetry_epoch_transition(
    &telemetry,
    old_epoch,
    new_epoch,
    block_number
);
```

**Emit block events:**

```rust
// When producing a block
asf_telemetry::telemetry_block_produced(
    &telemetry,
    &block_hash,
    block_number,
    extrinsic_count,
    &proposer_id
);

// When importing a block
asf_telemetry::telemetry_block_imported(
    &telemetry,
    &block_hash,
    block_number,
    is_new_best
);
```

### Using the Telemetry Aggregator

**Initialize:**

```rust
use crate::asf_telemetry::TelemetryAggregator;

let mut aggregator = TelemetryAggregator::new();
```

**Record events:**

```rust
aggregator.record_vote_received();
aggregator.record_vote_submitted();
aggregator.record_certificate_generated();
aggregator.record_block_finalized(12000); // 12 seconds to finality
aggregator.record_slashing_event();
aggregator.update_epoch(5);
```

**Get metrics snapshot:**

```rust
let snapshot = aggregator.snapshot();
println!("=== ASF Metrics ===");
println!("Votes Received: {}", snapshot.votes_received);
println!("Votes Submitted: {}", snapshot.votes_submitted);
println!("Certificates Generated: {}", snapshot.certificates_generated);
println!("Blocks Finalized: {}", snapshot.blocks_finalized);
println!("Avg Finality Time: {}ms", snapshot.avg_finality_time_ms);
println!("Slashing Events: {}", snapshot.slashing_events);
println!("Byzantine Detections: {}", snapshot.byzantine_detections);
```

**Periodic reporting:**

```rust
// In your consensus loop
loop {
    // ... consensus logic ...

    // Every 100 blocks, emit metrics
    if block_number % 100 == 0 {
        let snapshot = aggregator.snapshot();
        asf_telemetry::telemetry_consensus_metrics(
            &telemetry,
            snapshot.current_epoch,
            active_validators,
            total_stake,
            snapshot.blocks_finalized,
            snapshot.certificates_generated,
            snapshot.avg_finality_time_ms,
        );
    }
}
```

---

## Monitoring with Grafana

### Setting Up Telemetry Dashboard

**1. Configure Prometheus scraping:**

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'flarechain'
    static_configs:
      - targets: ['localhost:9615']
    metric_relabel_configs:
      - source_labels: [__name__]
        regex: 'asf_.*'
        action: keep
```

**2. Import Grafana dashboard:**

```json
{
  "dashboard": {
    "title": "ASF Consensus Metrics",
    "panels": [
      {
        "title": "Finality Level Distribution",
        "targets": [
          {
            "expr": "asf_finality_level_changed"
          }
        ]
      },
      {
        "title": "Certificate Generation Rate",
        "targets": [
          {
            "expr": "rate(asf_certificate_generated[5m])"
          }
        ]
      },
      {
        "title": "Average Finality Time",
        "targets": [
          {
            "expr": "asf_avg_finality_time_ms"
          }
        ]
      }
    ]
  }
}
```

---

## Development Workflow

### Running FlareChain with ASF

**1. Build the node:**

```bash
cd /Users/macbook/Desktop/etrid/05-multichain/flare-chain/node
cargo build --release
```

**2. Run with ASF enabled:**

```bash
./target/release/flare-chain-node \
  --dev \
  --enable-asf \
  --rpc-cors=all \
  --rpc-methods=Unsafe \
  --rpc-port=9944 \
  --prometheus-port=9615
```

**3. Test RPC endpoints:**

```bash
# Test finality query
curl -H "Content-Type: application/json" \
     -d '{"id":1, "jsonrpc":"2.0", "method":"asf_getValidatorSet", "params":[]}' \
     http://localhost:9944

# Test with polkadot.js apps
# Navigate to: https://polkadot.js.org/apps/?rpc=ws://localhost:9944
```

**4. Monitor telemetry:**

```bash
# View Prometheus metrics
curl http://localhost:9615/metrics | grep asf_

# Tail node logs
tail -f ~/.local/share/flare-chain/chains/dev/network/flarechain.log | grep asf
```

### Testing Vote Submission

**Create a test vote:**

```rust
use asf_algorithm::{Vote, ConsensusPhase};
use sp_core::sr25519::Pair;
use codec::Encode;

// Generate test keypair
let (pair, _) = Pair::generate_with_phrase(None);
let validator_id = ValidatorId::from(pair.public().0);

// Create vote
let vote = Vote {
    block_hash: Hash::from([0u8; 32]),
    block_number: 1,
    phase: ConsensusPhase::Prepare,
    validator: validator_id,
    stake_weight: 64_000_000_000_000_000_000_000, // 64 ËTR
    epoch: 1,
    timestamp: 1000000,
    signature: pair.sign(&vote_message),
};

// Encode to hex
let vote_hex = hex::encode(vote.encode());
let sig_hex = hex::encode(vote.signature.encode());

// Submit via RPC
// curl -H "Content-Type: application/json" \
//      -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\":\"asf_submitVote\", \"params\":[\"$vote_hex\", \"$sig_hex\"]}" \
//      http://localhost:9944
```

---

## Common Patterns

### Monitoring Finality Progression

```javascript
// Subscribe to new block headers
const unsubscribe = await api.rpc.chain.subscribeNewHeads(async (header) => {
  const blockHash = header.hash.toHex();
  const blockNumber = header.number.toNumber();

  // Query finality level
  const finality = await api.rpc.asf.getFinalityLevel(blockHash);

  console.log(`Block #${blockNumber} (${blockHash})`);
  console.log(`  Finality: ${finality.level_name} (${finality.certificate_count} certs)`);

  if (finality.is_finalized) {
    console.log(`  ✅ Finalized in ${finality.time_to_finalization_ms}ms`);
  }
});
```

### Validator Health Monitoring

```javascript
// Check validator health every 60 seconds
setInterval(async () => {
  const validators = await api.rpc.asf.getValidatorSet();

  validators.forEach(v => {
    if (!v.is_active) {
      console.warn(`⚠️  Validator ${v.validator_id} is OFFLINE`);
    }
    if (v.is_excluded) {
      console.error(`❌ Validator ${v.validator_id} is EXCLUDED (slashed)`);
    }
  });
}, 60000);
```

### Slashing Event Alerting

```javascript
// Poll for new slashing events
let lastSeenCount = 0;

setInterval(async () => {
  const history = await api.rpc.asf.getSlashingHistory(100);

  if (history.length > lastSeenCount) {
    const newEvents = history.slice(lastSeenCount);
    newEvents.forEach(event => {
      console.error(`🚨 SLASHING EVENT 🚨`);
      console.error(`  Validator: ${event.validator_id}`);
      console.error(`  Severity: ${event.severity}`);
      console.error(`  Reason: ${event.reason}`);
      console.error(`  Amount: ${event.amount_slashed}`);
      console.error(`  Excluded: ${event.excluded}`);

      // Send alert to Slack/Discord/etc
      sendAlert(event);
    });
  }

  lastSeenCount = history.length;
}, 10000);
```

---

## Troubleshooting

### RPC Endpoint Not Available

**Symptom:** `Method not found: asf_getFinalityLevel`

**Solution:**
```bash
# Ensure node was started with ASF enabled
./flare-chain-node --dev --enable-asf

# Check logs for RPC registration
tail -f ~/.local/share/flare-chain/chains/dev/network/flarechain.log | grep "Enabling ASF RPC"
```

### Vote Submission Fails

**Symptom:** `VoteSubmissionResult { success: false, message: "Invalid signature" }`

**Solution:**
```rust
// Ensure vote is signed correctly
let message = vote.encode();
let signature = pair.sign(&message);

// Double check validator is in committee
let validators = api.rpc.asf.getValidatorSet().await?;
assert!(validators.iter().any(|v| v.validator_id == my_validator_id));
```

### Telemetry Not Appearing

**Symptom:** No telemetry events in logs

**Solution:**
```bash
# Ensure telemetry is enabled
./flare-chain-node --dev --telemetry-url 'wss://telemetry.polkadot.io/submit 0'

# Check Prometheus metrics
curl http://localhost:9615/metrics | grep asf_
```

---

## Security Best Practices

### Vote Submission
1. ✅ **Always verify signatures** before accepting votes
2. ✅ **Check validator is in active committee** before processing
3. ✅ **Validate epoch** to prevent replay attacks
4. ✅ **Rate limit** vote submissions to prevent DoS

### RPC Access
1. ✅ **Use CORS restrictions** in production
2. ✅ **Implement authentication** for sensitive endpoints
3. ✅ **Rate limit queries** to prevent abuse
4. ✅ **Monitor for suspicious activity**

---

## Performance Tuning

### RPC Query Optimization
```rust
// Cache committee data (refresh every epoch)
let committee_cache = Arc::new(RwLock::new(None));

// In RPC handler
if let Some(cached) = committee_cache.read().unwrap().as_ref() {
    if cached.epoch == current_epoch {
        return Ok(cached.clone());
    }
}

// Query runtime and update cache
let committee = query_runtime_committee()?;
*committee_cache.write().unwrap() = Some(committee.clone());
```

### Telemetry Buffering
```rust
// Buffer telemetry events and emit in batches
let mut event_buffer = Vec::new();

// Collect events
event_buffer.push(event);

// Emit every 100 events or every 10 seconds
if event_buffer.len() >= 100 || time_since_last_emit > 10_000 {
    for event in event_buffer.drain(..) {
        emit_telemetry(event);
    }
}
```

---

## References

- **ASF Algorithm:** `/Users/macbook/Desktop/etrid/09-consensus/asf-algorithm/`
- **FlareChain Node:** `/Users/macbook/Desktop/etrid/05-multichain/flare-chain/node/`
- **Implementation Report:** `/Users/macbook/Desktop/etrid/09-consensus/asf-algorithm/ASF_SERVICE_IMPLEMENTATION_REPORT.md`
- **Ivory Papers:** (link to consensus specification)

---

**Last Updated:** 2025-11-15
**Maintainer:** ËTRID Foundation
