# Integration Guide: Using pallet-bridge-attestation in your PBC

This guide shows how to integrate the generic bridge attestation pallet into any PBC (Partition Burst Chain) in the Etrid ecosystem.

## Step 1: Add Dependency

Add to your PBC runtime's `Cargo.toml`:

```toml
[dependencies]
# ... other dependencies
pallet-bridge-attestation = { path = "../../pallets-shared/pallet-bridge-attestation", default-features = false }

[features]
default = ["std"]
std = [
    # ... other pallets
    "pallet-bridge-attestation/std",
]
runtime-benchmarks = [
    # ... other pallets
    "pallet-bridge-attestation/runtime-benchmarks",
]
try-runtime = [
    # ... other pallets
    "pallet-bridge-attestation/try-runtime",
]
```

## Step 2: Configure Runtime Constants

In your runtime's `lib.rs`, define the configuration constants:

```rust
use frame_support::parameter_types;

parameter_types! {
    /// Your PBC's unique chain ID
    /// Examples:
    ///   - EDSC PBC (Ethereum): 1
    ///   - BSC PBC: 56
    ///   - Polygon PBC: 137
    ///   - Avalanche PBC: 43114
    ///   - Primearc Core: 1000
    pub const YourPBCChainId: u32 = 100; // Replace with your actual chain ID

    /// Maximum number of attesters that can be registered
    /// Recommended: 50-200 depending on your security requirements
    pub const MaxAttesters: u32 = 100;

    /// Maximum number of attesters that can sign a single message
    /// Should be ≤ MaxAttesters, typically 10-30
    pub const MaxAttestersPerMessage: u32 = 20;

    /// Minimum signatures required (M in M-of-N)
    /// For Byzantine fault tolerance: M should be ≥ 2/3 of N
    /// Example: If N=10, M should be ≥ 7
    pub const MinSignatureThreshold: u32 = 2;

    /// Maximum age of attestation in blocks before expiry
    /// Example: 1000 blocks ≈ 2 hours (assuming 7s block time)
    pub const AttestationMaxAge: BlockNumber = 1000;
}
```

## Step 3: Implement Config Trait

```rust
impl pallet_bridge_attestation::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ChainId = YourPBCChainId;
    type MaxAttesters = MaxAttesters;
    type MaxAttestersPerMessage = MaxAttestersPerMessage;
    type MinSignatureThreshold = MinSignatureThreshold;
    type AttestationMaxAge = AttestationMaxAge;
    type WeightInfo = (); // Replace with actual weights after benchmarking
}
```

## Step 4: Add to Runtime Construction

```rust
construct_runtime!(
    pub enum Runtime {
        System: frame_system,
        // ... other pallets

        // Add bridge attestation pallet
        BridgeAttestation: pallet_bridge_attestation,

        // Your bridge-specific pallets that use attestation
        YourBridgeTokenMessenger: pallet_your_bridge_token_messenger,
    }
);
```

## Step 5: Use in Your Bridge Pallets

### Example: Token Messenger Integration

In your bridge token messenger pallet (e.g., `pallet-edsc-bridge-token-messenger`):

```rust
use pallet_bridge_attestation as BridgeAttestation;

#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Receive a cross-chain message with attestation
    #[pallet::call_index(0)]
    #[pallet::weight(10_000)]
    pub fn receive_message(
        origin: OriginFor<T>,
        message: Vec<u8>,
        attestation_hash: H256,
    ) -> DispatchResult {
        ensure_signed(origin)?;

        // Step 1: Verify attestation has sufficient valid signatures
        BridgeAttestation::<T>::verify_attestation_for_message(
            &message,
            attestation_hash,
        )?;

        // Step 2: Decode the message
        let decoded_message = Self::decode_bridge_message(&message)?;

        // Step 3: Process the message (mint tokens, transfer, etc.)
        Self::process_bridge_message(decoded_message)?;

        Ok(())
    }

    /// Check if message can be processed (read-only)
    pub fn can_process_message(message_hash: H256) -> bool {
        BridgeAttestation::<T>::is_attestation_valid(message_hash)
    }
}
```

### Example: Message Sender Integration

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Send a message to another chain
    #[pallet::call_index(1)]
    #[pallet::weight(10_000)]
    pub fn send_message(
        origin: OriginFor<T>,
        destination_chain_id: u32,
        recipient: Vec<u8>,
        amount: u128,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        // Create the bridge message
        let message = BridgeMessage {
            source_chain_id: BridgeAttestation::<T>::get_chain_id(),
            destination_chain_id,
            sender: sender.clone(),
            recipient,
            amount,
            nonce: BridgeAttestation::<T>::get_and_increment_nonce(),
        };

        // Encode and hash the message
        let encoded = message.encode();
        let message_hash = BridgeAttestation::<T>::hash_message(&encoded);

        // Emit event for attesters to watch
        Self::deposit_event(Event::MessageSent {
            message_hash,
            destination_chain_id,
            sender,
        });

        Ok(())
    }
}
```

## Step 6: Genesis Configuration

Add genesis configuration for initial attesters in your chain spec:

```rust
use pallet_bridge_attestation::AttesterInfo;

// In your chain_spec.rs
pub fn mainnet_genesis() -> serde_json::Value {
    serde_json::json!({
        "bridgeAttestation": {
            "attesters": [
                // Initial attesters with their public keys
                (0u32, AttesterInfo {
                    public_key: hex::decode("03a0b1c2d3e4f5...").unwrap(),
                    status: AttesterStatus::Active,
                    registered_at: 0,
                    messages_signed: 0,
                    last_signed_at: 0,
                }),
                // Add more initial attesters...
            ],
            "globalThreshold": ThresholdConfig {
                min_signatures: 2,
                total_attesters: 5,
                enabled: true,
            },
        }
    })
}
```

## Step 7: Governance Setup

### Register Attesters via Governance

```javascript
// Example Polkadot.js script to register attester
const publicKey = "0x03a0b1c2d3e4f5..."; // 33 bytes for ECDSA

await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.registerAttester(publicKey)
).signAndSend(sudoAccount);
```

### Configure Threshold

```javascript
// Set global threshold (2 of 5)
await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.configureThreshold(
    null,  // null = global
    2,     // min_signatures
    5      // total_attesters
  )
).signAndSend(sudoAccount);

// Set domain-specific threshold (3 of 7 for domain 1)
await api.tx.sudo.sudo(
  api.tx.bridgeAttestation.configureThreshold(
    1,   // domain_id
    3,   // min_signatures
    7    // total_attesters
  )
).signAndSend(sudoAccount);
```

## Step 8: Off-Chain Attester Setup

### Attester Service (Pseudocode)

```rust
// Example attester service that watches for messages and signs them
struct AttesterService {
    chain_rpc: String,
    private_key: ecdsa::Pair,
    attester_id: u32,
}

impl AttesterService {
    async fn watch_and_sign_messages(&self) {
        loop {
            // Listen for MessageSent events
            let events = self.subscribe_to_events().await;

            for event in events {
                if let Event::MessageSent { message_hash, destination_chain_id, .. } = event {
                    // Validate the message on source chain
                    let is_valid = self.validate_message(message_hash).await;

                    if is_valid {
                        // Sign the message hash
                        let signature = self.sign_message_hash(message_hash);

                        // Submit signature to destination chain
                        self.submit_signature(
                            self.attester_id,
                            message_hash,
                            signature,
                            self.get_chain_id(),
                            destination_chain_id,
                            event.nonce,
                        ).await;
                    }
                }
            }
        }
    }

    fn sign_message_hash(&self, message_hash: H256) -> Vec<u8> {
        // Sign the hash with private key
        let signature = self.private_key.sign(&message_hash.0);
        signature.0.to_vec()
    }
}
```

## Step 9: Testing

### Unit Tests

```rust
#[test]
fn integration_with_token_messenger_works() {
    new_test_ext().execute_with(|| {
        // Register attesters
        for i in 0..5 {
            let public_key = generate_test_key(i);
            assert_ok!(BridgeAttestation::register_attester(
                RuntimeOrigin::root(),
                public_key
            ));
        }

        // Set threshold
        assert_ok!(BridgeAttestation::configure_threshold(
            RuntimeOrigin::root(),
            None,
            3, // Require 3 signatures
            5  // Out of 5 attesters
        ));

        // Create a test message
        let message = create_bridge_message();
        let message_hash = BridgeAttestation::hash_message(&message);

        // Submit 3 signatures
        for i in 0..3 {
            let signature = sign_with_key(i, &message_hash);
            assert_ok!(BridgeAttestation::submit_signature(
                RuntimeOrigin::signed(1),
                i,
                message_hash,
                signature,
                1,  // source
                2,  // destination
                0,  // nonce
            ));
        }

        // Verify attestation
        assert_ok!(BridgeAttestation::verify_attestation(
            RuntimeOrigin::signed(1),
            message.clone(),
            message_hash,
        ));

        // Now use in token messenger
        assert_ok!(TokenMessenger::receive_message(
            RuntimeOrigin::signed(1),
            message,
            message_hash,
        ));
    });
}
```

## Step 10: Monitoring & Operations

### Query Attester Status

```javascript
// Get attester info
const attester = await api.query.bridgeAttestation.attesters(0);
console.log("Status:", attester.status);
console.log("Messages signed:", attester.messages_signed);

// Get active count
const activeCount = await api.query.bridgeAttestation.activeAttesterCount();
console.log("Active attesters:", activeCount.toNumber());

// Get threshold config
const threshold = await api.query.bridgeAttestation.globalThreshold();
console.log("Min signatures:", threshold.min_signatures);
console.log("Total attesters:", threshold.total_attesters);
```

### Monitor Attestations

```javascript
// Get attestation for a message
const attestation = await api.query.bridgeAttestation.attestations(messageHash);
console.log("Signature count:", attestation.signature_count);
console.log("Source chain:", attestation.source_chain_id);
console.log("Destination chain:", attestation.destination_chain_id);
console.log("Nonce:", attestation.nonce);

// Check if valid
const isValid = await api.query.bridgeAttestation.isAttestationValid(messageHash);
console.log("Is valid:", isValid);
```

## Example PBC Configurations

### EDSC Bridge PBC (Ethereum-compatible)

```rust
parameter_types! {
    pub const EDSCChainId: u32 = 1; // Ethereum mainnet
    pub const EDSCMaxAttesters: u32 = 50;
    pub const EDSCMaxAttestersPerMessage: u32 = 15;
    pub const EDSCMinSignatureThreshold: u32 = 10; // 2/3 of 15
    pub const EDSCAttestationMaxAge: BlockNumber = 1000;
}
```

### BSC Bridge PBC

```rust
parameter_types! {
    pub const BSCChainId: u32 = 56; // BSC mainnet
    pub const BSCMaxAttesters: u32 = 100;
    pub const BSCMaxAttestersPerMessage: u32 = 20;
    pub const BSCMinSignatureThreshold: u32 = 14; // 2/3 of 20
    pub const BSCAttestationMaxAge: BlockNumber = 800;
}
```

### Polkadot Bridge PBC (Substrate-native)

```rust
parameter_types! {
    pub const PolkadotChainId: u32 = 0; // Polkadot relay chain
    pub const PolkadotMaxAttesters: u32 = 200;
    pub const PolkadotMaxAttestersPerMessage: u32 = 30;
    pub const PolkadotMinSignatureThreshold: u32 = 20; // 2/3 of 30
    pub const PolkadotAttestationMaxAge: BlockNumber = 600;
}
```

## Security Best Practices

1. **Use 2/3 Threshold**: Set M ≥ 2/3 * N for Byzantine fault tolerance
2. **Distribute Attesters**: Choose geographically and organizationally diverse attesters
3. **Rotate Keys**: Periodically rotate attester keys
4. **Monitor Activity**: Track attester uptime and signing behavior
5. **Emergency Pause**: Have governance ready to pause if issues detected
6. **Test Thoroughly**: Run extensive integration tests before mainnet
7. **Gradual Rollout**: Start with testnet, then limited mainnet, then full production

## Troubleshooting

### "AttesterNotFound" Error
- Ensure attester is registered via `register_attester`
- Check attester ID is correct

### "InsufficientSignatures" Error
- Need M signatures but have fewer
- Check threshold configuration
- Verify attesters are active and signing

### "NonceAlreadyUsed" Error
- Replay attack detected
- Ensure each message has unique nonce
- Check nonce increment logic

### "InvalidSignature" Error
- Signature verification failed
- Check key/signature type match (ECDSA/SR25519)
- Verify signing logic in attester service

## Further Resources

- [Main README](./README.md) - Full pallet documentation
- [EDSC Bridge Reference](../../bridges/protocols/edsc-bridge/) - Example implementation
- [Substrate Docs](https://docs.substrate.io) - General Substrate development

## Support

For questions and issues:
- Open an issue in the main Etrid repository
- Join the Etrid developer Discord
- Check existing PBC implementations for examples
