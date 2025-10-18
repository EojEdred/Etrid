# pyE Quick Start Guide

Get started with pyE - ËTRID's Python CLI in 5 minutes!

## 1. Install (Choose One Method)

### Option A: Virtual Environment (Recommended)
```bash
cd /Users/macbook/Desktop/etrid/13-clients/pye
python3 -m venv venv
source venv/bin/activate
pip install -e .
```

### Option B: pipx (Isolated)
```bash
pipx install /Users/macbook/Desktop/etrid/13-clients/pye
```

### Option C: Development Mode
```bash
pip3 install --user -r requirements.txt
alias pye="python3 -m pye.cli"
```

## 2. Verify Installation

```bash
pye --version
pye --help
```

## 3. Common Commands

### Account Management
```bash
pye account create alice          # Create account
pye account list                  # List accounts
pye account show alice            # Show details
```

### Query Blockchain
```bash
pye query block                   # Latest block
pye query balance alice           # Check balance
pye query chain                   # Chain info
pye query health                  # Node health
```

### Send Transactions
```bash
pye send 0x123... 100 -f alice    # Send 100 ETR
```

### Staking
```bash
pye stake deposit 1000 -a alice   # Stake 1000 ETR
pye stake info -a alice           # Check stake
pye stake rewards -a alice        # Check rewards
pye stake claim -a alice          # Claim rewards
```

### Consensus
```bash
pye consensus status              # Consensus status
pye consensus proposals           # List proposals
pye consensus vote PROP-001 yes -a alice  # Vote
```

## 4. Configuration

### Set Custom Node
```bash
export ETRID_NODE_URL=ws://mainnet.etrid.io:9944
# OR
pye --node ws://testnet.etrid.io:9944 info
```

### Set Custom Keystore
```bash
export ETRID_KEYSTORE=/path/to/keystore
```

## 5. Getting Help

```bash
pye --help                        # Main help
pye account --help                # Account commands
pye query --help                  # Query commands
pye stake --help                  # Staking commands
pye consensus --help              # Consensus commands
```

## 6. Complete Workflow Example

```bash
# 1. Create account
pye account create alice

# 2. Check network
pye info

# 3. Check balance
pye query balance alice

# 4. Send tokens
pye send bob 100 -f alice

# 5. Stake tokens
pye stake deposit 1000 -a alice

# 6. Vote on proposal
pye consensus vote PROP-001 yes -a alice
```

## Troubleshooting

### Can't connect to node?
```bash
pye query health
# Try different URL
pye --node ws://localhost:9944 info
```

### Account not found?
```bash
pye account list  # List all accounts
```

### Need more help?
- See [README.md](README.md) for full documentation
- See [INSTALL.md](INSTALL.md) for installation help
- Visit https://docs.etrid.io/pye

---

That's it! You're ready to use pyE! 🚀
