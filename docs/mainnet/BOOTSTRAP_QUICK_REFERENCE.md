# Ëtrid FlareChain Mainnet - Bootstrap Quick Reference

**Last Updated:** November 2, 2025
**Use This:** For deploying validators 5-21

---

## Prerequisites Checklist

```bash
# 1. VM with public IP
# 2. Ubuntu 22.04+
# 3. 2+ CPU, 4GB+ RAM, 50GB+ storage
# 4. Firewall: Port 30333 open to public
# 5. Binary installed: /usr/local/bin/etrid-validator
```

---

## Step 1: Get Your Validator Information

From `/Users/macbook/Desktop/etrid/secrets/validator-keys/generated-keys/COMPLETE_VALIDATOR_NETWORK_MAP.md`:

```
Your Validator Number: _____
Your Validator Name: _____
Your Mnemonic Phrase: _____
Your Node Key: 000000000000000000000000000000000000000000000000000000000000000___
Your VM Public IP: ___.___.___.___
```

---

## Step 2: Copy Chainspec to VM

```bash
# On VM
cd ~
wget https://raw.githubusercontent.com/EojEdred/Etrid/main/docs/mainnet/chainspec-mainnet-raw.json
mv chainspec-mainnet-raw.json /home/azureuser/chainspec-mainnet-raw.json
```

Or copy from local:
```bash
# From your local machine
scp ~/Desktop/etrid/docs/mainnet/chainspec-mainnet-raw.json azureuser@YOUR_VM_IP:/home/azureuser/
```

---

## Step 3: Purge Old Data

```bash
sudo systemctl stop etrid-validator 2>/dev/null
sudo pkill -9 etrid-validator 2>/dev/null
rm -rf ~/.local/share/etrid-validator
```

---

## Step 4: Insert Session Keys

**IMPORTANT:** Replace `YOUR_MNEMONIC_HERE` with your actual mnemonic phrase (in quotes).

```bash
# Insert AURA key (sr25519)
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type aura --scheme sr25519 \
  --suri "YOUR_MNEMONIC_HERE"

# Insert GRANDPA key (ed25519)
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type gran --scheme ed25519 \
  --suri "YOUR_MNEMONIC_HERE"

# Insert ASF key (sr25519)
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type asfk --scheme sr25519 \
  --suri "YOUR_MNEMONIC_HERE"
```

**Verify keys inserted:**
```bash
ls -la ~/.local/share/etrid-validator/chains/*/keystore/
# Should show 3 files
```

---

## Step 5: Create Systemd Service

**CRITICAL:** Replace these 3 placeholders:
1. `VALIDATOR_NAME` - Your validator name (e.g., "validator-05")
2. `YOUR_NODE_KEY` - Your 64-character node key
3. `YOUR_PUBLIC_IP` - Your VM's public IP

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
  --node-key YOUR_NODE_KEY \
  --public-addr /ip4/YOUR_PUBLIC_IP/tcp/30333 \
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

**Example for Validator 5:**
```bash
# If validator-05 with IP 20.123.45.67
sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<'EOF'
[Unit]
Description=Ëtrid Validator - validator-05 - MAINNET
After=network.target

[Service]
Type=simple
User=azureuser
WorkingDirectory=/home/azureuser
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "validator-05-Mainnet" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --base-path /home/azureuser/.local/share/etrid-validator \
  --node-key 0000000000000000000000000000000000000000000000000000000000000005 \
  --public-addr /ip4/20.123.45.67/tcp/30333 \
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

---

## Step 6: Start Validator

```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator
```

---

## Step 7: Verify It's Working

```bash
# Wait for startup
sleep 10

# Check service status
sudo systemctl status etrid-validator

# Check logs
sudo journalctl -u etrid-validator -n 30 --no-pager
```

**What to look for:**
```
✅ Chain specification: Ëtrid FlareChain Mainnet
✅ finalized #0 (0x2fb6…84c4)
✅ 💤 Idle (3+ peers)
✅ Imported #XXXX
```

**Watch live:**
```bash
sudo journalctl -u etrid-validator -f
```

---

## Verification Checklist

After 60 seconds, verify:

```bash
# 1. Service is running
sudo systemctl is-active etrid-validator
# Should output: active

# 2. Check chain name
sudo journalctl -u etrid-validator --no-pager | grep "Chain specification"
# Should output: Chain specification: Ëtrid FlareChain Mainnet

# 3. Check genesis hash
sudo journalctl -u etrid-validator --no-pager | grep "finalized #0"
# Should contain: 0x2fb6…84c4

# 4. Check peer count
sudo journalctl -u etrid-validator -n 5 --no-pager | grep "Idle"
# Should show: (3+ peers)

# 5. Check block import
sudo journalctl -u etrid-validator -n 10 --no-pager | grep "Imported"
# Should show recent block imports
```

---

## Troubleshooting

### Problem: Service fails to start

```bash
# Check error
sudo journalctl -u etrid-validator -n 50 --no-pager

# Common causes:
# - Binary not found
# - Chainspec file not found
# - Keys not inserted
# - Port 30333 blocked
```

### Problem: Only 1 peer connected

```bash
# Check if --public-addr is set
sudo systemctl cat etrid-validator | grep public-addr

# If missing, add it to service file (Step 5)
```

### Problem: Wrong chain (Local Testnet)

```bash
# Verify chainspec
head -5 /home/azureuser/chainspec-mainnet-raw.json

# Should show: "name": "Ëtrid FlareChain Mainnet"
# If not, re-download chainspec (Step 2)
```

### Problem: Not importing blocks

```bash
# Check firewall
sudo ufw status
# Port 30333 must be open

# Check bootnode connection
sudo journalctl -u etrid-validator -n 100 --no-pager | grep -i "boot\|peer"
```

---

## Quick Service Commands

```bash
# View logs live
sudo journalctl -u etrid-validator -f

# View last 100 lines
sudo journalctl -u etrid-validator -n 100 --no-pager

# Restart validator
sudo systemctl restart etrid-validator

# Stop validator
sudo systemctl stop etrid-validator

# Start validator
sudo systemctl start etrid-validator

# Check status
sudo systemctl status etrid-validator
```

---

## Critical Configuration Values

**Bootnode (VM1):**
```
/ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

**Genesis Hash (verify this):**
```
0x2fb6d755006726bd6898f9334f31876b65ab5395436309f7ecf90540e73084c4
```

**Chain Name (verify this):**
```
Ëtrid FlareChain Mainnet
```

**Required Flags:**
```
--validator
--name "YourName-Mainnet"
--chain=/home/azureuser/chainspec-mainnet-raw.json
--base-path /home/azureuser/.local/share/etrid-validator
--node-key YOUR_NODE_KEY
--public-addr /ip4/YOUR_IP/tcp/30333  ← CRITICAL
--bootnodes /ip4/20.69.26.209/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
--port 30333
--rpc-port 9944
--rpc-cors all
```

---

## Node Key Reference

```
Validator 5:  0000000000000000000000000000000000000000000000000000000000000005
Validator 6:  0000000000000000000000000000000000000000000000000000000000000006
Validator 7:  0000000000000000000000000000000000000000000000000000000000000007
Validator 8:  0000000000000000000000000000000000000000000000000000000000000008
Validator 9:  0000000000000000000000000000000000000000000000000000000000000009
Validator 10: 000000000000000000000000000000000000000000000000000000000000000a
Validator 11: 000000000000000000000000000000000000000000000000000000000000000b
Validator 12: 000000000000000000000000000000000000000000000000000000000000000c
Validator 13: 000000000000000000000000000000000000000000000000000000000000000d
Validator 14: 000000000000000000000000000000000000000000000000000000000000000e
Validator 15: 000000000000000000000000000000000000000000000000000000000000000f
Validator 16: 0000000000000000000000000000000000000000000000000000000000000010
Validator 17: 0000000000000000000000000000000000000000000000000000000000000011
Validator 18: 0000000000000000000000000000000000000000000000000000000000000012
Validator 19: 0000000000000000000000000000000000000000000000000000000000000013
Validator 20: 0000000000000000000000000000000000000000000000000000000000000014
Validator 21: 0000000000000000000000000000000000000000000000000000000000000015
```

---

## Copy-Paste Template

Use this for quick deployment:

```bash
# === CONFIGURATION - EDIT THESE ===
VALIDATOR_NAME="validator-XX"
NODE_KEY="000000000000000000000000000000000000000000000000000000000000000X"
PUBLIC_IP="XXX.XXX.XXX.XXX"
MNEMONIC="your twelve word mnemonic phrase here"

# === BOOTSTRAP COMMANDS - RUN AS-IS ===
# Stop any existing validator
sudo systemctl stop etrid-validator 2>/dev/null
sudo pkill -9 etrid-validator 2>/dev/null
rm -rf ~/.local/share/etrid-validator

# Insert keys
/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type aura --scheme sr25519 \
  --suri "$MNEMONIC"

/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type gran --scheme ed25519 \
  --suri "$MNEMONIC"

/usr/local/bin/etrid-validator key insert \
  --base-path ~/.local/share/etrid-validator \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --key-type asfk --scheme sr25519 \
  --suri "$MNEMONIC"

# Create service
sudo tee /etc/systemd/system/etrid-validator.service > /dev/null <<EOF
[Unit]
Description=Ëtrid Validator - $VALIDATOR_NAME - MAINNET
After=network.target

[Service]
Type=simple
User=azureuser
WorkingDirectory=/home/azureuser
ExecStart=/usr/local/bin/etrid-validator \
  --validator \
  --name "$VALIDATOR_NAME-Mainnet" \
  --chain=/home/azureuser/chainspec-mainnet-raw.json \
  --base-path /home/azureuser/.local/share/etrid-validator \
  --node-key $NODE_KEY \
  --public-addr /ip4/$PUBLIC_IP/tcp/30333 \
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

# Start validator
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator

# Wait and verify
sleep 15
echo "=== Service Status ==="
sudo systemctl status etrid-validator | head -10

echo ""
echo "=== Recent Logs ==="
sudo journalctl -u etrid-validator -n 20 --no-pager | grep -E "(Chain|peers|Imported|Mainnet)"
```

---

**Status:** ✅ Tested and verified on 4 live validators
**Last Updated:** November 2, 2025
