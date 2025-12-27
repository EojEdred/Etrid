# 01. DETR P2P - Distributed Encrypted Transport & Peer-to-Peer Networking

## Overview

The DETR P2P layer provides secure, distributed peer-to-peer networking infrastructure for the Ëtrid blockchain. It handles node discovery, encrypted transport, peer management, message routing, and flow control across the entire network.

**Status:** 🟡 Structured (Core modules implemented, integration in progress)

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    DETR P2P Network Layer                    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Discovery  │  │   Transport  │  │     Crypto   │      │
│  │    (core)    │  │    (core)    │  │    (core)    │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                  │              │
│         └──────────────────┼──────────────────┘              │
│                            ↓                                 │
│              ┌─────────────────────────┐                     │
│              │   DETR P2P Protocol     │                     │
│              │   (detrp2p - Rust)      │                     │
│              └─────────┬───────────────┘                     │
│                        │                                     │
│         ┌──────────────┼──────────────┐                     │
│         ↓              ↓               ↓                     │
│  ┌───────────┐  ┌────────────┐  ┌──────────┐              │
│  │  AEComms  │  │   DPeers   │  │  Stored  │              │
│  │ (Encrypt) │  │   (Peers)  │  │ (Cache)  │              │
│  └─────┬─────┘  └─────┬──────┘  └────┬─────┘              │
│        │              │               │                     │
│        └──────────────┼───────────────┘                     │
│                       ↓                                     │
│            ┌─────────────────────┐                          │
│            │  Etrid Protocol     │                          │
│            │  (Message Defs)     │                          │
│            └──────────┬──────────┘                          │
│                       ↓                                     │
│            ┌─────────────────────┐                          │
│            │      Fluent         │                          │
│            │   (Flow Control)    │                          │
│            └─────────────────────┘                          │
│                       ↓                                     │
└───────────────────────┼─────────────────────────────────────┘
                        ↓
              Substrate/Primearc Core Chain
              Network Interface
```

## Components

### 1. core (Go) - Core P2P Infrastructure

**Location:** `01-detr-p2p/core/`
**Language:** Go 1.21
**Purpose:** Secure peer-to-peer protocol with node discovery and encrypted transport

**Modules:**
- **crypto/** - Cryptographic primitives for P2P
- **discovery/** - Node discovery mechanism
- **transport/** - Low-level encrypted transport layer

**Key Features:**
- Node discovery protocol
- Encrypted P2P connections
- Low-level transport layer
- Docker containerization support

**Status:** 🟡 Core implementation complete

---

### 2. detrp2p (Rust) - Distributed Encrypted Transport

**Location:** `01-detr-p2p/detrp2p/`
**Package:** `detrp2p`
**Purpose:** Distributed Encrypted Transport for Ëtrid Network

**Description:**
Main Rust implementation of the DETR P2P protocol, providing high-level distributed encrypted transport functionality.

**Dependencies:**
- `tokio` - Async runtime
- `serde`, `bincode` - Serialization
- `ecies` - Elliptic Curve Integrated Encryption Scheme

**Key Features:**
- Distributed encrypted transport
- Async I/O with Tokio
- ECIES encryption
- Binary serialization
- Announce PoW admission gate (default difficulty 14, increase-only override)

**Status:** 🟡 Implemented

---

### 3. aecomms (Rust) - Authenticated Encrypted Communications

**Location:** `01-detr-p2p/aecomms/`
**Package:** `etrid-aecomms`
**Purpose:** Encrypted communications layer

**Cryptographic Stack:**
- **Key Exchange:** X25519 (Diffie-Hellman)
- **Encryption:** ChaCha20-Poly1305 (AEAD)
- **Hashing:** SHA-2
- **RNG:** Cryptographically secure random number generation

**Key Features:**
- Authenticated encryption
- Forward secrecy with X25519
- AEAD cipher (ChaCha20-Poly1305)
- Async communications

**Status:** 🟡 Implemented

---

### 4. dpeers (Rust) - Distributed Peer Management

**Location:** `01-detr-p2p/dpeers/`
**Package:** `etrid-p2p-dpeers`
**Purpose:** Distributed peer management for Ëtrid P2P network

**Responsibilities:**
- Peer discovery and registration
- Peer connection lifecycle management
- Peer reputation and scoring
- Peer list maintenance

**Status:** 🟡 Implemented

---

### 5. etrid-protocol (Rust) - Protocol Message Definitions

**Location:** `01-detr-p2p/etrid-protocol/`
**Package:** `etrid-protocol`
**Purpose:** Ëtrid network protocol message definitions and serialization

**Description:**
Defines all P2P protocol messages used across the Ëtrid network, including serialization formats and message types.

**Dependencies:**
- `serde` - Serialization framework
- `bincode` - Binary serialization
- `sha2` - Message hashing
- `rand` - Nonce generation

**Key Features:**
- Protocol message type definitions
- Binary serialization/deserialization
- Message integrity (hashing)
- Gadget network bridge integration

**Status:** 🟡 Implemented

---

### 6. fluent (Rust) - Message Flow Control

**Location:** `01-detr-p2p/fluent/`
**Package:** `etrid-p2p-fluent`
**Purpose:** Message flow control for Ëtrid P2P

**Responsibilities:**
- Message queuing
- Priority routing
- Rate limiting
- Flow control
- Backpressure management

**Key Features:**
- Priority-based message queuing
- Configurable rate limiting
- Flow control mechanisms
- Async message handling

**Status:** 🟡 Implemented

---

### 7. stored (Rust) - Peer Storage & Caching

**Location:** `01-detr-p2p/stored/`
**Package:** `detrp2p-peerstored`
**Purpose:** Peer storage and caching module

**Responsibilities:**
- Peer data caching
- Persistent peer storage
- Peer metadata management
- Historical connection data

**Status:** 🟡 Implemented

---

## Data Flow

### Message Transmission Flow

```
Application Layer
      ↓
┌─────────────────┐
│ Etrid Protocol  │ ← Message serialization
└────────┬────────┘
         ↓
┌─────────────────┐
│     Fluent      │ ← Flow control, queuing, rate limiting
└────────┬────────┘
         ↓
┌─────────────────┐
│    DETR P2P     │ ← Protocol handling
└────────┬────────┘
         ↓
┌─────────────────┐
│    AEComms      │ ← Encryption (ChaCha20-Poly1305)
└────────┬────────┘
         ↓
┌─────────────────┐
│ Core Transport  │ ← Go-based encrypted transport
└────────┬────────┘
         ↓
    Network I/O
```

### Network Admission (PoW Announce)

DETR P2P applies a lightweight proof-of-work check on `Announce` messages to deter spam/Sybil peers.
Defaults are baked into the binary and can only be increased via environment variables.

**Defaults:**
- `DETR_P2P_POW_DIFFICULTY`: `14` (increase-only via env)
- `DETR_P2P_POW_MAX_NONCE`: `5_000_000`

**Notes:**
- This PoW gate affects peer admission only; it does **not** affect PoS consensus.
- Nodes compute a nonce once on startup and reuse it on reconnect.

### Peer Discovery Flow

```
1. Node starts → Discovery module (core/discovery)
2. Broadcast discovery message
3. Peers respond → DPeers registers peer
4. Peer metadata → Stored caches peer data
5. Connection established → AEComms encrypts
6. Peer available for messaging
```

## API Design

### Core P2P Protocol (detrp2p)

```rust
// Connect to peer
pub async fn connect_peer(peer_id: PeerId, addr: SocketAddr) -> Result<Connection>;

// Send message
pub async fn send_message(peer_id: PeerId, message: Message) -> Result<()>;

// Broadcast message
pub async fn broadcast(message: Message, peers: Vec<PeerId>) -> Result<()>;

// Disconnect peer
pub async fn disconnect_peer(peer_id: PeerId) -> Result<()>;
```

### Authenticated Encryption (aecomms)

```rust
// Establish encrypted channel
pub async fn establish_channel(remote_pubkey: PublicKey) -> Result<SecureChannel>;

// Encrypt and send
pub async fn encrypt_send(channel: &SecureChannel, data: &[u8]) -> Result<()>;

// Receive and decrypt
pub async fn receive_decrypt(channel: &SecureChannel) -> Result<Vec<u8>>;
```

### Peer Management (dpeers)

```rust
// Add peer
pub fn add_peer(peer_id: PeerId, metadata: PeerMetadata) -> Result<()>;

// Get peer info
pub fn get_peer(peer_id: &PeerId) -> Option<PeerInfo>;

// List active peers
pub fn list_peers() -> Vec<PeerId>;

// Update peer reputation
pub fn update_reputation(peer_id: &PeerId, score: i32) -> Result<()>;
```

### Message Protocol (etrid-protocol)

```rust
#[derive(Serialize, Deserialize)]
pub enum BlockSyncMessage {
    BlockAnnounce(BlockAnnounceMessage),
    BlockRequest(BlockRequestMessage),
    BlockResponse(BlockResponseMessage),
    StatusRequest(StatusRequestMessage),
    StatusResponse(StatusResponseMessage),
}

#[derive(Serialize, Deserialize)]
pub struct ProtocolMessage {
    pub version: u16,
    pub msg_type: u8,
    pub payload: Vec<u8>,
}
```

Block sync message definitions live in `etrid-protocol`, while `detrp2p::Message` provides
conversion helpers (`as_block_sync`) to interop with `BlockSyncMessage`.

### Flow Control (fluent)

```rust
// Configure flow control
pub fn configure(config: FlowConfig) -> FlowController;

// Send with priority
pub async fn send_priority(message: Message, priority: Priority) -> Result<()>;

// Set rate limit
pub fn set_rate_limit(limit: RateLimit) -> Result<()>;

// Get queue stats
pub fn queue_stats() -> QueueStats;
```

### Peer Storage (stored)

```rust
// Store peer data
pub async fn store_peer(peer_id: PeerId, data: PeerData) -> Result<()>;

// Retrieve peer data
pub async fn get_peer_data(peer_id: &PeerId) -> Result<Option<PeerData>>;

// Clear cache
pub async fn clear_cache() -> Result<()>;
```

## Protocol Layers

### Layer 1: Physical Network
- TCP/IP transport
- WebSocket support (planned)
- QUIC support (planned)

### Layer 2: Encrypted Transport (core)
- Node discovery (Go implementation)
- Low-level encryption
- Connection management

### Layer 3: Protocol Layer (detrp2p)
- DETR P2P protocol
- Message routing
- Peer authentication

### Layer 4: Application Layer
- Message definitions (etrid-protocol)
- Flow control (fluent)
- Peer management (dpeers)
- Caching (stored)

## Security Features

### Encryption
- **At-Rest:** Peer storage encryption (planned)
- **In-Transit:**
  - X25519 key exchange
  - ChaCha20-Poly1305 AEAD
  - Forward secrecy

### Authentication
- Peer identity verification
- Message integrity (SHA-2 hashing)
- Nonce-based replay protection

### Network Security
- Rate limiting (fluent)
- Reputation scoring (dpeers)
- DDoS protection (planned)

## Performance Characteristics

### Throughput
- **Message Processing:** Async with Tokio (high concurrency)
- **Encryption Overhead:** ChaCha20-Poly1305 (~1-2 GB/s on modern CPUs)
- **Serialization:** Binary (bincode) for minimal overhead

### Latency
- **Connection Establishment:** ~100-200ms (key exchange + handshake)
- **Message Latency:** <10ms (encryption + serialization)

### Scalability
- **Peers:** Designed for 1,000+ concurrent peers
- **Messages:** Queue-based flow control handles bursts

## Integration Points

### With Substrate
```rust
// Substrate network integration
impl NetworkService for DetrP2P {
    fn send_notification(&self, peer: PeerId, data: Vec<u8>);
    fn receive_notification(&self) -> Stream<(PeerId, Vec<u8>)>;
}
```

### With Primearc Core Chain
- Consensus messages
- Block propagation
- Transaction broadcasting

### With PBC Chains
- Cross-chain messaging
- Bridge communication
- State synchronization

## Configuration

### Network Configuration
```toml
[detrp2p]
listen_addr = "0.0.0.0:30333"
max_peers = 50
connection_timeout = "30s"
enable_discovery = true

[aecomms]
cipher = "ChaCha20Poly1305"
key_exchange = "X25519"

[fluent]
max_queue_size = 10000
rate_limit = "1000 msg/s"
priority_levels = 5

[dpeers]
max_peers = 100
reputation_threshold = 50
```

## Development Status

| Component | Implementation | Tests | Documentation | Status |
|-----------|---------------|-------|---------------|--------|
| core (Go) | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |
| detrp2p | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |
| aecomms | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |
| dpeers | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |
| etrid-protocol | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |
| fluent | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |
| stored | ✅ Complete | 🟡 Partial | 📝 Basic | Alpha |

## Testing

### Unit Tests
```bash
# Test all P2P modules
cd 01-detr-p2p
cargo test --all

# Test specific module
cargo test -p detrp2p
cargo test -p etrid-aecomms
cargo test -p etrid-p2p-dpeers
```

### Integration Tests
```bash
# Multi-node P2P tests
cargo test --test integration_tests
```

### Go Core Tests
```bash
cd 01-detr-p2p/core
go test ./...
```

## Known Issues

1. **Incomplete Test Coverage** - Unit tests need expansion
2. **Documentation Gaps** - API docs need completion
3. **Performance Tuning** - Flow control parameters need optimization
4. **WebSocket Support** - Not yet implemented
5. **QUIC Transport** - Planned but not implemented

## Roadmap

### Phase 1: Core Functionality (✅ Complete)
- [x] Basic P2P protocol
- [x] Encrypted transport
- [x] Peer management
- [x] Message definitions

### Phase 2: Production Hardening (🟡 In Progress)
- [ ] Comprehensive test suite
- [ ] Performance benchmarks
- [ ] Security audit
- [ ] API documentation

### Phase 3: Advanced Features (Planned)
- [ ] WebSocket transport
- [ ] QUIC protocol support
- [ ] Advanced discovery (DHT)
- [ ] Gossip protocol optimization

### Phase 4: Monitoring & Observability (Planned)
- [ ] Metrics collection
- [ ] Network visualization
- [ ] Peer analytics
- [ ] Health monitoring

## References

### Core Technologies
- **Tokio:** https://tokio.rs/
- **X25519:** Elliptic curve Diffie-Hellman (RFC 7748)
- **ChaCha20-Poly1305:** AEAD cipher (RFC 8439)
- **Go:** https://go.dev/

### Related Documentation
- [Substrate Networking](https://docs.substrate.io/fundamentals/node-and-network-types/)
- [libp2p Design](https://docs.libp2p.io/)
- [Noise Protocol](https://noiseprotocol.org/)

---

**Component:** 01-detr-p2p
**Version:** 0.1.0
**Status:** Alpha
**Last Updated:** October 20, 2025
