# ÉTRID Web UI Suite for Pinokio

This Pinokio configuration provides one-click installation and deployment of the ÉTRID blockchain web interface suite.

## Included Applications

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
