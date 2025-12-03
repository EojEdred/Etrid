# Wrapped Etrid (wETR) - BEP-20 Token

Production-ready BEP-20 token contract for wrapping Etrid (ETR) on Binance Smart Chain.

## Overview

**wETR** is a wrapped version of Etrid's native currency (ETR) that operates on BSC as a BEP-20 token. This enables ETR to be used in BSC's DeFi ecosystem while maintaining a 1:1 peg with native ETR through a secure bridge mechanism.

### Contract Details

- **Token Name**: Wrapped Etrid
- **Symbol**: wETR
- **Decimals**: 18
- **Standard**: BEP-20 (ERC-20 compatible)
- **Supply**: Dynamic (minted/burned through bridge)

### Features

- ✅ Mintable only by authorized bridge
- ✅ Burnable by users (to unwrap)
- ✅ Pausable for emergency situations
- ✅ Role-based access control
- ✅ Reentrancy protection
- ✅ OpenZeppelin security standards
- ✅ BscScan verification ready

## Installation

```bash
# Install dependencies
npm install

# Create environment file
cp .env.example .env
```

## Environment Setup

Create a `.env` file with the following variables:

```env
# Private key of deployer account (WITHOUT 0x prefix)
PRIVATE_KEY=your_private_key_here

# BscScan API key for contract verification
BSCSCAN_API_KEY=your_bscscan_api_key

# Optional: Custom RPC URLs
BSC_RPC_URL=https://bsc-dataseed1.binance.org
BSC_TESTNET_RPC_URL=https://data-seed-prebsc-1-s1.binance.org:8545
```

### Getting BscScan API Key

1. Go to [BscScan](https://bscscan.com)
2. Create an account
3. Navigate to API-KEYs section
4. Generate new API key

## Deployment

### Step 1: Compile Contract

```bash
npm run compile
```

### Step 2: Deploy to BSC Testnet (Recommended First)

```bash
npm run deploy:testnet
```

Expected output:
```
Deploying WrappedETR...
WrappedETR deployed to: 0x...
Contract verified successfully!
```

### Step 3: Deploy to BSC Mainnet

**Important**: Ensure you have enough BNB for deployment (~0.01-0.02 BNB)

```bash
npm run deploy:mainnet
```

### Estimated Gas Costs

| Network | Deployment Cost | Gas Price | Total Cost (BNB) |
|---------|----------------|-----------|------------------|
| BSC Mainnet | ~2,500,000 gas | 3 gwei | ~0.0075 BNB (~$1.50) |
| BSC Testnet | ~2,500,000 gas | 10 gwei | Free (testnet BNB) |

*Prices are estimates and may vary based on network congestion*

## Post-Deployment Configuration

### 1. Add Bridge Address

After deployment, you must add the bridge contract address:

```bash
# Open Hardhat console
npx hardhat console --network bsc

# In console:
const WrappedETR = await ethers.getContractFactory("WrappedETR");
const token = await WrappedETR.attach("DEPLOYED_CONTRACT_ADDRESS");
await token.addBridge("BRIDGE_CONTRACT_ADDRESS");
```

### 2. Verify on BscScan (if auto-verification failed)

```bash
npx hardhat verify --network bsc DEPLOYED_CONTRACT_ADDRESS "ADMIN_ADDRESS"
```

### 3. Test Minting (with bridge account)

```javascript
// In Hardhat console with bridge account
const tx = await token.mint(
  "USER_ADDRESS",
  ethers.utils.parseEther("100"), // 100 wETR
  "0x..." // Etrid transaction hash
);
await tx.wait();
```

### 4. Test Burning (from user account)

```javascript
// In Hardhat console with user account
const tx = await token.burnToBridge(
  ethers.utils.parseEther("50"), // 50 wETR
  "etrid1abc..." // Etrid address to receive native ETR
);
await tx.wait();
```

## Security Considerations

### Access Control Roles

1. **DEFAULT_ADMIN_ROLE**: Can manage all roles and pause contract
2. **BRIDGE_ROLE**: Can mint tokens (bridge contract only)
3. **PAUSER_ROLE**: Can pause/unpause contract in emergencies

### Best Practices for Production

1. **Use Multisig for Admin**: Transfer admin role to a multisig wallet (e.g., Gnosis Safe)
   ```javascript
   await token.grantRole(DEFAULT_ADMIN_ROLE, MULTISIG_ADDRESS);
   await token.renounceRole(DEFAULT_ADMIN_ROLE, DEPLOYER_ADDRESS);
   ```

2. **Secure Private Keys**: Never commit private keys to git
3. **Test on Testnet First**: Always deploy to testnet before mainnet
4. **Monitor Bridge Events**: Set up monitoring for BridgeMint and BridgeBurn events
5. **Emergency Pause**: Know how to pause contract if needed
   ```javascript
   await token.pause(); // Stops all transfers and minting
   await token.unpause(); // Resumes operations
   ```

## Contract Functions

### User Functions

- `transfer(to, amount)` - Transfer tokens
- `approve(spender, amount)` - Approve spending
- `burnToBridge(amount, etridAddress)` - Burn tokens to unwrap

### Bridge Functions (BRIDGE_ROLE only)

- `mint(to, amount, txHash)` - Mint tokens (bridge deposits)

### Admin Functions (DEFAULT_ADMIN_ROLE only)

- `addBridge(address)` - Grant bridge role
- `removeBridge(address)` - Revoke bridge role
- `pause()` - Pause contract
- `unpause()` - Unpause contract

## Monitoring

### Events to Monitor

```solidity
event BridgeMint(address indexed to, uint256 amount, bytes32 indexed txHash);
event BridgeBurn(address indexed from, uint256 amount, string etridAddress);
event BridgeAddressUpdated(address indexed oldBridge, address indexed newBridge);
```

### Check Token Info

```bash
npx hardhat console --network bsc

const token = await ethers.getContractAt("WrappedETR", "CONTRACT_ADDRESS");
console.log("Name:", await token.name());
console.log("Symbol:", await token.symbol());
console.log("Total Supply:", ethers.utils.formatEther(await token.totalSupply()));
console.log("Your Balance:", ethers.utils.formatEther(await token.balanceOf("YOUR_ADDRESS")));
```

## Troubleshooting

### Deployment Fails

- **Insufficient funds**: Ensure you have enough BNB
- **Network issues**: Try different RPC endpoint
- **Gas price too low**: Increase gasPrice in hardhat.config.js

### Verification Fails

- **Wrong constructor args**: Ensure admin address is correct
- **Wrong compiler version**: Use Solidity 0.8.20
- **API key invalid**: Check BSCSCAN_API_KEY in .env

### Transaction Reverts

- **"WrappedETR: mint to zero address"**: Check recipient address
- **"AccessControl: account ... is missing role"**: Ensure bridge role is granted
- **"Pausable: paused"**: Contract is paused, call unpause()

## Testing

```bash
# Run all tests
npm test

# Run with gas reporting
REPORT_GAS=true npm test
```

## BSC Resources

- **BSC Mainnet Explorer**: https://bscscan.com
- **BSC Testnet Explorer**: https://testnet.bscscan.com
- **BSC Testnet Faucet**: https://testnet.binance.org/faucet-smart
- **BSC Documentation**: https://docs.bnbchain.org

## Contract Addresses

### Mainnet
```
Contract Address: TBD (deploy and update)
Admin: TBD
Bridge: TBD
```

### Testnet
```
Contract Address: TBD (deploy and update)
Admin: TBD
Bridge: TBD
```

## Support

For issues or questions:
- GitHub Issues: [etrid/contracts](https://github.com/etrid/etrid)
- Documentation: [docs.etrid.io](https://docs.etrid.io)

## License

MIT License - see LICENSE file for details

## Audit Status

⚠️ **Not audited** - This contract has not been professionally audited. Use at your own risk. Consider getting a professional audit before mainnet deployment with significant funds.

## Changelog

### Version 1.0.0
- Initial release
- BEP-20 token implementation
- Bridge mint/burn functionality
- Role-based access control
- Pausable functionality
- Reentrancy protection
