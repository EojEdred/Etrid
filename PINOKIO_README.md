# ÉTRID Complete Suite for Pinokio

This Pinokio configuration provides one-click installation and deployment of the complete ÉTRID blockchain ecosystem:
- 🌐 **5 Web UI Applications** for blockchain interaction
- 🔧 **Validator Management Tools** for 21 distributed validators
- 🤖 **AI-Powered Monitoring** with intelligent health analysis

## Included Applications

### Web UI Applications

### 1. Lightning Landing (Port 3000)
Production-ready landing page for ÉTRID Lightning Network with:
- Feature highlights and animations
- Network statistics
- Deployment guides
- Responsive design

### 2. Validator Dashboard (Port 3002)
Real-time validator performance monitoring with:
- Validator performance metrics
- Nominator management
- Uptime tracking
- Reward analytics
- Polkadot.js integration

### 3. Watchtower Monitor (Port 3003)
Lightning-Bloc watchtower monitoring with:
- Real-time fraud detection
- Channel monitoring
- WebSocket updates
- Alert management

### 4. MasterChef Dashboard (Port 3001)
LP rewards management dashboard featuring:
- Liquidity pool tracking
- Reward calculations
- Staking interfaces
- Ethers.js integration

### 5. Wallet Web (Port 3000)
Comprehensive ÉTRID crypto wallet web application featuring:
- Multi-chain wallet support (Polkadot & Ethereum PBC)
- Transaction builder
- Staking interfaces (ETH PBC MasterChef)
- Governance voting
- Lightning Network swap interface
- WalletConnect integration with RainbowKit
- Dark/light theme support

### Validator Management Tools

#### Validator CLI Manager (`pinokio/validator-cli.js`)
Remote command-line access to 21 distributed validators across Azure and Oracle Cloud:

**Capabilities:**
- List all validators with accessibility status
- Check validator health and status remotely via SSH
- Execute commands on specific validators or all at once
- View real-time logs from validator services
- Restart validator services remotely
- Get detailed node information

**Supported Validators:**
- 16 **Accessible Validators** (SSH enabled)
- Distributed across: Azure (15), Oracle Cloud (1)
- Regions: Europe, Asia, Middle East, Africa, US
- Roles: Directors, Developers, Nodes

#### AI-Powered Monitor (`pinokio/ai-validator-monitor.js`)
Intelligent monitoring system that analyzes validator health and provides AI-driven recommendations:

**Features:**
- Real-time health scoring (0-100 per validator)
- Automatic alert detection (peers, disk, memory, block height)
- Smart recommendations based on detected issues
- Network-wide analysis across all validators
- Continuous monitoring with configurable intervals
- JSON report generation for historical analysis

**Monitoring Metrics:**
- Service status (running/stopped)
- Peer count and network connectivity
- Block height and synchronization
- Disk space usage
- Memory consumption
- Service uptime

## Installation

1. Open Pinokio
2. Navigate to the ÉTRID Web UI Suite
3. Click "Install"
4. Wait for all dependencies to be installed and built

## Running the Applications

After installation:
1. Click "Run" to start the Lightning Landing page
2. Access it at `http://localhost:3000`

### Running Individual Applications

To run other applications individually, navigate to their directories and run:

```bash
# Validator Dashboard
cd apps/validator-dashboard
npm start
# Access at http://localhost:3002

# Watchtower Monitor
cd apps/watchtower-monitor
npm start
# Access at http://localhost:3003

# MasterChef Dashboard
cd apps/masterchef-dashboard
npm start
# Access at http://localhost:3001

# Wallet Web
cd apps/wallet-web/etrid-crypto-website
npm start
# Access at http://localhost:3000
```

### Using Validator Management Tools

#### CLI Commands

```bash
# List all validators
node pinokio/validator-cli.js list

# Check status of all accessible validators
node pinokio/validator-cli.js status

# Check specific validator
node pinokio/validator-cli.js status 7

# Execute command on specific validator
node pinokio/validator-cli.js exec 7 "systemctl status etrid-validator"

# Execute command on all accessible validators
node pinokio/validator-cli.js exec-all "df -h"

# View validator logs
node pinokio/validator-cli.js logs 7 100

# Restart validator
node pinokio/validator-cli.js restart 7
```

#### AI Monitoring

```bash
# Run single monitoring check
node pinokio/ai-validator-monitor.js monitor

# Continuous monitoring (every 5 minutes)
node pinokio/ai-validator-monitor.js continuous 5
```

#### Quick NPM Scripts

```bash
cd pinokio

# Validator management
npm run validator:list           # List all validators
npm run validator:status         # Check all statuses
npm run validator:monitor        # Run AI monitoring
npm run validator:watch          # Continuous monitoring

# Web UI management
npm run web:build               # Build all web UIs
npm run web:start               # Start all web UIs
npm run web:status              # Check web UI status
npm run web:stop                # Stop all web UIs
```

## Configuration

### Environment Variables

Each application may require specific environment variables:

#### Validator Dashboard
- `NEXT_PUBLIC_WS_PROVIDER`: WebSocket endpoint for Polkadot node (default: `ws://localhost:9944`)
- `NEXT_PUBLIC_VALIDATOR_ADDRESS`: Your validator address

#### Watchtower Monitor
- `NEXT_PUBLIC_API_URL`: Watchtower API endpoint
- `NEXT_PUBLIC_WS_URL`: WebSocket endpoint for real-time updates

#### MasterChef Dashboard
- `NEXT_PUBLIC_RPC_URL`: Ethereum RPC endpoint
- `NEXT_PUBLIC_MASTERCHEF_ADDRESS`: MasterChef contract address

#### Wallet Web
- `NEXT_PUBLIC_WALLETCONNECT_PROJECT_ID`: WalletConnect project ID (get from https://cloud.walletconnect.com)
- `NEXT_PUBLIC_WS_PROVIDER`: WebSocket endpoint for Polkadot node (default: `ws://localhost:9944`)
- `NEXT_PUBLIC_CHAIN_ID`: Chain ID for ETH PBC (default: `8888`)

#### Validator Tools
- `SSH_KEY_PATH`: Path to SSH private key for validator access (default: `~/.ssh/id_rsa`)

### SSH Configuration for Validators

To use validator management tools, you need SSH access to the validators:

1. **Set up SSH key**:
   ```bash
   export SSH_KEY_PATH=~/.ssh/your-validator-key
   ```

2. **Test connection** to a validator:
   ```bash
   ssh -i ~/.ssh/your-validator-key runtime-dev01@20.224.104.239
   ```

3. **Validator configuration** is stored in:
   ```
   infrastructure/config/validator-ips.json
   ```

**Security Notes:**
- Never commit SSH private keys to the repository
- Keep SSH keys secure and rotate regularly
- Only 16 out of 21 validators are currently accessible (`"accessible": true`)
- Each validator has a unique SSH user (e.g., `runtime-dev01`, `compiler-dev01`)

## Build Status

✅ Lightning Landing - Built successfully
✅ Validator Dashboard - Built successfully
✅ Watchtower Monitor - Built successfully (with font configuration fix)
✅ MasterChef Dashboard - Built successfully
✅ Wallet Web - Built successfully (with Web3Provider layout fix)

## Technology Stack

- **Framework**: Next.js 14-15
- **Language**: TypeScript
- **Styling**: Tailwind CSS
- **UI Components**: Radix UI
- **Blockchain**: Polkadot.js, Ethers.js
- **Data Visualization**: Recharts

## Support

For issues or questions:
- Check the individual README files in each app directory
- Refer to the main ÉTRID documentation
- Submit issues to the ÉTRID GitHub repository

## Notes

- All applications are pre-built during installation
- The default runner starts Lightning Landing on port 3000
- You can modify `pinokio.js` to start different applications by default
- Ensure required ports are available before starting applications
