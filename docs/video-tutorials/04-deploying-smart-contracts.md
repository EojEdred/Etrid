# Video Tutorial 04: Deploying Smart Contracts on Etrid

**Duration:** 12 minutes
**Target Audience:** Developers familiar with programming, new to blockchain smart contracts
**Prerequisites:** Tutorial 01 completed, basic programming knowledge (Rust helpful but not required)

---

## Script Overview

This tutorial guides developers through the complete process of creating, testing, and deploying a smart contract on the Etrid blockchain using ink! (Rust-based smart contract language).

---

## Time Markers & Script

### 00:00 - 00:45 | Introduction & What We'll Build

**NARRATION:**
"Welcome to Etrid smart contract development! In this 12-minute tutorial, you'll write, test, and deploy your first smart contract on the Etrid blockchain. We'll build a simple 'Message Board' contract where users can post and read messages on-chain.

By the end of this tutorial, you'll understand the complete smart contract lifecycle: writing the contract in ink!, testing it locally, deploying to testnet, and interacting with it through a web interface. Even if you've never written a smart contract before, we'll guide you step by step.

Let's dive in!"

**VISUAL CUES:**
- Animated Etrid logo with smart contract visualization
- Show final result: working message board DApp
- Code editor preview (VS Code with ink! syntax highlighting)
- Split screen: contract code → deployment → live interaction
- Timeline graphic showing tutorial sections

**KEY POINTS TO EMPHASIZE:**
- Complete end-to-end smart contract tutorial
- Beginner-friendly, step-by-step approach
- Build a real, functioning application
- Deploy to actual testnet (not just theory)

---

### 00:45 - 02:30 | Understanding Etrid Smart Contracts

**NARRATION:**
"Etrid supports smart contracts through ÉtwasmVM - our enhanced WebAssembly virtual machine with built-in reentrancy protection. Unlike Ethereum's Solidity, Etrid contracts are written in ink! - a Rust-based embedded domain-specific language.

Why ink!? Three major advantages: First, Rust's safety guarantees prevent common bugs like buffer overflows and null pointer errors. Second, contracts compile to WebAssembly, which is faster and more portable than EVM bytecode. Third, you get better tooling - including the Rust compiler's excellent error messages.

ÉtwasmVM adds security features beyond standard Wasm: automatic reentrancy detection, gas metering that prevents infinite loops, and memory isolation between contracts. Plus, if you're migrating from Ethereum, we support EVM compatibility on certain partition burst chains.

Here's the development workflow: Write your contract in ink!, compile it to Wasm, test it locally with substrate-contracts-node, deploy to Etrid testnet, and finally interact with it through the Polkadot.js interface or custom frontend."

**VISUAL CUES:**
- Architecture diagram: ink! → Rust compiler → Wasm → ÉtwasmVM → Etrid blockchain
- Side-by-side comparison: Solidity vs. ink! syntax
- Security features list with checkmarks
- Development workflow flowchart
- Show contract lifecycle: Write → Test → Deploy → Interact

**CODE TO DISPLAY:**
```rust
// Solidity (Ethereum)
contract SimpleStorage {
    uint256 storedData;
    function set(uint256 x) public {
        storedData = x;
    }
}

// ink! (Etrid)
#[ink::contract]
mod simple_storage {
    #[ink(storage)]
    pub struct SimpleStorage {
        stored_data: u32,
    }

    impl SimpleStorage {
        #[ink(message)]
        pub fn set(&mut self, x: u32) {
            self.stored_data = x;
        }
    }
}
```

**KEY POINTS TO EMPHASIZE:**
- ink! = Rust-based smart contract language
- Compiles to WebAssembly (Wasm)
- ÉtwasmVM provides enhanced security
- Better type safety than Solidity
- Compatible tooling with Rust ecosystem

**COMMON MISTAKES TO MENTION:**
- Don't confuse ink! with Solidity - different syntax and paradigms
- ÉtwasmVM is not EVM - though we support EVM on some PBCs
- Always test contracts locally before deploying to testnet

---

### 02:30 - 04:00 | Setting Up Development Environment

**NARRATION:**
"Let's set up your development environment. You'll need three things: Rust toolchain, cargo-contract (the ink! compiler), and substrate-contracts-node for local testing.

First, install Rust if you haven't already. Open your terminal and run this command: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh. This installs rustup, the Rust installer. Follow the prompts and choose the default installation.

Next, add the Wasm target. This allows Rust to compile to WebAssembly: rustup target add wasm32-unknown-unknown.

Now install cargo-contract, the specialized tool for building ink! contracts: cargo install cargo-contract --force. This might take a few minutes to compile.

Finally, install substrate-contracts-node for local testing: cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force.

Let's verify everything is installed correctly. Run: cargo contract --version. You should see the version number. Perfect! Your environment is ready."

**VISUAL CUES:**
- Terminal screen recording (full screen, readable font size)
- Show each command being typed and executed
- Highlight successful installation messages
- Show progress bars for long installations
- Checklist animation as each tool is installed:
  - ☐ Rust toolchain
  - ☐ Wasm target
  - ☐ cargo-contract
  - ☐ substrate-contracts-node

**DEMO STEPS:**
1. Open terminal
2. Run Rust installation command
3. Show installation progress
4. Close and reopen terminal (source changes)
5. Run: `rustc --version` (verify Rust)
6. Run: `rustup target add wasm32-unknown-unknown`
7. Run: `cargo install cargo-contract --force`
8. Show compilation progress (may take 5-10 minutes)
9. Run: `cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --force`
10. Run: `cargo contract --version` (verify installation)
11. Run: `substrate-contracts-node --version` (verify installation)

**CODE/COMMANDS TO DISPLAY:**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the environment
source $HOME/.cargo/env

# Verify Rust installation
rustc --version
cargo --version

# Add Wasm compilation target
rustup target add wasm32-unknown-unknown

# Install cargo-contract
cargo install cargo-contract --force

# Install local test node
cargo install contracts-node \
  --git https://github.com/paritytech/substrate-contracts-node.git \
  --force

# Verify installations
cargo contract --version
substrate-contracts-node --version
```

**KEY POINTS TO EMPHASIZE:**
- All tools are free and open-source
- Installation is one-time setup
- cargo-contract is essential for ink! development
- substrate-contracts-node allows local testing without deploying
- Wasm target is required for contract compilation

**COMMON MISTAKES TO MENTION:**
- Mac users: May need to install Xcode Command Line Tools first: `xcode-select --install`
- Linux users: May need build dependencies: `apt-get install build-essential`
- Windows users: Use WSL2 (Windows Subsystem for Linux) for best experience
- Long installation times are normal (cargo-contract can take 10+ minutes)

---

### 04:00 - 06:30 | Writing Your First Smart Contract

**NARRATION:**
"Time to write our message board contract! Create a new directory and initialize the ink! project: cargo contract new message_board. This creates a complete project structure with a template contract.

Let's explore what was generated. Open lib.rs - this is our contract code. You'll see the contract attribute macro, the storage struct, the constructor, and some example messages. We'll modify this to create our message board.

Here's what our contract needs: First, storage to hold messages - we'll use a HashMap mapping account addresses to their messages. Second, a constructor to initialize the contract. Third, a 'post_message' function to let users write messages. And fourth, a 'get_message' function to read messages.

Let me show you the complete code. At the top, we import necessary types from ink!. Then we define our storage struct with a HashMap. The constructor creates an empty HashMap. The post_message function takes a String parameter and stores it, associating it with the caller's address. And get_message returns the message for a given address, or None if they haven't posted.

Notice the ink! attributes: #[ink::contract] marks this as a contract module, #[ink(storage)] marks our data structure, #[ink(constructor)] marks initialization functions, and #[ink(message)] marks callable functions.

Save this file. Now let's build it: cargo contract build --release. The compiler will check your code, compile to Wasm, and optimize the binary. You should see 'Your contract is ready!' with file paths to the generated artifacts."

**VISUAL CUES:**
- Terminal: cargo contract new command
- File explorer: show generated project structure
- Code editor (VS Code) split screen:
  - Left: template code (before)
  - Right: final message board code (after)
- Highlight ink! attribute macros in different colors
- Show compiler output with key messages highlighted
- Display final contract artifacts:
  - message_board.contract
  - message_board.wasm
  - metadata.json

**DEMO STEPS:**
1. Open terminal
2. Create project directory: `mkdir ~/etrid-contracts && cd ~/etrid-contracts`
3. Run: `cargo contract new message_board`
4. Show generated directory structure: `tree message_board` or `ls -la message_board/`
5. Open in VS Code: `code message_board/`
6. Show lib.rs template code
7. Replace with message board implementation
8. Explain each section (storage, constructor, messages)
9. Save file
10. Run: `cargo contract build --release`
11. Show compilation progress
12. Show success message and artifact locations
13. Run: `ls -lh target/ink/` to show generated files

**CODE TO DISPLAY:**

**Project Structure:**
```
message_board/
├── Cargo.toml
├── lib.rs
└── .gitignore
```

**Complete Contract (lib.rs):**
```rust
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod message_board {
    use ink::storage::Mapping;
    use ink::prelude::string::String;

    /// Message board contract storage
    #[ink(storage)]
    pub struct MessageBoard {
        /// Mapping from account to their message
        messages: Mapping<AccountId, String>,
    }

    impl MessageBoard {
        /// Constructor that initializes an empty message board
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                messages: Mapping::default(),
            }
        }

        /// Post a message to the board
        #[ink(message)]
        pub fn post_message(&mut self, message: String) {
            let caller = self.env().caller();
            self.messages.insert(caller, &message);
        }

        /// Get the message for a specific account
        #[ink(message)]
        pub fn get_message(&self, account: AccountId) -> Option<String> {
            self.messages.get(account)
        }

        /// Get your own message
        #[ink(message)]
        pub fn get_my_message(&self) -> Option<String> {
            let caller = self.env().caller();
            self.messages.get(caller)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn post_and_get_works() {
            let mut board = MessageBoard::new();
            let message = String::from("Hello, Etrid!");

            board.post_message(message.clone());

            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(board.get_message(accounts.alice), Some(message));
        }
    }
}
```

**Build Output:**
```
$ cargo contract build --release

 [1/3] Building cargo project
    Compiling message_board v0.1.0
    Finished release [optimized] target(s) in 12.34s

 [2/3] Post processing wasm file
    Wasm file optimized

 [3/3] Optimizing Wasm bytecode
    Original Wasm size: 45.2K, Optimized: 12.1K

✨ Your contract is ready! ✨

Contract artifacts:
  - message_board.contract (metadata + Wasm)
  - message_board.wasm (executable)
  - metadata.json (ABI and metadata)
```

**KEY POINTS TO EMPHASIZE:**
- ink! uses Rust syntax with special attributes
- Storage struct defines persistent data
- Mapping is like a HashMap that persists on-chain
- Messages are functions users can call
- self.env().caller() gets the transaction sender
- Tests are built into the contract file
- Compilation produces optimized Wasm bytecode
- .contract file contains everything needed for deployment

**COMMON MISTAKES TO MENTION:**
- Don't forget `&mut self` for functions that modify state
- String types require `ink::prelude::string::String`, not std::string
- Always handle Option types (messages may not exist)
- Build with `--release` flag for optimized production code
- Wasm target must be installed or compilation fails

---

### 06:30 - 08:00 | Testing Locally

**NARRATION:**
"Before deploying to testnet, let's test locally. First, we'll run the built-in unit tests, then we'll deploy to a local development node.

Run the tests: cargo test. You'll see ink! execute the test we wrote in the contract. It creates a new message board, posts a message, and verifies it can be retrieved. All tests passed!

Now let's test with a real blockchain. In a new terminal window, start the local substrate-contracts-node: substrate-contracts-node --dev. This starts a single-node blockchain on your computer. You'll see blocks being produced - one every few seconds.

Open the Polkadot.js Apps interface: go to polkadot.js.org/apps, then click the network dropdown, choose Development, and connect to local node. You should see 'Connected' in green.

Now we'll deploy our contract. Click Developer → Contracts → Upload & Deploy Code. Select your message_board.contract file from target/ink/. Give it a name like 'MessageBoard v1'. Click Next.

Choose the constructor - we only have 'new', so it's already selected. Click Deploy. Sign the transaction. In a few seconds, your contract is deployed! You'll see it listed with its contract address.

Let's interact with it. Click the contract name, then 'execute'. Choose 'postMessage' from the dropdown. Enter your message: 'Hello from local testnet!' Click Execute and sign the transaction.

Now read it back: switch to 'getMessage', enter your account address, and click Read. You'll see your message displayed! Congratulations - your contract works!"

**VISUAL CUES:**
- Split screen: Terminal (left) + Browser (right)
- Terminal: Show test output with passed assertions
- Terminal: Show substrate-contracts-node logs (block production)
- Browser: Full Polkadot.js Apps walkthrough
  - Highlight connection indicator (green)
  - Highlight Contracts menu navigation
  - Show upload dialog with file selection
  - Show deployment transaction confirmation
  - Show contract list with new contract
  - Show execute message dialog
  - Show read message result
- Animation: Local deployment flow diagram
- Success indicators (checkmarks) for each step

**DEMO STEPS:**

**Unit Tests:**
1. In contract directory: `cargo test`
2. Show test execution output
3. Verify all tests passed

**Local Node:**
4. Open new terminal tab
5. Run: `substrate-contracts-node --dev --tmp`
6. Show node starting and producing blocks
7. Keep this running in background

**Polkadot.js Apps:**
8. Open browser to polkadot.js.org/apps
9. Click network dropdown (top left)
10. Development → Local Node → Switch
11. Verify connection (green dot)

**Deploy Contract:**
12. Navigate: Developer → Contracts
13. Click "Upload & deploy code"
14. Click "Upload .contract" and select `target/ink/message_board.contract`
15. Enter contract name: "MessageBoard v1"
16. Click "Next"
17. Constructor: "new" (default selected)
18. Click "Deploy"
19. Click "Sign and Submit"
20. Wait for confirmation (green notification)
21. See contract in list with address

**Interact with Contract:**
22. Click on contract name
23. Click "Execute" button
24. Select message: "postMessage"
25. Enter message: "Hello from local testnet!"
26. Click "Execute"
27. Click "Sign and Submit"
28. Wait for confirmation

**Read from Contract:**
29. Change to "Call" tab
30. Select message: "getMessage"
31. Enter account parameter (your address)
32. Click "Call"
33. See message result displayed

**CODE/COMMANDS TO DISPLAY:**
```bash
# Run unit tests
$ cargo test

running 1 test
test message_board::tests::post_and_get_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

# Start local development node (new terminal)
$ substrate-contracts-node --dev --tmp

2025-10-22 20:00:00 Substrate Contracts Node
2025-10-22 20:00:00 Local node identity: 12D3KooW...
2025-10-22 20:00:00 Running JSON-RPC HTTP server: addr=127.0.0.1:9933
2025-10-22 20:00:00 Running JSON-RPC WS server: addr=127.0.0.1:9944
2025-10-22 20:00:05 Idle (0 peers), best: #0 (0x1234...)
2025-10-22 20:00:06 Prepared block for proposing at 1 [hash: 0x5678...]
2025-10-22 20:00:06 Block finalized: #1 (0x5678...)
```

**KEY POINTS TO EMPHASIZE:**
- Always test locally before deploying to testnet
- Unit tests catch basic logic errors
- Local node provides realistic blockchain environment
- Polkadot.js Apps is the standard interface for Substrate chains
- Contract deployment is a one-time transaction
- Contract addresses start with a specific prefix
- Execute = write transaction (costs gas, modifies state)
- Call = read-only query (free, doesn't modify state)
- Local testing is free - experiment as much as you want!

**COMMON MISTAKES TO MENTION:**
- Ensure local node is running before deploying
- Select correct .contract file (not .wasm or metadata.json alone)
- Remember to sign transactions when prompted
- Don't confuse "Execute" (write) with "Call" (read)
- If transaction fails, check you have enough balance for gas

---

### 08:00 - 10:00 | Deploying to Etrid Testnet

**NARRATION:**
"Now that our contract works locally, let's deploy to the real Etrid testnet where others can interact with it!

First, we need testnet tokens. Open your Etrid wallet and make sure you're on the testnet network - check the network selector at the top. If you don't have any ETR, visit the faucet at faucet.etrid.io and request tokens. You'll need about 1 ETR for deployment costs.

While we wait for the faucet, let's connect Polkadot.js Apps to Etrid testnet. Open polkadot.js.org/apps, click the network dropdown, scroll down to Test Networks, and select 'Etrid Testnet'. If you don't see it, you can add it manually: choose 'Custom Endpoint' and enter: wss://rpc-testnet.etrid.io. Click Switch.

Now we deploy exactly like we did locally. Go to Developer → Contracts → Upload & Deploy Code. Upload your message_board.contract file again. This time, add 'Testnet' to the name: 'MessageBoard Testnet v1'. Click Next, then Deploy.

Before signing, notice the deployment parameters. The code hash uniquely identifies your contract code. The constructor is 'new' with no parameters. Estimated gas usage is shown - this is what you'll pay in ETR. Click Sign and Submit.

The transaction is now being included in a block on the actual Etrid testnet. This takes about 15-20 seconds. Once confirmed, you'll see your contract in the list with its permanent address. Copy this address - this is how others will interact with your contract!

Let's post a message on the live testnet. Click your contract, execute 'postMessage', and enter: 'Hello from Etrid Testnet!' Sign the transaction. Wait for confirmation.

Perfect! Your message is now permanently stored on the Etrid blockchain. Anyone can read it by querying your contract. Let's verify: call 'getMessage' with your account address. There it is! Your first smart contract is live on Etrid testnet!"

**VISUAL CUES:**
- Split screen: Wallet (left) + Polkadot.js (right)
- Wallet: Show testnet selector and faucet interaction
- Browser: Polkadot.js network switching sequence
- Highlight custom endpoint addition process
- Show deployment dialog with all parameters
- Emphasize gas estimation and cost
- Show transaction pending → confirmed animation
- Display contract address with copy button
- Show block explorer view of deployment transaction
- Show final contract interaction on testnet

**DEMO STEPS:**

**Get Testnet Tokens:**
1. Open Etrid wallet
2. Verify network = Testnet (top bar)
3. Copy your address
4. Open faucet.etrid.io in new tab
5. Paste address, request tokens
6. Wait for confirmation (~15 seconds)
7. Refresh wallet, verify balance > 1 ETR

**Connect to Etrid Testnet:**
8. Open polkadot.js.org/apps
9. Click network dropdown
10. Scroll to "Test Networks"
11. If Etrid Testnet listed: click it
12. If not: click "Custom endpoint"
13. Enter: `wss://rpc-testnet.etrid.io`
14. Click "Switch"
15. Verify connection (green dot)

**Deploy Contract:**
16. Navigate: Developer → Contracts
17. Click "Upload & deploy code"
18. Upload: `target/ink/message_board.contract`
19. Name: "MessageBoard Testnet v1"
20. Click "Next"
21. Constructor: "new"
22. Review deployment parameters:
    - Code hash
    - Estimated gas
    - Cost in ETR
23. Click "Deploy"
24. Click "Sign and Submit" in wallet popup
25. Wait for confirmation (watch for green notification)
26. Contract appears in list

**Verify Deployment:**
27. Copy contract address (click copy icon)
28. Open block explorer (if available)
29. Paste contract address to verify deployment

**Interact with Deployed Contract:**
30. In Polkadot.js, click contract name
31. Click "Execute"
32. Select: "postMessage"
33. Message: "Hello from Etrid Testnet!"
34. Click "Execute"
35. Sign transaction
36. Wait for confirmation

**Read from Deployed Contract:**
37. Switch to "Call" tab
38. Select: "getMessage"
39. Account: (your address)
40. Click "Call"
41. Verify message displays

**CODE/ENDPOINTS TO DISPLAY:**
```
Etrid Testnet RPC Endpoints:
- WebSocket: wss://rpc-testnet.etrid.io
- HTTP: https://rpc-testnet.etrid.io

Contract Deployment Transaction:
- Block: #123,456
- Hash: 0x789abc...
- Gas Used: 1,234,567
- Cost: 0.0123 ETR

Contract Address (example):
5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
```

**KEY POINTS TO EMPHASIZE:**
- Testnet deployment is identical to mainnet (good practice!)
- Real blockchain, real transactions, but testnet ETR has no value
- Contract address is permanent - save it!
- Deployment cost depends on contract size and complexity
- Once deployed, code cannot be changed (but you can deploy new versions)
- Anyone can interact with your contract using the address
- Testnet is perfect for learning and experimentation

**COMMON MISTAKES TO MENTION:**
- Ensure you're on testnet, not mainnet (check network indicator)
- Must have sufficient ETR for deployment (at least 0.5 ETR recommended)
- Don't deploy untested contracts to mainnet
- Save your contract address - you'll need it to share with others
- If deployment fails, check gas settings and balance

---

### 10:00 - 11:30 | Best Practices & Security

**NARRATION:**
"Let's talk about best practices for smart contract development on Etrid. These tips will help you write secure, efficient, and maintainable contracts.

First, security. Always validate inputs - never trust user-provided data. Use Rust's type system to prevent invalid states. For example, use Options instead of assuming values exist, use Results for fallible operations, and define custom error types for clarity.

Watch out for common vulnerabilities. Reentrancy attacks are prevented automatically by ÉtwasmVM, but you should still follow checks-effects-interactions pattern. Integer overflow is also prevented by default in Rust, but be careful when using unchecked math. And always handle storage correctly - remember that storage operations are expensive!

Second, gas optimization. Storage reads and writes are the most expensive operations. Minimize storage usage by packing data efficiently, using local variables for intermediate calculations, and batching updates. Events are much cheaper than storage - emit events for off-chain indexing instead of storing everything on-chain.

Third, testing. Write comprehensive unit tests for all contract functions. Test edge cases: empty inputs, maximum values, unauthorized callers. Use integration tests to verify contract interactions. And most importantly, audit your contract or have someone review it before mainnet deployment.

Fourth, upgradeability. Etrid contracts are immutable once deployed, which is good for security but bad for bug fixes. Plan ahead: use proxy patterns for upgradeability, or design your contract to be modular where components can be replaced. Document your upgrade strategy clearly.

Finally, documentation. Comment your code extensively - explain why, not just what. Document all public functions with examples. Specify preconditions and postconditions. And maintain a README with deployment information and usage examples.

Remember: audited code is safer code. For production contracts holding real value, always get a professional security audit. The Etrid Foundation maintains a list of recommended auditors at docs.etrid.io/security."

**VISUAL CUES:**
- Security checklist with checkmarks/x-marks
- Code examples showing secure vs. insecure patterns
- Gas cost comparison table (storage vs. events vs. local variables)
- Test coverage visualization (green = tested, red = untested)
- Upgradeability pattern diagram (proxy → implementation)
- Documentation example (well-commented code)
- Security audit process flowchart
- Best practices summary slide

**CODE EXAMPLES:**

**Input Validation (Good):**
```rust
#[ink(message)]
pub fn set_price(&mut self, price: Balance) -> Result<(), Error> {
    if price == 0 {
        return Err(Error::InvalidPrice);
    }

    if price > MAX_PRICE {
        return Err(Error::PriceTooHigh);
    }

    self.price = price;
    self.env().emit_event(PriceUpdated { new_price: price });
    Ok(())
}
```

**Storage Optimization (Good vs. Bad):**
```rust
// ❌ Bad: Multiple storage reads
pub fn calculate_total(&self) -> Balance {
    let fee = self.fee_percentage; // storage read
    let amount = self.amount;       // storage read
    let base = self.base_amount;    // storage read
    amount + (amount * fee / 100) + base
}

// ✅ Good: Read once, calculate locally
pub fn calculate_total(&self) -> Balance {
    let StorageData { fee_percentage, amount, base_amount } = self.data;
    amount + (amount * fee_percentage / 100) + base_amount
}
```

**Comprehensive Testing:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[ink::test]
    fn post_message_works() {
        let mut board = MessageBoard::new();
        assert!(board.post_message("Hello".into()).is_ok());
    }

    #[ink::test]
    fn post_empty_message_fails() {
        let mut board = MessageBoard::new();
        assert!(board.post_message("".into()).is_err());
    }

    #[ink::test]
    fn get_nonexistent_message_returns_none() {
        let board = MessageBoard::new();
        let accounts = ink::env::test::default_accounts();
        assert_eq!(board.get_message(accounts.alice), None);
    }
}
```

**KEY POINTS TO EMPHASIZE:**
- Security is paramount - contracts are immutable and hold value
- Rust's type system prevents many common bugs
- ÉtwasmVM provides automatic reentrancy protection
- Gas optimization matters - storage is expensive
- Write tests before deploying
- Plan for upgradeability from the start
- Document everything
- Get security audits for production contracts

**COMMON MISTAKES TO MENTION:**
- Don't skip input validation
- Don't ignore error cases
- Don't store data unnecessarily
- Don't deploy untested contracts
- Don't assume contracts are upgradeable by default

---

### 11:30 - 12:00 | Next Steps & Resources

**NARRATION:**
"Congratulations! You've just deployed your first smart contract on Etrid! Let's recap what you learned and explore where to go next.

You now know how to: set up the ink! development environment, write smart contracts in Rust, test locally with substrate-contracts-node, deploy to Etrid testnet, and interact with deployed contracts. That's the complete development lifecycle!

Ready to build more? Here are some project ideas to practice: Create a simple token contract implementing a fungible token standard. Build a multi-signature wallet requiring multiple approvals for transactions. Make a decentralized voting system for on-chain governance. Or design an NFT marketplace with listing, buying, and royalty features.

For learning resources, start with the ink! documentation at ink.substrate.io - it has comprehensive tutorials and API references. Check out our Etrid Developer Guide at docs.etrid.io/developers for platform-specific information. Join the Substrate Stack Exchange for technical questions. And explore the awesome-ink repository on GitHub for community contracts and libraries.

Need help? Join our Discord server at discord.gg/etrid where the developer community is incredibly helpful. Follow the #smart-contracts channel for tips and discussions. Attend our weekly developer office hours every Tuesday. And don't forget to check the troubleshooting guide at docs.etrid.io/troubleshooting.

Coming up next in our tutorial series: Tutorial 05 shows you how to build a complete DApp frontend that interacts with your smart contract using React and the Polkadot.js API. You'll create a beautiful web interface for your message board!

Thank you for following along. Happy coding, and welcome to the Etrid developer community! See you in the next tutorial!"

**VISUAL CUES:**
- Recap checklist with checkmarks:
  - ✅ Environment setup
  - ✅ Contract written
  - ✅ Local testing
  - ✅ Testnet deployment
  - ✅ Contract interaction
- Project ideas carousel (visual mockups)
- Resource links display with QR codes
- Community screenshots (Discord, Stack Exchange)
- Next tutorial thumbnail and preview
- Call-to-action: Subscribe, Join Discord, Star GitHub repo
- Animated Etrid logo with "Welcome to the community" tagline

**RESOURCES TO DISPLAY:**
```
📚 Learning Resources:
- ink! Docs: https://ink.substrate.io
- Etrid Developer Guide: https://docs.etrid.io/developers
- Substrate Stack Exchange: https://substrate.stackexchange.com
- awesome-ink: https://github.com/paritytech/awesome-ink

💬 Community:
- Discord: https://discord.gg/etrid (#smart-contracts channel)
- Developer Office Hours: Tuesdays 2pm UTC
- Twitter: @EtridDevs
- GitHub: https://github.com/etrid

🎓 Next Steps:
1. Deploy your own variation of MessageBoard
2. Add features: edit/delete messages, message likes
3. Implement access control (only owner can delete)
4. Watch Tutorial 05: Building DApps with React

🚀 Project Ideas:
- ERC-20 token contract
- Multi-signature wallet
- On-chain voting system
- NFT marketplace
- Decentralized exchange (AMM)
```

**KEY POINTS TO EMPHASIZE:**
- You now have fundamental smart contract skills
- Practice by building real projects
- Community is here to help
- Lots of learning resources available
- Next tutorial builds on this knowledge
- Start small, iterate, and improve

---

## Production Notes

### Visual Assets Needed

**Static Graphics:**
1. Etrid + ink! logo combination
2. Development workflow diagram
3. Security best practices infographic
4. Gas optimization comparison chart
5. Testing pyramid visualization
6. Resource links end card
7. Project ideas showcase
8. Community platform screenshots

**Screen Recordings:**
1. Development environment setup (terminal)
2. VS Code with ink! contract editing
3. cargo contract build process
4. Unit test execution
5. substrate-contracts-node running
6. Polkadot.js Apps complete walkthrough:
   - Local deployment
   - Contract interaction
   - Testnet connection
   - Testnet deployment
   - Testnet contract usage
7. Faucet interaction
8. Block explorer contract view

**Code Examples:**
1. Solidity vs. ink! comparison
2. Complete MessageBoard contract
3. Security patterns (good/bad examples)
4. Gas optimization examples
5. Comprehensive test examples

### Demo Requirements

**Local Environment:**
- Rust toolchain installed
- cargo-contract installed
- substrate-contracts-node installed
- VS Code with rust-analyzer extension
- Terminal with good visibility

**Network Access:**
- Etrid testnet RPC: wss://rpc-testnet.etrid.io
- Polkadot.js Apps: polkadot.js.org/apps
- Faucet: faucet.etrid.io
- Block explorer (if available)

**Pre-Prepared:**
- Message board contract code ready
- Local node running in background
- Wallet with testnet ETR
- Polkadot.js already connected to testnet

### Editing Notes

**Pacing:**
- Environment setup can be sped up (timelapse for long installs)
- Slow down during code explanation
- Real-time for deployments (builds anticipation)
- Use picture-in-picture for terminal + browser simultaneously

**Graphics:**
- Syntax highlight all code snippets
- Zoom in on important parts of UI (buttons, forms)
- Add animated arrows pointing to key elements
- Use consistent color coding (testnet = yellow, mainnet = green)

**Audio:**
- Clear, enthusiastic narration
- Background music: tech/upbeat but subtle
- Sound effects for successful deployments
- Silence during long operations (with on-screen text)

**Accessibility:**
- Closed captions for all narration
- High contrast code themes
- Readable font sizes (minimum 14pt on code)
- Audio descriptions of visual diagrams

---

## Target Metrics

**Engagement Goals:**
- Watch time: 65%+ (complete tutorial)
- Click-through to Tutorial 05: 25%+
- Contract deployments (testnet): 20%+ of viewers
- Community joins: 15%+ join Discord

**Educational Goals:**
- 90%+ understand ink! vs. Solidity differences
- 85%+ successfully set up environment
- 75%+ deploy contract to local node
- 60%+ deploy contract to testnet
- 50%+ deploy their own variation

---

**Tutorial Complete**
Next: 05-building-dapps.md
