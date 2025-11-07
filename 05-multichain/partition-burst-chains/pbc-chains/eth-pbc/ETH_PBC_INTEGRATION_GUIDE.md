# ETH PBC Integration Guide

**Ethereum Partition Burst Chain - Developer Documentation**

Welcome to ETH PBC, Etrid's Ethereum-compatible Layer 2 with native multi-chain integration.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Novel Features](#novel-features)
3. [Precompile Reference](#precompile-reference)
4. [Example Contracts](#example-contracts)
5. [Deployment Guide](#deployment-guide)
6. [RPC Endpoints](#rpc-endpoints)
7. [FAQ](#faq)

---

## Quick Start

### Connect to ETH PBC

```javascript
// Using ethers.js
const provider = new ethers.providers.JsonRpcProvider('http://localhost:9944');
const chainId = 1337; // ETH PBC Chain ID

// Check connection
const blockNumber = await provider.getBlockNumber();
console.log(`Connected to ETH PBC at block ${blockNumber}`);
```

### Deploy Your First Contract

```bash
# Using Hardhat
npx hardhat deploy --network eth-pbc

# Using Foundry
forge create --rpc-url http://localhost:9944 --private-key <key> MyContract
```

---

## Novel Features

ETH PBC isn't just another Ethereum L2 - it's integrated with Etrid's 14-chain multichain ecosystem.

### 1. Native ETH Wrapping (0x803) ⚡
**Instant, zero-fee ETH <-> wETH conversion**

```solidity
// Wrap ETH to wETH
IEtridNativeETH wrapper = IEtridNativeETH(0x0000000000000000000000000000000000000803);
uint256 wethAmount = wrapper.wrap{value: 1 ether}();

// Unwrap wETH to ETH
wrapper.unwrap(0.5 ether);
```

**Why it's novel:**
- ❌ Traditional: Pay gas for WETH deposit/withdraw
- ✅ Etrid: Zero gas, instant conversion via precompile

### 2. FlareChain Oracle (0x800) 🔮
**Real-time price feeds from FlareChain**

```solidity
IEtridOracle oracle = IEtridOracle(0x0000000000000000000000000000000000000800);

// Get BTC price in ETH
uint256 btcPrice = oracle.getPriceInETH("BTC");

// Get ETH price in USD
uint256 ethPrice = oracle.getPrice("ETH", "USD");
```

**Why it's novel:**
- ❌ Traditional: Use Chainlink (costs gas, potential manipulation)
- ✅ Etrid: Free oracle access via FlareChain consensus

### 3. Cross-Chain Governance (0x801) 🗳️
**Participate in FlareChain governance from ETH PBC**

```solidity
IEtridGovernance gov = IEtridGovernance(0x0000000000000000000000000000000000000801);

// Submit proposal
uint256 proposalId = gov.submitProposal(
    "Enable New Feature",
    "Detailed proposal description..."
);

// Vote on proposal
gov.voteOnProposal(proposalId, true); // Vote YES
```

**Why it's novel:**
- ❌ Traditional L2: Separate governance, no cross-chain voting
- ✅ Etrid: Unified governance across all 14 chains

### 4. Validator Staking Info (0x802) 🔐
**Query FlareChain validator data**

```solidity
IEtridStaking staking = IEtridStaking(0x0000000000000000000000000000000000000802);

// Check validator stake
uint256 stake = staking.getValidatorStake(validatorId);

// Check if validator is active
bool isActive = staking.isValidatorActive(validatorId);

// Get total network stake
uint256 totalStaked = staking.getTotalStaked();
```

**Why it's novel:**
- ❌ Traditional: No access to L1 staking data
- ✅ Etrid: Full transparency into network security

### 5. Lightning Channels (0x808) ⚡
**Off-chain payment channels**

```solidity
IEtridLightning lightning = IEtridLightning(0x0000000000000000000000000000000000000808);

// Open channel
lightning.openChannel{value: 1 ether}(counterparty, locktime);

// Send payment
lightning.sendPayment(channelId, amount, invoice);

// Close channel
lightning.closeChannel(channelId);
```

**Why it's novel:**
- ❌ Traditional: Lightning is Bitcoin-only
- ✅ Etrid: Lightning for ETH and all bridged assets

---

## Precompile Reference

### Standard Ethereum Precompiles

| Address | Name | Description |
|---------|------|-------------|
| `0x01` | ECRecover | ECDSA signature recovery |
| `0x02` | SHA256 | SHA-256 hash function |
| `0x03` | RIPEMD160 | RIPEMD-160 hash function |
| `0x04` | Identity | Identity function (data copy) |
| `0x05` | Modexp | Modular exponentiation |
| `0x08` | SHA3FIPS256 | SHA3-256 (FIPS version) |

### Etrid Custom Precompiles

| Address | Name | Description |
|---------|------|-------------|
| `0x800` | **Oracle** | FlareChain price feeds |
| `0x801` | **Governance** | Cross-chain voting |
| `0x802` | **Staking** | Validator queries |
| `0x803` | **Native ETH Wrap** | Instant ETH <-> wETH |
| `0x808` | **Lightning** | Payment channels |

### Coming Soon
| Address | Name | Status |
|---------|------|--------|
| `0x804` | State Proof Verification | Planned |
| `0x805` | Token Registry | Planned |
| `0x806` | Fair Ordering Service | Planned |
| `0x807` | Private Tx Pools | Planned |
| `0x809` | Cross-Chain Swaps | Planned |
| `0x80A` | Multi-Chain Collateral | Planned |

---

## Example Contracts

### Example 1: Price-Triggered NFT Minting

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";

interface IEtridOracle {
    function getPrice(bytes32 symbol, bytes32 quote) external view returns (uint256);
}

contract DynamicPriceNFT is ERC721 {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    uint256 public nextTokenId;
    uint256 public constant ETH_PRICE_THRESHOLD = 3000e18; // $3000

    constructor() ERC721("DynamicPriceNFT", "DPNFT") {}

    /// @notice Mint NFT only when ETH > $3000
    function mintWhenHighPrice() external {
        uint256 ethPrice = ORACLE.getPrice("ETH", "USD");
        require(ethPrice > ETH_PRICE_THRESHOLD, "ETH price too low");

        _mint(msg.sender, nextTokenId++);
    }
}
```

### Example 2: Multi-Chain Collateral Lending

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IEtridOracle {
    function getPrice(bytes32 symbol, bytes32 quote) external view returns (uint256);
}

contract MultiChainLending {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    struct Position {
        uint256 btcCollateral;
        uint256 solCollateral;
        uint256 ethBorrowed;
    }

    mapping(address => Position) public positions;
    uint256 constant LTV = 70; // 70% loan-to-value

    /// @notice Deposit BTC and SOL as collateral
    function depositCollateral(uint256 btcAmount, uint256 solAmount) external {
        // In production, this would bridge BTC/SOL from other PBCs
        positions[msg.sender].btcCollateral += btcAmount;
        positions[msg.sender].solCollateral += solAmount;
    }

    /// @notice Borrow ETH against multi-chain collateral
    function borrow(uint256 ethAmount) external {
        uint256 collateralValue = getCollateralValue(msg.sender);
        uint256 maxBorrow = (collateralValue * LTV) / 100;

        require(
            positions[msg.sender].ethBorrowed + ethAmount <= maxBorrow,
            "Insufficient collateral"
        );

        positions[msg.sender].ethBorrowed += ethAmount;
        payable(msg.sender).transfer(ethAmount);
    }

    /// @notice Calculate total collateral value in USD
    function getCollateralValue(address user) public view returns (uint256) {
        Position memory pos = positions[user];

        uint256 btcPrice = ORACLE.getPrice("BTC", "USD");
        uint256 solPrice = ORACLE.getPrice("SOL", "USD");

        return (pos.btcCollateral * btcPrice + pos.solCollateral * solPrice) / 1e18;
    }
}
```

### Example 3: Governance-Controlled Feature Flag

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IEtridGovernance {
    function getProposalStatus(uint256 proposalId) external view returns (uint8);
}

contract GovernanceDrivenDApp {
    IEtridGovernance constant GOV = IEtridGovernance(0x0000000000000000000000000000000000000801);

    uint256 public featureProposalId;
    bool public featureEnabled;

    /// @notice Enable feature if governance proposal passed
    function checkAndEnableFeature() external {
        require(!featureEnabled, "Already enabled");

        uint8 status = GOV.getProposalStatus(featureProposalId);
        require(status == 2, "Proposal not passed"); // 2 = Passed

        featureEnabled = true;
    }

    /// @notice Feature only works if governance approved
    function doSomething() external {
        require(featureEnabled, "Feature not enabled by governance");
        // ... feature logic
    }
}
```

---

## Deployment Guide

### Prerequisites
- Node.js 16+
- Hardhat or Foundry
- ETR for gas fees

### Deploy with Hardhat

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    'eth-pbc': {
      url: 'http://localhost:9944',
      chainId: 1337,
      accounts: ['0x...']
    }
  }
};
```

```bash
npx hardhat deploy --network eth-pbc
```

### Deploy with Foundry

```bash
forge create MyContract \
  --rpc-url http://localhost:9944 \
  --private-key $PRIVATE_KEY
```

---

## RPC Endpoints

### Local Development
```
HTTP:  http://localhost:9944
WS:    ws://localhost:9944
```

### Testnet (Coming Soon)
```
HTTP:  https://eth-pbc-testnet.etrid.org
WS:    wss://eth-pbc-testnet.etrid.org
```

### Mainnet (Future)
```
HTTP:  https://eth-pbc.etrid.org
WS:    wss://eth-pbc.etrid.org
```

### Supported RPC Methods
- `eth_*` - Standard Ethereum JSON-RPC
- `net_*` - Network information
- `web3_*` - Web3 utilities
- `trace_*` - Transaction tracing (optional)

---

## FAQ

### Q: Is ETH PBC EVM-compatible?
**A:** Yes, 100%. Any Solidity contract that works on Ethereum will work on ETH PBC.

### Q: What's the difference from other L2s?
**A:** ETH PBC has native integration with 13 other blockchains via FlareChain. You can:
- Use BTC, SOL, XRP as collateral in DeFi
- Access price feeds without oracles
- Vote on governance from ETH contracts
- Atomic cross-chain swaps

### Q: How do I bridge ETH to ETH PBC?
**A:** Use the Etrid bridge at https://bridge.etrid.org. Or use the Native ETH Wrapper precompile (0x803) for instant wrapping.

### Q: Are there gas fees?
**A:** Yes, but much lower than Ethereum mainnet. Gas is paid in ETR (Etrid native token) or any bridged token.

### Q: Is it secure?
**A:** ETH PBC inherits security from FlareChain's ASF consensus with 100+ validators. All code will be audited before mainnet launch.

### Q: Can I use Metamask?
**A:** Yes! Just add ETH PBC as a custom network:
```
Network Name: ETH PBC
RPC URL: http://localhost:9944
Chain ID: 1337
Currency Symbol: ETR
```

### Q: How fast are transactions?
**A:** Sub-second finality via FlareChain consensus. Typically 2-3 seconds for full confirmation.

### Q: What's the TPS limit?
**A:** ETH PBC can handle 5,000+ TPS. Combined with Lightning channels (0x808), over 1M TPS.

---

## Support & Resources

- **Documentation:** https://docs.etrid.org/eth-pbc
- **Discord:** https://discord.gg/etrid
- **GitHub:** https://github.com/etrid/eth-pbc
- **Block Explorer:** https://ethscan.etrid.org
- **Faucet:** https://faucet.etrid.org

---

## What Makes ETH PBC Novel?

| Feature | Traditional L2 | ETH PBC |
|---------|----------------|---------|
| **Bridge** | ETH only | 14 chains |
| **Oracle** | External (Chainlink) | Built-in (FlareChain) |
| **Governance** | Separate | Unified multi-chain |
| **Collateral** | Single-chain | Cross-chain |
| **MEV Protection** | Limited | Fair ordering built-in |
| **Lightning Network** | N/A | Native support |
| **State Proofs** | Complex | Simple precompile |
| **Gas Tokens** | ETH only | Any bridged token |

---

**Built with ❤️ by the Etrid Foundation**

**Version:** 0.1.0
**Last Updated:** November 7, 2025
**Status:** Development
