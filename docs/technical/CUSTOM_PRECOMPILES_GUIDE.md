# Custom Ëtrid Precompiles Guide

## Overview

ETH-PBC provides **three custom precompiles** that allow Solidity smart contracts to interact with FlareChain services via XCM (Cross-Consensus Messaging). These precompiles bridge the EVM and Substrate ecosystems.

---

## Available Precompiles

| Address | Interface | Purpose | Status |
|---------|-----------|---------|--------|
| `0x800` | IEtridOracle | Access FlareChain oracle price feeds | ✅ Implemented |
| `0x801` | IEtridGovernance | Participate in FlareChain governance | ✅ Implemented |
| `0x802` | IEtridStaking | Query FlareChain validator/staking data | ✅ Implemented |

---

## 1. IEtridOracle (0x800)

### Purpose
Access real-time price feeds from the FlareChain oracle network.

### Solidity Interface
```solidity
interface IEtridOracle {
    function getPriceInETH(bytes32 symbol) external view returns (uint256 price);
    function getPrice(bytes32 symbol, bytes32 quoteCurrency) external view returns (uint256 price);
    function getLastUpdate(bytes32 symbol) external view returns (uint256 timestamp);
}
```

### Usage Example
```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

contract FlareSwap {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    function swap(bytes32 tokenIn, bytes32 tokenOut, uint256 amountIn)
        external
        returns (uint256 amountOut)
    {
        // Get real-time prices from FlareChain
        uint256 priceIn = ORACLE.getPriceInETH(tokenIn);
        uint256 priceOut = ORACLE.getPriceInETH(tokenOut);

        // Calculate output amount
        amountOut = (amountIn * priceIn) / priceOut;

        return amountOut;
    }
}
```

### Supported Symbols
- **Cryptocurrencies**: `"BTC"`, `"ETH"`, `"SOL"`, `"XRP"`, `"ADA"`, etc.
- **Fiat**: `"USD"`, `"EUR"`, `"GBP"`, etc.
- **Custom**: Any symbol supported by FlareChain oracle

### Price Format
- All prices are returned scaled by **1e18** for precision
- Example: BTC price of $50,000 = `50000000000000000000000` (50000 * 1e18)

### Notes
- ✅ **View function** - No gas cost for queries (until XCM is implemented)
- ⏸️ **Current**: Uses mock data for development
- 🔮 **Future**: Will query FlareChain oracle via XCM

---

## 2. IEtridGovernance (0x801)

### Purpose
Submit proposals and vote on FlareChain governance from Solidity contracts.

### Solidity Interface
```solidity
interface IEtridGovernance {
    function submitProposal(string memory title, string memory description)
        external
        returns (uint256 proposalId);

    function voteOnProposal(uint256 proposalId, bool support) external;

    function getProposalStatus(uint256 proposalId) external view returns (uint8 status);
}
```

### Proposal Status Values
- `0` - **Pending**: Proposal submitted, not yet active
- `1` - **Active**: Voting in progress
- `2` - **Passed**: Proposal approved and executed
- `3` - **Failed**: Proposal rejected or expired

### Usage Example
```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

contract DAOGovernance {
    IEtridGovernance constant GOV = IEtridGovernance(0x0000000000000000000000000000000000000801);

    mapping(address => bool) public members;

    function submitUpgradeProposal(string memory description)
        external
        returns (uint256 proposalId)
    {
        require(members[msg.sender], "Not a member");

        // Submit to FlareChain governance
        proposalId = GOV.submitProposal(
            "ETH-PBC Feature Upgrade",
            description
        );

        return proposalId;
    }

    function voteOnProposal(uint256 proposalId, bool support) external {
        require(members[msg.sender], "Not a member");

        // Vote on FlareChain
        GOV.voteOnProposal(proposalId, support);
    }

    function checkIfPassed(uint256 proposalId) external view returns (bool) {
        uint8 status = GOV.getProposalStatus(proposalId);
        return status == 2; // Passed
    }
}
```

### Requirements
- **Title**: Non-empty, max 256 characters
- **Description**: Non-empty, max 10,000 characters
- **Caller**: Must have sufficient stake on FlareChain (enforced via XCM)

### Notes
- ✅ **Write function** - Costs gas to submit/vote
- ⏸️ **Current**: Uses mock data for development
- 🔮 **Future**: Will submit to FlareChain governance via XCM

---

## 3. IEtridStaking (0x802)

### Purpose
Query FlareChain validator and staking information.

### Solidity Interface
```solidity
interface IEtridStaking {
    function getValidatorStake(bytes32 validatorId) external view returns (uint256 stake);
    function isValidatorActive(bytes32 validatorId) external view returns (bool active);
    function getTotalStaked() external view returns (uint256 totalStaked);
    function getValidatorCount() external view returns (uint256 count);
}
```

### Usage Example
```solidity
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

contract StakingRewards {
    IEtridStaking constant STAKING = IEtridStaking(0x0000000000000000000000000000000000000802);

    function calculateReward(address user, bytes32 validator)
        public
        view
        returns (uint256 reward)
    {
        // Check validator is active
        require(STAKING.isValidatorActive(validator), "Validator inactive");

        // Get validator's total stake
        uint256 validatorStake = STAKING.getValidatorStake(validator);

        // Get user's stake (from contract storage)
        uint256 userStake = getUserStake(user);

        // Calculate proportional reward
        uint256 totalRewards = 1000 ether; // Example
        reward = (userStake * totalRewards) / validatorStake;

        return reward;
    }

    function getNetworkStats()
        external
        view
        returns (uint256 totalStaked, uint256 validatorCount)
    {
        totalStaked = STAKING.getTotalStaked();
        validatorCount = STAKING.getValidatorCount();
    }
}
```

### Return Values
- **Stake amounts**: In wei (1 ETR = 1e18 wei)
- **Active status**: `true` if validator is active, `false` otherwise
- **Validator ID**: 32-byte identifier

### Notes
- ✅ **View function** - No gas cost for queries
- ⏸️ **Current**: Uses mock data for development
- 🔮 **Future**: Will query FlareChain staking pallet via XCM

---

## Development Workflow

### 1. Install Solidity Interfaces
```bash
# Copy interfaces to your Hardhat/Foundry project
cp -r eth-pbc/solidity-interfaces/I*.sol contracts/interfaces/
```

### 2. Import in Your Contract
```solidity
import "./interfaces/IEtridOracle.sol";
import "./interfaces/IEtridGovernance.sol";
import "./interfaces/IEtridStaking.sol";
```

### 3. Use Precompiles
```solidity
contract MyDApp {
    IEtridOracle private constant ORACLE =
        IEtridOracle(0x0000000000000000000000000000000000000800);

    function getPrice(bytes32 symbol) external view returns (uint256) {
        return ORACLE.getPriceInETH(symbol);
    }
}
```

### 4. Deploy to ETH-PBC
```bash
# Using Hardhat
npx hardhat deploy --network etrid-eth-pbc

# Using Foundry
forge create MyDApp --rpc-url https://eth-pbc.etrid.io
```

---

## Testing

### Local Testing (Mock Mode)
All precompiles currently use **mock data** for testing:

```javascript
// Test in Hardhat
describe("Oracle Precompile", function() {
    it("Should return BTC price", async function() {
        const oracle = await ethers.getContractAt(
            "IEtridOracle",
            "0x0000000000000000000000000000000000000800"
        );

        const price = await oracle.getPriceInETH(
            ethers.utils.formatBytes32String("BTC")
        );

        expect(price).to.be.gt(0);
    });
});
```

### Mock Data Values
- **BTC**: $50,000 (16.67 ETH)
- **ETH**: $3,000 (1 ETH)
- **SOL**: $100 (0.0333 ETH)
- **XRP**: $1 (0.000333 ETH)
- **Total Staked**: 1,000,000 ETR
- **Validator Count**: 100
- **Proposal ID**: Always returns 42

---

## Production XCM Integration (Future)

### How It Will Work

When XCM bridge is fully implemented:

```rust
// 1. Solidity contract calls precompile
oracle.getPriceInETH("BTC")

// 2. Precompile encodes XCM message
XcmMessage {
    query: OraclePrice { symbol: "BTC", quote: "ETH" }
}

// 3. Send to FlareChain via XCM
send_xcm(FlareChain, XcmMessage)

// 4. FlareChain oracle pallet responds
OracleResponse { price: 16_670_000_000_000_000_000 }

// 5. Precompile returns to Solidity
return 16_670_000_000_000_000_000
```

### Implementation Checklist

- [x] Precompile implementations (oracle, governance, staking)
- [x] XCM bridge infrastructure (mock)
- [x] Solidity interfaces
- [x] Example contracts
- [ ] XCM message encoding/decoding
- [ ] HRMP channel setup (FlareChain ↔ ETH-PBC)
- [ ] Response callback mechanism
- [ ] Gas cost optimization
- [ ] Security audit

---

## Examples

Full example contracts are available at:
- `/eth-pbc/solidity-interfaces/examples/FlareSwapExample.sol`
- `/eth-pbc/solidity-interfaces/examples/DAOGovernanceExample.sol`
- `/eth-pbc/solidity-interfaces/examples/StakingRewardsExample.sol`

---

## Gas Costs

### Current (Mock Mode)
- **View functions**: ~2,100 gas
- **Write functions**: ~21,000 gas

### Future (XCM Mode)
- **View functions**: ~100,000 gas (includes XCM round-trip)
- **Write functions**: ~200,000 gas (includes XCM submission)

---

## Security Considerations

1. **Oracle Freshness**: Always check `getLastUpdate()` to ensure data isn't stale
2. **Governance Authorization**: Only authorized addresses can submit proposals
3. **Validator Selection**: Verify validator is active before delegation
4. **Price Manipulation**: Use time-weighted average prices for critical operations
5. **XCM Timeouts**: Handle cases where XCM messages fail to deliver

---

## Troubleshooting

### "Unknown function selector" Error
- **Cause**: Incorrect function signature
- **Fix**: Verify ABI matches interface exactly

### "Validator not active" Error
- **Cause**: Validator is slashed, jailed, or has insufficient stake
- **Fix**: Query `isValidatorActive()` before delegation

### "Proposal not found" Error
- **Cause**: Proposal ID doesn't exist
- **Fix**: Check `getProposalStatus()` returns valid status

---

## References

- [EVM Architecture](./EVM_ARCHITECTURE.md)
- [XCM Documentation](https://wiki.polkadot.network/docs/learn-xcm)
- [Frontier Precompiles](https://github.com/polkadot-evm/frontier/tree/master/precompiles)
- [Solidity ABI Encoding](https://docs.soliditylang.org/en/latest/abi-spec.html)

---

**Document Version**: 1.0
**Last Updated**: 2025-11-05
**Status**: ✅ Implemented (Mock Mode) | ⏸️ XCM Integration Pending
