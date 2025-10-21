# **ÉTRID CONSTANTS & REFERENCE CARD**

Quick lookup for developers using Week 0 pallets.

---

## **CURRENCY DENOMINATIONS (ÉTR)**

```rust
// From pallet-native-currency::constants

// Atomic units (all conversions to smallest unit: Bitë)
ONE_BITE: Balance           = 1              // 1 Bitë
ONE_TRIBITE: Balance        = 10             // 1 Tribite = 10 Bitë
ONE_QUADRITE: Balance       = 100            // 1 Quadrite = 100 Bitë
ONE_OCTOBITE: Balance       = 1_000          // 1 Octobite = 1,000 Bitë
ONE_SEXTOBITE: Balance      = 10_000         // 1 Sextobite = 10,000 Bitë
ONE_ETRID: Balance          = 100_000        // 1 ÉTR = 100,000 Bitë [BASE]
ONE_KILO_ETRID: Balance     = 100_000_000           // 1 kËtr
ONE_MEGA_ETRID: Balance     = 100_000_000_000       // 1 mËtr
ONE_GIGA_ETRID: Balance     = 100_000_000_000_000   // 1 gÉTR [TOTAL SUPPLY]

// Total supplies
TOTAL_ETRID_SUPPLY: Balance = 1_000_000_000 * ONE_ETRID    // 1B ÉTR
TOTAL_ETD_SUPPLY: Balance   = 2_500_000_000 * ONE_ETRID    // 2.5B ETD

// Usage examples:
let ten_etrid = 10 * ONE_ETRID;                    // 10 ÉTR
let one_million_etrid = 1_000_000 * ONE_ETRID;     // 1M ÉTR
let treasury_initial = 970_000_000 * ONE_ETRID;    // 970M ÉTR
```

---

## **VIRTUAL MACHINE WATTS (VMw) - GAS PRICING**

```rust
// From pallet-native-currency::constants

// Operation costs (in VMw - non-tradable units)
VMW_CONTRACT_INIT: VMw      = 2_000   // Deploy new contract
VMW_CONTRACT_CALL: VMw      = 500     // Call existing contract
VMW_STORAGE_READ: VMw       = 100     // Read from storage
VMW_STORAGE_WRITE: VMw      = 300     // Write to storage
VMW_STATE_VERIFY: VMw       = 150     // Verify state
VMW_ADDRESS_CHECK: VMw      = 50      // Check address

// Per-block and per-transaction limits
VMW_BLOCK_LIMIT: VMw        = 10_000_000    // 10M VMw per block
VMW_TX_LIMIT: VMw           = 1_000_000     // 1M VMw per transaction

// Conversion rate
WATTS_PER_ETRID: VMw        = 1_000_000     // 1M VMw = 1 ÉTR

// Fee calculation
// Total Cost (ÉTR) = (VMw_Used * Op_Price) / WATTS_PER_ETRID

// Usage examples:
// Example 1: Contract call at op_price=2
let vmw_used = 500;
let op_price = 2u32;
let cost_etrid = (500u128 * 2u128) / 1_000_000;  // 0.001 ÉTR

// Example 2: Storage write at op_price=1
let vmw_used = 300;
let op_price = 1u32;
let cost_etrid = (300u128 * 1u128) / 1_000_000;  // 0.0003 ÉTR

// Example 3: Contract init at op_price=3
let vmw_used = 2_000;
let op_price = 3u32;
let cost_etrid = (2_000u128 * 3u128) / 1_000_000;  // 0.006 ÉTR
```

---

## **TRANSACTION TYPES**

```rust
// From pallet-transaction::TransactionType

// 1. REGULAR TRANSFER
TransactionType::Regular {
    recipient: Vec<u8>,        // Destination address
    amount: Balance,           // Amount (in atomic units)
    currency: CurrencyType,    // Etrid or Etd
}
// Used for: Normal payments, wage transfers, treasury distributions
// Cost: VMW_CONTRACT_CALL (500 VMw)

// Example:
TransactionType::Regular {
    recipient: bob_address,
    amount: 1_000_000 * ONE_ETRID,  // 1M ÉTR
    currency: CurrencyType::Etrid,
}

// ---

// 2. STAKE DEPOSIT
TransactionType::StakeDeposit {
    validator: Vec<u8>,        // Validator account address
    amount: Balance,           // Stake amount
    lock_period: u32,          // Blocks until withdrawal allowed
}
// Used for: Validator staking, delegation, earning rewards
// Cost: VMW_ADDRESS_CHECK (50 VMw)

// Example:
TransactionType::StakeDeposit {
    validator: alice_address,
    amount: 10_000_000 * ONE_ETRID,  // 10M ÉTR stake
    lock_period: 6_000,              // ~1 day at 6s/block
}

// ---

// 3. SMART CONTRACT CALL
TransactionType::SmartContractCall {
    contract: Vec<u8>,         // Contract address
    data: Vec<u8>,             // ABI-encoded function call
    vmw_limit: VMw,            // Max VMw allowed
    value: Balance,            // Value to transfer to contract
}
// Used for: DeFi, governance votes, state updates
// Cost: VMW_CONTRACT_CALL (500 VMw) + contract execution

// Example:
TransactionType::SmartContractCall {
    contract: uniswap_contract,
    data: abi_encoded_swap,     // swap(tokenA, tokenB, amount)
    vmw_limit: 10_000,          // 10K VMw limit
    value: 0,                   // No value transferred
}

// ---

// 4. CONTRACT INITIALIZATION (DEPLOY)
TransactionType::ContractInit {
    init_code: Vec<u8>,        // WASM bytecode
    vmw_limit: VMw,            // Max VMw for deployment
    value: Balance,            // Initial value
}
// Used for: Deploying new smart contracts, DAOs, protocols
// Cost: VMW_CONTRACT_INIT (2,000 VMw) + setup

// Example:
TransactionType::ContractInit {
    init_code: my_erc20_bytecode,  // WASM ERC20 implementation
    vmw_limit: 50_000,             // 50K VMw for deployment
    value: 0,
}

// ---

// 5. LIGHTNING BLOC (CROSS-CHAIN)
TransactionType::LightningBloc {
    target_chain: ChainId,     // Destination chain ID
    recipient: Vec<u8>,        // Recipient on target chain
    amount: Balance,           // Amount to send
    fee: Balance,              // Channel routing fee
}
// Used for: Cross-chain payments, bridge transfers, multichain swaps
// Cost: VMW_CONTRACT_CALL (500 VMw) + routing

// Example:
TransactionType::LightningBloc {
    target_chain: 2,           // Ethereum chain ID
    recipient: eth_address,    // Recipient on Ethereum
    amount: 100 * ONE_ETRID,   // 100 ÉTR
    fee: 1 * ONE_ETRID,        // 1 ÉTR routing fee
}
```

---

## **PALLET STORAGE QUERIES**

```rust
// From pallet-native-currency

// Account balances (ÉTR)
let balance = pallet_native_currency::EtridBalances::<Runtime>::get(&account_id);

// Account balances (ETD)
let balance = pallet_native_currency::EtdBalances::<Runtime>::get(&account_id);

// Total supply
let supply = pallet_native_currency::EtridTotalSupply::<Runtime>::get();

// VMw used in current block
let vmw_used = pallet_native_currency::VMwUsedThisBlock::<Runtime>::get();

// Current operation price
let op_price = pallet_native_currency::VMwOpPrice::<Runtime>::get();

// ---

// From pallet-transaction

// Account nonce (for TX ordering)
let nonce = pallet_transaction::AccountNonces::<Runtime>::get(&account_id);

// Staking balance
let stake = pallet_transaction::StakingPool::<Runtime>::get(&account_id);

// Contract code
let code = pallet_transaction::ContractCode::<Runtime>::get(&contract_address);

// Contract storage
let value = pallet_transaction::ContractStorage::<Runtime>::get(&contract, &key);

// Lightning Bloc channel
let channel = pallet_transaction::LightningBlocChannels::<Runtime>::get(chain_id);
```

---

## **EXTRINSICS (CALLABLE FUNCTIONS)**

```rust
// From pallet-native-currency

// Transfer ÉTR
pallet_native_currency::Call::<Runtime>::transfer_etrid {
    recipient: T::AccountId,
    amount: Balance,
}

// Transfer ETD
pallet_native_currency::Call::<Runtime>::transfer_etd {
    recipient: T::AccountId,
    amount: Balance,
}

// Mint ÉTR (governance)
pallet_native_currency::Call::<Runtime>::mint_etrid {
    account: T::AccountId,
    amount: Balance,
}

// Mint ETD (governance)
pallet_native_currency::Call::<Runtime>::mint_etd {
    account: T::AccountId,
    amount: Balance,
}

// Burn ÉTR
pallet_native_currency::Call::<Runtime>::burn_etrid {
    amount: Balance,
}

// Consume VMw (fee payment)
pallet_native_currency::Call::<Runtime>::consume_vmw {
    vmw_amount: VMw,
}

// Set VMw price (governance)
pallet_native_currency::Call::<Runtime>::set_vmw_price {
    price: u32,
}

// ---

// From pallet-transaction

// Submit regular transfer
pallet_transaction::Call::<Runtime>::submit_regular_transfer {
    recipient: Vec<u8>,
    amount: Balance,
    currency: CurrencyType,
}

// Submit stake deposit
pallet_transaction::Call::<Runtime>::submit_stake_deposit {
    validator: Vec<u8>,
    amount: Balance,
    lock_period: u32,
}

// Submit contract call
pallet_transaction::Call::<Runtime>::submit_contract_call {
    contract: Vec<u8>,
    data: Vec<u8>,
    vmw_limit: VMw,
    value: Balance,
}

// Deploy contract
pallet_transaction::Call::<Runtime>::deploy_contract {
    init_code: Vec<u8>,
    vmw_limit: VMw,
    value: Balance,
}

// Submit Lightning Bloc payment
pallet_transaction::Call::<Runtime>::submit_lightning_bloc {
    target_chain: ChainId,
    recipient: Vec<u8>,
    amount: Balance,
    fee: Balance,
}

// Withdraw stake
pallet_transaction::Call::<Runtime>::withdraw_stake {
    amount: Balance,
}
```

---

## **EVENTS (EMITTED)**

```rust
// From pallet-native-currency

Event::EtridTransferred {
    from: T::AccountId,
    to: T::AccountId,
    amount: Balance,
}

Event::EtdTransferred {
    from: T::AccountId,
    to: T::AccountId,
    amount: Balance,
}

Event::EtridMinted {
    account: T::AccountId,
    amount: Balance,
}

Event::EtdMinted {
    account: T::AccountId,
    amount: Balance,
}

Event::EtridBurned {
    account: T::AccountId,
    amount: Balance,
}

Event::EtdBurned {
    account: T::AccountId,
    amount: Balance,
}

Event::VMwConsumed {
    vmw_used: VMw,
    cost_in_etrid: Balance,
}

Event::VMwBlockLimitExceeded

Event::VMwTxLimitExceeded

Event::StorageRentCharged {
    account: T::AccountId,
    amount: Balance,
}

// ---

// From pallet-transaction

Event::TransactionExecuted {
    tx_hash: [u8; 32],
    sender: T::AccountId,
    tx_type: Vec<u8>,
}

Event::TransactionFailed {
    tx_hash: [u8; 32],
    sender: T::AccountId,
    reason: Vec<u8>,
}

Event::StakeDeposited {
    account: T::AccountId,
    amount: Balance,
    lock_period: u32,
}

Event::StakeWithdrawn {
    account: T::AccountId,
    amount: Balance,
}

Event::ContractCalled {
    contract: Vec<u8>,
    caller: T::AccountId,
    vmw_used: VMw,
}

Event::ContractDeployed {
    contract: Vec<u8>,
    deployer: T::AccountId,
    code_hash: [u8; 32],
}

Event::LightningBlocCreated {
    channel_id: u32,
    target_chain: ChainId,
}

Event::LightningBlocPaymentRouted {
    channel_id: u32,
    sender: T::AccountId,
    recipient: Vec<u8>,
    amount: Balance,
}
```

---

## **ERRORS (DISPATCHABLE)**

```rust
// From pallet-native-currency

Error::InsufficientEtridBalance       // Not enough ÉTR
Error::InsufficientEtdBalance         // Not enough ETD
Error::VMwBlockLimitExceeded          // Block hit 10M VMw limit
Error::VMwTxLimitExceeded             // TX hit 1M VMw limit
Error::EtridSupplyCapped              // Would exceed 1B ÉTR
Error::EtdSupplyCapped                // Would exceed 2.5B ETD
Error::InvalidAmount                  // Amount is zero
Error::AccountNotFound                // Account doesn't exist
Error::VMwPriceNotSet                 // Op price not initialized
Error::InvalidVMwCost                 // Invalid cost value

// ---

// From pallet-transaction

Error::NonceMismatch                  // Nonce doesn't match
Error::InvalidSignature               // Signature verification failed
Error::InsufficientBalance            // Not enough to cover fees
Error::VMwLimitExceeded               // TX VMw > 1M limit
Error::ContractNotFound               // Contract doesn't exist
Error::InvalidContractCode            // Code is invalid WASM
Error::ContractExecutionFailed        // Contract call reverted
Error::StakeLocked                    // Stake still in lock period
Error::LightningBlocChannelNotFound   // Channel doesn't exist
Error::LightningBlocPaymentFailed     // Payment routing failed
Error::InvalidRecipient               // Recipient address invalid
Error::TransactionDuplicate           // TX already in pool
Error::InvalidTransactionFormat       // Malformed TX data
Error::ChainIdMismatch                // Wrong chain for TX
```

---

## **CONVERSION HELPER (CurrencyUnit)**

```rust
use pallet_native_currency::CurrencyUnit;

impl<T: Config> Pallet<T> {
    pub fn convert_currency(
        amount: Balance,
        from_unit: CurrencyUnit,
        to_unit: CurrencyUnit,
    ) -> Balance {
        // Convert between any two currency units
        // Example: 1 ÉTR to Bitë
        let result = Pallet::<T>::convert_currency(
            1 * ONE_ETRID,
            CurrencyUnit::Etrid,
            CurrencyUnit::Bite,
        );
        // result = 100_000 (100K Bitë)
    }
}

// Available units:
CurrencyUnit::Bite
CurrencyUnit::Tribite
CurrencyUnit::Quadrite
CurrencyUnit::Octobite
CurrencyUnit::Sextobite
CurrencyUnit::Etrid           // Base
CurrencyUnit::KiloEtrid
CurrencyUnit::MegaEtrid
CurrencyUnit::GigaEtrid
```

---

## **QUICK MATH**

```
Fee Calculation:
Fee (ÉTR) = (VMw_Used × Op_Price) ÷ 1,000,000

Examples:
- 500 VMw @ price=1  → (500 × 1) ÷ 1M = 0.0005 ÉTR
- 500 VMw @ price=2  → (500 × 2) ÷ 1M = 0.001 ÉTR
- 2000 VMw @ price=3 → (2000 × 3) ÷ 1M = 0.006 ÉTR
- 300 VMw @ price=1  → (300 × 1) ÷ 1M = 0.0003 ÉTR

Token Supply Check:
- Current ÉTR minted < 1,000,000,000 ÉTR (1B)
- Current ETD minted < 2,500,000,000 ETD (2.5B)

VMw Limits:
- Block has max 10,000,000 VMw available
- Each TX max 1,000,000 VMw
- If TX uses 1M VMw at price=2: Fee = 2 ÉTR

Currency Conversion:
- 1 ÉTR = 100,000 Bitë
- 1 Tribite = 10 Bitë
- 1 Quadrite = 100 Bitë
- 1 Octobite = 1,000 Bitë
- 1 Sextobite = 10,000 Bitë
```

---

**Version:** 1.0  
**Date:** October 15, 2025  
**For:** ÉTRID Week 0 Pallets  
**Keep nearby while developing!** 📝
