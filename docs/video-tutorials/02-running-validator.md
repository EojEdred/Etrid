# Video Tutorial 02: Running a Validator on Etrid

**Duration:** 10 minutes
**Target Audience:** Technical users, validators, node operators
**Prerequisites:** Linux/macOS terminal experience, basic blockchain knowledge

---

## Script Overview

This tutorial guides users through the complete process of setting up and running an Etrid validator node, from hardware requirements to on-chain registration and monitoring.

---

## Time Markers & Script

### 00:00 - 00:45 | Introduction & Validator Role

**NARRATION:**
"Welcome to part two of the Etrid tutorial series. Today, you're going to learn how to become a validator - one of the most important roles in the Etrid ecosystem.

What does a validator do? Validators secure the Etrid FlareChain by producing new blocks, validating transactions, and participating in our Adaptive Stake Finality consensus. In return, you earn block rewards and transaction fees. But it's not passive income - you're responsible for maintaining high uptime, securing your node, and acting honestly. Poor performance or malicious behavior can result in slashing - losing a portion of your stake.

Is it worth it? Validators can earn 10-20% annual returns on their staked ETR, but you'll need technical skills, dedicated hardware, and a stake of at least 10,000 ETR to get started. If that sounds like you, let's begin!"

**VISUAL CUES:**
- Etrid network visualization with validator nodes highlighted
- Animation: Validator producing block → Other validators finalizing
- ASF consensus diagram: Stake + Coinage = Voting Power
- Rewards dashboard showing APY: 10-20%
- Slashing warning graphic (skull icon, -10% stake)
- Requirements checklist overlay:
  - ✅ Technical skills
  - ✅ Dedicated server
  - ✅ 10,000 ETR minimum stake

**KEY POINTS TO EMPHASIZE:**
- Validators are network security backbone
- Earn rewards but have responsibilities
- Slashing penalty for misbehavior or downtime
- 10,000 ETR minimum stake requirement
- 10-20% APY potential
- Technical knowledge required

**COMMON MISTAKES TO MENTION:**
- "Don't confuse validators (FlareChain) with collators (PBCs) - different roles"
- "This is NOT passive income - requires active monitoring"

---

### 00:45 - 02:00 | Hardware and Software Requirements

**NARRATION:**
"Let's talk hardware. Etrid validators need reliable, performant servers. Here's what you'll need:

A modern CPU - 4 cores minimum, 8 cores recommended. Intel Xeon or AMD EPYC are popular choices. RAM: 16 gigabytes minimum, but 32 gigabytes is better for long-term operation. Storage: at least 500 gigabytes of fast SSD storage - NVMe drives are ideal. The blockchain grows over time, so plan for expansion.

Network connectivity is critical. You need a stable connection with at least 100 megabits per second bandwidth. Uptime matters - aim for 99.9% availability. Many validators use cloud providers like AWS, DigitalOcean, or Hetzner. A bare metal server costs about $50-$100 per month.

For software, you'll need a Linux server - Ubuntu 22.04 or Debian 12 recommended. We'll also need Rust, Git, and build tools. Don't worry, we'll install everything step-by-step."

**VISUAL CUES:**
- Server rack or data center footage
- Requirements table:
  ```
  Component       Minimum    Recommended
  ─────────────────────────────────────
  CPU             4 cores    8 cores
  RAM             16 GB      32 GB
  Storage         500 GB     1 TB NVMe
  Bandwidth       100 Mbps   1 Gbps
  Uptime          99%        99.9%+
  ```
- Cloud provider logos: AWS, DigitalOcean, Hetzner, Vultr
- Monthly cost estimate: $50-$100
- OS compatibility list:
  - ✅ Ubuntu 22.04 LTS (recommended)
  - ✅ Debian 12
  - ✅ CentOS/RHEL 8+
  - ✅ Arch Linux
- Terminal screenshot showing `uname -a` output

**DEMO STEPS:**
1. Show server specification in cloud provider dashboard
2. Display `lscpu` output (CPU info)
3. Display `free -h` output (RAM info)
4. Display `df -h` output (storage info)
5. Display `speedtest-cli` results (bandwidth)

**CODE TO DISPLAY:**
```bash
# Check system specifications
lscpu | grep -E "Model name|CPU\(s\)"
# Output: Model name: Intel Xeon E5-2686 v4
#         CPU(s): 8

free -h
# Output:              total        used        free
#         Mem:           32Gi       2.1Gi        28Gi

df -h | grep nvme
# Output: /dev/nvme0n1  1.0T   50G   950G   5% /

ping -c 5 8.8.8.8
# Verify network connectivity
```

**KEY POINTS TO EMPHASIZE:**
- Hardware requirements are minimum for stable operation
- SSD storage is mandatory (HDD too slow)
- Network reliability more important than raw speed
- Cloud hosting acceptable, not required to self-host
- Budget $50-100/month for infrastructure
- Linux required (Windows not supported)

**COMMON MISTAKES TO MENTION:**
- "Don't use shared hosting or VPS with fewer than 4 cores"
- "Don't run validators on residential internet with frequent outages"
- "Don't skimp on storage - the chain grows about 10GB/month"

---

### 02:00 - 04:00 | Installation Process

**NARRATION:**
"Time to install the Etrid node. Open your terminal and SSH into your server. First, we'll install Rust, the programming language Etrid is built with.

Run this command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`. Press enter to accept the defaults. This takes about 5 minutes. Once complete, reload your shell configuration.

Now let's install system dependencies. On Ubuntu, run: `sudo apt update && sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev pkg-config`. This installs the compiler and libraries needed to build Etrid.

Next, clone the Etrid repository. Run: `git clone https://github.com/etrid/etrid.git` and then `cd etrid`. You're now in the Etrid source directory.

Here's the big step: building the node. Run: `cargo build --release -p etrid`. This compiles the entire Etrid codebase. On an 8-core server, expect 20-30 minutes. On a 4-core server, maybe 40-60 minutes. You'll see lots of output - that's normal. Grab some coffee!

When it completes, verify the binary exists: `ls -lh target/release/etrid`. You should see an executable about 50-100 megabytes. Perfect! Let's test it: `./target/release/etrid --version`. You should see the Etrid version number. Installation complete!"

**VISUAL CUES:**
- Terminal screen recording (full screen, readable font size)
- Step-by-step command execution
- Progress bars for long operations
- Highlight important output lines
- Split screen: terminal on left, progress checklist on right:
  ```
  Installation Progress:
  ✅ 1. Install Rust
  ✅ 2. Install dependencies
  ✅ 3. Clone repository
  ⏳ 4. Build node (20-40 min)
  ⏸️ 5. Verify installation
  ```
- Coffee cup icon during build wait time
- File size comparison: source (2GB) → binary (80MB)

**DEMO STEPS:**
1. SSH into server: `ssh validator@etrid-node-01`
2. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
3. Source shell: `source $HOME/.cargo/env`
4. Verify Rust: `rustc --version`
5. Install dependencies: `sudo apt update && sudo apt install -y build-essential git clang curl libssl-dev llvm libudev-dev pkg-config`
6. Clone repo: `git clone https://github.com/etrid/etrid.git`
7. Enter directory: `cd etrid`
8. Build node: `cargo build --release -p etrid`
9. (Wait for build - show progress)
10. Verify binary: `ls -lh target/release/etrid`
11. Test version: `./target/release/etrid --version`

**CODE TO DISPLAY:**
```bash
# Step 1: Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
# Output: rustc 1.70.0 (90c541806 2023-05-31)

# Step 2: Install system dependencies (Ubuntu/Debian)
sudo apt update
sudo apt install -y build-essential git clang curl libssl-dev \
  llvm libudev-dev pkg-config

# Step 3: Clone repository
git clone https://github.com/etrid/etrid.git
cd etrid

# Step 4: Build node (20-40 minutes)
cargo build --release -p etrid

# Step 5: Verify installation
ls -lh target/release/etrid
# Output: -rwxr-xr-x 1 user user 83M Oct 22 14:30 etrid

./target/release/etrid --version
# Output: etrid 1.0.0-alpha
```

**KEY POINTS TO EMPHASIZE:**
- Rust installation is one-time setup
- Build time varies by CPU (20-60 minutes typical)
- Binary is self-contained (80-100MB)
- All dependencies compiled from source for security
- No pre-built binaries to ensure reproducibility

**COMMON MISTAKES TO MENTION:**
- "Don't interrupt the build process - let it complete fully"
- "If build fails, run `cargo clean` and try again"
- "Make sure you have at least 20GB free space for build artifacts"
- "Don't skip the `source $HOME/.cargo/env` step"

---

### 04:00 - 06:00 | Configuration and Key Generation

**NARRATION:**
"Now let's configure your validator and generate keys. First, we'll create a dedicated directory for your node data. Run: `mkdir -p ~/.etrid/validator-01`. This keeps everything organized.

Validators need two types of keys: session keys for consensus and a stash key for staking. Let's generate them. Run: `./target/release/etrid key generate --scheme Sr25519`. You'll see output with a secret phrase, public key, and address. This is your STASH key - it holds your stake. Write down the secret phrase immediately and store it somewhere extremely secure. This controls your funds.

Now generate session keys. Run: `./target/release/etrid key generate-session-keys --base-path ~/.etrid/validator-01`. You'll get a long hex string - this is your session key. Copy this - we'll need it for on-chain registration.

Let's insert the session key into your node's keystore. Run this command - I'll show it on screen - it uses the key insert subcommand. You'll paste your session key when prompted.

Finally, we need a node key for peer-to-peer networking. Run: `./target/release/etrid key generate-node-key --file ~/.etrid/validator-01/node-key`. This creates a unique identifier for your validator on the network.

All keys generated! Remember: your stash key secret phrase is your most important asset. Lose it and you lose your stake. Never share it with anyone."

**VISUAL CUES:**
- Terminal recording with syntax highlighting
- Key generation output with sensitive parts blurred
- Animated diagram showing key hierarchy:
  ```
  Validator Keys:
  ├── Stash Key (Sr25519)
  │   ├── Public: 5GrwvaEF5zXb...
  │   └── Secret: [REDACTED]
  ├── Session Keys
  │   ├── BABE: 0x1234abcd...
  │   ├── GRANDPA: 0x5678ef01...
  │   └── ImOnline: 0x9abc2345...
  └── Node Key (Ed25519)
      └── PeerId: 12D3KooW...
  ```
- Security warning overlay: "🔒 NEVER SHARE SECRET PHRASES"
- File system visualization showing ~/.etrid/validator-01/ structure
- Checklist of keys created:
  - ✅ Stash key generated
  - ✅ Session keys generated
  - ✅ Session keys inserted
  - ✅ Node key generated

**DEMO STEPS:**
1. Create data directory: `mkdir -p ~/.etrid/validator-01`
2. Generate stash key: `./target/release/etrid key generate --scheme Sr25519`
3. Save output (blur secret phrase in recording)
4. Generate session keys: `./target/release/etrid key generate-session-keys --base-path ~/.etrid/validator-01`
5. Copy session key output
6. Insert BABE key: `./target/release/etrid key insert --base-path ~/.etrid/validator-01 --chain flare --scheme Sr25519 --suri "<secret>" --key-type babe`
7. Generate node key: `./target/release/etrid key generate-node-key --file ~/.etrid/validator-01/node-key`
8. Verify keystore: `ls ~/.etrid/validator-01/chains/flare/keystore/`

**CODE TO DISPLAY:**
```bash
# Create validator data directory
mkdir -p ~/.etrid/validator-01

# Generate stash key (controls funds)
./target/release/etrid key generate --scheme Sr25519

# Output (EXAMPLE - use your own):
# Secret phrase: [12 words - WRITE THIS DOWN SECURELY]
# Network ID:    substrate
# Secret seed:   0x████████████████████████████████████
# Public key:    0x████████████████████████████████████
# Account ID:    0x████████████████████████████████████
# SS58 Address:  5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

# Generate session keys
./target/release/etrid key generate-session-keys \
  --base-path ~/.etrid/validator-01

# Output: 0x1234567890abcdef... (copy this)

# Insert session keys into keystore
./target/release/etrid key insert \
  --base-path ~/.etrid/validator-01 \
  --chain flare \
  --scheme Sr25519 \
  --suri "your twelve word secret phrase here" \
  --key-type babe

# Repeat for other key types (grandpa, imonline)

# Generate node key for P2P networking
./target/release/etrid key generate-node-key \
  --file ~/.etrid/validator-01/node-key

# Output: 12D3KooWExamplePeerIdHere...

# Verify keystore
ls ~/.etrid/validator-01/chains/flare/keystore/
# Should see multiple key files
```

**KEY POINTS TO EMPHASIZE:**
- Stash key = your money. Protect it like gold.
- Session keys rotate regularly for security
- Node key identifies your validator on P2P network
- All keys stored locally in keystore directory
- Secret phrases are recovery mechanism
- Each validator needs unique keys

**COMMON MISTAKES TO MENTION:**
- "Don't reuse keys from testnets on mainnet"
- "Don't share your stash secret phrase with anyone - no exceptions"
- "Don't lose your backup - write it on paper, store in safe"
- "Don't confuse session keys (can rotate) with stash key (permanent)"

---

### 06:00 - 07:30 | Starting the Validator

**NARRATION:**
"Keys are ready. Let's start your validator! We'll use systemd to manage the validator as a service, so it starts automatically on boot and restarts if it crashes.

First, copy the binary to a system location: `sudo cp target/release/etrid /usr/local/bin/`. Now create a systemd service file. I'll show the contents on screen - pause the video to copy it.

Notice the key parameters: we're running as validator with the FlareChain chain spec, using our custom base path, and exposing RPC on localhost only for security. The validator flag tells the node to participate in consensus.

Save the service file, then reload systemd: `sudo systemctl daemon-reload`. Enable the service to start on boot: `sudo systemctl enable etrid-validator`. And start it: `sudo systemctl start etrid-validator`.

Your validator is now running! Check the status: `sudo systemctl status etrid-validator`. You should see 'active (running)' in green. Let's watch the logs: `journalctl -u etrid-validator -f`. You'll see your node syncing the blockchain, importing blocks, and eventually participating in consensus.

Look for these messages: 'Syncing', 'Imported block', and eventually 'Starting consensus session'. That last one means you're validating! Let it sync fully before registering on-chain - this can take a few hours for the full blockchain."

**VISUAL CUES:**
- Terminal recording showing systemd setup
- Split screen: code editor (service file) + terminal
- Service file with syntax highlighting
- systemctl status output with colored status indicators
- journalctl logs scrolling with key messages highlighted:
  - 🔄 "Syncing 15.4 bps, target=#1234567"
  - ✅ "Imported #123456 (0x1a2b...)"
  - 🎯 "Starting consensus session"
- Progress bar overlay: "Blockchain sync: 45% (6h remaining)"
- System resource monitor (htop showing etrid process)

**DEMO STEPS:**
1. Copy binary: `sudo cp target/release/etrid /usr/local/bin/`
2. Verify: `which etrid` (should show /usr/local/bin/etrid)
3. Create service file: `sudo nano /etc/systemd/system/etrid-validator.service`
4. Paste service configuration
5. Save and exit (Ctrl+X, Y, Enter)
6. Reload systemd: `sudo systemctl daemon-reload`
7. Enable service: `sudo systemctl enable etrid-validator`
8. Start service: `sudo systemctl start etrid-validator`
9. Check status: `sudo systemctl status etrid-validator`
10. View logs: `journalctl -u etrid-validator -f --lines 50`
11. Wait for sync messages
12. Show sync progress in logs

**CODE TO DISPLAY:**
```bash
# Copy binary to system location
sudo cp target/release/etrid /usr/local/bin/
which etrid
# Output: /usr/local/bin/etrid

# Create systemd service file
sudo nano /etc/systemd/system/etrid-validator.service

# Service file contents:
[Unit]
Description=Etrid Validator Node
After=network.target

[Service]
Type=simple
User=validator
WorkingDirectory=/home/validator
ExecStart=/usr/local/bin/etrid \
  --base-path /home/validator/.etrid/validator-01 \
  --chain flare \
  --validator \
  --name "MyValidator" \
  --rpc-port 9933 \
  --ws-port 9944 \
  --rpc-methods=Safe \
  --prometheus-port 9615
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target

# Reload systemd and start service
sudo systemctl daemon-reload
sudo systemctl enable etrid-validator
sudo systemctl start etrid-validator

# Check status
sudo systemctl status etrid-validator
# Output: ● etrid-validator.service - Etrid Validator Node
#         Loaded: loaded (/etc/systemd/system/etrid-validator.service)
#         Active: active (running) since Tue 2025-10-22 14:30:00 UTC

# Monitor logs
journalctl -u etrid-validator -f

# Key log messages to watch for:
# ⏳ Syncing 15.4 bps, target=#1234567 (6 peers)
# ✅ Imported #123456 (0x1a2b3c4d…)
# 🎯 Starting consensus session on top of parent 0x5e6f7g8h…
```

**KEY POINTS TO EMPHASIZE:**
- systemd ensures validator restarts after crashes
- Validator starts automatically on server reboot
- RPC/WS on localhost only for security (not exposed to internet)
- Prometheus metrics on port 9615 for monitoring
- Sync can take 2-8 hours depending on chain size
- Must fully sync before registering as validator

**COMMON MISTAKES TO MENTION:**
- "Don't expose RPC/WS ports to the internet - security risk!"
- "Don't register on-chain before full sync completes"
- "Don't run as root user - create dedicated 'validator' user"
- "Check firewall isn't blocking P2P port 30333"

---

### 07:30 - 08:45 | Registering On-Chain

**NARRATION:**
"Your node is synced and running. Now let's register it on-chain so you can start validating and earning rewards.

Open the Etrid wallet at wallet.etrid.io and import your stash account using the secret phrase you saved earlier. Click 'Import Account', paste your 12-word phrase, create a password, and give it a name like 'Validator Stash'.

You'll need at least 10,000 ETR to stake. If you're on mainnet, you'll need to purchase this. On testnet, use the faucet we showed in tutorial one.

Now navigate to the Staking section in the sidebar. Click 'Validate'. Enter your session keys - remember that long hex string we generated? Paste it here. Set your commission rate - this is the percentage of nominator rewards you keep. 10% is typical for new validators.

Enter your stake amount - at least 10,000 ETR. Review carefully - staked tokens are locked and can't be withdrawn immediately. There's an unbonding period of 28 days.

Click 'Bond and Validate'. Sign the transaction with your password. Once it finalizes, you're now a validator candidate! It may take a few hours to be included in the active validator set, depending on network conditions.

Congratulations! You're running an Etrid validator!"

**VISUAL CUES:**
- Screen recording: Wallet interface
- Import account flow with blurred secret phrase
- Staking dashboard showing:
  - Total stake
  - Commission rate slider (0-100%)
  - Active validators list
  - Validator candidates list
- Transaction signing modal
- Success notification: "Validator registration complete!"
- Validator status card:
  ```
  Validator: MyValidator
  Status: Candidate (waiting for active set)
  Stake: 10,000 ETR
  Commission: 10%
  Session Keys: 0x1234...
  Nominators: 0
  ```
- Timeline visualization: Registration → Candidate → Active (1-6 hours)

**DEMO STEPS:**
1. Navigate to wallet.etrid.io
2. Click "Import Account"
3. Paste 12-word stash secret phrase
4. Create password
5. Name account: "Validator Stash"
6. Click "Import"
7. Verify balance ≥ 10,000 ETR
8. Navigate to "Staking" in sidebar
9. Click "Validate" button
10. Paste session keys in field
11. Set commission: 10%
12. Enter stake amount: 10,000 ETR
13. Review transaction details
14. Click "Bond and Validate"
15. Enter password to sign
16. Wait for transaction finality
17. View validator in "Candidates" list
18. Check "Active Validators" for inclusion

**CODE TO DISPLAY:**
```bash
# Check your validator status from CLI (optional)
curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' | jq

# Output:
# {
#   "jsonrpc": "2.0",
#   "result": {
#     "peers": 25,
#     "isSyncing": false,
#     "shouldHavePeers": true
#   },
#   "id": 1
# }

# View your validator session keys
cat ~/.etrid/validator-01/chains/flare/keystore/*babe*
# Output: 0x1234567890abcdef... (your session keys)
```

**KEY POINTS TO EMPHASIZE:**
- 10,000 ETR minimum stake requirement
- Commission rate affects nominator attractiveness (lower = more nominators)
- Staked tokens have 28-day unbonding period
- Must be in top 21 validators by stake to be active
- Rewards begin only when in active set
- Can update commission and stake later

**COMMON MISTAKES TO MENTION:**
- "Don't stake your entire balance - leave some for transaction fees"
- "Don't set commission too high (>20%) - nominators will avoid you"
- "Don't change session keys while actively validating - causes issues"
- "Verify your node is synced BEFORE registering"

---

### 08:45 - 09:30 | Monitoring Validator Status

**NARRATION:**
"Your validator is registered. Now let's monitor its performance. Etrid provides several monitoring tools.

First, the built-in Prometheus metrics. We exposed these on port 9615 in our service file. Install Grafana for beautiful dashboards. We have pre-built dashboards in the Etrid repository - just import them into Grafana.

You'll see key metrics: block production rate, finalized blocks, peer count, CPU and memory usage. Watch for these red flags: peer count dropping below 10, block production gaps, or high memory usage above 90%.

Use the wallet's validator dashboard to check your on-chain status. You'll see your total stake including nominations, your commission, and your rewards earned. The rewards chart shows your daily earnings.

Set up alerts! Use Grafana alerts or simple scripts to notify you if your validator goes offline. Here's a simple health check script you can run via cron every 5 minutes.

Check the logs regularly: `journalctl -u etrid-validator -f`. Look for warnings or errors. Common issues include database corruption, network connectivity problems, or running out of disk space.

Pro tip: Join the Etrid validator Discord channel. Other validators share tips and warn about network issues. Community support is invaluable!"

**VISUAL CUES:**
- Grafana dashboard showing:
  - Block height graph (line chart)
  - Peer count (gauge: 25/50)
  - CPU usage (area chart: 45%)
  - Memory usage (area chart: 60%)
  - Network I/O (line chart)
  - Finality lag (bar chart)
- Wallet validator dashboard showing:
  - Validator card with stats
  - Rewards chart (bar chart by day)
  - Nominations list
  - Performance metrics (uptime: 99.8%)
- Alert configuration in Grafana:
  - Rule: "Peer count < 10"
  - Action: "Send Discord webhook"
- Terminal showing health check script execution
- Discord screenshot: #validators channel

**DEMO STEPS:**
1. Open browser to http://localhost:3000 (Grafana)
2. Login with default credentials
3. Import Etrid dashboard JSON
4. Show live metrics updating
5. Point out key metrics
6. Open wallet validator dashboard
7. Show stake, commission, rewards
8. Show rewards chart over time
9. Demonstrate health check script
10. Show crontab entry for automated checks
11. Show Discord #validators channel

**CODE TO DISPLAY:**
```bash
# Install Grafana (Ubuntu)
sudo apt install -y software-properties-common
sudo add-apt-repository "deb https://packages.grafana.com/oss/deb stable main"
wget -q -O - https://packages.grafana.com/gpg.key | sudo apt-key add -
sudo apt update && sudo apt install grafana

# Start Grafana
sudo systemctl start grafana-server
sudo systemctl enable grafana-server

# Import Etrid dashboard
# Navigate to http://localhost:3000
# Login: admin / admin
# Import dashboard from etrid/scripts/grafana-dashboard.json

# Simple health check script
cat > ~/check-validator.sh << 'EOF'
#!/bin/bash
HEALTH=$(curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' | jq -r '.result.isSyncing')

if [ "$HEALTH" == "true" ]; then
  echo "WARNING: Node is syncing - not validating!"
  # Send alert (Discord, email, SMS, etc.)
fi

PEERS=$(curl -s http://localhost:9933 -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' | jq -r '.result.peers')

if [ "$PEERS" -lt 10 ]; then
  echo "WARNING: Low peer count: $PEERS"
fi
EOF

chmod +x ~/check-validator.sh

# Add to cron (every 5 minutes)
crontab -e
# Add line: */5 * * * * /home/validator/check-validator.sh

# Monitor logs
journalctl -u etrid-validator -f --since "1 hour ago"

# Check disk space (important!)
df -h | grep -E "Filesystem|/home"
# Should have >100GB free
```

**KEY POINTS TO EMPHASIZE:**
- Monitoring is critical for maintaining validator performance
- Set up alerts before problems occur
- Dashboard metrics update every 15 seconds
- Low peer count = network connectivity issue
- Disk space monitoring prevents catastrophic failures
- Community support available in Discord

**COMMON MISTAKES TO MENTION:**
- "Don't ignore low peer counts - investigate immediately"
- "Don't forget to monitor disk space - fills up quickly"
- "Set up alerts BEFORE you go on vacation"
- "Check logs after any network upgrade"

---

### 09:30 - 10:00 | Common Issues and Troubleshooting + Outro

**NARRATION:**
"Let's cover common issues you might encounter.

Problem: Node won't start. Solution: Check disk space and database integrity. Run `sudo systemctl status etrid-validator` for error messages.

Problem: Not producing blocks. Solution: Verify session keys are correct, check you're in the active set, and ensure the node is fully synced.

Problem: Low peer count. Solution: Check firewall allows port 30333, verify internet connectivity, and consider adding bootnodes manually.

Problem: High memory usage. Solution: Restart the validator periodically or increase server RAM. Substrate nodes can leak memory over time.

Problem: Database corruption. Solution: Purge the chain and resync from scratch. Always keep backups of your keystore!

Remember: Running a validator is a commitment. You're securing the network and earning rewards, but you need to maintain uptime and monitor performance. Start small on testnet before committing to mainnet.

Congratulations - you're now running an Etrid validator! Check out tutorial three to learn about becoming a nominator, or tutorial four if you want to deploy smart contracts. Join us on Discord and welcome to the validator community!"

**VISUAL CUES:**
- Troubleshooting flowchart:
  ```
  Node won't start
  ↓
  Check systemctl status
  ↓
  ├─ Disk full? → Free space
  ├─ Database error? → Purge chain
  └─ Config error? → Review service file
  ```
- Terminal showing common error messages with solutions overlaid
- Resource links:
  - 📚 docs.etrid.io/validators
  - 💬 discord.gg/etrid #validators
  - 📖 Troubleshooting guide
  - 🔧 GitHub issues
- Checklist overlay:
  ```
  Validator Checklist:
  ✅ Hardware meets requirements
  ✅ Node installed and synced
  ✅ Keys generated and secured
  ✅ Registered on-chain
  ✅ Monitoring configured
  ✅ Alerts set up
  ✅ Community joined
  ```
- Next tutorial thumbnails

**KEY POINTS TO EMPHASIZE:**
- Testnet first, mainnet second
- Monitoring and maintenance required
- Community support available
- Keep learning and improving

---

## Production Notes

### Visual Assets Needed

**Static Graphics:**
1. Validator network visualization
2. Hardware requirements table
3. Key hierarchy diagram
4. Systemd service architecture
5. Grafana dashboard templates
6. Troubleshooting flowchart
7. Resource links end card

**Screen Recordings:**
1. Terminal sessions (Rust install, build, key generation, systemd)
2. Grafana dashboard navigation
3. Wallet staking interface
4. Validator registration flow
5. Log monitoring (journalctl)

**Diagrams:**
1. ASF consensus visualization
2. Validator reward distribution
3. Slashing penalty scenarios
4. Network topology

### Demo Requirements

**Infrastructure:**
- Clean Ubuntu 22.04 server (8 cores, 32GB RAM)
- Etrid testnet running
- Pre-funded stash account with 10,000+ ETR
- Grafana instance configured
- Discord account for community demo

**Preparation:**
- Pre-install dependencies to save time
- Have backup recordings of long operations (build, sync)
- Test all commands in advance
- Prepare cue cards for terminal commands

### Editing Notes

**Pacing:**
- Speed up long operations (3x for compilation, 10x for sync)
- Add progress indicators for wait times
- Use chapter markers for each major section

**Graphics:**
- Highlight terminal commands before execution
- Add tooltips for technical terms
- Use split screen for code + visualization

---

**Tutorial Complete**
Next: 03-staking-nominator.md
