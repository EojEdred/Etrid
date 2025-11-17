# Etrid ETH PBC Precompile Solidity Interfaces

Solidity interfaces for accessing FlareChain features from ETH PBC (Ethereum Partition Burst Chain) smart contracts.

## Overview

ETH PBC provides 7 custom precompiled contracts that enable EVM contracts to interact with FlareChain functionality via XCM (Cross-Consensus Messaging). These precompiles unlock novel features not available on traditional Ethereum L2s.

## Precompile Addresses

| Address | Interface | Description |
|---------|-----------|-------------|
| `0x800` | `IEtridOracle` | FlareChain price feeds (free oracle access) |
| `0x801` | `IEtridGovernance` | Cross-chain governance voting |
| `0x802` | `IEtridStaking` | Validator and staking queries |
| `0x803` | `IEtridNativeETH` | Zero-fee ETH wrapping |
| `0x804` | `IEtridBridge` | XCM bridge to FlareChain |
| `0x805` | `IEtridTokenRegistry` | Auto-discover Ethereum tokens |
| `0x806` | `IEtridStateProof` | Verify Ethereum state trustlessly |

## Installation

### Hardhat

```bash
npm install @etrid/solidity-interfaces
```

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@etrid/solidity-interfaces/IEtridOracle.sol";

contract MyContract {
    IEtridOracle constant oracle = IEtridOracle(0x0000000000000000000000000000000000000800);

    function getBTCPrice() public view returns (uint256) {
        return oracle.getPrice(bytes32("BTC"), bytes32("USD"));
    }
}
```

### Foundry

```bash
forge install etrid/solidity-interfaces
```

```solidity
import "etrid-solidity-interfaces/IEtridOracle.sol";
```

---

## Usage Examples

### 1. Oracle - Free Price Feeds

**Novel Feature:** Zero-cost oracle queries via FlareChain consensus. No Chainlink fees!

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridOracle.sol";

contract DynamicPricingNFT {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    uint256 public constant BASE_PRICE = 0.1 ether;

    /**
     * @notice NFT price adjusts based on ETH/USD price
     * @dev Uses FlareChain oracle - zero gas for price query
     */
    function getMintPrice() public view returns (uint256) {
        // Get ETH price in USD (scaled by 1e18)
        uint256 ethPriceUSD = ORACLE.getPrice(bytes32("ETH"), bytes32("USD"));

        // If ETH > $3000, mint price is 0.1 ETH
        // If ETH < $2000, mint price is 0.15 ETH (cheaper in dollar terms)
        if (ethPriceUSD > 3000e18) {
            return BASE_PRICE;
        } else if (ethPriceUSD < 2000e18) {
            return BASE_PRICE * 150 / 100;
        } else {
            return BASE_PRICE * 125 / 100;
        }
    }

    function mint() public payable {
        require(msg.value >= getMintPrice(), "Insufficient payment");
        // Mint NFT logic...
    }
}
```

**Multi-Asset Price Feed:**

```solidity
contract MultiAssetPricing {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    /**
     * @notice Get BTC price in ETH
     */
    function getBTCPriceInETH() public view returns (uint256) {
        return ORACLE.getPriceInETH(bytes32("BTC"));
    }

    /**
     * @notice Check if price data is fresh
     */
    function isPriceFresh(bytes32 symbol) public view returns (bool) {
        uint256 lastUpdate = ORACLE.getLastUpdate(symbol);
        return block.timestamp - lastUpdate < 5 minutes;
    }
}
```

---

### 2. Governance - Cross-Chain Voting

**Novel Feature:** Participate in FlareChain governance from ETH PBC contracts.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridGovernance.sol";

contract DAOWithFlareChainGovernance {
    IEtridGovernance constant GOV = IEtridGovernance(0x0000000000000000000000000000000000000801);

    mapping(address => bool) public members;
    uint256 public memberCount;

    /**
     * @notice DAO members can submit proposals to FlareChain governance
     * @dev Proposal costs are subsidized by DAO treasury
     */
    function submitDAOProposal(string memory title, string memory description)
        public
        returns (uint256 proposalId)
    {
        require(members[msg.sender], "Not a DAO member");

        // Submit to FlareChain governance via XCM
        proposalId = GOV.submitProposal(title, description);

        emit ProposalSubmitted(msg.sender, proposalId, title);
    }

    /**
     * @notice Members vote on FlareChain proposals
     */
    function voteOnProposal(uint256 proposalId, bool support) public {
        require(members[msg.sender], "Not a DAO member");

        GOV.voteOnProposal(proposalId, support);

        emit VoteCast(msg.sender, proposalId, support);
    }

    /**
     * @notice Check proposal status before executing
     */
    function executeIfPassed(uint256 proposalId) public {
        uint8 status = GOV.getProposalStatus(proposalId);
        require(status == uint8(IEtridGovernance.ProposalStatus.Passed), "Proposal not passed");

        // Execute approved action
        _executeApprovedAction(proposalId);
    }

    event ProposalSubmitted(address indexed member, uint256 indexed proposalId, string title);
    event VoteCast(address indexed member, uint256 indexed proposalId, bool support);
}
```

---

### 3. Staking - Validator Transparency

**Novel Feature:** Access L1 (FlareChain) staking data from L2 (ETH PBC).

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridStaking.sol";

contract ValidatorDelegationPool {
    IEtridStaking constant STAKING = IEtridStaking(0x0000000000000000000000000000000000000802);

    bytes32 public validatorId;

    /**
     * @notice Only accept delegations if validator is active and well-staked
     */
    function delegate() public payable {
        // Check validator is active
        require(STAKING.isValidatorActive(validatorId), "Validator inactive");

        // Check validator has sufficient stake (security threshold)
        uint256 validatorStake = STAKING.getValidatorStake(validatorId);
        require(validatorStake >= 1000 ether, "Validator undercollateralized");

        // Accept delegation
        _processDelegation(msg.sender, msg.value);
    }

    /**
     * @notice Display validator's network share
     */
    function getValidatorNetworkShare() public view returns (uint256 basisPoints) {
        uint256 validatorStake = STAKING.getValidatorStake(validatorId);
        uint256 totalStake = STAKING.getTotalStaked();

        // Return share in basis points (1 bp = 0.01%)
        return (validatorStake * 10000) / totalStake;
    }

    /**
     * @notice Check network decentralization
     */
    function isNetworkDecentralized() public view returns (bool) {
        uint256 validatorCount = STAKING.getValidatorCount();
        return validatorCount >= 100; // Require at least 100 validators
    }
}
```

---

### 4. Native ETH Wrap - Zero-Fee Wrapping

**Novel Feature:** Instant, ZERO-GAS ETH <-> wETH conversion.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridNativeETH.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract GasFreeWrapper {
    IEtridNativeETH constant WRAPPER = IEtridNativeETH(0x0000000000000000000000000000000000000803);
    IERC20 constant WETH = IERC20(0x...); // wETH address on ETH PBC

    /**
     * @notice Wrap ETH to wETH with ZERO wrapping fees
     * @dev Traditional WETH.deposit() costs ~24k gas
     *      Etrid's precompile costs ZERO gas for the wrap itself
     */
    function wrapETH() public payable returns (uint256 wethReceived) {
        // Zero-fee wrapping via precompile
        wethReceived = WRAPPER.wrap{value: msg.value}();

        // Transfer wETH to caller
        require(WETH.transfer(msg.sender, wethReceived), "Transfer failed");
    }

    /**
     * @notice Unwrap wETH to ETH with ZERO unwrapping fees
     */
    function unwrapWETH(uint256 amount) public returns (bool) {
        // Transfer wETH from caller
        require(WETH.transferFrom(msg.sender, address(this), amount), "Transfer failed");

        // Zero-fee unwrapping
        bool success = WRAPPER.unwrap(amount);
        require(success, "Unwrap failed");

        // Send ETH to caller
        payable(msg.sender).transfer(amount);
        return true;
    }

    /**
     * @notice Check if wrapping rate is favorable
     */
    function checkWrapRate() public view returns (uint256 rate) {
        rate = WRAPPER.getWrapRate();
        require(rate >= 0.99e18, "Unfavorable rate");
    }
}
```

---

### 5. XCM Bridge - Cross-Chain Transfers

**Novel Feature:** Atomic cross-chain transfers to FlareChain.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridBridge.sol";

contract CrossChainVault {
    IEtridBridge constant BRIDGE = IEtridBridge(0x0000000000000000000000000000000000000804);

    mapping(bytes32 => bool) public pendingBridges;

    /**
     * @notice Deposit and bridge to FlareChain
     */
    function depositAndBridge(uint256 amount) public payable {
        require(msg.value == amount, "Incorrect value");

        // Bridge to FlareChain via XCM
        bytes32 messageId = BRIDGE.bridgeToFlareChain{value: amount}(amount);

        pendingBridges[messageId] = true;
        emit BridgeInitiated(msg.sender, amount, messageId);
    }

    /**
     * @notice Check bridge completion status
     */
    function checkBridgeStatus(bytes32 messageId) public view returns (string memory) {
        (uint8 status, uint256 amount) = BRIDGE.getBridgeStatus(messageId);

        if (status == 0) return "Pending";
        if (status == 1) return "Confirmed";
        if (status == 2) return "Failed";
        return "Unknown";
    }

    /**
     * @notice Get total liquidity locked in bridge
     */
    function getTotalBridgedLiquidity() public view returns (uint256) {
        return BRIDGE.getTotalBridgedToFlareChain();
    }

    event BridgeInitiated(address indexed user, uint256 amount, bytes32 messageId);
}
```

---

### 6. Token Registry - Auto-Discovery

**Novel Feature:** Automatically fetch Ethereum token metadata without oracles.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridTokenRegistry.sol";

contract CrossChainDEX {
    IEtridTokenRegistry constant REGISTRY = IEtridTokenRegistry(0x0000000000000000000000000000000000000805);

    /**
     * @notice Register USDC from Ethereum mainnet
     * @dev Automatically fetches name, symbol, decimals from L1
     */
    function registerUSDC() public {
        address USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48; // Ethereum USDC

        bool success = REGISTRY.registerToken(USDC);
        require(success, "Registration failed");

        // Token metadata now available
        (string memory name, string memory symbol, uint8 decimals,) = REGISTRY.getTokenInfo(USDC);

        require(keccak256(bytes(symbol)) == keccak256("USDC"), "Wrong token");
        require(decimals == 6, "Wrong decimals");
    }

    /**
     * @notice List all bridged tokens
     */
    function listBridgedTokens() public view returns (address[] memory) {
        return REGISTRY.getBridgedTokens();
    }

    /**
     * @notice Verify token before trading
     */
    function verifyToken(address token) public view returns (bool) {
        if (!REGISTRY.isTokenRegistered(token)) {
            return false;
        }

        (,, uint8 decimals, uint256 supply) = REGISTRY.getTokenInfo(token);

        // Verify sane parameters
        return decimals > 0 && decimals <= 18 && supply > 0;
    }
}
```

---

### 7. State Proof - Trustless Verification

**Novel Feature:** Verify Ethereum state without external oracles.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridStateProof.sol";

contract TrustlessBridge {
    IEtridStateProof constant STATE_PROOF = IEtridStateProof(0x0000000000000000000000000000000000000806);

    /**
     * @notice Verify user's Ethereum USDC balance before bridging
     * @param proof Merkle proof from Ethereum state
     * @param userAddress User's Ethereum address
     * @param claimedBalance Claimed USDC balance
     */
    function verifyEthereumBalance(
        bytes32[] calldata proof,
        address userAddress,
        uint256 claimedBalance
    ) public view returns (bool) {
        // Get latest Ethereum state root
        (,, bytes32 stateRoot,) = STATE_PROOF.getLatestEthBlock();

        // Compute storage key for USDC balance
        address USDC = 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
        bytes32 key = keccak256(abi.encode(userAddress, 0)); // Slot 0 = balances mapping

        // Encode claimed balance
        bytes memory value = abi.encode(claimedBalance);

        // Verify proof
        return STATE_PROOF.verifyStateProof(stateRoot, proof, key, value);
    }

    /**
     * @notice Check Ethereum block freshness
     */
    function isEthereumDataFresh() public view returns (bool) {
        (,, , uint256 timestamp) = STATE_PROOF.getLatestEthBlock();
        return block.timestamp - timestamp < 1 hours;
    }
}
```

---

## Advanced Example: Multi-Chain Collateral Lending

Combining multiple precompiles for a novel DeFi use case:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "./IEtridOracle.sol";
import "./IEtridBridge.sol";
import "./IEtridStateProof.sol";

/**
 * @title MultiChainCollateralLending
 * @notice Novel Feature: Use BTC, SOL, XRP on other PBCs as collateral
 *         to borrow ETH on ETH PBC
 */
contract MultiChainCollateralLending {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);
    IEtridBridge constant BRIDGE = IEtridBridge(0x0000000000000000000000000000000000000804);

    struct Position {
        uint256 btcCollateral;  // On BTC PBC
        uint256 solCollateral;  // On SOL PBC
        uint256 ethBorrowed;    // On ETH PBC
    }

    mapping(address => Position) public positions;
    uint256 constant LTV = 70; // 70% loan-to-value

    /**
     * @notice Borrow ETH against multi-chain collateral
     * @dev Collateral verification happens via XCM messages
     */
    function borrow(uint256 ethAmount) public {
        uint256 collateralValueUSD = getCollateralValue(msg.sender);
        uint256 maxBorrow = (collateralValueUSD * LTV) / 100;

        // Convert max borrow from USD to ETH
        uint256 ethPriceUSD = ORACLE.getPrice(bytes32("ETH"), bytes32("USD"));
        uint256 maxBorrowETH = (maxBorrow * 1e18) / ethPriceUSD;

        require(
            positions[msg.sender].ethBorrowed + ethAmount <= maxBorrowETH,
            "Insufficient collateral"
        );

        positions[msg.sender].ethBorrowed += ethAmount;
        payable(msg.sender).transfer(ethAmount);
    }

    /**
     * @notice Calculate total collateral value in USD
     * @dev Uses free oracle queries for all assets
     */
    function getCollateralValue(address user) public view returns (uint256 valueUSD) {
        Position memory pos = positions[user];

        // Get prices from FlareChain oracle (free!)
        uint256 btcPriceUSD = ORACLE.getPrice(bytes32("BTC"), bytes32("USD"));
        uint256 solPriceUSD = ORACLE.getPrice(bytes32("SOL"), bytes32("USD"));

        // Calculate total value
        uint256 btcValueUSD = (pos.btcCollateral * btcPriceUSD) / 1e18;
        uint256 solValueUSD = (pos.solCollateral * solPriceUSD) / 1e18;

        return btcValueUSD + solValueUSD;
    }

    /**
     * @notice Check if position is healthy
     */
    function isPositionHealthy(address user) public view returns (bool) {
        Position memory pos = positions[user];
        if (pos.ethBorrowed == 0) return true;

        uint256 collateralValue = getCollateralValue(user);
        uint256 ethPriceUSD = ORACLE.getPrice(bytes32("ETH"), bytes32("USD"));
        uint256 borrowValueUSD = (pos.ethBorrowed * ethPriceUSD) / 1e18;

        // Position is healthy if borrowed < 70% of collateral
        return borrowValueUSD <= (collateralValue * LTV) / 100;
    }
}
```

---

## Testing

### Hardhat

```javascript
import { ethers } from "hardhat";

describe("Etrid Precompiles", function () {
  it("Should query BTC price from oracle", async function () {
    const oracle = await ethers.getContractAt(
      "IEtridOracle",
      "0x0000000000000000000000000000000000000800"
    );

    const btcSymbol = ethers.utils.formatBytes32String("BTC");
    const usdSymbol = ethers.utils.formatBytes32String("USD");

    const price = await oracle.getPrice(btcSymbol, usdSymbol);
    expect(price).to.be.gt(0);

    console.log(`BTC Price: $${ethers.utils.formatUnits(price, 18)}`);
  });
});
```

### Foundry

```solidity
// test/PrecompileTest.t.sol
pragma solidity ^0.8.0;

import "forge-std/Test.sol";
import "../src/IEtridOracle.sol";

contract PrecompileTest is Test {
    IEtridOracle oracle = IEtridOracle(0x0000000000000000000000000000000000000800);

    function testGetPrice() public {
        uint256 price = oracle.getPrice(bytes32("BTC"), bytes32("USD"));
        assertGt(price, 0);
    }
}
```

---

## Network Configuration

### Hardhat

```javascript
// hardhat.config.js
module.exports = {
  networks: {
    'eth-pbc': {
      url: 'http://localhost:9944',
      chainId: 1337,
      accounts: ['0x...']
    },
    'eth-pbc-testnet': {
      url: 'https://eth-pbc-testnet.etrid.org',
      chainId: 1338,
      accounts: ['0x...']
    }
  }
};
```

### Foundry

```toml
# foundry.toml
[profile.default]
eth_rpc_url = "http://localhost:9944"
chain_id = 1337

[rpc_endpoints]
eth_pbc = "http://localhost:9944"
eth_pbc_testnet = "https://eth-pbc-testnet.etrid.org"
```

---

## FAQ

**Q: Are precompile calls gas-efficient?**
A: Yes! Precompiles execute in native code, not EVM bytecode. Oracle queries, staking lookups, and wrapping operations cost minimal gas.

**Q: How are XCM responses handled?**
A: XCM responses are asynchronous. For queries (oracle, staking), responses are cached on ETH PBC for instant access. For transactions (governance votes, bridges), you must poll for completion.

**Q: Can I use these on Ethereum mainnet?**
A: No, these precompiles only exist on ETH PBC. On Ethereum mainnet, these addresses contain no code.

**Q: Are there any security audits?**
A: Precompile implementations will be audited before mainnet launch. Use on testnet only for now.

**Q: What happens if FlareChain is down?**
A: Cached oracle data remains available. New XCM messages will queue until FlareChain recovers.

---

## Support

- **Documentation:** https://docs.etrid.org/eth-pbc
- **Discord:** https://discord.gg/etrid
- **GitHub Issues:** https://github.com/etrid/solidity-interfaces/issues

---

**Built with ❤️ by the Etrid Foundation**

Version: 1.0.0
Last Updated: November 16, 2025
License: MIT
