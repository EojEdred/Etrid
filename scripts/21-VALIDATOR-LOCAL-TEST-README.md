# Local 21-Validator Test Network

This directory contains scripts to run a full 21-validator network locally for testing GRANDPA finality and other consensus features before deploying to production.

## Quick Start

```bash
# 1. Build the project
cd 05-multichain/flare-chain
cargo build --release

# 2. Generate the 21-validator chain spec preset
cd ../..
python3 scripts/generate-21-validator-preset.py

# 3. Build the chain spec
cd 05-multichain/flare-chain
./target/release/etrid build-spec \
  --chain runtime/presets/test_21validator.json \
  > chainspec-21validator.json

# Convert to raw format
./target/release/etrid build-spec \
  --chain chainspec-21validator.json \
  --raw > chainspec-21validator-raw.json

# 4. Insert keys for all validators
cd ../..
chmod +x scripts/run-21-validators.sh
./scripts/run-21-validators.sh insert-keys

# 5. Start all 21 validators
./scripts/run-21-validators.sh start

# 6. Check status
./scripts/run-21-validators.sh status

# 7. View logs for a specific validator (e.g., validator 0 = Alice)
./scripts/run-21-validators.sh logs 0

# 8. Stop all validators
./scripts/run-21-validators.sh stop

# 9. Clean all data
./scripts/run-21-validators.sh clean
```

## Port Configuration

Each validator runs on different ports:

| Validator | P2P Port | RPC Port | WS Port |
|-----------|----------|----------|---------|
| 0 (Alice) | 30333    | 9944     | 9933    |
| 1 (Bob)   | 30334    | 9945     | 9934    |
| 2 (Charlie) | 30335  | 9946     | 9935    |
| ...       | ...      | ...      | ...     |
| 20 (Validator9) | 30353 | 9964  | 9953    |

## Testing GRANDPA Finality

To verify GRANDPA finality is working:

```bash
# Watch validator 0's logs for GRANDPA activity
./scripts/run-21-validators.sh logs 0 | grep -E "(finalized|GRANDPA|Completed round)"

# Check finality progression in multiple terminals
# Terminal 1:
./scripts/run-21-validators.sh logs 0 | grep "finalized"

# Terminal 2:
./scripts/run-21-validators.sh logs 1 | grep "finalized"

# Terminal 3:
./scripts/run-21-validators.sh logs 2 | grep "finalized"
```

You should see:
1. **GRANDPA rounds completing** - e.g., `Completed round 1535`
2. **Finality progressing** - e.g., `finalized #334` → `finalized #335` → `finalized #336`
3. **No txpool crashes** - validators should run indefinitely without crashing

## Validator Names

The 21 validators use these names (matching Substrate test accounts):

1. Alice
2. Bob
3. Charlie
4. Dave
5. Eve
6. Ferdie
7. Alice//stash
8. Bob//stash
9. Charlie//stash
10. Dave//stash
11. Eve//stash
12. Ferdie//stash
13-21. Validator1 through Validator9

## Directory Structure

```
/tmp/etrid-21-validators/
├── validator-0/          # Alice's data
├── validator-1/          # Bob's data
├── ...
├── validator-20/         # Validator9's data
├── logs/
│   ├── validator-0.log
│   ├── validator-1.log
│   └── ...
└── pids/
    ├── validator-0.pid
    ├── validator-1.pid
    └── ...
```

## Troubleshooting

### Keys not inserted
```bash
# Re-insert keys for all validators
./scripts/run-21-validators.sh stop
./scripts/run-21-validators.sh insert-keys
./scripts/run-21-validators.sh start
```

### Validators not connecting
- Check that all validators started successfully
- Verify no port conflicts
- Look for peer discovery messages in logs

### GRANDPA not finalizing
- Verify all validators have GRANDPA keys inserted
- Check that at least 15 validators (2/3 of 21) are running
- Look for "GRANDPA voter starting" messages in logs

### High resource usage
- Reduce the number of validators (edit VALIDATORS array in run-21-validators.sh)
- Use `--log info` instead of debug logging
- Increase slot duration in the chain spec

## Monitoring Tools

```bash
# Show real-time status with watch
watch -n 2 './scripts/run-21-validators.sh status'

# Monitor finality across all validators
for i in {0..20}; do
  echo "=== Validator $i ==="
  tail -5 /tmp/etrid-21-validators/logs/validator-$i.log | grep finalized
done

# Check GRANDPA rounds
for i in {0..20}; do
  echo "=== Validator $i ==="
  tail -10 /tmp/etrid-21-validators/logs/validator-$i.log | grep "Completed round"
done
```

## Cleaning Up

```bash
# Stop and remove all data
./scripts/run-21-validators.sh clean

# Or manually:
pkill -f "etrid.*--validator"
rm -rf /tmp/etrid-21-validators
```

## Next Steps

Once local testing confirms GRANDPA finality works with 21 validators:

1. Deploy the fix to Azure production validators
2. Insert GRANDPA keys on all production validators
3. Verify finality progresses on production network
4. Monitor for txpool crashes (should not occur)

## See Also

- `/tmp/GRANDPA_FIX_SUMMARY.md` - Details of the GRANDPA finality fix
- `05-multichain/flare-chain/node/src/asf_service.rs:438` - Fork choice strategy fix
