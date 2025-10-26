# NFT (ERC721) Contract

**Difficulty**: ⭐⭐⭐⭐ Advanced
**Time to Complete**: 2-3 hours

---

## 📖 What You'll Learn

This contract implements the ERC721 standard for Non-Fungible Tokens (NFTs):
- ✅ **ERC721 standard interface** (transfer, approve, transferFrom)
- ✅ **Unique token ownership** (each token is unique)
- ✅ **Token metadata** (URI for token data/images)
- ✅ **Approval mechanism** (single token and operator approvals)
- ✅ **Minting & burning** (create and destroy NFTs)
- ✅ **Balance tracking** (count tokens per owner)
- ✅ **Comprehensive events** (Transfer, Approval, ApprovalForAll)
- ✅ **Safe transfers** (ERC721 compatible)

---

## 🏗️ Contract Overview

The NFT (ERC721) contract enables creation and management of unique, non-fungible tokens.

### Storage
```rust
name: String                                    // Collection name
symbol: String                                  // Collection symbol
token_owner: Mapping<u32, AccountId>           // Token ID → Owner
token_approvals: Mapping<u32, AccountId>       // Token ID → Approved address
owned_tokens_count: Mapping<AccountId, u32>    // Owner → Token count
operator_approvals: Mapping<(owner, operator), bool>  // Operator permissions
token_uris: Mapping<u32, String>               // Token ID → Metadata URI
total_supply: u32                               // Total minted tokens
owner: AccountId                                // Contract owner
```

### Core Functions

| Function | Type | Access | Description |
|----------|------|--------|-------------|
| `new(name, symbol)` | Constructor | Public | Create NFT collection |
| `name()` | Query | Public | Get collection name |
| `symbol()` | Query | Public | Get collection symbol |
| `total_supply()` | Query | Public | Get total minted |
| `contract_owner()` | Query | Public | Get contract owner |
| `balance_of(owner)` | Query | Public | Get token count for owner |
| `owner_of(token_id)` | Query | Public | Get token owner |
| `get_approved(token_id)` | Query | Public | Get approved address |
| `is_approved_for_all(owner, operator)` | Query | Public | Check operator approval |
| `token_uri(token_id)` | Query | Public | Get token metadata URI |
| `approve(to, token_id)` | Transaction | Token Owner | Approve address for token |
| `set_approval_for_all(operator, approved)` | Transaction | Public | Set operator for all tokens |
| `transfer_from(from, to, token_id)` | Transaction | Approved | Transfer token |
| `safe_transfer_from(from, to, token_id)` | Transaction | Approved | Safe transfer |
| `mint(to, token_id, uri)` | Transaction | Owner Only | Mint new NFT |
| `burn(token_id)` | Transaction | Token Owner | Burn NFT |
| `transfer_ownership(new_owner)` | Transaction | Owner Only | Change contract owner |

### Events
- `Transfer(from, to, token_id)` - Token transferred (indexed)
- `Approval(owner, approved, token_id)` - Token approved (indexed)
- `ApprovalForAll(owner, operator, approved)` - Operator set (indexed)

### Errors
- `NotOwner` - Caller not contract owner
- `NotApproved` - Not approved to transfer
- `TokenExists` - Token ID already minted
- `TokenNotFound` - Token doesn't exist
- `NotTokenOwner` - Caller not token owner
- `NotAllowed` - Operation not allowed
- `TransferToZeroAddress` - Cannot transfer to 0x0
- `EmptyTokenUri` - Token URI cannot be empty

---

## 🚀 Quick Start

### 1. Build the Contract

```bash
cd 06-nft-erc721
cargo contract build --release
```

### 2. Deploy Contract

```bash
# Deploy NFT collection
cargo contract instantiate \
  --constructor new \
  --args "Ëtrid Apes" "EAPE" \
  --suri //Alice
```

### 3. Interact with Contract

```bash
# Get collection info
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message name \
  --suri //Alice \
  --dry-run

# Mint NFT #1
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message mint \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 1 "ipfs://Qm.../1.json" \
  --suri //Alice

# Check owner of token #1
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message owner_of \
  --args 1 \
  --suri //Alice \
  --dry-run

# Check balance of Alice
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message balance_of \
  --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --suri //Alice \
  --dry-run

# Get token metadata URI
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message token_uri \
  --args 1 \
  --suri //Alice \
  --dry-run

# Approve Bob to transfer token #1
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message approve \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty 1 \
  --suri //Alice

# Transfer token #1 from Alice to Charlie (as Bob)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message transfer_from \
  --args 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY 5FLSigC9HGgKVZdwjRuSpzL4YZTYCBz9VYYzc8bDJsGTAQHT 1 \
  --suri //Bob

# Set Bob as operator for all Alice's tokens
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message set_approval_for_all \
  --args 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty true \
  --suri //Alice

# Burn token #1 (owner only)
cargo contract call \
  --contract <CONTRACT_ADDRESS> \
  --message burn \
  --args 1 \
  --suri //Alice
```

---

## 🧪 Testing

### Run Unit Tests

```bash
cargo test
```

**Expected output**: All 17 tests pass

**Tests cover**:
- Collection creation
- Minting (owner only, duplicate prevention)
- Token ownership tracking
- Balance counting
- Transfers (owner, approved, operator)
- Approvals (single token, all tokens)
- Burning
- Multiple tokens per owner

### Run Integration Tests (E2E)

```bash
# Start local node
substrate-contracts-node --dev --tmp

# Run E2E tests
cargo test --features e2e-tests
```

---

## 📝 Code Walkthrough

### NFTs vs Fungible Tokens

**Fungible (ERC20)**:
```rust
// All tokens are identical
token.transfer(bob, 100);  // Any 100 tokens
```

**Non-Fungible (ERC721)**:
```rust
// Each token is unique
nft.transfer_from(alice, bob, 42);  // Specific token #42
```

### Minting NFTs

```rust
// Mint token #1 to Alice with metadata URI
nft.mint(
    alice,
    1,  // Unique token ID
    "ipfs://QmHash.../metadata.json".into()
)?;

// Token metadata (stored off-chain on IPFS):
{
  "name": "Ape #1",
  "description": "First ape",
  "image": "ipfs://QmHash.../image.png",
  "attributes": [...]
}
```

**Key points**:
- Each token has a unique ID
- Metadata stored off-chain (gas efficient)
- IPFS commonly used for decentralized storage

### Ownership Tracking

```rust
// Check who owns token #1
let owner = nft.owner_of(1);  // Returns Some(alice)

// Check how many tokens Alice owns
let count = nft.balance_of(alice);  // Returns 3

// Get metadata URI
let uri = nft.token_uri(1);  // Returns "ipfs://..."
```

### Transfer Mechanisms

**Direct Transfer** (owner only):
```rust
// Alice owns token #1
nft.transfer_from(alice, bob, 1)?;
// Now Bob owns token #1
```

**Approved Transfer**:
```rust
// Alice approves Bob for token #1
nft.approve(bob, 1)?;

// Bob can now transfer token #1
nft.transfer_from(alice, charlie, 1)?;
```

**Operator Transfer** (approved for all):
```rust
// Alice sets Bob as operator
nft.set_approval_for_all(bob, true)?;

// Bob can now transfer ANY of Alice's tokens
nft.transfer_from(alice, charlie, 1)?;
nft.transfer_from(alice, charlie, 2)?;
nft.transfer_from(alice, charlie, 3)?;
```

### Safe Transfers

```rust
// Same as transfer_from for basic implementation
// In full ERC721, checks if recipient can receive NFTs
nft.safe_transfer_from(from, to, token_id)?;
```

---

## 💡 Try It Yourself

### Challenge 1: Add Enumeration
Track all token IDs for efficient enumeration.

**Hint**: Add `token_by_index: Mapping<u32, u32>` and `token_of_owner_by_index: Mapping<(AccountId, u32), u32>`.

### Challenge 2: Add Royalties
Implement ERC2981 royalty standard.

**Hint**: Add `royalties: Mapping<u32, (AccountId, u8)>` with (recipient, percentage).

### Challenge 3: Add Whitelist Minting
Only allow whitelisted addresses to mint during initial sale.

**Hint**: Add `whitelist: Mapping<AccountId, bool>` and check in `mint()`.

### Challenge 4: Add Max Supply
Limit total tokens that can be minted.

**Hint**: Add `max_supply: u32`, check in `mint()`.

### Challenge 5: Add Batch Minting
Mint multiple tokens in one transaction.

**Hint**: Create `batch_mint(&mut self, to: AccountId, token_ids: Vec<u32>, uris: Vec<String>)`.

### Challenge 6: Add Soulbound Tokens
Create non-transferable NFTs.

**Hint**: Add `is_soulbound: Mapping<u32, bool>`, check before transfers.

---

## 🔍 Gas Costs (VMw)

Typical costs on Ëtrid:

| Operation | VMw Cost | Notes |
|-----------|----------|-------|
| Deploy contract | ~200,000 | One-time |
| `mint()` | ~8,000 | Create new NFT |
| `transfer_from()` | ~6,000 | Transfer ownership |
| `approve()` | ~3,500 | Approve address |
| `set_approval_for_all()` | ~3,000 | Set operator |
| `burn()` | ~5,000 | Destroy NFT |
| `owner_of()` | ~200 | Read token owner |
| `token_uri()` | ~300 | Read metadata URI |

**Optimization tips**:
- Store metadata off-chain (IPFS, Arweave)
- Batch mint during initial sale
- Use operator approvals for marketplaces
- Lazy minting (mint on first purchase)

---

## 🌐 Real-World Use Cases

### 1. NFT Art Collection
```rust
// Deploy collection
NftErc721::new("Crypto Punks".into(), "PUNK".into())

// Mint artwork NFTs
nft.mint(artist, 1, "ipfs://artwork1_metadata.json")
nft.mint(artist, 2, "ipfs://artwork2_metadata.json")
```

### 2. Gaming Items
```rust
// Deploy game items collection
NftErc721::new("Epic Swords".into(), "SWORD".into())

// Mint unique sword with stats in metadata
nft.mint(player, 42, "ipfs://sword_legendary_42.json")
// Metadata includes: attack, durability, rarity
```

### 3. Domain Names
```rust
// Deploy ENS-style name service
NftErc721::new("Ëtrid Names".into(), "ETRNAME".into())

// Mint domain name
nft.mint(user, hash("alice.etr"), "ipfs://metadata_alice.json")
```

### 4. Event Tickets
```rust
// Deploy ticket collection
NftErc721::new("Concert Tickets 2025".into(), "TIX".into())

// Mint ticket NFTs
nft.mint(attendee, 1, "ipfs://seat_A1_metadata.json")
// Includes: event, date, seat, venue
```

### 5. Real Estate
```rust
// Deploy property tokens
NftErc721::new("Property Deeds".into(), "DEED".into())

// Mint property deed
nft.mint(owner, property_id, "ipfs://property_123_deed.json")
// Includes: address, size, legal docs
```

---

## 📊 Comparison: ERC20 vs ERC721

| Feature | ERC20 (Fungible) | ERC721 (Non-Fungible) |
|---------|------------------|----------------------|
| Token Type | Identical copies | Unique tokens |
| Transfer | Amount (100 tokens) | Token ID (token #42) |
| Balance | Total count | List of owned IDs |
| Use Cases | Currency, points | Art, collectibles |
| Metadata | Token-level | Per-token unique |
| Divisibility | Yes (decimals) | No (whole tokens) |
| Example | 100 USDT | Crypto Punk #1234 |

---

## 🐛 Common Issues

### "TokenExists: token already minted"
**Cause**: Trying to mint token ID that already exists
**Solution**: Use unique token IDs, track `next_token_id`

### "NotApproved: not approved to transfer"
**Cause**: Trying to transfer without approval
**Solution**: Get approval first with `approve()` or `set_approval_for_all()`

### "EmptyTokenUri: token URI cannot be empty"
**Cause**: Minting without metadata URI
**Solution**: Always provide IPFS/HTTP URL to metadata

### "NotTokenOwner: caller not token owner"
**Cause**: Trying to burn token you don't own
**Solution**: Only token owner can burn

---

## 🔗 Metadata Standard

### JSON Metadata Format
```json
{
  "name": "Ape #1234",
  "description": "A rare golden ape",
  "image": "ipfs://QmHash.../image.png",
  "attributes": [
    {
      "trait_type": "Background",
      "value": "Blue"
    },
    {
      "trait_type": "Fur",
      "value": "Gold"
    },
    {
      "trait_type": "Rarity",
      "value": "Legendary"
    }
  ]
}
```

### Storing on IPFS
```bash
# Upload to IPFS
ipfs add metadata.json
# Returns: QmHash...

# Use in contract
nft.mint(user, 1, "ipfs://QmHash.../metadata.json")
```

---

## 📚 Next Steps

After mastering this contract:
1. **Build NFT Marketplace** - List and sell NFTs
2. **Add Staking** - Stake NFTs to earn rewards
3. **Create Game** - Use NFTs as in-game items
4. **Deploy Collection** - Launch your own NFT project!

---

## 📖 Resources

- ERC721 Standard: https://eips.ethereum.org/EIPS/eip-721
- OpenSea Metadata: https://docs.opensea.io/docs/metadata-standards
- IPFS: https://ipfs.io/
- ink! NFT example: https://use.ink/examples/erc721

---

**Questions?** Ask in Discord: https://discord.gg/etrid

**Happy NFT Building! 🎨🖼️**
