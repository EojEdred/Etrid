# Unified Multi-Chain Deployment Strategy

## Overview
This strategy leverages ETH PBC's full Frontier EVM integration to deploy smart contracts using industry-standard Ethereum tooling.

## Architecture

```
FlareChain (Substrate)
    ↕ (Bridge Pallets)
ETH PBC (EVM) ← MasterChef + Bridge Adapter
    ↕ (EDSC Bridge)
7 External EVM Chains (ETH, BSC, SOL-EVM, XRP-EVM, etc.)
```

## Deployment Phases

### Phase 1: Core Token Contracts
Deploy foundational token contracts needed for the entire ecosystem:

1. **ETR Token** (Native on FlareChain, wrapped elsewhere)
   - FlareChain: Native token (already exists)
   - ETH PBC: WrappedETR (ERC20)
   - External chains: WrappedETR (ERC20) on each

2. **EDSC Stablecoin** (Cross-chain)
   - ETH PBC: EDSC token (ERC20)
   - External chains: EDSC token (ERC20)

3. **TokenMessenger** (Bridge infrastructure)
   - ETH PBC: TokenMessenger contract
   - External chains: TokenMessenger contracts

### Phase 2: DeFi Infrastructure
1. **MasterChef** (Yield Farming)
   - Deploy to ETH PBC
   - Configure reward rates
   - Add LP token pools

2. **Bridge Adapter**
   - Deploy ETHPBCBridgeAdapter
   - Connect to TokenMessenger
   - Link to MasterChef

### Phase 3: Multi-Chain Expansion
Deploy bridge adapters to all 7 external chains:
- Ethereum
- BNB Chain
- Solana (EVM via Neon)
- Stellar (EVM via Soroban)
- XRP Ledger (EVM sidechain)
- Tron
- USDT chains

## Technical Implementation

### Tools & Technologies
- **Hardhat**: Primary deployment framework
- **OpenZeppelin**: Battle-tested contract libraries
- **CREATE2**: Deterministic addresses across chains
- **Multi-sig**: Gnosis Safe for admin operations

### Smart Contract Structure

```
contracts/
├── tokens/
│   ├── WrappedETR.sol          # ERC20 wrapper for ETR
│   ├── EDSC.sol                 # Stablecoin token
│   └── TokenMessenger.sol       # Bridge messaging
├── bridges/
│   ├── ETHPBCBridgeAdapter.sol  # ETH PBC bridge
│   ├── EVMBridgeAdapter.sol     # Generic EVM bridge
│   └── attestation/
│       └── OracleRegistry.sol   # 3-of-5 multisig oracles
├── defi/
│   ├── MasterChef.sol           # Yield farming
│   ├── UniswapV2Factory.sol     # DEX factory
│   └── UniswapV2Router.sol      # DEX router
└── governance/
    └── Timelock.sol             # Governance timelock
```

### Environment Configuration

```bash
# .env.deployment
# Network RPCs
ETH_PBC_RPC=http://localhost:8545
ETHEREUM_RPC=https://eth-mainnet.g.alchemy.com/v2/YOUR-KEY
BSC_RPC=https://bsc-dataseed.bnbchain.org
# Add other chains...

# Deployer
DEPLOYER_PRIVATE_KEY=0x...

# Contract Addresses (filled after each deployment)
ETR_TOKEN_ETH_PBC=
EDSC_TOKEN_ETH_PBC=
TOKEN_MESSENGER_ETH_PBC=
MASTERCHEF_ETH_PBC=
```

## Deployment Scripts

### Automated Multi-Chain Deployment
```javascript
// scripts/deploy-multi-chain.js
const chains = ['eth-pbc', 'ethereum', 'bsc', 'polygon', ...]

for (const chain of chains) {
  await deployToChain(chain, {
    contracts: ['WrappedETR', 'EDSC', 'TokenMessenger'],
    verify: true,
    saveToDisk: true
  })
}
```

### CREATE2 Deterministic Deployment
Same address across all chains for easier integration:
```javascript
const salt = ethers.utils.id('ETRID_V1')
const factory = await ethers.getContractFactory('WrappedETR')
const initCode = factory.getDeployTransaction().data
const address = ethers.utils.getCreate2Address(
  DEPLOYER,
  salt,
  ethers.utils.keccak256(initCode)
)
// Address is identical on all chains
```

## Security Considerations

### 1. Admin Keys
- Use multi-sig (Gnosis Safe) for all admin operations
- Minimum 3-of-5 threshold
- Hardware wallet signers

### 2. Bridge Security
- 3-of-5 oracle attestation required
- Rate limiting on bridges
- Emergency pause functionality
- Time-delayed admin actions (24-48hr)

### 3. Audit Requirements
- Smart contract audits before mainnet
- Formal verification for critical contracts
- Bug bounty program

## Testing Strategy

### Local Testing
```bash
# 1. Start ETH PBC node
cd eth-pbc-collator
./target/release/eth-pbc-collator --dev --tmp

# 2. Deploy contracts locally
npx hardhat run scripts/deploy-all.js --network localhost

# 3. Run integration tests
npx hardhat test --network localhost
```

### Testnet Deployment
1. Deploy to ETH PBC testnet
2. Deploy to external testnets (Goerli, BSC Testnet, etc.)
3. Test cross-chain flows
4. Perform security audits

### Mainnet Deployment
1. Deploy Phase 1 (Tokens)
2. Wait 48 hours, monitor
3. Deploy Phase 2 (DeFi)
4. Wait 48 hours, monitor
5. Deploy Phase 3 (Multi-chain)

## Monitoring & Maintenance

### On-Chain Monitoring
- Bridge transaction monitoring
- TVL tracking
- Gas optimization alerts
- Oracle health checks

### Off-Chain Infrastructure
- Relayer uptime monitoring
- Database backups
- API endpoint health

## Next Steps

### Immediate Actions
1. Create missing smart contracts (WrappedETR, EDSC, TokenMessenger)
2. Set up hardhat workspace with multi-chain configs
3. Write deployment scripts with CREATE2
4. Configure test environments

### Short-term (1-2 weeks)
1. Deploy to ETH PBC testnet
2. Integration testing
3. Security audit preparation

### Medium-term (1 month)
1. External testnet deployments
2. Cross-chain testing
3. Mainnet deployment preparation

## Cost Estimates

### Gas Costs (approximate)
- WrappedETR deployment: ~1M gas = $30-50 (per chain)
- MasterChef deployment: ~3M gas = $90-150
- Bridge Adapter: ~2M gas = $60-100
- Total per chain: ~$200-300
- 7 chains total: ~$1,400-2,100

### Infrastructure Costs
- RPC nodes: $100-500/month
- Monitoring: $50/month
- Audits: $20k-50k (one-time)

## Conclusion

This strategy provides a comprehensive, secure, and efficient path to deploying Ëtrid's multi-chain infrastructure. By leveraging ETH PBC's full EVM compatibility, we can use battle-tested Ethereum tooling and patterns while maintaining the benefits of Substrate's flexibility.
