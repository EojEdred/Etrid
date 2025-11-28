# Account Recovery System

## Overview

The Ëtrid Protocol Account Recovery System implements a social recovery mechanism that allows users to regain access to their accounts if they lose their private keys. This feature is critical for user experience and security, providing a safety net while maintaining decentralization through a guardian-based approval system.

## Architecture

### Core Structures

#### RecoveryConfig

Defines the recovery parameters for an account:

```rust
pub struct RecoveryConfig<AccountId, BlockNumber> {
    pub guardians: BoundedVec<AccountId, ConstU32<MAX_GUARDIANS>>,
    pub threshold: u32,  // M-of-N threshold
    pub delay_period: BlockNumber,  // Blocks to wait before recovery
}
```

- **guardians**: Up to 10 trusted accounts who can approve recovery
- **threshold**: Minimum number of guardian approvals required (M-of-N)
- **delay_period**: Number of blocks to wait after threshold is met before recovery can be executed

#### ActiveRecovery

Tracks an ongoing recovery process:

```rust
pub struct ActiveRecovery<AccountId, BlockNumber> {
    pub new_account: AccountId,
    pub approvals: BoundedVec<AccountId, ConstU32<MAX_GUARDIANS>>,
    pub created_at: BlockNumber,
    pub executable_at: BlockNumber,
}
```

- **new_account**: The destination account that will receive the recovered assets
- **approvals**: List of guardians who have approved this recovery
- **created_at**: Block number when recovery was initiated
- **executable_at**: Block number when recovery becomes executable

## Workflow

### 1. Setup Recovery Configuration

An account owner must first set up their recovery configuration:

```rust
Accounts::create_recovery(
    origin,
    guardians: vec![guardian1, guardian2, guardian3],
    threshold: 2,  // 2-of-3
    delay_period: 10_000,  // ~48 hours at 6s blocks
)
```

**Requirements:**
- `threshold > 0`
- `threshold <= guardians.len()`
- `guardians.len() <= MAX_GUARDIANS (10)`
- At least one guardian must be specified

### 2. Initiate Recovery

When an account is lost, any guardian can initiate the recovery process:

```rust
Accounts::initiate_recovery(
    origin,  // Guardian's account
    lost_account,
    new_account  // Destination for recovered assets
)
```

**Requirements:**
- Caller must be a registered guardian
- No active recovery must exist for the lost account
- A valid recovery config must exist

**Effects:**
- Creates an `ActiveRecovery` entry
- Records the initiating guardian's approval
- Sets `executable_at = current_block + delay_period`

### 3. Approve Recovery

Additional guardians approve the recovery:

```rust
Accounts::approve_recovery(
    origin,  // Guardian's account
    lost_account
)
```

**Requirements:**
- Caller must be a registered guardian
- An active recovery must exist
- Guardian must not have already approved

**Effects:**
- Adds guardian to the approvals list
- Emits `RecoveryApproved` event with current approval count

### 4. Execute Recovery

Once threshold and delay are met, anyone can execute the recovery:

```rust
Accounts::execute_recovery(
    origin,  // Any account
    lost_account
)
```

**Requirements:**
- Active recovery must exist
- Approval threshold must be met
- Delay period must have passed (`current_block >= executable_at`)

**Effects:**
- Transfers all ETR balance from lost to new account
- Transfers all ETD balance from lost to new account
- Transfers validator status and reputation
- Clears the lost account
- Removes active recovery entry
- Removes recovery config

### 5. Cancel Recovery (Optional)

The original account owner can cancel an active recovery:

```rust
Accounts::cancel_recovery(
    origin,  // Must be the lost account owner
    account
)
```

**Requirements:**
- Caller must be the account owner
- Active recovery must exist

**Effects:**
- Removes the active recovery entry
- Recovery config remains intact for future use

## Security Features

### 1. M-of-N Guardian Threshold

The threshold system requires multiple guardians to collude to recover an account, preventing single points of failure:

- Minimum: 1 guardian (suitable for testing, not recommended for production)
- Recommended: 2-of-3, 3-of-5, or higher for high-value accounts
- Maximum: 10 guardians

### 2. Time Delay

The delay period provides a window for the legitimate owner to cancel malicious recovery attempts:

- Recommended: 24-72 hours (14,400 - 43,200 blocks at 6s/block)
- Allows time for monitoring and response
- Balance between security and usability

### 3. Owner Can Cancel

At any point during the recovery process, the original owner can cancel if they regain access to their account.

### 4. Guardian Authorization

Only registered guardians can initiate or approve recoveries, preventing unauthorized recovery attempts.

### 5. Single Active Recovery

Only one recovery can be active per account at a time, preventing confusion and potential conflicts.

## Events

```rust
RecoveryCreated { account, threshold }
// Emitted when a recovery config is created

RecoveryInitiated { lost_account, new_account, guardian }
// Emitted when a guardian initiates recovery

RecoveryApproved { lost_account, guardian, approvals }
// Emitted when a guardian approves recovery (includes current approval count)

RecoveryExecuted { lost_account, new_account }
// Emitted when recovery is successfully executed

RecoveryCancelled { account }
// Emitted when recovery is cancelled by the owner
```

## Error Handling

```rust
InvalidThreshold        // Threshold is 0
ThresholdTooHigh        // Threshold > number of guardians
NoGuardians            // Empty guardian list
TooManyGuardians       // More than MAX_GUARDIANS (10)
NoRecoveryConfig       // Account has no recovery config
RecoveryAlreadyActive  // Recovery already in progress
NotGuardian            // Caller is not a registered guardian
NoActiveRecovery       // No active recovery to approve/execute
AlreadyApproved        // Guardian already approved this recovery
ThresholdNotMet        // Insufficient approvals to execute
DelayNotPassed         // Delay period hasn't elapsed
NotAccountOwner        // Only owner can cancel
```

## Best Practices

### For Account Owners

1. **Choose Guardians Wisely**
   - Select trusted individuals or accounts
   - Use a mix of personal and institutional guardians
   - Ensure guardians are independent (no single entity controls multiple guardians)

2. **Set Appropriate Threshold**
   - Too low: Easier for attackers if guardians are compromised
   - Too high: Harder to recover if guardians are unavailable
   - Recommended: 2-of-3 or 3-of-5

3. **Configure Adequate Delay**
   - 24 hours minimum for monitoring
   - 48-72 hours recommended for high-value accounts
   - Balance security with recovery urgency

4. **Monitor Your Account**
   - Watch for `RecoveryInitiated` events
   - Cancel immediately if unauthorized
   - Keep guardian list updated

### For Guardians

1. **Verify Recovery Requests**
   - Confirm identity through out-of-band channels
   - Verify the new account address
   - Check for legitimate loss of access

2. **Respond Promptly**
   - Be available to approve legitimate recoveries
   - Decline suspicious requests
   - Communicate with other guardians

3. **Maintain Security**
   - Secure your own account
   - Use hardware wallets or secure storage
   - Update contact information

## Usage Examples

### Example 1: Basic Setup

```rust
// Alice sets up recovery with Bob and Charlie as guardians (2-of-2)
Accounts::create_recovery(
    RuntimeOrigin::signed(alice),
    vec![bob, charlie],
    2,
    28_800,  // 48 hours
)?;
```

### Example 2: Full Recovery Flow

```rust
// 1. Bob initiates recovery for Alice's lost account
Accounts::initiate_recovery(
    RuntimeOrigin::signed(bob),
    alice,
    alice_new,
)?;

// 2. Charlie approves (reaches 2-of-2 threshold)
Accounts::approve_recovery(
    RuntimeOrigin::signed(charlie),
    alice,
)?;

// 3. Wait 48 hours...

// 4. Anyone executes the recovery
Accounts::execute_recovery(
    RuntimeOrigin::signed(dave),
    alice,
)?;

// Result: alice_new now has all of alice's assets
```

### Example 3: Cancel Malicious Recovery

```rust
// Attacker initiates recovery
Accounts::initiate_recovery(
    RuntimeOrigin::signed(malicious_guardian),
    alice,
    attacker,
)?;

// Alice (still has access) cancels immediately
Accounts::cancel_recovery(
    RuntimeOrigin::signed(alice),
    alice,
)?;

// Recovery cancelled, alice's assets are safe
```

## Integration

### Runtime Integration

Add to your runtime's `lib.rs`:

```rust
impl pallet_accounts::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u64;
    type GovernanceOrigin = EnsureRoot<AccountId>;
}

construct_runtime! {
    pub enum Runtime {
        // ... other pallets
        Accounts: pallet_accounts,
    }
}
```

### UI Integration

Front-end applications should:

1. **Display Recovery Status**
   - Show if recovery is configured
   - Display guardian list and threshold
   - Alert on active recoveries

2. **Provide Recovery Management**
   - Guardian selection interface
   - Threshold and delay configuration
   - Recovery initiation for guardians
   - Approval interface for guardians

3. **Monitor Events**
   - Subscribe to recovery events
   - Alert users of recovery attempts
   - Provide cancel button for owners

## Performance Considerations

- **Storage**: O(1) lookups for configs and active recoveries
- **Computation**: Guardian list operations are O(n) where n ≤ 10
- **Weight**: All extrinsics use constant weight (10,000)
- **Memory**: Bounded vectors prevent unbounded growth

## Security Auditing Checklist

- [ ] Guardian uniqueness validation
- [ ] Threshold bounds checking (1 ≤ M ≤ N ≤ 10)
- [ ] Delay period enforcement
- [ ] Authorization checks on all operations
- [ ] Duplicate approval prevention
- [ ] Storage cleanup on execution
- [ ] Event emission completeness
- [ ] Error handling coverage
- [ ] Reentrancy protection
- [ ] Balance transfer validation

## Future Enhancements

### Potential Improvements

1. **Dynamic Guardian Updates**
   - Allow adding/removing guardians without recreating config
   - Require guardian approval for changes

2. **Multi-Asset Recovery**
   - Support for recovering locked/staked assets
   - NFT and multi-token recovery

3. **Tiered Recovery**
   - Different thresholds for different recovery amounts
   - Partial recovery options

4. **Guardian Incentives**
   - Small rewards for guardians who help with recovery
   - Reputation system for reliable guardians

5. **Recovery Templates**
   - Pre-configured guardian sets (e.g., certified custodians)
   - Recommended configurations based on account value

6. **Social Graph Integration**
   - Suggested guardians based on transaction history
   - Web-of-trust based guardian discovery

## Conclusion

The Account Recovery System provides a crucial safety net for Ëtrid Protocol users while maintaining security through multiple independent checks. By following best practices and carefully selecting guardians, users can confidently secure their accounts against loss while retaining the benefits of decentralized control.

For support and questions, refer to the Ëtrid Protocol documentation or community channels.

---

**Version**: 1.0.0
**Last Updated**: 2025-10-22
**Status**: Alpha (95% → 100% Complete)
