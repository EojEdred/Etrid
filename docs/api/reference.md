# Ëtrid API Reference

> Complete API reference for all Ëtrid components, pallets, and services

**Last Updated:** October 20, 2025
**Version:** 1.0.0

---

## Table of Contents

1. [Overview](#overview)
2. [JSON-RPC API](#json-rpc-api)
3. [Component APIs](#component-apis)
4. [Pallet Extrinsics](#pallet-extrinsics)
5. [Storage Queries](#storage-queries)
6. [Events](#events)
7. [Error Codes](#error-codes)
8. [Service APIs](#service-apis)

---

## Overview

Ëtrid provides multiple API layers:

- **JSON-RPC:** Standard blockchain queries and transactions
- **Pallet Extrinsics:** On-chain functions callable via transactions
- **Storage Queries:** Read blockchain state
- **Events:** Subscribe to blockchain events
- **Service APIs:** Off-chain services (REST/WebSocket)

### API Endpoints

**FlareChain (Mainnet - Planned):**
- RPC: `wss://rpc.etrid.io`
- WebSocket: `wss://ws.etrid.io`

**Testnet:**
- RPC: `wss://ember-rpc.etrid.io`
- WebSocket: `wss://ember-ws.etrid.io`

**Local Development:**
- RPC: `http://localhost:9944`
- WebSocket: `ws://localhost:9944`

---

## JSON-RPC API

### System Methods

#### system_chain

Get chain name.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "system_chain",
  "params": []
}
```

**Response:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "Etrid FlareChain"
}
```

#### system_health

Get node health status.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "system_health",
  "params": []
}
```

**Response:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "isSyncing": false,
    "peers": 25,
    "shouldHavePeers": true
  }
}
```

#### system_peers

Get connected peers.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "system_peers",
  "params": []
}
```

**Response:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": [
    {
      "peerId": "12D3Koo...",
      "roles": "AUTHORITY",
      "bestHash": "0x1234...",
      "bestNumber": 12345
    }
  ]
}
```

### Chain Methods

#### chain_getHeader

Get block header by hash (or latest if omitted).

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "chain_getHeader",
  "params": ["0x1234..."]
}
```

**Response:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": {
    "number": "0x2a",
    "parentHash": "0x5678...",
    "stateRoot": "0x9abc...",
    "extrinsicsRoot": "0xdef0..."
  }
}
```

#### chain_getBlock

Get full block by hash.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "chain_getBlock",
  "params": ["0x1234..."]
}
```

#### chain_getBlockHash

Get block hash by number.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "chain_getBlockHash",
  "params": [42]
}
```

#### chain_subscribeNewHeads

Subscribe to new block headers.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "chain_subscribeNewHeads",
  "params": []
}
```

**Response (subscription):**
```json
{
  "jsonrpc": "2.0",
  "method": "chain_newHead",
  "params": {
    "subscription": "abc123",
    "result": {
      "number": "0x2b",
      "parentHash": "0x1234..."
    }
  }
}
```

### State Methods

#### state_getStorage

Query storage value at key.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "state_getStorage",
  "params": ["0x1234..."]
}
```

#### state_getMetadata

Get runtime metadata.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "state_getMetadata",
  "params": []
}
```

#### state_call

Call runtime API method.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "state_call",
  "params": ["AccountNonceApi_account_nonce", "0x1234..."]
}
```

### Author Methods

#### author_submitExtrinsic

Submit signed extrinsic.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "author_submitExtrinsic",
  "params": ["0xabcd..."]
}
```

**Response:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "result": "0x1234..."
}
```

#### author_pendingExtrinsics

Get pending extrinsics in pool.

**Request:**
```json
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "author_pendingExtrinsics",
  "params": []
}
```

---

## Component APIs

### 01. DETR P2P

**Package:** `etrid-p2p-dpeers`

#### Peer Management API

```rust
/// Add peer to peer list
pub fn add_peer(peer_id: PeerId, metadata: PeerMetadata) -> Result<()>;

/// Get peer information
pub fn get_peer(peer_id: &PeerId) -> Option<PeerInfo>;

/// List all active peers
pub fn list_peers() -> Vec<PeerId>;

/// Update peer reputation score
pub fn update_reputation(peer_id: &PeerId, score: i32) -> Result<()>;

/// Remove peer
pub fn remove_peer(peer_id: &PeerId) -> Result<()>;
```

**Package:** `etrid-aecomms`

#### Encrypted Communications API

```rust
/// Establish encrypted channel
pub async fn establish_channel(remote_pubkey: PublicKey) -> Result<SecureChannel>;

/// Encrypt and send data
pub async fn encrypt_send(channel: &SecureChannel, data: &[u8]) -> Result<()>;

/// Receive and decrypt data
pub async fn receive_decrypt(channel: &SecureChannel) -> Result<Vec<u8>>;

/// Close channel
pub async fn close_channel(channel: SecureChannel) -> Result<()>;
```

**Package:** `etrid-p2p-fluent`

#### Flow Control API

```rust
/// Configure flow control
pub fn configure(config: FlowConfig) -> FlowController;

/// Send message with priority
pub async fn send_priority(message: Message, priority: Priority) -> Result<()>;

/// Set rate limit
pub fn set_rate_limit(limit: RateLimit) -> Result<()>;

/// Get queue statistics
pub fn queue_stats() -> QueueStats;
```

---

### 02. OpenDID

**Package:** `etrid-did-registry`

#### DID Registry API

```rust
/// Register new DID
pub fn register_did(did: Did, document: DidDocument) -> Result<()>;

/// Update DID document
pub fn update_did(did: Did, document: DidDocument) -> Result<()>;

/// Deactivate DID
pub fn deactivate_did(did: Did) -> Result<()>;

/// Get DID document
pub fn get_did_document(did: &Did) -> Option<DidDocument>;

/// Check if account is controller
pub fn is_controller(did: &Did, account: &AccountId) -> bool;
```

**Package:** `etrid-did-resolver`

#### DID Resolver API

```rust
/// Resolve DID to document
pub async fn resolve(did: &str) -> Result<DidDocument>;

/// Resolve with metadata
pub async fn resolve_with_metadata(did: &str) -> Result<(DidDocument, Metadata)>;
```

**Package:** `aidid`

#### AI Identity API

```rust
/// Register AI identity
pub fn register_ai(aidid: AIDID, document: AIDIDDocument) -> Result<()>;

/// Update AI profile
pub fn update_profile(aidid: &AIDID, profile: AIProfile) -> Result<()>;

/// Attest AI model
pub fn attest_model(aidid: &AIDID, attestation: Attestation) -> Result<()>;

/// Get AI profile
pub fn get_profile(aidid: &AIDID) -> Option<AIProfile>;
```

---

### 03. Security

**Package:** `etrid-cryptography`

#### Cryptography API

```rust
/// Generate Ed25519 keypair
pub fn generate_keypair() -> Ed25519Keypair;

/// Sign message
pub fn sign(keypair: &Ed25519Keypair, message: &[u8]) -> Signature;

/// Verify signature
pub fn verify(public_key: &PublicKey, message: &[u8], signature: &Signature) -> bool;

/// X25519 key exchange
pub fn diffie_hellman(secret: &X25519Secret, peer_public: &PublicKey) -> SharedSecret;

/// SHA-256 hash
pub fn sha256(data: &[u8]) -> Hash;

/// HKDF derive key
pub fn derive_key(input: &[u8], salt: &[u8], info: &[u8]) -> DerivedKey;
```

**Package:** `etrid-key-management`

#### Key Management API

```rust
/// Store key securely
pub async fn store_key(key_id: KeyId, key: PrivateKey) -> Result<()>;

/// Retrieve key (requires authorization)
pub async fn get_key(key_id: &KeyId, auth: &Authorization) -> Result<PrivateKey>;

/// Rotate key
pub async fn rotate_key(key_id: &KeyId) -> Result<PrivateKey>;

/// Delete key
pub async fn delete_key(key_id: &KeyId) -> Result<()>;

/// Backup keys
pub async fn backup_keys(backup_file: &Path, password: &str) -> Result<()>;

/// Restore keys
pub async fn restore_keys(backup_file: &Path, password: &str) -> Result<KeyStore>;
```

---

### 04. Accounts

**Package:** `pallet-accounts`

#### Account Pallet API

See [Pallet Extrinsics](#pallet-extrinsics) section below.

---

### 05. Multichain

**Package:** `flarechain-runtime`

#### FlareChain Runtime API

```rust
/// Get account balance
pub fn account_balance(account_id: AccountId) -> Balance;

/// Get account nonce
pub fn account_nonce(account_id: AccountId) -> u64;

/// Get account type
pub fn account_type(account_id: AccountId) -> AccountType;
```

**Bridge APIs** - See individual bridge pallets in [Pallet Extrinsics](#pallet-extrinsics).

---

### 06. Native Currency

**Package:** `etrid-coin`

#### ÉTR Token API

```rust
/// Get total supply
pub fn total_supply() -> Balance;

/// Get account balance
pub fn balance_of(account: AccountId) -> Balance;

/// Transfer tokens
pub fn transfer(from: AccountId, to: AccountId, amount: Balance) -> Result<()>;
```

**Package:** `etd-stablecoin`

#### EDSC Stablecoin API

```rust
/// Mint EDSC (collateralized)
pub fn mint(account: AccountId, amount: Balance) -> Result<()>;

/// Burn EDSC (redeem ÉTR)
pub fn burn(account: AccountId, amount: Balance) -> Result<()>;

/// Get collateralization ratio
pub fn collateralization_ratio() -> Ratio;
```

**Package:** `vmw-gas`

#### VMw Gas API

```rust
/// Calculate gas cost
pub fn calculate_gas_cost(vmw_used: u64, op_price: Balance) -> Balance;

/// Get current gas price
pub fn current_gas_price() -> Balance;

/// Set gas price (governance only)
pub fn set_gas_price(price: Balance) -> Result<()>;
```

---

### 07. Transactions

**Package:** `lightning-bloc`

#### Lightning Bloc API

```rust
/// Open payment channel
pub fn open_channel(sender: AccountId, receiver: AccountId, deposit: Balance) -> Result<ChannelId>;

/// Send off-chain payment
pub fn send_payment(channel_id: ChannelId, amount: Balance) -> Result<()>;

/// Close channel and settle on-chain
pub fn close_channel(channel_id: ChannelId) -> Result<()>;

/// Get channel state
pub fn get_channel_state(channel_id: ChannelId) -> Option<ChannelState>;
```

---

### 08. ETWasm VM

**Package:** `pallet-etwasm`

#### Smart Contract API

```rust
/// Deploy contract
pub fn deploy_contract(code: Vec<u8>, init_params: Vec<u8>) -> Result<AccountId>;

/// Call contract
pub fn call_contract(contract: AccountId, method: Vec<u8>, params: Vec<u8>) -> Result<Vec<u8>>;

/// Get contract storage
pub fn get_storage(contract: AccountId, key: Vec<u8>) -> Option<Vec<u8>>;
```

---

### 09. Consensus (ASF)

**Package:** `pallet-consensus-asf`

#### Consensus API

See [Pallet Extrinsics](#pallet-extrinsics) for validator operations.

---

### 10-13. Other Components

See respective ARCHITECTURE.md files for detailed component APIs:
- [10-foundation](10-foundation/ARCHITECTURE.md)
- [11-peer-roles](11-peer-roles/ARCHITECTURE.md)
- [12-consensus-day](12-consensus-day/ARCHITECTURE.md)
- [13-clients](13-clients/ARCHITECTURE.md)

---

## Pallet Extrinsics

### System Pallet

**Module:** `frame_system`

#### set_code

Update runtime code (requires root).

**Parameters:**
- `code: Vec<u8>` - New runtime WASM

**Weight:** Variable (depends on code size)

**Errors:**
- `InvalidCode` - Code validation failed

---

### Balances Pallet

**Module:** `pallet_balances`

#### transfer

Transfer tokens to another account.

**Parameters:**
- `dest: AccountId` - Destination account
- `value: Balance` - Amount to transfer

**Weight:** `10_000`

**Events:**
- `Transfer { from, to, amount }`

**Errors:**
- `InsufficientBalance`
- `ExistentialDeposit`

#### transfer_keep_alive

Transfer ensuring sender remains above existential deposit.

**Parameters:**
- `dest: AccountId`
- `value: Balance`

**Weight:** `10_000`

**Errors:**
- `InsufficientBalance`
- `KeepAlive`

---

### Accounts Pallet

**Module:** `pallet_accounts`

#### create_account

Create new account with specific type.

**Parameters:**
- `account_type: AccountType` - EBCA, RCA, RCWA, SCA, or SSCA

**Weight:** `50_000`

**Events:**
- `AccountCreated { account_id, account_type }`

**Errors:**
- `AccountAlreadyExists`
- `InvalidAccountType`

---

### Staking Pallet

**Module:** `pallet_peer_roles`

#### bond

Bond tokens for staking.

**Parameters:**
- `value: Balance` - Amount to bond
- `payee: RewardDestination` - Where to send rewards

**Weight:** `100_000`

**Events:**
- `Bonded { stash, amount }`

**Errors:**
- `InsufficientBalance`
- `AlreadyBonded`

#### unbond

Schedule tokens for unbonding.

**Parameters:**
- `value: Balance` - Amount to unbond

**Weight:** `80_000`

**Events:**
- `Unbonded { stash, amount }`

**Errors:**
- `NotBonded`
- `InsufficientBonded`

#### validate

Declare intention to validate.

**Parameters:**
- `prefs: ValidatorPrefs` - Commission, etc.

**Weight:** `50_000`

**Events:**
- `ValidatorPrefsSet { stash, prefs }`

**Errors:**
- `NotController`
- `InsufficientBond`

---

### Governance Pallet

**Module:** `pallet_foundation`

#### propose

Submit governance proposal.

**Parameters:**
- `proposal: Proposal` - Proposal details
- `deposit: Balance` - Proposal deposit

**Weight:** `200_000`

**Events:**
- `Proposed { proposal_id, proposer }`

**Errors:**
- `InsufficientDeposit`
- `ProposalAlreadyExists`

#### vote

Vote on proposal.

**Parameters:**
- `proposal_id: ProposalId`
- `vote: Vote` - Aye, Nay, or Abstain

**Weight:** `50_000`

**Events:**
- `Voted { voter, proposal_id, vote }`

**Errors:**
- `ProposalNotFound`
- `AlreadyVoted`

---

### Consensus Day Pallet

**Module:** `pallet_consensus_day`

#### submit_consensus_day_proposal

Submit proposal for annual Consensus Day.

**Parameters:**
- `title: Vec<u8>`
- `description: Vec<u8>`
- `category: ProposalCategory`

**Weight:** `300_000`

**Events:**
- `ConsensusProposalSubmitted { proposal_id, title }`

**Errors:**
- `NotConsensusDayPeriod`
- `ProposalTooLarge`

#### cast_consensus_vote

Vote on Consensus Day proposal.

**Parameters:**
- `proposal_id: ProposalId`
- `vote: Vote`

**Weight:** `100_000`

**Events:**
- `ConsensusVoteCast { voter, proposal_id, vote, weight }`

**Errors:**
- `NotVotingPeriod`
- `InsufficientStake`

---

### EDSC Bridge Pallet

**Module:** `pallet_token_messenger`

#### deposit_for_burn

Burn EDSC on Ëtrid to mint on Ethereum.

**Parameters:**
- `amount: Balance`
- `dest_chain: u32`
- `recipient: Vec<u8>` - Ethereum address

**Weight:** `500_000`

**Events:**
- `DepositForBurn { sender, amount, dest_chain, nonce }`

**Errors:**
- `InsufficientBalance`
- `InvalidDestinationChain`

#### receive_message

Receive cross-chain message (mint EDSC).

**Parameters:**
- `message: BridgeMessage`
- `attestation: Attestation`

**Weight:** `800_000`

**Events:**
- `MintAndWithdraw { recipient, amount }`

**Errors:**
- `InvalidAttestation`
- `NonceAlreadyUsed`
- `InvalidMessage`

---

## Storage Queries

### Query Format

```
<pallet>.<storage_item>(<key>?)
```

### Examples

#### Get Account Balance

**Storage:** `System.Account`

**Query:**
```rust
let account_id = AccountId::from([1u8; 32]);
let account_info = frame_system::Account::<Runtime>::get(&account_id);
let balance = account_info.data.free;
```

**RPC:**
```bash
curl -d '{"id":1, "jsonrpc":"2.0", "method":"state_getStorage", "params":["0x26aa..."]}' http://localhost:9944
```

#### Get Total Issuance

**Storage:** `Balances.TotalIssuance`

**Query:**
```rust
let total = pallet_balances::TotalIssuance::<Runtime>::get();
```

#### Get Validator Count

**Storage:** `PeerRoles.ValidatorCount`

**Query:**
```rust
let count = pallet_peer_roles::ValidatorCount::<Runtime>::get();
```

#### Get DID Document

**Storage:** `DIDRegistry.DIDs`

**Query:**
```rust
let did = b"did:etrid:1234...";
let doc = pallet_did_registry::DIDs::<Runtime>::get(did);
```

---

## Events

### Event Subscription

**WebSocket:**
```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

const provider = new WsProvider('ws://localhost:9944');
const api = await ApiPromise.create({ provider });

// Subscribe to all events
api.query.system.events((events) => {
  events.forEach((record) => {
    const { event } = record;
    console.log(`${event.section}.${event.method}:`, event.data.toString());
  });
});
```

### Common Events

#### Transfer

**Module:** `Balances`

**Event:** `Transfer { from, to, amount }`

**Example:**
```json
{
  "phase": "ApplyExtrinsic",
  "event": {
    "method": "Transfer",
    "section": "balances",
    "data": ["5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty", "1000000000000"]
  }
}
```

#### Voted

**Module:** `Foundation`

**Event:** `Voted { voter, proposal_id, vote }`

#### Bonded

**Module:** `PeerRoles`

**Event:** `Bonded { stash, amount }`

#### BlockProduced

**Module:** `ConsensusASF`

**Event:** `BlockProduced { producer, block_number }`

---

## Error Codes

### System Errors

| Code | Error | Description |
|------|-------|-------------|
| 0 | `InvalidSpecName` | Spec name invalid |
| 1 | `SpecVersionNeedsToIncrease` | Spec version must increase |
| 2 | `FailedToExtractRuntimeVersion` | Cannot extract runtime version |

### Balances Errors

| Code | Error | Description |
|------|-------|-------------|
| 0 | `VestingBalance` | Account has vesting balance |
| 1 | `LiquidityRestrictions` | Liquidity restrictions prevent operation |
| 2 | `InsufficientBalance` | Insufficient balance |
| 3 | `ExistentialDeposit` | Below existential deposit |
| 4 | `KeepAlive` | Would kill account |

### Staking Errors

| Code | Error | Description |
|------|-------|-------------|
| 0 | `NotController` | Not the controller account |
| 1 | `NotStash` | Not the stash account |
| 2 | `AlreadyBonded` | Already bonded |
| 3 | `AlreadyPaired` | Already paired |
| 4 | `InsufficientBond` | Insufficient bond amount |

---

## Service APIs

### Attestation Service (EDSC Bridge)

**Base URL:** `http://localhost:3000` (dev)

#### POST /attest

Request attestation for bridge message.

**Request:**
```json
{
  "messageHash": "0x1234...",
  "sourceDomain": 0,
  "destDomain": 1,
  "nonce": 42
}
```

**Response:**
```json
{
  "attestation": "0xabcd...",
  "signature": "0x5678...",
  "timestamp": 1234567890
}
```

#### GET /health

Health check.

**Response:**
```json
{
  "status": "healthy",
  "uptime": 3600,
  "attestations": 142
}
```

---

### Relayer Service (EDSC Bridge)

**Base URL:** `http://localhost:3001` (dev)

#### POST /relay

Relay cross-chain message.

**Request:**
```json
{
  "message": "0x1234...",
  "attestation": "0xabcd...",
  "destChain": 1
}
```

**Response:**
```json
{
  "txHash": "0x5678...",
  "status": "pending"
}
```

#### GET /status/:txHash

Get relay status.

**Response:**
```json
{
  "txHash": "0x5678...",
  "status": "confirmed",
  "blockNumber": 12345
}
```

---

## SDK Examples

### Rust SDK

```rust
use etrid_sdk::*;

// Connect to node
let client = EtridClient::new("ws://localhost:9944").await?;

// Get account balance
let balance = client.account_balance(&account_id).await?;

// Transfer tokens
let tx = client.transfer(&from, &to, amount).await?;

// Subscribe to events
client.subscribe_events(|event| {
    println!("Event: {:?}", event);
}).await?;
```

### TypeScript SDK

```typescript
import { EtridClient } from '@etrid/sdk';

// Connect
const client = new EtridClient('ws://localhost:9944');
await client.connect();

// Get balance
const balance = await client.getBalance(accountId);

// Transfer
const tx = await client.transfer(from, to, amount);
await tx.wait();

// Subscribe to events
client.on('Transfer', (event) => {
  console.log('Transfer:', event);
});
```

### Python SDK

```python
from etrid_sdk import EtridClient

# Connect
client = EtridClient('ws://localhost:9944')
await client.connect()

# Get balance
balance = await client.get_balance(account_id)

# Transfer
tx = await client.transfer(from_account, to_account, amount)
await tx.wait()

# Subscribe to events
async for event in client.subscribe_events():
    print(f'Event: {event}')
```

---

## Rate Limits

**RPC Endpoints:**
- Anonymous: 100 requests/minute
- Authenticated: 1000 requests/minute
- WebSocket: 100 subscriptions/connection

**Service APIs:**
- Attestation: 60 requests/minute
- Relayer: 30 requests/minute

---

## Versioning

Ëtrid API uses semantic versioning:

- **Runtime Version:** Incremented with runtime upgrades
- **Spec Version:** Current: 1
- **Transaction Version:** Current: 1

**Breaking Changes:** Will be announced 30 days in advance.

---

## Additional Resources

- [Developer Guide](DEVELOPER_GUIDE.md)
- [Component Architecture Docs](README.md#component-architecture-documentation-new)
- [Polkadot.js API Docs](https://polkadot.js.org/docs/api/)
- [Substrate RPC](https://docs.substrate.io/reference/command-line-tools/node-rpc/)

---

**Last Updated:** October 20, 2025
**Maintainers:** Ëtrid Core Team
**License:** MIT

---

<p align="center">
  <sub>The Free and Open Decentralized Democracy of Stakeholders</sub>
</p>
