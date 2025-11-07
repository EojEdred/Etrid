# ETH-PBC Capabilities & Roadmap

## Current Status: ✅ Fully Functional EVM (Q4 2025)

ETH-PBC is **production-ready** for standard Ethereum development. Developers can deploy Solidity contracts, use MetaMask, and interact with web3.js/ethers.js **today**.

---

## ✅ What's IMPLEMENTED (Working Now)

### 1. Full EVM Support
- ✅ Solidity smart contract deployment
- ✅ EVM bytecode execution
- ✅ Gas metering and fee market
- ✅ Contract state storage
- ✅ Events and logs

### 2. Ethereum Compatibility
- ✅ Ethereum JSON-RPC API (all methods)
- ✅ MetaMask wallet support
- ✅ web3.js and ethers.js libraries
- ✅ Ethereum transaction format
- ✅ EIP-1559 dynamic fees

### 3. Standard Precompiles (Ethereum Native)
All standard Ethereum precompiles are working:

| Address | Precompile | Purpose | Status |
|---------|-----------|---------|--------|
| `0x01` | ECRecover | Signature recovery from ECDSA | ✅ Working |
| `0x02` | SHA256 | SHA-256 hash function | ✅ Working |
| `0x03` | RIPEMD160 | RIPEMD-160 hash function | ✅ Working |
| `0x04` | Identity | Data copy function | ✅ Working |
| `0x05` | Modexp | Modular exponentiation | ✅ Working |
| `0x08` | SHA3FIPS256 | SHA3 FIPS-256 hash | ✅ Working |

**Usage Example**:
```solidity
// Standard Ethereum precompiles work exactly as expected
contract Example {
    function recoverSigner(bytes32 hash, bytes memory sig) public pure returns (address) {
        // ECRecover precompile at 0x01
        return ecrecover(hash, v, r, s);
    }

    function hashData(bytes memory data) public pure returns (bytes32) {
        // SHA256 precompile at 0x02
        return sha256(data);
    }
}
```

### 4. Advanced Features

#### EIP-7702 Authorization Lists ✅ SUPPORTED
ETH-PBC supports **EIP-7702** (Set EOA account code for one transaction):

**What is EIP-7702?**
Allows regular wallets (EOAs) to temporarily delegate execution to smart contract code for a single transaction. This enables:
- **Batch transactions**: Combine multiple actions into one
- **Gas abstraction**: Pay gas in tokens other than ETR
- **Social recovery**: Implement recovery logic without converting to contract wallet
- **Temporary permissions**: Grant limited access without full control

**Use Case Example**:
```javascript
// User wants to swap tokens but needs to approve first
// Without EIP-7702: 2 transactions (approve + swap)
// With EIP-7702: 1 transaction (delegated execution)

const authorizationList = [{
  chainId: 3141,  // ETH-PBC chain ID
  address: dexContractAddress,  // Delegate to DEX contract
  nonce: 0,
  yParity: 0,
  r: "0x...",
  s: "0x..."
}];

// Single transaction that both approves and swaps
const tx = {
  to: dexAddress,
  data: dex.interface.encodeFunctionData("swapWithApproval", [usdc, eth, amount]),
  authorizationList: authorizationList
};

await wallet.sendTransaction(tx);
```

**Status**: ✅ Fully implemented via `ethereum 0.18+` and `frontier-stable2506`

---

## ⏸️ What's PLANNED (Future Work)

### 1. Custom Ëtrid Precompiles (XCM Bridge)

These are **NOT yet implemented** but are planned for cross-chain interoperability between ETH-PBC and FlareChain:

| Address | Precompile | Purpose | Status |
|---------|-----------|---------|--------|
| `0x800` | **IEtridOracle** | Access FlareChain oracle price feeds | ⏸️ Planned |
| `0x801` | **IEtridGovernance** | Submit proposals to FlareChain governance | ⏸️ Planned |
| `0x802` | **IEtridStaking** | Query FlareChain validator/staking info | ⏸️ Planned |
| `0x803` | **IEtridXCM** | Generic cross-chain messaging | ⏸️ Planned |

#### Planned Usage (Future):
```solidity
// Future: Access FlareChain oracle from Solidity
interface IEtridOracle {
    function getPriceInETH(bytes32 symbol) external view returns (uint256);
    function getPrice(bytes32 symbol, bytes32 quoteCurrency) external view returns (uint256);
    function getLastUpdate(bytes32 symbol) external view returns (uint256);
}

contract FlareSwap {
    IEtridOracle constant ORACLE = IEtridOracle(0x0000000000000000000000000000000000000800);

    function swap(address tokenIn, address tokenOut, uint256 amountIn) public {
        // Get real-time price from FlareChain oracle
        uint256 btcPrice = ORACLE.getPriceInETH("BTC");
        uint256 ethPrice = ORACLE.getPriceInETH("ETH");

        // Calculate swap based on oracle prices
        uint256 amountOut = calculateSwap(btcPrice, ethPrice, amountIn);

        // Execute swap
        _executeSwap(tokenIn, tokenOut, amountIn, amountOut);
    }
}
```

```solidity
// Future: Submit governance proposal from Solidity
interface IEtridGovernance {
    function submitProposal(string memory title, string memory description) external returns (uint256 proposalId);
    function voteOnProposal(uint256 proposalId, bool support) external;
}

contract DAOGovernance {
    IEtridGovernance constant GOV = IEtridGovernance(0x0000000000000000000000000000000000000801);

    function proposeUpgrade(string memory description) public onlyMember {
        // Submit to FlareChain governance
        uint256 proposalId = GOV.submitProposal("ETH-PBC Upgrade", description);
        emit ProposalSubmitted(proposalId);
    }
}
```

```solidity
// Future: Query FlareChain staking info
interface IEtridStaking {
    function getValidatorStake(bytes32 validator) external view returns (uint256);
    function isValidatorActive(bytes32 validator) external view returns (bool);
    function getTotalStaked() external view returns (uint256);
}

contract StakingRewards {
    IEtridStaking constant STAKING = IEtridStaking(0x0000000000000000000000000000000000000802);

    function calculateReward(address user, bytes32 validator) public view returns (uint256) {
        // Check if validator is active on FlareChain
        require(STAKING.isValidatorActive(validator), "Validator not active");

        // Get validator's total stake
        uint256 validatorStake = STAKING.getValidatorStake(validator);

        // Calculate proportional reward
        return userStake * rewardRate / validatorStake;
    }
}
```

### 2. Implementation Requirements

To implement custom Ëtrid precompiles, we need:

1. **XCM Bridge Setup** (Cross-Consensus Messaging)
   - Configure message passing between ETH-PBC and FlareChain
   - Set up HRMP channels (if using Polkadot relay)
   - Implement message handlers on both chains

2. **Precompile Implementation**
   ```rust
   // Example: Oracle precompile implementation
   // Location: eth-pbc/runtime/src/precompiles/oracle.rs

   pub struct EtridOraclePrecompile<R>(PhantomData<R>);

   impl<R: pallet_evm::Config> Precompile for EtridOraclePrecompile<R> {
       fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
           // Parse input: symbol to query
           let symbol = handle.read_bytes(0, 32)?;

           // Send XCM message to FlareChain oracle
           let xcm_msg = Xcm(vec![
               QueryResponse {
                   query_id: 0,
                   response: Response::Assets(/* ... */),
               }
           ]);

           // Send via XCM bridge
           pallet_xcm_bridge::send_xcm(
               MultiLocation::Parent,  // FlareChain
               xcm_msg
           )?;

           // Wait for response (async handling needed)
           let price = pallet_xcm_bridge::get_response()?;

           // Return price to EVM
           Ok(PrecompileOutput {
               exit_status: ExitSucceed::Returned,
               output: price.encode(),
           })
       }
   }
   ```

3. **Testing & Security**
   - Test XCM message delivery
   - Implement timeout handling
   - Add access controls (who can call?)
   - Benchmark gas costs

---

## 🚀 Current Capabilities (Production Ready)

### What You Can Do TODAY on ETH-PBC:

✅ **Deploy Solidity Contracts**
```bash
# Use Hardhat, Truffle, or Foundry
npx hardhat deploy --network etrid-eth-pbc
```

✅ **Connect MetaMask**
```javascript
await ethereum.request({
  method: 'wallet_addEthereumChain',
  params: [{
    chainId: '0xC45',  // 3141 in hex (example)
    chainName: 'Ëtrid ETH-PBC',
    rpcUrls: ['https://eth-pbc.etrid.io'],
    nativeCurrency: { name: 'ETR', symbol: 'ETR', decimals: 18 }
  }]
});
```

✅ **Deploy DeFi Protocols**
```solidity
// Standard Uniswap V2, Aave, Compound, etc. work out of the box
contract FlareSwap is IUniswapV2Router {
    // Full Ethereum compatibility
}
```

✅ **Use EIP-7702 Features**
```javascript
// Batch transactions with authorization lists
const tx = {
  authorizationList: [{ chainId, address, nonce, signature }]
};
```

---

## ❌ What You CANNOT Do Yet (Requires XCM Precompiles)

❌ **Access FlareChain Oracle from Solidity**
```solidity
// This will NOT work until precompile 0x800 is implemented:
IEtridOracle oracle = IEtridOracle(0x800);
uint256 price = oracle.getPriceInETH("BTC");  // ❌ Not yet available
```

**Workaround**: Deploy your own oracle contract on ETH-PBC or use Chainlink oracles

❌ **Submit FlareChain Governance Proposals from Solidity**
```solidity
// This will NOT work until precompile 0x801 is implemented:
IEtridGovernance gov = IEtridGovernance(0x801);
gov.submitProposal("...");  // ❌ Not yet available
```

**Workaround**: Submit proposals directly to FlareChain via Polkadot.js

❌ **Query FlareChain Staking Info from Solidity**
```solidity
// This will NOT work until precompile 0x802 is implemented:
IEtridStaking staking = IEtridStaking(0x802);
uint256 stake = staking.getValidatorStake(validator);  // ❌ Not yet available
```

**Workaround**: Query FlareChain RPC directly from frontend

---

## 📅 Roadmap

### Phase 1: ✅ **EVM Foundation (COMPLETE)**
- ✅ Full EVM support
- ✅ Standard Ethereum precompiles
- ✅ MetaMask compatibility
- ✅ EIP-7702 authorization lists

### Phase 2: ⏸️ **XCM Integration (PLANNED - Q1 2026)**
- ⏸️ Set up XCM bridge between ETH-PBC and FlareChain
- ⏸️ Implement custom precompile 0x800 (Oracle)
- ⏸️ Implement custom precompile 0x801 (Governance)
- ⏸️ Implement custom precompile 0x802 (Staking)
- ⏸️ Test cross-chain message passing
- ⏸️ Security audit

### Phase 3: 🔮 **Advanced Features (FUTURE)**
- 🔮 Precompile 0x803 (Generic XCM messaging)
- 🔮 Asset transfers ETH-PBC ↔ FlareChain
- 🔮 Cross-chain DEX (swap assets across chains)
- 🔮 Bridge to other PBCs (BTC-PBC, SOL-PBC, etc.)

---

## 🎯 Summary

**Current State (2025-11-05)**:

| Feature | Status | Notes |
|---------|--------|-------|
| **EVM Execution** | ✅ Working | Full Solidity support |
| **MetaMask** | ✅ Working | Connect and deploy |
| **Standard Precompiles** | ✅ Working | All 6 Ethereum precompiles |
| **EIP-7702** | ✅ Working | Authorization lists supported |
| **Ethereum JSON-RPC** | ✅ Working | All endpoints |
| **Custom Ëtrid Precompiles** | ⏸️ Planned | Need XCM bridge setup |
| **FlareChain Oracle Access** | ⏸️ Planned | Precompile 0x800 |
| **FlareChain Governance Access** | ⏸️ Planned | Precompile 0x801 |
| **FlareChain Staking Queries** | ⏸️ Planned | Precompile 0x802 |

**Bottom Line**:
- ✅ **ETH-PBC is production-ready** for standard Ethereum dApps
- ⏸️ **XCM precompiles are planned** for cross-chain FlareChain integration
- 🔮 **Future enhancements** will add more advanced features

---

## 📚 References

- [EVM Architecture](./EVM_ARCHITECTURE.md) - Why EVM is on ETH-PBC, not FlareChain
- [Frontier Documentation](https://github.com/polkadot-evm/frontier) - EVM integration
- [EIP-7702 Specification](https://eips.ethereum.org/EIPS/eip-7702) - Authorization lists
- [XCM Documentation](https://wiki.polkadot.network/docs/learn-xcm) - Cross-chain messaging

**Document Version**: 1.0
**Last Updated**: 2025-11-05
**Status**: Current capabilities documented ✅
