# Ëtrid Smart Contract Examples

**Runtime**: EtwasmVM (WebAssembly-based)
**Language**: Rust → WASM
**Framework**: Substrate `pallet-contracts` compatible

---

## 📚 Examples Included

This folder contains example smart contracts for building on Ëtrid:

### 1. **hello-world** - Basic Contract
Learn: Contract structure, deployment, calling functions
- Simple "Hello World" contract
- No state storage
- Demonstrates basic contract anatomy

### 2. **counter** - State Management
Learn: Storage, state mutations, events
- Increment/decrement counter
- Persistent state storage
- Event emission

### 3. **erc20-token** - Fungible Token
Learn: Token standards, transfers, allowances
- ERC20-compatible token
- Balance tracking
- Transfer and approve functions
- Total supply management

### 4. **simple-dao** - Governance
Learn: Voting, proposals, access control
- Proposal creation
- Member voting
- Threshold-based execution
- Role-based permissions

### 5. **escrow** - Multi-Party Contracts
Learn: Conditional logic, multi-sig, refunds
- Buyer/seller escrow
- Arbiter resolution
- Timeout refunds
- Fund management

---

## 🛠️ Prerequisites

### Install Rust & WASM toolchain

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install cargo-contract
cargo install cargo-contract --force
```

### Install Substrate Contracts Node (for local testing)

```bash
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node --force
```

---

## 🚀 Quick Start

### 1. Build a Contract

```bash
# Navigate to example folder
cd hello-world

# Build contract
cargo contract build --release

# Output: target/ink/hello_world.wasm
```

### 2. Start Local Node

```bash
# Start substrate-contracts-node
substrate-contracts-node --dev --tmp

# RPC endpoint: ws://127.0.0.1:9944
```

### 3. Deploy Contract

```bash
# Deploy using cargo-contract
cargo contract instantiate \
  --constructor new \
  --args "Hello from Ëtrid!" \
  --suri //Alice \
  --salt $(date +%s)

# Returns: Contract address
```

### 4. Call Contract

```bash
# Call a function
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_message \
  --suri //Alice \
  --dry-run
```

---

## 📖 Contract Structure

Each contract follows this structure:

```
example-name/
├── Cargo.toml          ← Rust dependencies
├── lib.rs              ← Contract code
├── README.md           ← Example-specific docs
└── tests/
    └── integration.rs  ← Tests
```

---

## 🧪 Testing Contracts

### Unit Tests

```bash
cd example-name
cargo test
```

### Integration Tests

```bash
# Start local node first
substrate-contracts-node --dev --tmp

# Run integration tests
cargo test --test integration
```

---

## 📦 Deploying to Ember Testnet

### 1. Get Testnet ÉTR

Visit faucet: https://faucet.ember.etrid.org

### 2. Deploy Contract

```bash
cargo contract instantiate \
  --constructor new \
  --url wss://ember.etrid.org \
  --suri "your mnemonic phrase here" \
  --salt $(date +%s)
```

### 3. Interact via UI

Visit: https://contracts-ui.substrate.io/
- Connect to: wss://ember.etrid.org
- Upload contract metadata
- Call functions via UI

---

## 🎨 Using with Ëtrid SDK

### JavaScript Example

```javascript
import { EtridSDK } from 'js-etrid-sdk';

const sdk = new EtridSDK({
  provider: 'wss://ember.etrid.org'
});

// Deploy contract
const contract = await sdk.contracts.deploy({
  wasm: fs.readFileSync('./target/ink/hello_world.wasm'),
  metadata: require('./target/ink/metadata.json'),
  constructorName: 'new',
  args: ['Hello from Ëtrid!']
});

console.log('Contract address:', contract.address);

// Call contract
const result = await contract.call('get_message');
console.log('Message:', result);
```

---

## 📚 Learning Path

**Recommended order**:

1. **hello-world** → Understand contract basics
2. **counter** → Learn state management
3. **erc20-token** → Build fungible tokens
4. **simple-dao** → Implement governance
5. **escrow** → Create multi-party contracts

---

## 🔗 Resources

### Official Docs
- **ink! Documentation**: https://use.ink/
- **Substrate Contracts**: https://docs.substrate.io/tutorials/smart-contracts/
- **Ëtrid Docs**: https://docs.etrid.org (coming soon)

### Tutorials
- ink! Basics: https://use.ink/basics/
- Contract Testing: https://use.ink/basics/testing
- Cross-Contract Calls: https://use.ink/macros-attributes/cross-contract-call

### Community
- Discord: https://discord.gg/etrid
- Telegram: https://t.me/etrid
- GitHub: https://github.com/EojEdred/Etrid

---

## ⚠️ Important Notes

### Gas Limits (VMw)
Ëtrid uses **VMw (Virtual Machine Watts)** instead of traditional gas:
- Simple call: ~100 VMw
- Storage write: ~2,000 VMw
- Complex computation: ~10,000+ VMw

See: `docs/EMBER_TESTNET_ARCHITECTURE.md` for VMw details

### Storage Costs
- Storage rent applies (pay-per-byte)
- Use efficient data structures
- Consider off-chain storage for large data

### Security Best Practices
- ✅ Use `#[ink(payable)]` for functions receiving funds
- ✅ Check caller permissions (`self.env().caller()`)
- ✅ Validate inputs before state changes
- ✅ Use `Result<T, E>` for error handling
- ✅ Emit events for important state changes
- ❌ Avoid reentrancy (use checks-effects-interactions pattern)
- ❌ Avoid unbounded loops
- ❌ Don't store sensitive data on-chain

---

## 🆘 Troubleshooting

### "Contract trap during execution"
**Cause**: Out of gas, panic, or assertion failed
**Solution**: Increase gas limit or fix contract logic

### "Storage deposit too low"
**Cause**: Not enough balance to cover storage rent
**Solution**: Fund account or reduce storage usage

### "Contract not found"
**Cause**: Wrong contract address or wrong chain
**Solution**: Verify address and RPC endpoint

---

## 🎯 Next Steps

After completing these examples:

1. **Build Your Own Contract**
   - Use these as templates
   - Combine patterns from multiple examples
   - Test thoroughly before deploying

2. **Deploy to Ember Testnet**
   - Get testnet ÉTR from faucet
   - Deploy and test your contract
   - Get feedback from community

3. **Prepare for Mainnet**
   - Security audit (if handling funds)
   - Comprehensive testing
   - Documentation for users

---

## 📝 Contributing

Found a bug? Have a suggestion?
- Open an issue: https://github.com/EojEdred/Etrid/issues
- Submit a PR: https://github.com/EojEdred/Etrid/pulls

---

**Happy Building! 🚀**

*These examples are MIT licensed. Use them freely in your projects.*
