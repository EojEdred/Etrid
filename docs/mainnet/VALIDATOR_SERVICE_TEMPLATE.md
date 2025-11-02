# Ëtrid FlareChain Mainnet - Validator Service Configuration Template

**Last Updated:** November 2, 2025
**Status:** ✅ Production Tested - All 4 Genesis Validators Running
**Genesis Hash:** `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`

---

## Critical Configuration Requirements

This document captures the **exact working configuration** from the 4 live mainnet validators. These configurations have been tested and verified to work correctly.

### Critical Fixes Implemented:

1. ✅ **`--public-addr` flag** - REQUIRED for peer discovery (was missing, caused connectivity issues)
2. ✅ **Correct chainspec** - `chainspec-mainnet-raw.json` (not testnet chainspec)
3. ✅ **Correct bootnode peer ID** - `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`
4. ✅ **Unique node-key** - Each validator needs unique node-key
5. ✅ **Mainnet binary** - `etrid-validator` (Azure VMs) or `etrid` (Oracle Cloud)

---

## Mainnet Network Configuration

### Bootnode Information (VM1 - EojEdred)
```
IP: 20.69.26.209
Port: 30333
Peer ID: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
Multiaddr: /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

### Chainspec File
- **Location:** `/home/azureuser/chainspec-mainnet-raw.json` (or `/home/ubuntu/`)
- **Chain Name:** "Ëtrid FlareChain Mainnet"
- **Chain ID:** `flarechain_mainnet`
- **Genesis Hash:** `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`
- **Copy from:** This repo at `docs/mainnet/chainspec-mainnet-raw.json`

### Port Requirements
- **30333** - P2P networking (must be public)
- **9944** - RPC (optional, can be restricted)
- **9615** - Prometheus metrics (optional)

---

## Systemd Service Template (Azure VMs)

### Template for Validators 2-21 (Azure Standard)

**File:** `/etc/systemd/system/etrid-validator.service`

```ini
[Unit]
Description=Ëtrid Validator - VALIDATOR_NAME - MAINNET
After=network.target

[Service]
Type=simple
User=azureuser
WorkingDirectory=/home/azureuser
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "VALIDATOR_NAME-Mainnet" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --base-path /home/azureuser/.local/share/etrid-validator \
  --node-key NODE_KEY_HERE \
  --public-addr /ip4/PUBLIC_IP_HERE/tcp/30333 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
  --rpc-cors all \
  --rpc-port 9944 \
  --port 30333
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

### Template for VM1 (Bootnode + Validator)

**File:** `/etc/systemd/system/etrid-validator.service`

```ini
[Unit]
Description=Ëtrid Validator - EojEdred (Founder) - MAINNET BOOTNODE
After=network.target

[Service]
Type=simple
User=azureuser
WorkingDirectory=/home/azureuser
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "EojEdred-Validator-Bootnode" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --base-path /home/azureuser/.local/share/etrid-validator \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --public-addr /ip4/20.69.26.209/tcp/30333 \
  --unsafe-rpc-external \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --rpc-port 9944 \
  --port 30333
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

**Note:** VM1 does NOT use `--bootnodes` flag (it IS the bootnode)

---

## Working Validator Configurations

### Validator 1: EojEdred (VM1 - Azure) - BOOTNODE ✅

```bash
IP: 20.69.26.209
Name: EojEdred-Validator-Bootnode
Node Key: 0000000000000000000000000000000000000000000000000000000000000001
Peer ID: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
Binary: /usr/local/bin/etrid-validator
User: azureuser
Base Path: /home/azureuser/.local/share/etrid-validator
Status: ✅ Running, authoring blocks, 20 peers
```

**Critical Flags:**
- `--public-addr /ip4/20.69.26.209/tcp/30333` ← REQUIRED
- NO `--bootnodes` flag (this IS the bootnode)
- `--unsafe-rpc-external` (for RPC access)

---

### Validator 2: Gizzi Foundation (Oracle Cloud) ✅

```bash
IP: 64.181.215.19
Name: Gizzi-Foundation-Validator-Mainnet
Node Key: 0000000000000000000000000000000000000000000000000000000000000002
Binary: /usr/local/bin/etrid
User: ubuntu
Base Path: /var/lib/etrid
Status: ✅ Running, 3 peers
```

**Service File:**
```ini
[Unit]
Description=Ëtrid FlareChain Validator Node (Gizzi) - MAINNET
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/var/lib/etrid
ExecStart=/usr/local/bin/etrid \
  --validator \
  --name "Gizzi-Foundation-Validator-Mainnet" \
  --base-path /var/lib/etrid \
  --chain /home/ubuntu/chainspec-mainnet-raw.json \
  --node-key 0000000000000000000000000000000000000000000000000000000000000002 \
  --public-addr /ip4/64.181.215.19/tcp/30333 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
  --port 30333 \
  --rpc-port 9944 \
  --unsafe-rpc-external \
  --rpc-cors all \
  --rpc-methods=Unsafe
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Note:** Oracle Cloud uses `ubuntu` user and `/var/lib/etrid` base path

---

### Validator 3: governance-dev01 (VM2 - Azure) ✅

```bash
IP: 20.186.91.207
Name: governance-dev01-Validator-Mainnet
Node Key: 0000000000000000000000000000000000000000000000000000000000000003
Binary: /usr/local/bin/etrid-validator
User: azureuser
Base Path: /home/azureuser/.local/share/etrid-validator
Status: ✅ Running, 20 peers
```

**Uses standard Azure template above with:**
- `VALIDATOR_NAME` = "governance-dev01"
- `NODE_KEY_HERE` = "0000000000000000000000000000000000000000000000000000000000000003"
- `PUBLIC_IP_HERE` = "20.186.91.207"

---

### Validator 4: security-dev01 (VM3 - Azure) ✅

```bash
IP: 52.252.142.146
Name: security-dev01-Validator-Mainnet
Node Key: 0000000000000000000000000000000000000000000000000000000000000004
Binary: /usr/local/bin/etrid-validator
User: azureuser
Base Path: /home/azureuser/.local/share/etrid-validator
Status: ✅ Running, 20 peers
```

**Uses standard Azure template above with:**
- `VALIDATOR_NAME` = "security-dev01"
- `NODE_KEY_HERE` = "0000000000000000000000000000000000000000000000000000000000000004"
- `PUBLIC_IP_HERE` = "52.252.142.146"

---

## Bootstrap Steps for New Validator (Validators 5-21)

### Prerequisites
1. Ubuntu VM with public IP
2. Firewall open on port 30333
3. Binary installed at `/usr/local/bin/etrid-validator`
4. Chainspec file at `/home/azureuser/chainspec-mainnet-raw.json`
5. Session keys ready (from validator-keys directory)

### Step 1: Purge Old Data (if exists)
```bash
sudo systemctl stop etrid-validator 2>/dev/null
sudo pkill -9 etrid-validator 2>/dev/null
rm -rf ~/.local/share/etrid-validator
```

### Step 2: Insert Session Keys
```bash
# Replace MNEMONIC_HERE with your validator's mnemonic from the keys file

# Insert AURA key (sr25519)
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type aura --scheme sr25519 \
  --suri "MNEMONIC_HERE"

# Insert GRANDPA key (ed25519)
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type gran --scheme ed25519 \
  --suri "MNEMONIC_HERE"

# Insert ASF key (sr25519)
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type asfk --scheme sr25519 \
  --suri "MNEMONIC_HERE"
```

### Step 3: Create Systemd Service

**CRITICAL:** Replace these placeholders:
- `VALIDATOR_NAME` - Your validator's name (e.g., "validator-05")
- `NODE_KEY_HERE` - Your unique node key (increment: ...0005, ...0006, etc.)
- `PUBLIC_IP_HERE` - Your VM's public IP address

```bash
sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<'EOF'
[Unit]
Description=Ëtrid Validator - VALIDATOR_NAME - MAINNET
After=network.target

[Service]
Type=simple
User=azureuser
WorkingDirectory=/home/azureuser
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "VALIDATOR_NAME-Mainnet" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --base-path /home/azureuser/.local/share/etrid-validator \
  --node-key NODE_KEY_HERE \
  --public-addr /ip4/PUBLIC_IP_HERE/tcp/30333 \
  --bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
  --rpc-cors all \
  --rpc-port 9944 \
  --port 30333
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
```

### Step 4: Start Validator
```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator
```

### Step 5: Verify
```bash
# Wait 10 seconds for startup
sleep 10

# Check logs
sudo journalctl -u etrid-validator -n 30 --no-pager | grep -E "(Chain specification|Mainnet|best:|peers)"
```

**Expected Output:**
```
Chain specification: Ëtrid FlareChain Mainnet
💤 Idle (3+ peers), best: #XXXX
```

---

## Flag Reference

### Required Flags for All Validators

| Flag | Value | Purpose |
|------|-------|---------|
| `--validator` | - | Enable validator mode |
| `--name` | "YourName-Mainnet" | Node name (shows in telemetry) |
| `--chain` | `/path/to/chainspec-mainnet-raw.json` | Mainnet chainspec |
| `--base-path` | `/home/azureuser/.local/share/etrid-validator` | Data directory |
| `--node-key` | `000...00X` (64 hex chars) | Unique network identity |
| `--public-addr` | `/ip4/YOUR_IP/tcp/30333` | **CRITICAL** - Advertises your IP for peers |
| `--bootnodes` | `/ip4/20.69.26.209/tcp/30333/p2p/...` | Connect to bootnode (NOT for VM1) |
| `--port` | `30333` | P2P port |
| `--rpc-port` | `9944` | RPC port |
| `--rpc-cors` | `all` | Allow RPC connections |

### Optional Flags

| Flag | Purpose | Use When |
|------|---------|----------|
| `--unsafe-rpc-external` | Allow external RPC access | Need public RPC |
| `--rpc-methods Unsafe` | Allow all RPC methods | Development/debugging |
| `--prometheus-external` | Expose Prometheus metrics | Monitoring setup |
| `--prometheus-port` | Custom metrics port | Default 9615 conflicts |

---

## Common Issues & Solutions

### Issue 1: Only 1 Peer Connected ❌

**Symptom:**
```
💤 Idle (1 peers), best: #1234
```

**Root Cause:** Missing `--public-addr` flag

**Solution:** Add to service file:
```bash
--public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333
```

Then restart:
```bash
sudo systemctl daemon-reload
sudo systemctl restart etrid-validator
```

---

### Issue 2: Chain Shows "Local Testnet" ❌

**Symptom:**
```
Chain specification: Ëtrid FlareChain Local Testnet
```

**Root Cause:** Using wrong chainspec file

**Solution:**
1. Copy correct chainspec from repo: `docs/mainnet/chainspec-mainnet-raw.json`
2. Update service file to point to correct chainspec
3. Restart validator

---

### Issue 3: Wrong Genesis Hash ❌

**Symptom:**
```
finalized #0 (0xABCD...1234)  # Wrong hash
```

**Root Cause:** Using wrong chainspec

**Solution:**
1. Verify genesis hash should be: `0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4`
2. If different, you're on wrong chain
3. Stop validator, purge data, use correct chainspec

---

### Issue 4: Wrong Bootnode Peer ID ❌

**Root Cause:** Old/incorrect bootnode multiaddr

**Solution:** Ensure bootnode line is exactly:
```
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

---

## Validation Checklist

Before starting your validator:

- [ ] Chainspec file is `chainspec-mainnet-raw.json` (NOT testnet)
- [ ] Service includes `--public-addr /ip4/YOUR_IP/tcp/30333`
- [ ] Bootnode peer ID is `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`
- [ ] Node key is unique (not same as another validator)
- [ ] Port 30333 is open in firewall
- [ ] Session keys inserted (3 keys: aura, gran, asfk)
- [ ] Binary is correct version (`etrid-validator --version`)

After starting:

- [ ] Service is active: `sudo systemctl status etrid-validator`
- [ ] Shows "Mainnet" in logs (not "Local Testnet")
- [ ] Genesis hash: `0x2fb6...84c4`
- [ ] Connected to 3+ peers within 30 seconds
- [ ] Importing blocks: `best: #XXXX`

---

## Node Key Assignment

Validators 1-21 use sequential node keys:

```
Validator 1 (EojEdred):    0000000000000000000000000000000000000000000000000000000000000001
Validator 2 (Gizzi):       0000000000000000000000000000000000000000000000000000000000000002
Validator 3 (governance):  0000000000000000000000000000000000000000000000000000000000000003
Validator 4 (security):    0000000000000000000000000000000000000000000000000000000000000004
Validator 5:               0000000000000000000000000000000000000000000000000000000000000005
...
Validator 21:              0000000000000000000000000000000000000000000000000000000000000015 (hex 15 = decimal 21)
```

**Important:** Each validator MUST have a unique node key.

---

## Files in This Directory

| File | Purpose |
|------|---------|
| `VALIDATOR_SERVICE_TEMPLATE.md` | This file - Service configurations |
| `chainspec-mainnet-raw.json` | Working mainnet chainspec |
| `VALIDATOR_QUICKSTART.md` | Quick deployment guide |
| `VALIDATOR_FIREWALL_RULES.md` | Firewall setup |

---

## Production Status

**Current Mainnet Status (Nov 2, 2025):**
- ✅ 4 validators running
- ✅ All on correct mainnet chain
- ✅ All with 3-20 peers connected
- ✅ Producing blocks at #1800+
- ✅ Consensus working (ASF PPFA + GRANDPA)

**Remaining Work:**
- Add validators 5-21 (17 more validators)
- Session keys are pre-generated and ready
- Use templates from this document

---

**Last Verified:** November 2, 2025 at 05:35 UTC
**Verified By:** Claude (Eoj's AI assistant)
**Network:** Ëtrid FlareChain Mainnet
**Status:** ✅ Production Ready
