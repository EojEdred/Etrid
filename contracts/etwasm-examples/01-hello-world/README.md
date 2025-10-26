# Hello World Contract

**Difficulty**: ⭐ Beginner
**Time to Complete**: 15-30 minutes

---

## 📖 What You'll Learn

This is the simplest possible Ëtrid smart contract. You'll learn:
- ✅ Basic contract structure
- ✅ Storage (state variables)
- ✅ Constructors
- ✅ Getter functions (read-only)
- ✅ Setter functions (mutable)
- ✅ Events
- ✅ Error handling
- ✅ Unit testing

---

## 🏗️ Contract Overview

The Hello World contract stores a simple greeting message and tracks how many times it's been changed.

### Storage
```rust
message: String         // The greeting message
change_count: u32       // How many times it was changed
```

### Functions

| Function | Type | Description |
|----------|------|-------------|
| `new(init_message)` | Constructor | Create contract with custom message |
| `default()` | Constructor | Create contract with "Hello, World!" |
| `get_message()` | Query | Get the current message |
| `get_change_count()` | Query | Get how many times message was changed |
| `set_message(new_message)` | Transaction | Update the message |
| `reset()` | Transaction | Reset to default message |

### Events
- `MessageChanged` - Emitted when message is updated

---

## 🚀 Quick Start

### 1. Build the Contract

```bash
cd 01-hello-world
cargo contract build --release
```

**Output**: `target/ink/hello_world.wasm` + metadata

### 2. Start Local Node (Optional)

```bash
substrate-contracts-node --dev --tmp
```

### 3. Deploy Contract

```bash
# Deploy with default message
cargo contract instantiate \
  --constructor default \
  --suri //Alice

# Or with custom message
cargo contract instantiate \
  --constructor new \
  --args "Hello from Ëtrid!" \
  --suri //Alice
```

### 4. Interact with Contract

```bash
# Get the message (read-only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_message \
  --suri //Alice \
  --dry-run

# Set a new message (transaction)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message set_message \
  --args "New greeting!" \
  --suri //Alice

# Get change count
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message get_change_count \
  --suri //Alice \
  --dry-run
```

---

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

**Expected output**: All 7 tests pass

### Run Integration Tests (E2E)

```bash
# Start local node first
substrate-contracts-node --dev --tmp

# Run E2E tests
cargo test --features e2e-tests
```

---

## 📝 Code Walkthrough

### Contract Structure

```rust
#[ink::contract]
mod hello_world {
    // Storage definition
    #[ink(storage)]
    pub struct HelloWorld {
        message: String,
        change_count: u32,
    }

    // Constructor
    #[ink(constructor)]
    pub fn new(init_message: String) -> Self {
        Self {
            message: init_message,
            change_count: 0,
        }
    }

    // Read-only function
    #[ink(message)]
    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    // Mutable function
    #[ink(message)]
    pub fn set_message(&mut self, new_message: String) -> Result<()> {
        // Validate input
        if new_message.is_empty() {
            return Err(Error::EmptyMessage);
        }

        // Update state
        self.message = new_message.clone();
        self.change_count += 1;

        // Emit event
        self.env().emit_event(MessageChanged {
            from: Some(self.env().caller()),
            old_message,
            new_message,
        });

        Ok(())
    }
}
```

### Key Concepts

#### Storage (`#[ink(storage)]`)
- Stores persistent data on-chain
- Costs storage rent (pay-per-byte)
- Use efficient types (`u32` vs `u128`)

#### Constructor (`#[ink(constructor)]`)
- Called once when contract is deployed
- Initializes storage
- Can have multiple constructors

#### Message (`#[ink(message)]`)
- Functions callable from outside
- `&self` = read-only (no gas for state reads)
- `&mut self` = mutable (gas charged for writes)

#### Events (`#[ink(event)]`)
- Logged to blockchain
- Can be indexed by `#[ink(topic)]`
- Useful for tracking history off-chain

#### Error Handling
- Use `Result<T, E>` for fallible operations
- Define custom error types
- Return errors instead of panicking (saves gas)

---

## 💡 Try It Yourself

### Challenge 1: Add a Counter
Add a function `increment()` that increases a counter each time it's called.

### Challenge 2: Add Access Control
Make `set_message()` only callable by the contract owner.

### Challenge 3: Add a Prefix
Add a function `add_prefix(prefix: String)` that prepends text to the message.

### Challenge 4: Store Multiple Messages
Instead of one message, store a `Vec<String>` of messages with timestamps.

---

## 🔍 Gas Costs (VMw)

Typical costs on Ëtrid:

| Operation | VMw Cost | Notes |
|-----------|----------|-------|
| Deploy contract | ~50,000 | One-time cost |
| `get_message()` | ~100 | Read-only, nearly free |
| `set_message()` | ~2,500 | Writes to storage |
| `reset()` | ~2,500 | Similar to set_message |

**Tip**: Use `--dry-run` to estimate gas before spending ÉTR.

---

## 🐛 Common Issues

### "Error: Failed to instantiate"
**Cause**: Insufficient balance for storage deposit
**Solution**: Fund your account with testnet ÉTR

### "Error: Module not found"
**Cause**: Forgot to build contract first
**Solution**: Run `cargo contract build --release`

### "Test failed: assertion failed"
**Cause**: Logic error in contract
**Solution**: Review test output, fix logic, re-test

---

## 📚 Next Steps

After mastering this contract:
1. **Move to Counter** (`02-counter`) - Learn more advanced state management
2. **Try modifying this contract** - Add your own functions
3. **Deploy to Ember testnet** - Test on live network

---

## 📖 Resources

- ink! Documentation: https://use.ink/
- Substrate Contracts: https://docs.substrate.io/tutorials/smart-contracts/
- Ëtrid Docs: https://docs.etrid.org

---

**Questions?** Ask in Discord: https://discord.gg/etrid

**Happy Coding! 🎉**
