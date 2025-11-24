# PBC Collator Quick Reference

## Currently Running Collators

### Stop All Collators
```bash
pkill -f "pbc-collator"
```

### Stop Individual Collators
```bash
kill 33131  # BTC PBC
kill 33164  # SOL PBC
kill 33208  # BNB PBC
```

## Restart Commands

### BTC PBC
```bash
cd ~/Desktop/etrid/pbc-test-deployment/btc-pbc
nohup ~/Desktop/etrid-binaries/btc-pbc-collator \
  --name "BTC-PBC-Test-Node" \
  --base-path ./data \
  --chain dev \
  --port 30334 \
  --rpc-port 9944 \
  --rpc-cors all \
  --rpc-methods=Unsafe \
  --unsafe-rpc-external \
  --dev \
  > ./logs/btc-pbc.log 2>&1 &
```

### SOL PBC
```bash
cd ~/Desktop/etrid/pbc-test-deployment/sol-pbc
nohup ~/Desktop/etrid-binaries/sol-pbc-collator \
  --name "SOL-PBC-Test-Node" \
  --base-path ./data \
  --chain dev \
  --port 30335 \
  --rpc-port 9945 \
  --rpc-cors all \
  --rpc-methods=Unsafe \
  --unsafe-rpc-external \
  --dev \
  > ./logs/sol-pbc.log 2>&1 &
```

### BNB PBC
```bash
cd ~/Desktop/etrid/pbc-test-deployment/bnb-pbc
nohup ~/Desktop/etrid-binaries/bnb-pbc-collator \
  --name "BNB-PBC-Test-Node" \
  --base-path ./data \
  --chain dev \
  --port 30336 \
  --rpc-port 9946 \
  --rpc-cors all \
  --rpc-methods=Unsafe \
  --unsafe-rpc-external \
  --dev \
  > ./logs/bnb-pbc.log 2>&1 &
```

## Monitor Logs

### Tail Logs (Live)
```bash
tail -f ~/Desktop/etrid/pbc-test-deployment/btc-pbc/logs/btc-pbc.log
tail -f ~/Desktop/etrid/pbc-test-deployment/sol-pbc/logs/sol-pbc.log
tail -f ~/Desktop/etrid/pbc-test-deployment/bnb-pbc/logs/bnb-pbc.log
```

### View Last 50 Lines
```bash
tail -50 ~/Desktop/etrid/pbc-test-deployment/btc-pbc/logs/btc-pbc.log
tail -50 ~/Desktop/etrid/pbc-test-deployment/sol-pbc/logs/sol-pbc.log
tail -50 ~/Desktop/etrid/pbc-test-deployment/bnb-pbc/logs/bnb-pbc.log
```

### Search for Errors
```bash
grep -i error ~/Desktop/etrid/pbc-test-deployment/*/logs/*.log
```

## Check Status

### Running Processes
```bash
ps aux | grep pbc-collator | grep -v grep
```

### Port Usage
```bash
lsof -i :30334 -i :30335 -i :30336  # P2P ports
lsof -i :9944 -i :9945 -i :9946      # RPC ports
```

## Clean Data (Fresh Start)

### Remove All Chain Data
```bash
rm -rf ~/Desktop/etrid/pbc-test-deployment/*/data/
```

### Remove Logs
```bash
rm -f ~/Desktop/etrid/pbc-test-deployment/*/logs/*.log
```

## Peer Information

- **BTC PBC:** 12D3KooWRdCW6EW58wcuaMPPTtRnwuyVqWdmXPG6SbGd5E2iSBKP
- **SOL PBC:** 12D3KooWDMGKRFN4FiKMGNK4mTAyyK6UWvCG5LwWVTz2FjK6rfGh
- **BNB PBC:** 12D3KooWAyz9jRrYgnHCgK7i5te8TBvukmZMAa3v5fS2tvpcURQj

## Endpoints (when RPC is active)

- **BTC PBC:** http://localhost:9944
- **SOL PBC:** http://localhost:9945
- **BNB PBC:** http://localhost:9946

## Testing with Other PBC Collators

Available ARM64 collators in ~/Desktop/etrid-binaries/:
- ada-pbc-collator
- doge-pbc-collator
- edsc-pbc-collator
- link-pbc-collator
- matic-pbc-collator
- sc-usdt-pbc-collator
- trx-pbc-collator
- xlm-pbc-collator
- xrp-pbc-collator

To deploy another collator, use the same pattern with different ports:
- P2P: 30337, 30338, etc.
- RPC: 9947, 9948, etc.
