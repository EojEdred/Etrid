# ERC20 Token Example - STATUS

**Status**: 📋 Planned (Not Yet Implemented)
**Priority**: High
**Estimated Time**: 2-3 hours

---

## Planned Features

This contract will demonstrate:
- ✅ ERC20 standard interface
- ✅ Total supply management
- ✅ Balance tracking per user
- ✅ Transfer function
- ✅ Approve/allowance mechanism
- ✅ TransferFrom (delegated transfers)
- ✅ Minting (owner only)
- ✅ Burning (anyone can burn their own tokens)
- ✅ Events (Transfer, Approval)
- ✅ Complete test suite

---

## Implementation Plan

### Storage
```rust
total_supply: Balance
balances: Mapping<AccountId, Balance>
allowances: Mapping<(AccountId, AccountId), Balance>
owner: AccountId
```

### Core Functions
- `transfer(to, amount)` - Send tokens
- `transfer_from(from, to, amount)` - Send on behalf
- `approve(spender, amount)` - Allow spending
- `balance_of(account)` - Get balance
- `allowance(owner, spender)` - Get allowance
- `mint(to, amount)` - Create new tokens (owner only)
- `burn(amount)` - Destroy own tokens

---

## Next Steps

1. Create Cargo.toml
2. Implement lib.rs with full ERC20 interface
3. Write comprehensive tests
4. Create tutorial README
5. Test on local node

---

**To implement this**, run:
```bash
cd 03-erc20-token
# Create Cargo.toml, lib.rs, README.md
cargo contract build --release
cargo test
```

---

**Estimated completion**: 2-3 hours for full implementation with tests and docs
