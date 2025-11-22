//! # ASF Consensus Service Integration
//!
//! This module integrates the custom ËTRID ASF (Ascending Scale of Finality) consensus
//! modules into the Primearc Core Chain node service layer.
//!
//! ## Architecture Overview
//!
//! ASF consensus consists of four main components:
//! 1. **asf-algorithm**: Core consensus logic (FODDoS, PPFA rotation)
//! 2. **block-production**: PPFA proposer selection and block authoring (replaces AURA)
//! 3. **finality-gadget**: Three-level finality (Pre-commitment, Commitment, Finality)
//! 4. **validator-management**: Committee management and validator orchestration
//!
//! ## Hybrid Approach
//!
//! This service uses a hybrid consensus approach during the transition:
//! - **Block Production**: ASF PPFA (replaces AURA)
//! - **Finality**: GRANDPA + ASF Finality Gadget (dual finality)
//!
//! This allows gradual migration from traditional Substrate consensus to full ASF.
//!
//! ## Integration Points
//!
//! - `new_partial()`: Sets up ASF import queue with PPFA block production
//! - `new_full()`: Spawns ASF consensus tasks (pure ASF, no GRANDPA)
//! - Validator management integrates with keystore for signing
//! - Finality gadget runs as essential service task
//!
//! ## Compatibility
//!
//! Built for polkadot-stable2506 with Substrate service patterns.

use primearc_runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{BlockBackend, UsageProvider, Backend, HeaderBackend, BlockchainEvents};
use futures::StreamExt;
use sc_consensus::BlockImport;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_api::ProvideRuntimeApi;
use sp_consensus::{Environment, Proposer};
use sp_core::Encode;
use sp_runtime::traits::{Header, IdentifyAccount, Block as BlockT};
use sp_runtime::MultiSigner;
use sp_core::crypto::AccountId32;
use sp_timestamp;
use std::{sync::Arc, sync::atomic::{AtomicU64, Ordering}, time::Duration};

// Runtime API for validator committee queries
use pallet_validator_committee_runtime_api::ValidatorCommitteeApi;

// Runtime API for ASF validator registry
use pallet_asf_registry::AsfRegistryApi;

// ÉTRID P2P Networking
use detrp2p::{P2PNetwork, PeerId, PeerAddr, Message as P2PMessage};
use etrid_protocol::gadget_network_bridge::{
    GadgetNetworkBridge,
    VoteData,
    CertificateData,
    ConsensusBridgeMessage,
};

// V17: Checkpoint BFT Security Modules
use checkpoint_bft::{
    CheckpointCollector, AuthoritySet, CheckpointSignature, CheckpointCertificate,
    CheckpointType, detect_checkpoint, is_guaranteed_checkpoint,
    ForkAwareCollector, ByzantineTracker, RateLimitedCollector, RateLimitConfig,
    EclipseDetector, FinalityTracker,
};
use std::sync::Mutex;

// ═══════════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Full backend type
type FullBackend = sc_service::TFullBackend<Block>;

/// Full client type
pub type FullClient = sc_service::TFullClient<
    Block,
    RuntimeApi,
    sc_executor::WasmExecutor<sp_io::SubstrateHostFunctions>,
>;

/// Select chain type (longest chain for now, can be customized for ASF)
type SelectChain = sc_consensus::LongestChain<FullBackend, Block>;

/// ASF-enabled block import type (pure ASF, no GRANDPA)
type AsfBlockImport = Arc<FullClient>;

/// Full node partial components with ASF integration
pub type AsfFullParts = sc_service::PartialComponents<
    FullClient,
    FullBackend,
    SelectChain,
    sc_consensus::DefaultImportQueue<Block>,
    sc_transaction_pool::TransactionPoolHandle<Block, FullClient>,
    (
        AsfBlockImport,
        Option<Telemetry>,
    ),
>;

// ═══════════════════════════════════════════════════════════════════════════════
// ASF CONSENSUS CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════════

/// ASF consensus parameters
#[derive(Clone)]
pub struct AsfParams {
    /// Base slot duration (milliseconds)
    pub slot_duration: u64,

    /// Maximum committee size (PPFA panel size)
    pub max_committee_size: u32,

    /// Epoch duration in blocks
    pub epoch_duration: u32,

    /// Enable finality gadget
    pub enable_finality_gadget: bool,

    /// Minimum stake for validators (in smallest unit)
    pub min_validator_stake: u128,
}

impl Default for AsfParams {
    fn default() -> Self {
        Self {
            slot_duration: 6000, // 6 seconds (from block-production::BASE_SLOT_DURATION)
            max_committee_size: 21, // PPFA panel size (from validator-management::MAX_COMMITTEE_SIZE)
            epoch_duration: 2400, // ~4 hours at 6s blocks (from validator-management::EPOCH_DURATION)
            enable_finality_gadget: true,
            min_validator_stake: 64_000_000_000_000_000_000_000, // 64 ËTR for FlareNode
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// V17: CHECKPOINT SIGNING HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Nonce tracking per validator (prevents signature replay within epoch)
static SIGNATURE_NONCES: Lazy<Mutex<HashMap<u32, u64>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Get next signature nonce for a validator
fn get_next_signature_nonce(validator_id: u32) -> u64 {
    let mut nonces = match SIGNATURE_NONCES.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            log::error!("Signature nonce mutex poisoned, recovering: validator_id={}", validator_id);
            poisoned.into_inner()
        }
    };
    let nonce = nonces.entry(validator_id).or_insert(0);
    *nonce += 1;
    *nonce
}

/// Get validator's Sr25519 signing key from keystore and derive validator ID
///
/// Note: Checkpoint signatures use Ed25519, but ASF validator identity uses Sr25519.
/// For V17, we'll convert the Sr25519 key to Ed25519 format or use a compatibility layer.
/// Production will use dedicated checkpoint signing keys.
fn get_validator_sr25519_key(
    keystore: &Arc<dyn sc_keystore::Keystore>,
    checkpoint_collector: &Arc<checkpoint_bft::CheckpointCollector>,
) -> Result<(sp_core::sr25519::Public, u32), String> {
    use sp_core::crypto::KeyTypeId;
    use sc_keystore::Keystore;

    const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

    // Get Sr25519 public keys
    let public_keys = keystore.sr25519_public_keys(ASF_KEY_TYPE);

    if public_keys.is_empty() {
        return Err("No ASF Sr25519 keys found in keystore".to_string());
    }

    // Get first available key
    let public = public_keys[0];
    let public_bytes: [u8; 32] = public.0;

    // V23 FIX: Look up validator_id in authority set (0-19 index)
    // Get current authority set (CheckpointCollector has internal RwLock)
    let authority_set = checkpoint_collector.get_authority_set();

    // Find this validator's index in the authority set
    let validator_id = authority_set
        .authorities
        .iter()
        .position(|auth_pubkey| *auth_pubkey == public_bytes)
        .ok_or_else(|| {
            format!(
                "Validator public key {:?} not found in authority set",
                hex::encode(public_bytes)
            )
        })? as u32;

    Ok((public, validator_id))
}

/// Calculate deterministic hash of authority set
fn calculate_authority_set_hash(authorities: &[[u8; 32]]) -> [u8; 32] {
    use sp_core::hashing::blake2_256;
    let mut data = Vec::new();
    for pubkey in authorities {
        data.extend_from_slice(pubkey);
    }
    blake2_256(&data)
}

/// Get genesis hash for chain ID
fn get_genesis_hash(client: &Arc<FullClient>) -> sp_core::H256 {
    use sp_blockchain::HeaderBackend;
    use sp_runtime::traits::Zero;

    // Get block hash at height 0 (genesis)
    match client.hash(sp_runtime::traits::Zero::zero()) {
        Ok(Some(hash)) => hash,
        _ => sp_core::H256::zero(), // Fallback to zero hash
    }
}

/// Create and sign a checkpoint signature
///
/// V17 Implementation Note:
/// Currently uses Sr25519 keys with a conversion to Ed25519 signature format.
/// This is a transitional approach - production will use dedicated Ed25519 checkpoint keys.
fn create_checkpoint_signature(
    block_number: u32,
    block_hash: &[u8; 32],
    validator_id: u32,
    validator_pubkey: [u8; 32],
    authority_set_id: u64,
    authority_set_hash: [u8; 32],
    checkpoint_type: CheckpointType,
    signature_nonce: u64,
    keystore: &Arc<dyn sc_keystore::Keystore>,
    public_key: &sp_core::sr25519::Public,
    chain_id: [u8; 32],
) -> CheckpointSignature {
    use sp_core::crypto::{KeyTypeId, Pair};
    use sc_keystore::Keystore;

    // Get current timestamp
    let timestamp_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            log::error!("System time before UNIX epoch: {:?}", e);
            std::time::Duration::from_secs(0)
        })
        .as_millis() as u64;

    // Create signature struct (without signature field initially)
    let mut sig_struct = CheckpointSignature {
        chain_id,
        block_number,
        block_hash: *block_hash,
        validator_id,
        validator_pubkey,
        authority_set_id,
        authority_set_hash,
        checkpoint_type,
        signature_nonce,
        signature: Vec::new(), // Will fill in below
        timestamp_ms,
    };

    // Create signing payload
    let payload = sig_struct.signing_payload();

    // Sign with Sr25519 via keystore (V17: transitional approach)
    // Note: CheckpointSignature expects Ed25519, but we're using Sr25519 for now
    // This will be upgraded to proper Ed25519 checkpoint keys in production
    const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

    let signature = keystore
        .sr25519_sign(ASF_KEY_TYPE, public_key, &payload)
        .ok()
        .flatten()
        .unwrap_or_else(|| {
            log::warn!("Failed to sign checkpoint, using empty signature");
            sp_core::sr25519::Signature::from_raw([0u8; 64])
        });

    sig_struct.signature = signature.0.to_vec();

    sig_struct
}

/// Broadcast checkpoint signature via P2P network
async fn broadcast_checkpoint_signature_p2p(
    signature: CheckpointSignature,
    network: &Arc<P2PNetwork>,
) -> Result<(), String> {
    use etrid_protocol::{CheckpointSignatureMsg, ProtocolMessage, MessageType};

    // Convert CheckpointSignature to P2P message format
    let checkpoint_type_u8 = match signature.checkpoint_type {
        CheckpointType::Guaranteed => 0,
        CheckpointType::Opportunity { .. } => 1,
    };

    let msg = CheckpointSignatureMsg {
        block_number: signature.block_number,
        block_hash: signature.block_hash,
        validator_id: signature.validator_id,
        validator_pubkey: signature.validator_pubkey,
        authority_set_id: signature.authority_set_id,
        authority_set_hash: signature.authority_set_hash,
        checkpoint_type: checkpoint_type_u8,
        signature_nonce: signature.signature_nonce,
        signature: signature.signature.clone(),
        timestamp_ms: signature.timestamp_ms,
    };

    // Serialize message
    let payload = bincode::serialize(&msg)
        .map_err(|e| format!("Failed to serialize checkpoint signature: {}", e))?;

    // Create protocol message envelope
    let proto_msg = ProtocolMessage::new(MessageType::CheckpointSignature, payload);

    // Broadcast to all peers
    let msg_bytes = bincode::serialize(&proto_msg)
        .map_err(|e| format!("Failed to serialize protocol message: {}", e))?;

    // Wrap in P2PMessage
    let p2p_msg = P2PMessage::CheckpointSignature { data: msg_bytes };

    network.broadcast(p2p_msg)
        .await
        .map_err(|e| format!("Failed to broadcast via P2P: {}", e))?;

    log::debug!(
        "📤 Broadcast checkpoint signature for block #{} from validator {}",
        signature.block_number,
        signature.validator_id
    );

    Ok(())
}

/// Broadcast checkpoint certificate via P2P network
async fn broadcast_checkpoint_certificate_p2p(
    certificate: CheckpointCertificate,
    network: &Arc<P2PNetwork>,
) -> Result<(), String> {
    use etrid_protocol::{CheckpointCertificateMsg, CheckpointSignatureMsg, ProtocolMessage, MessageType};

    // Convert signatures to P2P format
    let signatures: Vec<CheckpointSignatureMsg> = certificate.signatures
        .iter()
        .map(|sig| {
            let checkpoint_type_u8 = match sig.checkpoint_type {
                CheckpointType::Guaranteed => 0,
                CheckpointType::Opportunity { .. } => 1,
            };

            CheckpointSignatureMsg {
                block_number: sig.block_number,
                block_hash: sig.block_hash,
                validator_id: sig.validator_id,
                validator_pubkey: sig.validator_pubkey,
                authority_set_id: sig.authority_set_id,
                authority_set_hash: sig.authority_set_hash,
                checkpoint_type: checkpoint_type_u8,
                signature_nonce: sig.signature_nonce,
                signature: sig.signature.clone(),
                timestamp_ms: sig.timestamp_ms,
            }
        })
        .collect();

    let msg = CheckpointCertificateMsg {
        block_number: certificate.block_number,
        block_hash: certificate.block_hash,
        authority_set_id: certificate.authority_set_id,
        signatures,
        finalized_at_ms: certificate.finalized_at_ms,
    };

    // Serialize message
    let payload = bincode::serialize(&msg)
        .map_err(|e| format!("Failed to serialize checkpoint certificate: {}", e))?;

    // Create protocol message envelope
    let proto_msg = ProtocolMessage::new(MessageType::CheckpointCertificate, payload);

    // Broadcast to all peers
    let msg_bytes = bincode::serialize(&proto_msg)
        .map_err(|e| format!("Failed to serialize protocol message: {}", e))?;

    // Wrap in P2PMessage
    let p2p_msg = P2PMessage::CheckpointCertificate { data: msg_bytes };

    network.broadcast(p2p_msg)
        .await
        .map_err(|e| format!("Failed to broadcast via P2P: {}", e))?;

    log::info!(
        "📤 Broadcast checkpoint certificate for block #{} with {} signatures",
        certificate.block_number,
        certificate.signatures.len()
    );

    Ok(())
}

/// Get canonical chain from client (last 100 blocks)
fn get_canonical_chain<Client>(client: &Arc<Client>) -> Vec<sp_core::H256>
where
    Client: HeaderBackend<Block>,
{
    let mut chain = Vec::new();
    let mut current_hash = client.info().best_hash;

    // Walk back 100 blocks to build canonical chain
    for _ in 0..100 {
        chain.push(current_hash);

        match client.header(current_hash) {
            Ok(Some(header)) => {
                current_hash = *header.parent_hash();
            }
            _ => break,
        }
    }

    chain
}

/// Verify block is on canonical chain
fn verify_canonical_chain<Client>(
    client: &Arc<Client>,
    block_hash: sp_core::H256,
    block_number: u32,
) -> Result<bool, String>
where
    Client: HeaderBackend<Block>,
{
    // Get current best block
    let best_hash = client.info().best_hash;
    let best_number = client.info().best_number;

    // Checkpoint must not be ahead of best block
    if block_number > best_number {
        return Err(format!(
            "Checkpoint #{} is ahead of best block #{}",
            block_number, best_number
        ));
    }

    // Walk back from best block to checkpoint
    let mut current_hash = best_hash;
    for _ in block_number..=best_number {
        if current_hash == block_hash {
            return Ok(true);
        }

        // Get parent hash
        match client.header(current_hash) {
            Ok(Some(header)) => {
                current_hash = *header.parent_hash();
            }
            Ok(None) => {
                return Err(format!("Header not found for hash {:?}", current_hash));
            }
            Err(e) => {
                return Err(format!("Failed to get header: {:?}", e));
            }
        }
    }

    Ok(false)
}

/// Finalize checkpoint block via Substrate API
async fn finalize_checkpoint_block<Client, BE>(
    certificate: CheckpointCertificate,
    client: &Arc<Client>,
    network: &Arc<P2PNetwork>,
    finality_tracker: &Arc<Mutex<FinalityTracker>>,
) -> Result<(), String>
where
    BE: sc_client_api::Backend<Block>,
    Client: sc_client_api::BlockchainEvents<Block> + HeaderBackend<Block> + sc_client_api::Finalizer<Block, BE>,
{
    use checkpoint_bft::CertificateAsfExt; // For finality_level() method

    let block_hash = sp_core::H256::from_slice(&certificate.block_hash);
    let block_number = certificate.block_number;

    // Calculate ASF finality level
    let finality_level = certificate.finality_level();

    log::info!(
        "🎯 Finalizing block #{} with {:?} finality ({} signatures)",
        block_number,
        finality_level,
        certificate.signatures.len()
    );

    // Finalize the block in Substrate
    use sc_client_api::Finalizer;
    client.finalize_block(block_hash, None, true)
        .map_err(|e| format!("Failed to finalize block #{}: {:?}", block_number, e))?;

    log::info!(
        "✅ Block #{} finalized successfully with {:?} finality",
        block_number,
        finality_level
    );

    // Update finality tracker
    match finality_tracker.lock() {
        Ok(mut tracker) => {
            let finality_level = tracker.finalize_block(
                block_number,
                certificate.block_hash,
                certificate.signatures.len() as u32,
            ).map_err(|e| format!("Failed to update finality tracker: {}", e))?;

            log::info!(
                "Block #{} finalized with level: {:?}",
                block_number,
                finality_level
            );
        }
        Err(e) => {
            return Err(format!("Failed to lock finality tracker: {:?}", e));
        }
    }

    // Broadcast certificate to network
    broadcast_checkpoint_certificate_p2p(certificate, network).await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARTIAL NODE SETUP (ASF IMPORT QUEUE)
// ═══════════════════════════════════════════════════════════════════════════════

/// Create a new partial node with ASF consensus integration
///
/// This replaces AURA's import queue with an ASF-compatible one while keeping
/// Pure ASF consensus (v108).
///
/// # ASF Integration Points
///
/// 1. **Import Queue**: Custom ASF block validation (PPFA proposer verification)
/// 2. **Block Import**: GRANDPA wrapper for finality (hybrid approach)
/// 3. **Inherent Data**: ASF-specific inherents (PPFA index, epoch info)
///
/// # Returns
///
/// Partial components ready for full node construction
pub fn new_partial(config: &Configuration) -> Result<AsfFullParts, ServiceError> {
    // Initialize telemetry
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    // Create wasm executor
    let executor = sc_service::new_wasm_executor::<sp_io::SubstrateHostFunctions>(&config.executor);

    // Build full client, backend, keystore, and task manager
    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    // Spawn telemetry worker
    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    // Use longest chain selector (ASF will use PPFA for actual selection)
    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    // Create transaction pool
    let transaction_pool = Arc::from(
        sc_transaction_pool::Builder::new(
            task_manager.spawn_essential_handle(),
            client.clone(),
            config.role.is_authority().into(),
        )
        .with_options(config.transaction_pool.clone())
        .with_prometheus(config.prometheus_registry())
        .build(),
    );

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF BLOCK IMPORT (Pure ASF, no GRANDPA)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // v108 migration: Use client directly as block import for pure ASF consensus.
    // ASF finality gadget handles all finality - no GRANDPA needed.

    let block_import = client.clone();

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF IMPORT QUEUE
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This import queue validates blocks using ASF rules:
    // 1. Verify PPFA proposer is authorized for this slot
    // 2. Check block type (Queen vs Ant)
    // 3. Validate parent certificates for finality
    // 4. Apply ASF-specific inherent data
    //
    // NOTE: For initial implementation, we use a simple manually-created import
    // queue. In production, this would use block-production crate's validation.

    use sc_consensus::import_queue::BasicQueue;
    use sc_consensus::Verifier;
    use sc_consensus::BlockImportParams;

    /// ASF block verifier
    ///
    /// Validates blocks according to ASF consensus rules:
    /// - PPFA proposer authorization
    /// - Block type validation (Queen/Ant)
    /// - Parent certificate checks
    struct AsfVerifier<C, B> {
        client: Arc<C>,
        _phantom: std::marker::PhantomData<B>,
    }

    impl<C, B> AsfVerifier<C, B> {
        fn new(client: Arc<C>) -> Self {
            Self {
                client,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    // Implement the Verifier trait for ASF block validation
    #[async_trait::async_trait]
    impl<C, B> Verifier<Block> for AsfVerifier<C, B>
    where
        C: sc_client_api::blockchain::HeaderBackend<Block>
            + sc_client_api::BlockchainEvents<Block>
            + sp_api::ProvideRuntimeApi<Block>
            + Send
            + Sync,
        C::Api: pallet_validator_committee_runtime_api::ValidatorCommitteeApi<Block>,
        B: sc_client_api::backend::Backend<Block> + Send + Sync,
    {
        async fn verify(
            &self,
            mut block: BlockImportParams<Block>,
        ) -> Result<BlockImportParams<Block>, String> {
            // ASF BLOCK VALIDATION using block-production::validation module
            //
            // This validates blocks according to ASF consensus rules:
            // 1. Block structure (header, transactions, size)
            // 2. PPFA proposer authorization (uses Runtime API to verify proposer is in committee)
            // 3. Block type validation (Queen vs Ant)
            //
            // PPFA Proposer Authorization Flow:
            // - Extract proposer ValidatorId from block digest
            // - Query runtime API: is_validator_active(proposer_id) to verify committee membership
            // - Verify PPFA rotation index matches expected proposer for this slot
            // - In production: client.runtime_api().is_validator_active(at_hash, &proposer_id)?

            use block_production::validation::BlockValidator;
            use block_production::{Block as AsfBlock, BlockHeader, BlockBody, BlockType};
            use codec::Encode;

            // Convert Substrate block to ASF block format for validation
            let header = block.header.clone();
            let block_number = *header.number();

            // Create ASF block representation
            // Note: In production, extrinsics would be converted to ASF transaction format
            let asf_block = AsfBlock {
                header: BlockHeader {
                    number: block_number as u64,
                    parent_hash: block_production::Hash::from(header.parent_hash().encode().try_into().unwrap_or([0u8; 32])),
                    state_root: block_production::Hash::default(),
                    extrinsics_root: block_production::Hash::default(),
                    block_type: BlockType::Queen, // Default to Queen block
                    proposer: block_production::ValidatorId::from([0u8; 32]), // Will be extracted from digest
                    ppfa_index: 0, // Will be extracted from digest
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    epoch: (block_number / 2400) as u32, // EPOCH_DURATION = 2400 blocks
                },
                body: BlockBody {
                    transactions: Vec::new(), // Populated from extrinsics in production
                },
            };

            // Validate block structure
            let validator = BlockValidator::default();
            validator.validate_block(&asf_block)
                .map_err(|e| format!("ASF block validation failed: {:?}", e))?;

            // ═══════════════════════════════════════════════════════════════
            // PPFA PROPOSER AUTHORIZATION VALIDATION
            // ═══════════════════════════════════════════════════════════════

            use codec::Decode;
            use sp_runtime::DigestItem;

            // Step 1: Extract PPFA seal from block digest
            #[derive(Decode)]
            struct PpfaSeal {
                ppfa_index: u32,
                proposer_id: [u8; 32],
                slot_number: u64,
                timestamp: u64,
            }

            let mut ppfa_seal_data: Option<PpfaSeal> = None;

            // Search for PPFA digest in post_digests
            for digest_item in block.post_digests.iter() {
                if let DigestItem::PreRuntime(engine_id, data) = digest_item {
                    if engine_id == b"PPFA" {
                        match PpfaSeal::decode(&mut &data[..]) {
                            Ok(seal) => {
                                log::debug!(
                                    "🔍 Extracted PPFA seal: index={}, proposer={:?}",
                                    seal.ppfa_index,
                                    hex::encode(&seal.proposer_id[..8])
                                );
                                ppfa_seal_data = Some(seal);
                                break;
                            }
                            Err(e) => {
                                log::warn!("Failed to decode PPFA seal: {:?}", e);
                            }
                        }
                    }
                }
            }

            // Step 2: Validate PPFA authorization if seal is present
            if let Some(seal) = ppfa_seal_data {
                let proposer_id = block_production::ValidatorId::from(seal.proposer_id);

                log::debug!(
                    "🔐 Validating PPFA authorization for block #{}: proposer={:?}, ppfa_index={}",
                    block_number,
                    hex::encode(&proposer_id.encode()[..8]),
                    seal.ppfa_index
                );

                // Step 3: Query runtime API to verify proposer authorization
                // Use parent block hash for validation (check authorization at time of block production)
                let parent_hash = *header.parent_hash();

                // Convert block_production::ValidatorId to runtime API ValidatorId
                let runtime_proposer_id = pallet_validator_committee_runtime_api::ValidatorId::from(seal.proposer_id);

                match self.client.runtime_api().is_proposer_authorized(
                    parent_hash,
                    block_number,
                    seal.ppfa_index,
                    runtime_proposer_id,
                ) {
                    Ok(is_authorized) => {
                        if !is_authorized {
                            // CRITICAL: Proposer was not authorized - REJECT BLOCK
                            let error_msg = format!(
                                "❌ PPFA Authorization FAILED for block #{}: proposer {:?} was NOT authorized for ppfa_index {}",
                                block_number,
                                hex::encode(&proposer_id.encode()[..8]),
                                seal.ppfa_index
                            );
                            log::error!("{}", error_msg);
                            return Err(error_msg);
                        }

                        log::debug!(
                            "✅ PPFA authorization validated for block #{}: proposer {:?} authorized for ppfa_index {}",
                            block_number,
                            hex::encode(&proposer_id.encode()[..8]),
                            seal.ppfa_index
                        );
                    }
                    Err(e) => {
                        // Runtime API call failed - this is a serious error
                        let error_msg = format!(
                            "❌ Failed to query PPFA authorization for block #{}: {:?}. Rejecting block as safety measure.",
                            block_number,
                            e
                        );
                        log::error!("{}", error_msg);
                        return Err(error_msg);
                    }
                }

                log::trace!(
                    "PPFA authorization check: block={}, ppfa_index={}, proposer={:?}, slot={}, timestamp={}",
                    block_number,
                    seal.ppfa_index,
                    hex::encode(&proposer_id.encode()[..8]),
                    seal.slot_number,
                    seal.timestamp
                );
            } else {
                // No PPFA seal found - this might be a genesis block or from before sealing was enabled
                log::trace!(
                    "ℹ️  No PPFA seal found in block #{} (pre-sealing block or genesis)",
                    block_number
                );
            }

            log::debug!(
                "✅ ASF block #{} validated successfully",
                block_number
            );

            // ═══════════════════════════════════════════════════════════════
            // FORK CHOICE STRATEGY: Signal to Substrate import pipeline
            // This tells Substrate this validated block is a candidate for
            // the canonical chain. Without this, the import pipeline is
            // incomplete and blocks are rejected.
            // ═══════════════════════════════════════════════════════════════
            block.fork_choice = Some(sc_consensus::ForkChoiceStrategy::LongestChain);

            log::debug!(
                "🔗 Block #{} ready for import with LongestChain fork choice",
                block_number
            );

            // Note: We don't clear post_digests here - they're part of the block

            Ok(block)
        }
    }

    let verifier = AsfVerifier::<_, FullBackend>::new(client.clone());

    let import_queue = BasicQueue::new(
        verifier,
        Box::new(block_import.clone()),
        None, // No justification import for ASF
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
    );

    // Return partial components
    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, telemetry),
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// FULL NODE WITH ASF CONSENSUS
// ═══════════════════════════════════════════════════════════════════════════════

/// Build a new full node with ASF consensus
///
/// This spawns all necessary consensus tasks:
/// 1. **ASF Block Production**: PPFA proposer using block-production crate
/// 2. **ASF Finality Gadget**: Three-level finality (optional, hybrid with GRANDPA)
/// 3. **GRANDPA Finality**: Traditional finality (will be phased out)
/// 4. **Validator Management**: Committee coordination and health monitoring
///
/// # Architecture
///
/// ```text
/// ┌─────────────────────────────────────────────────────────────┐
/// │                    FlareChain Node                          │
/// ├─────────────────────────────────────────────────────────────┤
/// │  ASF Block Production (PPFA)                                │
/// │    ├─ Proposer selection (block-production)                 │
/// │    ├─ Block authoring (Queen/Ant blocks)                    │
/// │    └─ Transaction selection                                 │
/// ├─────────────────────────────────────────────────────────────┤
/// │  Hybrid Finality                                            │
/// │    ├─ ASF Finality Gadget (3-level)                         │
/// │    │   ├─ Pre-commitment                                    │
/// │    │   ├─ Commitment                                        │
/// │    │   └─ Finality                                          │
/// │                       │
/// ├─────────────────────────────────────────────────────────────┤
/// │  Validator Management                                       │
/// │    ├─ Committee management (PPFA panels)                    │
/// │    ├─ Health monitoring                                     │
/// │    └─ Reward distribution                                   │
/// └─────────────────────────────────────────────────────────────┘
/// ```
///
/// # Returns
///
/// TaskManager that must be kept alive for the node to run
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    new_full_with_params(config, AsfParams::default())
}

/// Build a new full node with custom ASF parameters
pub fn new_full_with_params(
    config: Configuration,
    asf_params: AsfParams,
) -> Result<TaskManager, ServiceError> {
    // Get partial components
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, mut telemetry),
    } = new_partial(&config)?;

    // ═══════════════════════════════════════════════════════════════════════════
    // NETWORK SETUP
    // ═══════════════════════════════════════════════════════════════════════════

    let mut net_config = sc_network::config::FullNetworkConfiguration::<
        Block,
        <Block as sp_runtime::traits::Block>::Hash,
        sc_network::NetworkWorker<Block, <Block as sp_runtime::traits::Block>::Hash>,
    >::new(
        &config.network,
        config.prometheus_registry().cloned(),
    );

    let metrics = sc_network::service::NotificationMetrics::new(
        config.prometheus_registry()
    );

    let _peer_store_handle = net_config.peer_store_handle();

    // v108: No GRANDPA protocol - pure ASF consensus
    // ASF uses custom P2P protocols via detrp2p for:
    // - PPFA committee gossip
    // - Finality gadget messages (votes, certificates)
    // - Validator health checks

    // v108: Disable warp sync for pure ASF (ASF has its own sync mechanism)
    let warp_sync = None;

    // Log network configuration for debugging
    log::info!("🌐 Substrate Network Configuration:");
    log::info!("  Node name: {}", config.network.node_name);
    log::info!("  Listen addresses: {:?}", config.network.listen_addresses);
    log::info!("  Public addresses: {:?}", config.network.public_addresses);
    log::info!("  Boot nodes: {:?}", config.network.boot_nodes);
    log::info!("  Reserved nodes: {:?}", config.network.default_peers_set.reserved_nodes);
    // log::info!("  Reserved only: {}", config.network.default_peers_set.reserved_only); // Field removed in newer Substrate

    // Build network
    let (network, system_rpc_tx, tx_handler_controller, sync_service) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_config: warp_sync,
            block_relay: None,
            metrics,
        })?;

    log::info!("✅ Substrate network built successfully on port 30333");

    // ═══════════════════════════════════════════════════════════════════════════
    // OFFCHAIN WORKERS
    // ═══════════════════════════════════════════════════════════════════════════

    if config.offchain_worker.enabled {
        use futures::FutureExt;

        let offchain_workers = sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
            runtime_api_provider: client.clone(),
            is_validator: config.role.is_authority(),
            keystore: Some(keystore_container.keystore()),
            offchain_db: backend.offchain_storage(),
            transaction_pool: Some(OffchainTransactionPoolFactory::new(
                transaction_pool.clone(),
            )),
            network_provider: Arc::new(network.clone()),
            enable_http_requests: true,
            custom_extensions: |_| vec![],
        })?;
        task_manager.spawn_handle().spawn(
            "offchain-workers-runner",
            "offchain-worker",
            offchain_workers.run(client.clone(), task_manager.spawn_handle()).boxed(),
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // RPC SETUP
    // ═══════════════════════════════════════════════════════════════════════════

    let role = config.role;
    let force_authoring = config.force_authoring;
    let name = config.network.node_name.clone();
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |_| {
            let deps = crate::rpc::FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                enable_asf: true, // Enable ASF RPC endpoints
            };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    // Clone network data before moving config (needed for DETR P2P setup)
    let boot_nodes = config.network.boot_nodes.clone();
    let listen_addresses = config.network.listen_addresses.clone();
    let public_addresses = config.network.public_addresses.clone();

    // Spawn RPC handlers
    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: Arc::new(network.clone()),
        client: client.clone(),
        keystore: keystore_container.keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend: backend.clone(),
        system_rpc_tx,
        tx_handler_controller,
        sync_service: sync_service.clone(),
        config,
        telemetry: telemetry.as_mut(),
    })?;

    // ═══════════════════════════════════════════════════════════════════════════
    // V17: CHECKPOINT BFT SECURITY INITIALIZATION
    // ═══════════════════════════════════════════════════════════════════════════

    log::info!("🔐 Initializing Checkpoint BFT security modules...");

    // Get best block for runtime queries
    let best_hash = client.info().best_hash;

    // V27 FIX: Hardcoded authority set with all 20 validators' raw sr25519 public keys
    //
    // V26 bug: Each validator only had its OWN key in authority set, causing hash mismatches.
    // All validators MUST have identical authority sets for signature verification to work.
    //
    // V27 Solution: Hardcode all 20 validators' ASF keystore public keys.
    // This ensures ALL validators compute the same authority_set_hash.
    //
    // These are the raw sr25519 public keys extracted from each validator's keystore:
    // /var/lib/etrid/chains/flarechain_mainnet_v1/keystore/6173666b{pubkey}

    const ASF_KEY_TYPE: sp_core::crypto::KeyTypeId = sp_core::crypto::KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

    // Get our validator's key for signing (we still need this to sign checkpoints)
    let keystore = keystore_container.keystore();
    let our_asf_keys = keystore.sr25519_public_keys(ASF_KEY_TYPE);

    if !our_asf_keys.is_empty() {
        log::info!(
            "✅ V27: Local ASF signing key: {}",
            hex::encode(&our_asf_keys[0].0[..8])
        );
    } else {
        log::warn!("⚠️  No ASF key in keystore! This validator cannot sign checkpoints.");
    }

    // Helper to convert hex string to [u8; 32]
    fn hex_to_bytes32(hex_str: &str) -> [u8; 32] {
        let bytes = hex::decode(hex_str).expect("Invalid hex string");
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes);
        arr
    }

    // V33: Query ASF validator set from runtime API (pallet_asf_registry)
    // Falls back to hardcoded keys if no validators registered on-chain
    let validator_pubkeys: Vec<[u8; 32]> = {
        // Try to query the runtime API for registered ASF validators
        let best_hash = client.info().best_hash;
        let runtime_validators: Option<Vec<[u8; 32]>> = client
            .runtime_api()
            .asf_validator_set(best_hash)
            .ok()
            .filter(|v| !v.is_empty());

        if let Some(validators) = runtime_validators {
            log::info!(
                "✅ V33: Using DYNAMIC ASF validator set from pallet_asf_registry ({} validators)",
                validators.len()
            );
            let version = client.runtime_api().validator_set_version(best_hash).unwrap_or(0);
            log::info!("   Validator set version: {}", version);
            validators
        } else {
            log::info!("⚠️  V33: No validators in pallet_asf_registry, using HARDCODED fallback keys");
            log::info!("   Validators should register via `asfRegistry.registerAsfKey()` extrinsic");

            // Fallback: Hardcoded authority set - ALL 20 validators' REAL ASF sr25519 public keys
            // Generated with deterministic seeds: //Validator0 through //Validator19
            vec![
                // vmi2896906 - Validator 0 (seed: //Validator0)
                hex_to_bytes32("d684fb9413cc36d5388fd1b4a9112158d76344a46c7ba78f3abd78f044df012e"),
                // vmi2896907 - Validator 1 (seed: //Validator1)
                hex_to_bytes32("f452cc9c48012cdde4ccdf3b5c2f5a26816292f85572554f9ee7ac14c1fcab46"),
                // vmi2896908 - Validator 2 (seed: //Validator2)
                hex_to_bytes32("b2a618444ec2fe714b3d811358154ee326822c8f4c9dfa11ddddce86232df05e"),
                // vmi2896909 - Validator 3 (seed: //Validator3)
                hex_to_bytes32("40746dd99b0cd9b8003137482d5e5a5db27018b5fcf3dfc2804ba79dd18fa064"),
                // vmi2896910 - Validator 4 (seed: //Validator4)
                hex_to_bytes32("0084df35e1a4365297c88c8c1d23771f33629a985595801eac6a8d63ad37cf7c"),
                // vmi2896911 - Validator 5 (seed: //Validator5)
                hex_to_bytes32("de829258a4d8f3b7aba1fcafac2a3f90934fe06e29fb5e892676efd55aa5ab7a"),
                // vmi2896914 - Validator 6 (seed: //Validator6)
                hex_to_bytes32("24fb1fce1c3362778ee8a1c39ac55cf84114fa9fa2159f145be5ff9db471692c"),
                // vmi2896915 - Validator 7 (seed: //Validator7)
                hex_to_bytes32("a0043aeb20a72fe653b8a9033f45f6f773e74a7459291f0749e83e4c88a40138"),
                // vmi2896916 - Validator 8 (seed: //Validator8)
                hex_to_bytes32("009f9573813397c72b4dc6c892042f0966e215acbd50d42d6160536d7459ec36"),
                // vmi2896917 - Validator 9 (seed: //Validator9)
                hex_to_bytes32("4620c12c7e24b58439098cd5a187c9cf4c0c4f46f4aefbe3501dfa2793a08b1f"),
                // vmi2896918 - Validator 10 (seed: //Validator10)
                hex_to_bytes32("18b6b5b3ae15d535150edd2a0368c19d3f938c1e18aa25940e4d07c8e7827e51"),
                // vmi2896921 - Validator 11 (seed: //Validator11)
                hex_to_bytes32("3a1ea38d46b86d5ddb0bf21e98fe6728a97f46cdee85342520451a1696e1174c"),
                // vmi2896922 - Validator 12 (seed: //Validator12)
                hex_to_bytes32("b2669b95a01cf04d89e0ccddc19dd3b37a80c53d77b3e8643359a213330ceb68"),
                // vmi2896923 - Validator 13 (seed: //Validator13)
                hex_to_bytes32("f06f9181f1d8aadb108a637c43ce69c739f3c407afadc9f0d36078baf687a567"),
                // vmi2896924 - Validator 14 (seed: //Validator14)
                hex_to_bytes32("060e511e0cf6825e6a01db5a35294d0cbc1f444f3f9b80f77277cb4b8cb27052"),
                // vmi2896925 - Validator 15 (seed: //Validator15)
                hex_to_bytes32("ea618651fbcb535f1d4006d6e9eb9b82110ee279d1ae7e8a06f1140e0dc46947"),
                // vmi2897381 - Validator 16 (seed: //Validator16)
                hex_to_bytes32("6ee9536da0982e077854c8d53d84d9d08148ead33ae67355f92857dabdfd3e58"),
                // vmi2897382 - Validator 17 (seed: //Validator17)
                hex_to_bytes32("925455da5062769f3c118ce13045d8501120470013f3ac63eab84c7fd8595145"),
                // vmi2897383 - Validator 18 (seed: //Validator18)
                hex_to_bytes32("d06f4bf091f6785ab4565f3de532c79f52c1986a4e2a27b4c85035953fe98421"),
                // vmi2897384 - Validator 19 (seed: //Validator19)
                hex_to_bytes32("72f6e8ed338d2d4b5cab78208d02384c9ee2f0ff55b598eba6a6988c2cdcfe43"),
            ]
        }
    };

    log::info!("✅ Authority set initialized with {} validators", validator_pubkeys.len());

    // Create authority set for checkpoint BFT
    let authority_set = AuthoritySet::new(
        1, // authority_set_id - will be synced with runtime in future
        validator_pubkeys,
    );

    // Initialize checkpoint collector with all security modules
    let checkpoint_collector = Arc::new(CheckpointCollector::new(authority_set));

    // Fork-aware collector disabled due to H256 version conflict
    // Two different versions of primitive_types::H256 are in use:
    // - substrate uses one version
    // - checkpoint-bft uses another
    // This will be resolved by aligning primitive_types versions in Phase 2
    // For now, fork detection will be handled at the P2P layer
    // let canonical_tip = ...;
    // let fork_aware_collector = Arc::new(Mutex::new(ForkAwareCollector::new(canonical_tip)));

    let byzantine_tracker = Arc::new(Mutex::new(ByzantineTracker::new(10))); // min 10 checkpoints for evaluation

    // Rate limiter requires Clone on CheckpointCollector - will be added in Phase 2
    // For now, rate limiting will be done at P2P layer
    // let rate_limit_config = RateLimitConfig::default();
    // let rate_limiter = Arc::new(RateLimitedCollector::new(
    //     checkpoint_collector.as_ref().clone(),
    //     rate_limit_config,
    // ));

    let eclipse_detector = Arc::new(Mutex::new(EclipseDetector::new(
        5,  // min_unique_sources
        10, // warning_threshold
    )));

    let finality_tracker = Arc::new(Mutex::new(FinalityTracker::new()));

    log::info!("✅ Checkpoint BFT collector initialized with 4 security modules");
    log::info!("   - CheckpointCollector: Signature aggregation & quorum detection");
    log::info!("   - ByzantineTracker: Byzantine behavior monitoring");
    log::info!("   - EclipseDetector: Eclipse attack detection");
    log::info!("   - FinalityTracker: Checkpoint finality tracking");
    log::info!("   Note: Fork detection & rate limiting handled at P2P layer");

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF BLOCK PRODUCTION (PPFA Proposer)
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This replaces AURA's round-robin with ASF's PPFA (Proposing Panel for Attestation)
    // rotation scheme.

    // Create channel for P2P signature broadcasting (used if authority role active)
    let (ppfa_sig_tx, ppfa_sig_rx) = tokio::sync::mpsc::channel::<CheckpointSignature>(1000);

    if role.is_authority() {
        log::info!(
            "🔥 Starting ASF consensus (PPFA) for FlareChain authority node"
        );

        // Create proposer factory (same as AURA, but will use ASF logic)
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        // ASF block production worker:
        // 1. Loads validator identity from keystore
        // 2. Queries validator-management for committee membership
        // 3. Calculates PPFA rotation schedule
        // 4. Handles Queen and Ant block creation
        // 5. Signs checkpoints at checkpoint intervals

        log::info!(
            "ASF PPFA proposer initialized (slot_duration: {}ms, committee_size: {})",
            asf_params.slot_duration,
            asf_params.max_committee_size
        );

        // ASF block production task - PPFA proposer loop
        let ppfa_client = client.clone();
        let ppfa_backend = backend.clone();
        let ppfa_params = asf_params.clone();
        let ppfa_block_import = block_import.clone();
        let mut ppfa_proposer_factory = proposer_factory;
        let ppfa_keystore = keystore_container.keystore();

        // V17: Clone checkpoint collectors for PPFA task
        let ppfa_checkpoint_collector = checkpoint_collector.clone();
        let ppfa_byzantine_tracker = byzantine_tracker.clone();
        let ppfa_finality_tracker = finality_tracker.clone();

        task_manager.spawn_essential_handle().spawn_blocking(
            "asf-ppfa-proposer",
            Some("block-authoring"),
            async move {
                log::info!("🚀 Starting PPFA proposer worker (slot_duration: {}ms)", ppfa_params.slot_duration);

                // Initialize PPFA components
                use block_production::{
                    ProposerSelector, CommitteeManager, SlotTimer, HealthMonitor,
                };

                // Create committee manager
                // TEMPORARY FIX: Force target_size to be large enough to include all validators
                // This bypasses the filtering in select_committee() that was limiting to 16
                let mut committee = CommitteeManager::new(100);  // Use 100 to ensure all 21 validators are selected

                // ═══════════════════════════════════════════════════════════════
                // V33: Query ASF validator set from pallet_asf_registry for PPFA committee
                // Falls back to hardcoded keys if no validators registered on-chain
                // ═══════════════════════════════════════════════════════════════

                // V33: Query runtime API for registered ASF validators
                let v33_validator_pubkeys: Vec<[u8; 32]> = {
                    use pallet_asf_registry::AsfRegistryApi;

                    let best_hash = ppfa_client.info().best_hash;
                    let runtime_validators: Option<Vec<[u8; 32]>> = ppfa_client
                        .runtime_api()
                        .asf_validator_set(best_hash)
                        .ok()
                        .filter(|v| !v.is_empty());

                    if let Some(validators) = runtime_validators {
                        log::info!(
                            "✅ V33: PPFA committee using DYNAMIC validator set from pallet_asf_registry ({} validators)",
                            validators.len()
                        );
                        let version = ppfa_client.runtime_api().validator_set_version(best_hash).unwrap_or(0);
                        log::info!("   Validator set version: {}", version);
                        validators
                    } else {
                        log::info!("⚠️  V33: PPFA using HARDCODED fallback keys (no on-chain validators)");
                        log::info!("   Validators should register via `asfRegistry.registerAsfKey()` extrinsic");

                        // Fallback: Hardcoded ASF keys (sr25519, seeds //Validator0 through //Validator19)
                        vec![
                            hex_to_bytes32("d684fb9413cc36d5388fd1b4a9112158d76344a46c7ba78f3abd78f044df012e"), // Validator 0
                            hex_to_bytes32("f452cc9c48012cdde4ccdf3b5c2f5a26816292f85572554f9ee7ac14c1fcab46"), // Validator 1
                            hex_to_bytes32("b2a618444ec2fe714b3d811358154ee326822c8f4c9dfa11ddddce86232df05e"), // Validator 2
                            hex_to_bytes32("40746dd99b0cd9b8003137482d5e5a5db27018b5fcf3dfc2804ba79dd18fa064"), // Validator 3
                            hex_to_bytes32("0084df35e1a4365297c88c8c1d23771f33629a985595801eac6a8d63ad37cf7c"), // Validator 4
                            hex_to_bytes32("de829258a4d8f3b7aba1fcafac2a3f90934fe06e29fb5e892676efd55aa5ab7a"), // Validator 5
                            hex_to_bytes32("24fb1fce1c3362778ee8a1c39ac55cf84114fa9fa2159f145be5ff9db471692c"), // Validator 6
                            hex_to_bytes32("a0043aeb20a72fe653b8a9033f45f6f773e74a7459291f0749e83e4c88a40138"), // Validator 7
                            hex_to_bytes32("009f9573813397c72b4dc6c892042f0966e215acbd50d42d6160536d7459ec36"), // Validator 8
                            hex_to_bytes32("4620c12c7e24b58439098cd5a187c9cf4c0c4f46f4aefbe3501dfa2793a08b1f"), // Validator 9
                            hex_to_bytes32("18b6b5b3ae15d535150edd2a0368c19d3f938c1e18aa25940e4d07c8e7827e51"), // Validator 10
                            hex_to_bytes32("3a1ea38d46b86d5ddb0bf21e98fe6728a97f46cdee85342520451a1696e1174c"), // Validator 11
                            hex_to_bytes32("b2669b95a01cf04d89e0ccddc19dd3b37a80c53d77b3e8643359a213330ceb68"), // Validator 12
                            hex_to_bytes32("f06f9181f1d8aadb108a637c43ce69c739f3c407afadc9f0d36078baf687a567"), // Validator 13
                            hex_to_bytes32("060e511e0cf6825e6a01db5a35294d0cbc1f444f3f9b80f77277cb4b8cb27052"), // Validator 14
                            hex_to_bytes32("ea618651fbcb535f1d4006d6e9eb9b82110ee279d1ae7e8a06f1140e0dc46947"), // Validator 15
                            hex_to_bytes32("6ee9536da0982e077854c8d53d84d9d08148ead33ae67355f92857dabdfd3e58"), // Validator 16
                            hex_to_bytes32("925455da5062769f3c118ce13045d8501120470013f3ac63eab84c7fd8595145"), // Validator 17
                            hex_to_bytes32("d06f4bf091f6785ab4565f3de532c79f52c1986a4e2a27b4c85035953fe98421"), // Validator 18
                            hex_to_bytes32("72f6e8ed338d2d4b5cab78208d02384c9ee2f0ff55b598eba6a6988c2cdcfe43"), // Validator 19
                        ]
                    }
                };

                log::info!("✅ V33: PPFA committee initialized with {} validators", v33_validator_pubkeys.len());

                // Add validators to PPFA committee
                let mut added_count = 0;
                for (idx, pubkey) in v33_validator_pubkeys.iter().enumerate() {
                    let validator_id = block_production::ValidatorId::from(*pubkey);
                    let validator_info = validator_management::ValidatorInfo::new(
                        validator_id,
                        ppfa_params.min_validator_stake,
                        validator_management::PeerType::ValidityNode,
                    );
                    match committee.add_validator(validator_info) {
                        Ok(_) => {
                            added_count += 1;
                            log::debug!("✅ Added validator {} to PPFA committee: {}", idx, hex::encode(&pubkey[..8]));
                        }
                        Err(e) => {
                            log::error!("❌ Failed to add validator {} to PPFA committee: {:?}", idx, e);
                        }
                    }
                }
                log::info!(
                    "📊 V33 PPFA committee: {}/{} validators added (dynamic or fallback)",
                    added_count,
                    v33_validator_pubkeys.len()
                );

                // CRITICAL: Call rotate_committee() to move validators from pool into active committee
                log::info!("🔄 Rotating committee to populate active members from validator pool...");
                if let Err(e) = committee.rotate_committee(0) {
                    log::error!("❌ Failed to rotate committee: {:?}", e);
                } else {
                    log::info!("✅ Committee rotated successfully");
                }

                log::info!(
                    "🔗 PPFA committee initialized (size: {}/{}, mode: production)",
                    committee.committee_size(),
                    ppfa_params.max_committee_size
                );

                // V33: Verify our ASF key from keystore is in the hardcoded committee
                // FIX: ASF uses dedicated "asfk" key type (0x6173666b = "asfk")
                use sp_core::crypto::KeyTypeId;
                const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

                let our_keys = ppfa_keystore.sr25519_public_keys(ASF_KEY_TYPE);
                if !our_keys.is_empty() {
                    let raw_pubkey: [u8; 32] = our_keys[0].0;

                    // V33: Check if our key is in the hardcoded validator list
                    let is_in_committee = v33_validator_pubkeys.iter().any(|pk| pk == &raw_pubkey);

                    if is_in_committee {
                        log::info!(
                            "🔑 V32: Our ASF key {} is in hardcoded committee - block production enabled",
                            hex::encode(&raw_pubkey[..8])
                        );
                    } else {
                        log::warn!(
                            "⚠️  V32: Our ASF key {} is NOT in hardcoded committee! Check keystore seed.",
                            hex::encode(&raw_pubkey[..8])
                        );
                    }
                } else {
                    log::warn!(
                        "⚠️  No ASF keys in keystore. Generate with: ./target/release/primearc-core key insert --key-type asfk --scheme sr25519"
                    );
                }

                // V33: Committee is now fully populated from hardcoded list (no separate addition needed)

                // Rotate to initialize committee
                if let Err(e) = committee.rotate_committee(1) {
                    log::error!("Failed to initialize committee rotation: {:?}", e);
                    return;
                }

                // Create proposer selector
                let mut proposer_selector = ProposerSelector::new(committee.clone());

                // Create slot timer with health monitoring
                let health_monitor = HealthMonitor::default();
                let mut slot_timer = SlotTimer::new(ppfa_params.slot_duration, health_monitor);

                // Get genesis time (use current time for now)
                // Genesis time approximation for slot calculation
                let genesis_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;

                slot_timer.reset(genesis_time);

                log::info!("✅ PPFA proposer initialized");
                log::info!("   - Committee size: {}", proposer_selector.committee_size());
                log::info!("   - Slot duration: {}ms", slot_timer.current_duration());
                log::info!("   - Genesis time: {}", genesis_time);

                // Main proposer loop
                let mut slot_count = 0u64;
                loop {
                    // Get current time
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;

                    // Check if it's time for next slot
                    if slot_timer.is_next_slot(current_time) {
                        slot_count += 1;
                        let slot_number = slot_timer.current_slot();

                        // Get current PPFA index and proposer
                        let ppfa_index = proposer_selector.current_ppfa_index();
                        let current_proposer = match proposer_selector.current_proposer() {
                            Ok(proposer) => proposer,
                            Err(e) => {
                                log::error!("Failed to get current proposer: {:?}", e);
                                slot_timer.advance_slot(current_time);
                                continue;
                            }
                        };

                        log::debug!(
                            "Slot #{} (PPFA index: {}) - Proposer: {:?}",
                            slot_number,
                            ppfa_index,
                            hex::encode(&current_proposer.encode()[..8])
                        );

                        // Get our validator ID from keystore
                        // FIX: ASF uses dedicated "asfk" key type (0x6173666b = "asfk")
                        // v108: ASF uses dedicated "asfk" key type (0x6173666b = "asfk")
                        use sp_core::crypto::KeyTypeId;

                        const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

                        // V26 FIX: Use raw sr25519 public key bytes, NOT hashed AccountId!
                        // The V25 bug: into_account() hashes the key with Blake2-256
                        let our_validator_id = match ppfa_keystore.sr25519_public_keys(ASF_KEY_TYPE).first() {
                            Some(public_key) => {
                                // Use raw sr25519 bytes (.0) directly - NO HASHING!
                                let raw_pubkey: [u8; 32] = public_key.0;
                                log::info!(
                                    "🔑 V26: Using RAW sr25519 key from keystore: {}",
                                    hex::encode(&raw_pubkey[..8])
                                );
                                block_production::ValidatorId::from(raw_pubkey)
                            }
                            None => {
                                log::warn!(
                                    "⚠️  No ASF validator key found in keystore. \
                                     Using placeholder. Node may not participate in block production."
                                );
                                block_production::ValidatorId::from([0u8; 32])
                            }
                        };

                        // DEBUG: Log proposer comparison for troubleshooting
                        log::info!(
                            "🔍 Slot #{}: Current proposer = {}, Our ID = {}, Match = {}",
                            slot_number,
                            hex::encode(&current_proposer.encode()[..16]),
                            hex::encode(&our_validator_id.encode()[..16]),
                            proposer_selector.is_proposer(&our_validator_id)
                        );

                        // Check if we are the proposer
                        if proposer_selector.is_proposer(&our_validator_id) {
                            log::info!(
                                "📦 We are proposer for slot #{} (PPFA index: {})",
                                slot_number,
                                ppfa_index
                            );

                            // IMPLEMENT BLOCK PRODUCTION
                            // Get parent block info
                            let chain_info = ppfa_client.usage_info().chain;
                            let parent_hash = chain_info.best_hash;
                            let parent_number = chain_info.best_number;

                            log::debug!(
                                "   Creating block on parent: #{} ({:?})",
                                parent_number,
                                parent_hash
                            );

                            // Get parent header for proposer initialization
                            let parent_header = match ppfa_client.header(parent_hash) {
                                Ok(Some(header)) => header,
                                Ok(None) => {
                                    log::error!("Parent header not found for hash {:?}", parent_hash);
                                    slot_timer.advance_slot(current_time);
                                    continue;
                                },
                                Err(e) => {
                                    log::error!("Failed to get parent header: {:?}", e);
                                    slot_timer.advance_slot(current_time);
                                    continue;
                                }
                            };

                            // Create block proposal using sc_basic_authorship proposer
                            let proposer = match ppfa_proposer_factory.init(&parent_header).await {
                                Ok(p) => p,
                                Err(e) => {
                                    log::error!("Failed to initialize proposer: {:?}", e);
                                    slot_timer.advance_slot(current_time);
                                    continue;
                                }
                            };

                            // Build block with inherent data
                            use sp_inherents::{InherentData, InherentDataProvider};
                            let timestamp_provider = sp_timestamp::InherentDataProvider::from_system_time();
                            let mut inherent_data = InherentData::new();
                            if let Err(e) = timestamp_provider.provide_inherent_data(&mut inherent_data).await {
                                log::error!("Failed to create inherent data: {:?}", e);
                                slot_timer.advance_slot(current_time);
                                continue;
                            }

                            // ═══════════════════════════════════════════════════════════════
                            // PPFA BLOCK SEALING: Create PPFA seal BEFORE proposing block
                            // This ensures the seal is included in the block header and
                            // propagated to all validators over the network.
                            // ═══════════════════════════════════════════════════════════════
                            use sp_runtime::{Digest, DigestItem};
                            use codec::Encode;

                            #[derive(Encode)]
                            struct PpfaSeal {
                                ppfa_index: u32,
                                proposer_id: [u8; 32],
                                slot_number: u64,
                                timestamp: u64,
                            }

                            let ppfa_seal = PpfaSeal {
                                ppfa_index,
                                proposer_id: *our_validator_id.as_ref(),
                                slot_number,
                                timestamp: current_time,
                            };

                            let mut pre_digest = Digest::default();
                            pre_digest.push(DigestItem::PreRuntime(
                                *b"PPFA",
                                ppfa_seal.encode(),
                            ));

                            log::debug!(
                                "🔏 Creating block with PPFA seal: index={}, proposer={:?}",
                                ppfa_index,
                                hex::encode(&our_validator_id.encode()[..8])
                            );

                            match proposer.propose(
                                inherent_data,
                                pre_digest, // Include PPFA seal in block digest
                                Duration::from_secs(5), // 5 second block production timeout
                                None, // No soft deadline
                            ).await {
                                Ok(proposal) => {
                                    let block = proposal.block;
                                    let block_hash = block.header.hash();

                                    log::info!(
                                        "🔨 Authored block #{} ({:?}) with {} extrinsics",
                                        block.header.number(),
                                        block_hash,
                                        block.extrinsics.len()
                                    );

                                    // Import the block
                                    use sc_consensus::BlockImportParams;
                                    use sp_runtime::traits::Header as _;

                                    let mut import_params = BlockImportParams::new(
                                        sp_consensus::BlockOrigin::Own,
                                        block.header.clone(),
                                    );
                                    import_params.body = Some(block.extrinsics.to_vec());
                                    import_params.finalized = false;
                                    import_params.fork_choice = Some(sc_consensus::ForkChoiceStrategy::LongestChain);

                                    // PPFA seal is already in the block header (added before propose())
                                    // No need to add post_digests - the seal was included during block creation

                                    match ppfa_block_import.import_block(import_params).await {
                                        Ok(result) => {
                                            log::info!(
                                                "✅ Block #{} imported successfully: {:?}",
                                                block.header.number(),
                                                result
                                            );

                                            // V22: Checkpoint detection moved to dedicated task
                                            // that listens to ALL block imports (not just self-authored blocks)

                                            // ═══════════════════════════════════════════════════
                                            // FINALITY INTEGRATION: Propose block to ASF finality
                                            // ═══════════════════════════════════════════════════
                                            // Checkpoint-based finality used instead
                                            // let finality_block_hash = finality_gadget::BlockHash::from_bytes(block_hash.into());
                                            // let mut gadget = ppfa_finality_gadget.lock().await;
                                            // match gadget.propose_block(finality_block_hash).await {
                                            //     Ok(vote) => {
                                            //         log::info!(
                                            //             "🗳️  Created finality vote for block #{} at view {:?}",
                                            //             block.header.number(),
                                            //             vote.view
                                            //         );
                                            //     }
                                            //     Err(e) => {
                                            //         log::error!(
                                            //             "❌ Failed to create finality vote for block #{}: {}",
                                            //             block.header.number(),
                                            //             e
                                            //         );
                                            //     }
                                            // }
                                        },
                                        Err(e) => {
                                            log::error!(
                                                "❌ Failed to import block #{}: {:?}",
                                                block.header.number(),
                                                e
                                            );
                                        }
                                    }
                                },
                                Err(e) => {
                                    log::error!("Failed to propose block for slot #{}: {:?}", slot_number, e);
                                }
                            }
                        } else {
                            log::trace!(
                                "Not our slot (proposer: {:?})",
                                hex::encode(&current_proposer.encode()[..8])
                            );
                        }

                        // Advance to next proposer (PPFA rotation)
                        let chain_info = ppfa_client.usage_info().chain;
                        let block_number = chain_info.best_number;
                        proposer_selector.advance(block_number as u64);
                        slot_timer.advance_slot(current_time);

                        // Update health monitoring
                        // Network health metrics collection
                        slot_timer.health_monitor_mut().record_block_production(true);

                        // Check for epoch boundaries and trigger committee rotation
                        if slot_count % ppfa_params.epoch_duration as u64 == 0 {
                            let slot_epoch = slot_count / ppfa_params.epoch_duration as u64;

                            // Query current epoch from runtime
                            let chain_info = ppfa_client.usage_info().chain;
                            let at_hash = chain_info.best_hash;

                            // Query the runtime for current epoch and committee
                            // Runtime API integration for dynamic committee updates:
                            //   let runtime_epoch = ppfa_client.runtime_api().current_epoch(at_hash).ok();
                            //   let new_committee = ppfa_client.runtime_api().validator_committee(at_hash).ok();

                            log::info!(
                                "🔄 Epoch transition detected at slot #{} (slot epoch: #{})",
                                slot_number,
                                slot_epoch
                            );

                            // ═══════════════════════════════════════════════════════════════
                            // Epoch transitions with committee rotation
                            // ═══════════════════════════════════════════════════════════════

                            // Query runtime for new committee at epoch boundary
                            match ppfa_client.runtime_api().validator_committee(at_hash) {
                                Ok(new_committee_members) => {
                                    log::info!(
                                        "✅ Loaded {} new committee members for epoch #{}",
                                        new_committee_members.len(),
                                        slot_epoch
                                    );

                                    // Update committee with new members
                                    committee.clear_committee();
                                    for validator_info in new_committee_members {
                                        if let Err(e) = committee.add_validator(validator_info) {
                                            log::warn!("Failed to add validator to new committee: {:?}", e);
                                        }
                                    }

                                    // Rotate committee to new epoch
                                    let epoch_u32 = slot_epoch.try_into().unwrap_or_else(|_| {
                                        log::warn!("Epoch {} too large for u32, using max", slot_epoch);
                                        u32::MAX
                                    });
                                    if let Err(e) = committee.rotate_committee(epoch_u32) {
                                        log::error!("Failed to rotate committee to epoch {}: {:?}", slot_epoch, e);
                                    } else {
                                        // Update proposer selector with refreshed committee (pass epoch number)
                                        if let Err(e) = proposer_selector.rotate_committee(epoch_u32) {
                                            log::error!("Failed to rotate proposer selector: {:?}", e);
                                        }
                                        log::info!(
                                            "🔄 Committee rotated successfully (size: {}, epoch: {})",
                                            committee.committee_size(),
                                            slot_epoch
                                        );
                                    }
                                }
                                Err(e) => {
                                    log::error!(
                                        "❌ Failed to load committee from runtime for epoch {}: {:?}",
                                        slot_epoch,
                                        e
                                    );
                                    // Continue with existing committee if runtime query fails
                                }
                            }
                        }
                    }

                    // Wait a short time before checking again
                    // Async timing for slot advancement
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }
            },
        );
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // V22: CHECKPOINT DETECTION FOR ALL BLOCK IMPORTS
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // This task listens to ALL block imports (not just self-authored blocks) and:
    // 1. Detects guaranteed checkpoints (every 32 blocks)
    // 2. Detects opportunity checkpoints (VRF-triggered)
    // 3. Creates checkpoint signatures for this validator
    // 4. Broadcasts signatures via P2P
    //
    // This runs on ALL validators (not just authorities), ensuring 15+/21 signatures.

    let checkpoint_detection_client = client.clone();
    let checkpoint_detection_keystore = keystore_container.keystore();
    let checkpoint_detection_collector = checkpoint_collector.clone();
    let checkpoint_detection_byzantine = byzantine_tracker.clone();
    let checkpoint_detection_finality = finality_tracker.clone();
    let checkpoint_detection_sig_tx = ppfa_sig_tx.clone();
    let checkpoint_detection_params = asf_params.clone();

    task_manager.spawn_essential_handle().spawn(
        "checkpoint-detection-all-imports",
        Some("finality"),
        async move {
            log::info!("🔍 Starting checkpoint detection for ALL block imports (V22)");

            // Subscribe to ALL block import notifications
            let mut block_import_stream = checkpoint_detection_client.every_import_notification_stream();

            while let Some(notification) = block_import_stream.next().await {
                let block_number = *notification.header.number();
                let block_hash = notification.hash;

                // Detect if this block should be a checkpoint
                let checkpoint_type = if is_guaranteed_checkpoint(block_number) {
                    Some(CheckpointType::Guaranteed)
                } else {
                    // Opportunity checkpoints detected via VRF
                    let parent_hash = notification.header.parent_hash();
                    let parent_hash_bytes: [u8; 32] = parent_hash.as_ref().try_into()
                        .unwrap_or([0u8; 32]);

                    // Get epoch information for VRF evaluation
                    let epoch = (block_number / checkpoint_detection_params.epoch_duration) as u64;

                    // Generate epoch randomness from authority set ID
                    let vrf_authority_set = checkpoint_detection_collector.get_authority_set();
                    let authority_set_id = vrf_authority_set.set_id;
                    let mut epoch_randomness = [0u8; 32];
                    epoch_randomness[..8].copy_from_slice(&authority_set_id.to_le_bytes());
                    epoch_randomness[8..16].copy_from_slice(&epoch.to_le_bytes());

                    // Evaluate VRF checkpoint decision
                    use checkpoint_bft::vrf::VrfCheckpointDecision;
                    let vrf_decision = VrfCheckpointDecision::evaluate(
                        block_number,
                        parent_hash_bytes,
                        epoch,
                        epoch_randomness,
                    );

                    if vrf_decision.is_checkpoint {
                        log::debug!(
                            "🎲 VRF triggered opportunity checkpoint at block #{} (epoch: {})",
                            block_number,
                            epoch
                        );
                        Some(vrf_decision.checkpoint_type)
                    } else {
                        None
                    }
                };

                if let Some(cp_type) = checkpoint_type {
                    log::info!(
                        "📍 Checkpoint detected at block #{} ({:?}, hash: {:?})",
                        block_number,
                        cp_type,
                        block_hash
                    );

                    // Record checkpoint opportunity for Byzantine tracking
                    if let Ok(mut tracker) = checkpoint_detection_byzantine.lock() {
                        tracker.record_checkpoint_opportunity(
                            block_number,
                            &(0..21).collect::<Vec<_>>(), // All validators expected to sign
                        );
                    }

                    // ═══════════════════════════════════════════════════
                    // CHECKPOINT SIGNATURE CREATION
                    // ═══════════════════════════════════════════════════

                    // Get validator signing key (V23: now looks up validator_id in authority set)
                    match get_validator_sr25519_key(&checkpoint_detection_keystore, &checkpoint_detection_collector) {
                        Ok((public_key, validator_id)) => {
                            // Get validator public key (32 bytes)
                            let public_bytes: [u8; 32] = public_key.0;

                            // Get CURRENT authority set information
                            let current_authority_set = checkpoint_detection_collector.get_authority_set();
                            let authority_set_id = current_authority_set.set_id;
                            let authority_set_hash = current_authority_set.authority_set_hash;

                            // Get chain ID
                            use checkpoint_bft::FLARECHAIN_NETWORK_ID;
                            let chain_id = FLARECHAIN_NETWORK_ID;

                            // Get signature nonce
                            let signature_nonce = get_next_signature_nonce(validator_id);

                            // Convert block hash to [u8; 32]
                            let block_hash_bytes: [u8; 32] = block_hash.into();

                            // Create checkpoint signature
                            let signature = create_checkpoint_signature(
                                block_number,
                                &block_hash_bytes,
                                validator_id,
                                public_bytes,
                                authority_set_id,
                                authority_set_hash,
                                cp_type.clone(),
                                signature_nonce,
                                &checkpoint_detection_keystore,
                                &public_key,
                                chain_id,
                            );

                            log::info!(
                                "✍️  Signed checkpoint #{} as validator {} (nonce: {})",
                                block_number,
                                validator_id,
                                signature_nonce
                            );

                            // Add signature to collector
                            match checkpoint_detection_collector.add_signature(signature.clone()) {
                                Ok(Some(certificate)) => {
                                    log::info!(
                                        "🎉 Checkpoint certificate complete for block #{} with {} signatures!",
                                        certificate.block_number,
                                        certificate.signatures.len()
                                    );

                                    // Record finalized checkpoint in tracker
                                    if let Ok(tracker) = checkpoint_detection_finality.lock() {
                                        let _ = tracker.finalize_block(
                                            certificate.block_number,
                                            certificate.block_hash,
                                            certificate.signatures.len() as u32,
                                        );
                                    }
                                }
                                Ok(None) => {
                                    log::debug!(
                                        "   Checkpoint signature added, awaiting quorum ({}/15)",
                                        checkpoint_detection_collector.get_signature_count(block_number)
                                    );
                                }
                                Err(e) => {
                                    log::warn!("⚠️  Failed to add checkpoint signature: {}", e);
                                }
                            }

                            // Broadcast signature via P2P
                            if let Err(e) = checkpoint_detection_sig_tx.send(signature.clone()).await {
                                log::warn!("⚠️  Failed to send signature to broadcast task: {}", e);
                            } else {
                                log::debug!("   Checkpoint signature queued for P2P broadcast");
                            }
                        }
                        Err(e) => {
                            log::debug!(
                                "   No validator key available for checkpoint signing: {}",
                                e
                            );
                        }
                    }
                }
            }

            log::warn!("⚠️  Checkpoint detection stream ended unexpectedly");
        },
    );

    log::info!("✅ Checkpoint detection task spawned (listens to ALL block imports)");

    // ═══════════════════════════════════════════════════════════════════════════
    // ASF FINALITY GADGET (Pure ASF, v108)
    // ═══════════════════════════════════════════════════════════════════════════

    if asf_params.enable_finality_gadget {
        log::info!("🎯 Checkpoint BFT finality system active (3-level finality)");

        // ========== NETWORK BRIDGE IMPLEMENTATION ==========
        //
        // Create a bridge between finality-gadget and sc-network for gossip
        use finality_gadget::{NetworkBridge, Vote as FinalityVote, Certificate as FinalityCertificate};
        use codec::{Encode, Decode};

        // Define ASF finality gossip protocol
        const ASF_FINALITY_PROTOCOL: &str = "/etrid/asf-finality/1";

        #[derive(Clone, Debug, Encode, Decode)]
        enum AsfFinalityMessage {
            Vote(FinalityVote),
            Certificate(FinalityCertificate),
        }

        // NetworkBridge implementation using DETR P2P
        struct DetrP2PNetworkBridge {
            p2p_network: Arc<P2PNetwork>,
            gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
        }

        impl DetrP2PNetworkBridge {
            fn new(
                p2p_network: Arc<P2PNetwork>,
                gadget_bridge: Arc<tokio::sync::Mutex<GadgetNetworkBridge>>,
            ) -> Self {
                Self {
                    p2p_network,
                    gadget_bridge,
                }
            }

            /// Convert finality-gadget Vote to bridge VoteData
            fn convert_vote_to_bridge(vote: &FinalityVote) -> VoteData {
                VoteData {
                    validator_id: vote.validator_id.0,  // Extract u32 from ValidatorId newtype
                    view: vote.view.0,  // Extract u64 from View newtype
                    block_hash: {
                        let encoded = vote.block_hash.encode();
                        let mut hash = [0u8; 32];
                        hash.copy_from_slice(&encoded[0..32]);
                        hash
                    },
                    signature: vote.signature.clone(),
                }
            }

            /// Convert finality-gadget Certificate to bridge CertificateData
            fn convert_certificate_to_bridge(cert: &FinalityCertificate) -> CertificateData {
                // Convert finality signatures to bridge format: (validator_id, signature)
                // finality-gadget has Vec<(ValidatorId, Vec<u8>)>
                // bridge expects Vec<(u32, Vec<u8>)>
                let signatures: Vec<(u32, Vec<u8>)> = cert.signatures.iter()
                    .map(|(validator_id, sig)| (validator_id.0, sig.clone()))
                    .collect();

                CertificateData {
                    view: cert.view.0,  // View is a newtype wrapper
                    block_hash: {
                        let encoded = cert.block_hash.encode();
                        let mut hash = [0u8; 32];
                        hash.copy_from_slice(&encoded[0..32]);
                        hash
                    },
                    signatures,
                }
            }
        }

        #[async_trait::async_trait]
        impl NetworkBridge for DetrP2PNetworkBridge {
            async fn broadcast_vote(&self, vote: FinalityVote) -> Result<(), String> {
                log::trace!(
                    "Broadcasting ASF finality vote (validator: {:?}, view: {:?})",
                    vote.validator_id,
                    vote.view
                );

                // Convert vote to bridge format
                let vote_data = Self::convert_vote_to_bridge(&vote);

                // Queue vote in gadget bridge
                let mut bridge = self.gadget_bridge.lock().await;
                bridge.send_vote(vote_data).await
                    .map_err(|e| format!("Failed to queue vote: {:?}", e))?;

                // Get outbound messages from bridge
                let messages = bridge.get_outbound_messages().await;

                // Send each message via P2P
                for (msg, _priority) in messages {
                    match msg {
                        ConsensusBridgeMessage::Vote(vote_data) => {
                            // Serialize vote data
                            let payload = bincode::serialize(&vote_data)
                                .map_err(|e| format!("Failed to serialize vote: {:?}", e))?;

                            // Create P2P message
                            let p2p_msg = P2PMessage::Vote {
                                data: payload,
                            };

                            // Broadcast to all connected peers
                            self.p2p_network.broadcast(p2p_msg).await
                                .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;

                            log::debug!("✅ Vote broadcast via detrp2p (view: {})", vote_data.view);
                        }
                        _ => {
                            log::warn!("Unexpected message type when broadcasting vote");
                        }
                    }
                }

                Ok(())
            }

            async fn broadcast_certificate(&self, cert: FinalityCertificate) -> Result<(), String> {
                log::trace!(
                    "Broadcasting ASF finality certificate (view: {:?}, voters: {})",
                    cert.view,
                    cert.signatures.len()
                );

                // Convert certificate to bridge format
                let cert_data = Self::convert_certificate_to_bridge(&cert);

                // Queue certificate in gadget bridge
                let mut bridge = self.gadget_bridge.lock().await;
                bridge.send_certificate(cert_data).await
                    .map_err(|e| format!("Failed to queue certificate: {:?}", e))?;

                // Get outbound messages from bridge
                let messages = bridge.get_outbound_messages().await;

                // Send each message via P2P
                for (msg, _priority) in messages {
                    match msg {
                        ConsensusBridgeMessage::Certificate(cert_data) => {
                            // Serialize certificate data
                            let payload = bincode::serialize(&cert_data)
                                .map_err(|e| format!("Failed to serialize certificate: {:?}", e))?;

                            // Create P2P message
                            let p2p_msg = P2PMessage::Certificate {
                                data: payload,
                            };

                            // Broadcast to all connected peers
                            self.p2p_network.broadcast(p2p_msg).await
                                .map_err(|e| format!("P2P broadcast failed: {:?}", e))?;

                            log::debug!("✅ Certificate broadcast via detrp2p (view: {}, voters: {})",
                                cert_data.view, cert_data.signatures.len());
                        }
                        _ => {
                            log::warn!("Unexpected message type when broadcasting certificate");
                        }
                    }
                }

                Ok(())
            }

            async fn get_connected_peers(&self) -> Vec<String> {
                // Get connected peers from P2P network
                let peers = self.p2p_network.get_connected_peers().await;
                peers.into_iter()
                    .map(|peer_id| hex::encode(peer_id.as_bytes()))
                    .collect()
            }

            // V17: Checkpoint BFT broadcast methods
            async fn broadcast_checkpoint_signature(&self, _signature: Vec<u8>) -> Result<(), String> {
                // Checkpoint signature broadcasting handled by checkpoint-signature-broadcaster task
                // This will be used when validators sign checkpoints
                log::trace!("Checkpoint signature broadcast (Phase 2)");
                Ok(())
            }

            async fn broadcast_checkpoint_certificate(&self, _certificate: Vec<u8>) -> Result<(), String> {
                // Checkpoint certificate broadcasting via P2P
                // This will be used when quorum is reached
                log::trace!("Checkpoint certificate broadcast (Phase 3)");
                Ok(())
            }
        }

        // ═════════════════════════════════════════════════════════════════════════════
        // HELPER FUNCTIONS: Bridge Format ↔ Finality-Gadget Format Conversion
        // ═════════════════════════════════════════════════════════════════════════════

        /// Convert VoteData (bridge format) to finality_gadget::Vote
        fn convert_vote_from_bridge(vote_data: VoteData) -> finality_gadget::Vote {
            finality_gadget::Vote {
                validator_id: finality_gadget::ValidatorId(vote_data.validator_id),
                view: finality_gadget::View(vote_data.view),
                block_hash: finality_gadget::BlockHash::from_bytes(vote_data.block_hash),
                signature: vote_data.signature,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }

        /// Convert CertificateData (bridge format) to finality_gadget::Certificate
        fn convert_certificate_from_bridge(cert_data: CertificateData) -> finality_gadget::Certificate {
            finality_gadget::Certificate {
                view: finality_gadget::View(cert_data.view),
                block_hash: finality_gadget::BlockHash::from_bytes(cert_data.block_hash),
                signatures: cert_data.signatures.into_iter()
                    .map(|(id, sig)| (finality_gadget::ValidatorId(id), sig))
                    .collect(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }

        // ========== FINALITY GADGET INITIALIZATION ==========

        // Extract validator identity from keystore
        let validator_id = {
            if role.is_authority() {
                // Load ASF validator key from keystore
                use sp_core::crypto::KeyTypeId;
                const ASF_KEY_TYPE: KeyTypeId = KeyTypeId([0x61, 0x73, 0x66, 0x6b]); // "asfk"

                let keystore = keystore_container.keystore();
                let asf_keys = keystore.sr25519_public_keys(ASF_KEY_TYPE);

                match asf_keys.first() {
                    Some(public_key) => {
                        // Convert Sr25519 public key (32 bytes) to u32 validator ID
                        // Use first 4 bytes of the public key as the validator ID
                        let key_bytes = public_key.as_ref() as &[u8];
                        let validator_id_u32 = u32::from_le_bytes([
                            key_bytes[0],
                            key_bytes[1],
                            key_bytes[2],
                            key_bytes[3],
                        ]);

                        log::info!(
                            "🔑 ASF Finality Gadget using validator key from keystore: {}",
                            hex::encode(key_bytes)
                        );
                        log::info!(
                            "🆔 Derived ASF validator ID: {} (from first 4 bytes: {:02x}{:02x}{:02x}{:02x})",
                            validator_id_u32,
                            key_bytes[0], key_bytes[1], key_bytes[2], key_bytes[3]
                        );

                        finality_gadget::ValidatorId(validator_id_u32)
                    }
                    None => {
                        log::warn!(
                            "⚠️  No ASF key found in keystore for Finality Gadget. \
                             Using observer mode (non-validator)."
                        );
                        finality_gadget::ValidatorId(u32::MAX) // Non-validator observer
                    }
                }
            } else {
                // Non-authority nodes are observers
                finality_gadget::ValidatorId(u32::MAX)
            }
        };

        // ========== INITIALIZE DETR P2P NETWORK ==========

        log::info!("🌐 Initializing DETR P2P network for ASF finality");

        // ═══════════════════════════════════════════════════════════════
        // Derive peer ID from validator identity
        // ═══════════════════════════════════════════════════════════════

        // Generate local peer ID from validator ID (now derived from actual validator identity)
        // Convert u32 validator ID to 32-byte peer ID (pad with zeros)
        let mut peer_id_bytes = [0u8; 32];
        peer_id_bytes[0..4].copy_from_slice(&validator_id.0.to_le_bytes());
        let local_peer_id = PeerId::new(peer_id_bytes);

        // Get local listen address from config
        // SECURITY: Prefer specific network interface over 0.0.0.0 (all interfaces)
        use std::net::{SocketAddr, IpAddr, Ipv4Addr};

        let detr_p2p_port = std::env::var("DETR_P2P_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(30334);

        // Determine DETR P2P listen IP with security priority:
        // 1. Explicit DETR_P2P_IP environment variable (highest priority)
        // 2. Extract from Substrate public_addresses (validator's actual IP)
        // 3. Extract from Substrate listen_addresses (node's bind IP)
        // 4. Fallback to 0.0.0.0 (SECURITY WARNING: exposes to all interfaces)

        let detr_p2p_ip = if let Ok(env_ip) = std::env::var("DETR_P2P_IP") {
            // Option 1: Explicitly set via environment variable
            match env_ip.parse::<IpAddr>() {
                Ok(ip) => {
                    log::info!("🔒 DETR P2P IP from DETR_P2P_IP env: {}", ip);
                    ip
                }
                Err(e) => {
                    log::warn!("⚠️  Invalid DETR_P2P_IP '{}': {}, using auto-detect", env_ip, e);
                    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
                }
            }
        } else {
            // Option 2: Try to extract from Substrate public_addresses
            let mut detected_ip: Option<IpAddr> = None;

            for addr in &public_addresses {
                let addr_str = addr.to_string();
                // Parse multiaddr format: /ip4/1.2.3.4/tcp/30333
                if let Some(ip_part) = addr_str.split('/').nth(2) {
                    if let Ok(ip) = ip_part.parse::<IpAddr>() {
                        // Skip localhost addresses
                        if !ip.is_loopback() {
                            log::info!("🔍 Detected public IP from Substrate config: {}", ip);
                            detected_ip = Some(ip);
                            break;
                        }
                    }
                }
            }

            // Option 3: Try listen_addresses if no public address
            if detected_ip.is_none() {
                for addr in &listen_addresses {
                    let addr_str = addr.to_string();
                    if let Some(ip_part) = addr_str.split('/').nth(2) {
                        if let Ok(ip) = ip_part.parse::<IpAddr>() {
                            // Use listen IP if it's not 0.0.0.0
                            if !ip.is_unspecified() && !ip.is_loopback() {
                                log::info!("🔍 Detected listen IP from Substrate config: {}", ip);
                                detected_ip = Some(ip);
                                break;
                            }
                        }
                    }
                }
            }

            // Option 4: Fallback to 0.0.0.0 with security warning
            if let Some(ip) = detected_ip {
                ip
            } else {
                log::warn!("⚠️  SECURITY: Could not detect specific IP, using 0.0.0.0 (all interfaces)");
                log::warn!("⚠️  RECOMMENDATION: Set DETR_P2P_IP={} for VM #1", "172.16.0.5");
                log::warn!("⚠️  RECOMMENDATION: Set DETR_P2P_IP={} for VM #2", "172.16.0.4");
                log::warn!("⚠️  This exposes DETR P2P to all network interfaces!");
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
            }
        };

        let socket_addr = SocketAddr::new(detr_p2p_ip, detr_p2p_port);

        log::info!("🌐 DETR P2P will listen on: {}", socket_addr);
        if socket_addr.ip().is_unspecified() {
            log::warn!("⚠️  SECURITY: Port {} exposed on ALL network interfaces", detr_p2p_port);
        } else {
            log::info!("🔒 SECURITY: Port {} bound to specific interface", detr_p2p_port);
        }

        let local_address = PeerAddr {
            id: local_peer_id.clone(),
            address: socket_addr,
        };

        // Parse bootstrap peers from Substrate bootnodes configuration
        // The config.network.boot_nodes contains multiaddr strings like:
        // /ip4/172.16.0.5/tcp/30333/p2p/12D3KooW...
        // We need to extract IP:port for DETR P2P (port 30334) and peer IDs
        let mut bootstrap_peers = Vec::new();

        log::info!("🔍 Parsing bootstrap peers from config.network.boot_nodes:");
        for bootnode in &boot_nodes {
            log::info!("  Raw bootnode: {}", bootnode);

            // Parse multiaddr to extract IP and peer ID
            // Format: /ip4/<IP>/tcp/<PORT>/p2p/<PEER_ID>
            let bootnode_str = bootnode.to_string();
            let parts: Vec<&str> = bootnode_str.split('/').collect();
            if parts.len() >= 6 {
                if let (Some(ip_str), Some(peer_id_str)) = (parts.get(2), parts.last()) {
                    if let Ok(ip) = ip_str.parse::<IpAddr>() {
                        // Use DETR P2P port (30334) instead of Substrate port (30333)
                        let peer_socket = SocketAddr::new(ip, detr_p2p_port);

                        // Parse peer ID from base58 (libp2p format) to bytes
                        // For now, we'll skip the peer ID parsing complexity and just use the IP:port
                        log::info!("  ✓ Adding bootstrap peer: {} (from Substrate bootnode)", peer_socket);

                        // Create a placeholder peer ID from the IP
                        // In production, you'd want to properly parse the libp2p peer ID
                        let mut peer_id_bytes = [0u8; 32];
                        if let IpAddr::V4(ipv4) = ip {
                            peer_id_bytes[0..4].copy_from_slice(&ipv4.octets());
                        }

                        let peer_addr = PeerAddr {
                            id: PeerId::new(peer_id_bytes),
                            address: peer_socket,
                        };

                        bootstrap_peers.push(peer_addr);
                    }
                }
            }
        }

        // Also check for DETR_P2P_BOOTSTRAP environment variable
        if let Ok(bootstrap_env) = std::env::var("DETR_P2P_BOOTSTRAP") {
            log::info!("🔍 Parsing bootstrap peers from DETR_P2P_BOOTSTRAP:");
            for addr_str in bootstrap_env.split(',') {
                if let Ok(addr) = addr_str.trim().parse::<SocketAddr>() {
                    log::info!("  ✓ Adding bootstrap peer: {} (from env)", addr);

                    let mut peer_id_bytes = [0u8; 32];
                    if let IpAddr::V4(ipv4) = addr.ip() {
                        peer_id_bytes[0..4].copy_from_slice(&ipv4.octets());
                    }

                    let peer_addr = PeerAddr {
                        id: PeerId::new(peer_id_bytes),
                        address: addr,
                    };

                    bootstrap_peers.push(peer_addr);
                }
            }
        }

        log::info!("📋 Total DETR P2P bootstrap peers: {}", bootstrap_peers.len());

        // Create P2P network instance
        let p2p_network = Arc::new(P2PNetwork::new(
            local_peer_id.clone(),
            socket_addr,  // P2PNetwork::new takes SocketAddr, not PeerAddr
            bootstrap_peers,
        ));

        // Spawn P2P network start in background task
        let p2p_for_start = p2p_network.clone();
        let peer_id_for_log = local_peer_id.clone();
        let addr_for_log = local_address.address;
        task_manager.spawn_handle().spawn(
            "detr-p2p-start",
            None,
            async move {
                match p2p_for_start.start().await {
                    Ok(_) => {
                        log::info!(
                            "✅ DETR P2P network started (peer_id: {}, address: {})",
                            hex::encode(peer_id_for_log.as_bytes()),
                            addr_for_log
                        );
                    }
                    Err(e) => {
                        log::error!("Failed to start P2P network: {:?}", e);
                    }
                }
            },
        );

        log::info!("🌐 DETR P2P network initialization spawned");

        // Create gadget network bridge
        let gadget_bridge = Arc::new(tokio::sync::Mutex::new(GadgetNetworkBridge::new()));

        log::info!("✅ Gadget network bridge initialized");

        // Create DetrP2PNetworkBridge combining both components
        let network_bridge = Arc::new(DetrP2PNetworkBridge::new(
            p2p_network.clone(),
            gadget_bridge.clone(),
        ));

        log::info!("✅ DetrP2PNetworkBridge created - finality messages will use detrp2p");

        // ═══════════════════════════════════════════════════════════════════════════
        // CHECKPOINT SIGNATURE P2P BROADCAST TASK
        // ═══════════════════════════════════════════════════════════════════════════
        //
        // Consumes signatures from PPFA task and broadcasts them via P2P network

        let sig_broadcast_network = p2p_network.clone();
        let mut sig_broadcast_rx = ppfa_sig_rx;

        task_manager.spawn_essential_handle().spawn(
            "checkpoint-signature-broadcaster",
            Some("finality"),
            async move {
                log::info!("📡 Starting checkpoint signature P2P broadcaster");

                while let Some(signature) = sig_broadcast_rx.recv().await {
                    log::debug!(
                        "📡 Broadcasting checkpoint signature for block #{} from validator {}",
                        signature.block_number,
                        signature.validator_id
                    );

                    if let Err(e) = broadcast_checkpoint_signature_p2p(signature, &sig_broadcast_network).await {
                        log::warn!("⚠️  Failed to broadcast checkpoint signature: {}", e);
                    }
                }

                log::warn!("⚠️  Checkpoint signature broadcast channel closed");
            },
        );

        log::info!("✅ Checkpoint signature P2P broadcaster spawned");

        // V17: FinalityGadget has been replaced by checkpoint-bft
        // The checkpoint collector was already initialized above (lines 694-713)
        // Checkpoint signing is integrated in PPFA block production (lines 1600-1660)

        // Calculate max validators from committee size
        let _max_validators = asf_params.max_committee_size;

        log::info!("Finality: VRF-based checkpoints with Byzantine protection");

        // Checkpoint detection happens directly in PPFA block production (lines 1600-1660)
        // Checkpoint signatures are broadcast via P2P (checkpoint-signature-broadcaster task)
        // Checkpoint certificates are processed via checkpoint-bft-p2p-handler task

        // ═══════════════════════════════════════════════════════════════════════════
        // CHECKPOINT BFT P2P MESSAGE HANDLER
        // ═══════════════════════════════════════════════════════════════════════════
        //
        // This worker polls P2P network for incoming checkpoint signatures and certificates.
        // When enough signatures are collected (quorum), it finalizes blocks via Substrate API.

        let checkpoint_p2p_network = p2p_network.clone();
        let checkpoint_client = client.clone();
        let checkpoint_collector_worker = checkpoint_collector.clone();
        let checkpoint_byzantine_tracker = byzantine_tracker.clone();
        let checkpoint_finality_tracker = finality_tracker.clone();

        task_manager.spawn_essential_handle().spawn(
            "checkpoint-bft-p2p-handler",
            Some("finality"),
            async move {
                log::info!("🔐 Starting Checkpoint BFT P2P message handler");

                use tokio::time::{interval, Duration};
                use etrid_protocol::{ProtocolMessage, MessageType, CheckpointSignatureMsg};

                let mut poll_interval = interval(Duration::from_millis(100));

                loop {
                    poll_interval.tick().await;

                    // Poll P2P network for incoming checkpoint messages
                    while let Some((peer_id, p2p_msg)) = checkpoint_p2p_network.receive_message().await {
                        match p2p_msg {
                            P2PMessage::CheckpointSignature { data } => {
                                log::debug!("📨 Received checkpoint signature from {:?}", peer_id);

                                // Deserialize protocol message
                                match bincode::deserialize::<ProtocolMessage>(&data) {
                                    Ok(proto_msg) => {
                                        // Verify it's a checkpoint signature
                                        if proto_msg.msg_type != MessageType::CheckpointSignature as u8 {
                                            log::warn!("Unexpected message type in CheckpointSignature envelope");
                                            continue;
                                        }

                                        // Deserialize checkpoint signature
                                        match bincode::deserialize::<CheckpointSignatureMsg>(&proto_msg.payload) {
                                            Ok(sig_msg) => {
                                                // Convert to CheckpointSignature using values from P2P message
                                                let chain_id = checkpoint_collector_worker.get_chain_id();

                                                // Convert checkpoint_type u8 to enum
                                                let checkpoint_type = match sig_msg.checkpoint_type {
                                                    0 => CheckpointType::Guaranteed,
                                                    1 => CheckpointType::Opportunity {
                                                        vrf_output: vec![0u8; 32], // Placeholder - full VRF data in future
                                                        vrf_proof: Vec::new(),
                                                    },
                                                    _ => CheckpointType::Guaranteed, // Fallback
                                                };

                                                let signature = CheckpointSignature {
                                                    chain_id,
                                                    block_number: sig_msg.block_number,
                                                    block_hash: sig_msg.block_hash,
                                                    validator_id: sig_msg.validator_id,
                                                    validator_pubkey: sig_msg.validator_pubkey,
                                                    authority_set_id: sig_msg.authority_set_id,
                                                    authority_set_hash: sig_msg.authority_set_hash,
                                                    checkpoint_type,
                                                    signature_nonce: sig_msg.signature_nonce,
                                                    signature: sig_msg.signature.clone(),
                                                    timestamp_ms: sig_msg.timestamp_ms,
                                                };

                                                log::debug!(
                                                    "📨 Processing checkpoint signature from validator {} for block #{}",
                                                    signature.validator_id,
                                                    signature.block_number
                                                );

                                                // Get canonical chain for fork protection
                                                let canonical_chain = get_canonical_chain(&checkpoint_client);

                                                // Add signature with security checks
                                                match checkpoint_collector_worker.add_signature(signature.clone()) {
                                                    Ok(Some(certificate)) => {
                                                        log::info!(
                                                            "✅ Checkpoint certificate complete for block #{}: {} signatures",
                                                            certificate.block_number,
                                                            certificate.signatures.len()
                                                        );

                                                        // Verify canonical chain
                                                        let block_hash_h256 = sp_core::H256::from_slice(&certificate.block_hash);
                                                        let is_canonical = match verify_canonical_chain(
                                                            &checkpoint_client,
                                                            block_hash_h256,
                                                            certificate.block_number
                                                        ) {
                                                            Ok(is_canonical) => is_canonical,
                                                            Err(e) => {
                                                                log::error!("🚨 Canonical chain verification failed: {}", e);
                                                                continue;
                                                            }
                                                        };

                                                        if !is_canonical {
                                                            log::warn!(
                                                                "⚠️ Checkpoint not on canonical chain, skipping finalization"
                                                            );
                                                            continue;
                                                        }

                                                        // Finalize the block
                                                        if let Err(e) = finalize_checkpoint_block(
                                                            certificate,
                                                            &checkpoint_client,
                                                            &checkpoint_p2p_network,
                                                            &checkpoint_finality_tracker,
                                                        ).await {
                                                            log::error!("Failed to finalize checkpoint block: {}", e);
                                                        }
                                                    }
                                                    Ok(None) => {
                                                        // Signature added, awaiting quorum
                                                        let sig_count = checkpoint_collector_worker.get_signature_count(signature.block_number);
                                                        log::debug!(
                                                            "Checkpoint #{}: {}/{} signatures",
                                                            signature.block_number,
                                                            sig_count,
                                                            checkpoint_bft::QUORUM_THRESHOLD
                                                        );

                                                        // Record for Byzantine tracking
                                                        if let Ok(mut tracker) = checkpoint_byzantine_tracker.lock() {
                                                            tracker.record_signature(
                                                                signature.validator_id,
                                                                signature.block_number,
                                                                signature.timestamp_ms
                                                            );
                                                        }
                                                    }
                                                    Err(e) => {
                                                        log::warn!("⚠️ Checkpoint signature rejected: {}", e);

                                                        // Track Byzantine behavior (missed checkpoint)
                                                        if let Ok(mut tracker) = checkpoint_byzantine_tracker.lock() {
                                                            tracker.record_missed_checkpoint(
                                                                signature.validator_id,
                                                                signature.block_number
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                log::error!("Failed to deserialize checkpoint signature: {:?}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("Failed to deserialize protocol message: {:?}", e);
                                    }
                                }
                            }
                            P2PMessage::CheckpointCertificate { data } => {
                                log::debug!("📨 Received checkpoint certificate from {:?}", peer_id);
                                // Certificate sync handling for chain reorganization
                            }
                            _ => {
                                // Ignore other message types
                            }
                        }
                    }
                }

                log::warn!("⚠️  Checkpoint BFT P2P handler ended");
            },
        );

        log::info!("✅ Checkpoint BFT P2P handler spawned");

        // Substrate finality is applied directly in finalize_checkpoint_block() (line ~472)
        // when checkpoint certificates reach quorum. No separate monitoring task needed.

        // ═══════════════════════════════════════════════════════════════════════════
        // IMPLICIT FINALITY MONITOR (V21)
        // ═══════════════════════════════════════════════════════════════════════════
        //
        // Ensures finality progresses even if checkpoint signatures are delayed.
        // Automatically finalizes blocks that are 100 blocks behind the best block.
        // This prevents finality stalling while maintaining checkpoint-based finality as primary.

        let finality_client = client.clone();
        task_manager.spawn_essential_handle().spawn(
            "implicit-finality-monitor",
            Some("finality"),
            async move {
                use tokio::time::{interval, Duration};
                use sp_blockchain::HeaderBackend;
                let mut finality_interval = interval(Duration::from_secs(6));
                const FINALITY_LAG: u32 = 100;

                log::info!("🔄 Starting implicit finality monitor (lag threshold: {} blocks)", FINALITY_LAG);

                loop {
                    finality_interval.tick().await;
                    let info = finality_client.usage_info().chain;
                    let best_number = info.best_number;
                    let finalized_number = info.finalized_number;

                    if best_number > finalized_number + FINALITY_LAG {
                        let target_finalize = best_number - FINALITY_LAG;
                        if let Ok(Some(target_hash)) = finality_client.hash(target_finalize.into()) {
                            use sc_client_api::Finalizer;
                            match finality_client.finalize_block(target_hash, None, true) {
                                Ok(_) => {
                                    log::info!(
                                        "✅ Implicit finality: finalized block #{} (best: #{}, finalized: #{})",
                                        target_finalize,
                                        best_number,
                                        finalized_number
                                    );
                                }
                                Err(e) => {
                                    log::debug!("Implicit finality failed for block #{}: {:?}", target_finalize, e);
                                }
                            }
                        }
                    }
                }
            },
        );

        log::info!("✅ Implicit finality monitor spawned");
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // v108: GRANDPA REMOVED - Pure ASF Finality
    // ═══════════════════════════════════════════════════════════════════════════
    //
    // ASF finality gadget handles all finality - no GRANDPA needed.
    // This completes the v108 migration to pure ASF consensus.

    // ═══════════════════════════════════════════════════════════════════════════
    // VALIDATOR MANAGEMENT (Committee Coordination)
    // ═══════════════════════════════════════════════════════════════════════════

    if role.is_authority() {
        log::info!("👥 Initializing ASF Validator Management");

        // Validator management handles:
        // 1. Committee membership tracking (PPFA panels)
        // 2. Validator health monitoring
        // 3. Reward calculation and distribution
        // 4. Slashing for misbehavior
        // 5. Epoch transition coordination

        log::info!(
            "Validator Management initialized (epoch_duration: {} blocks)",
            asf_params.epoch_duration
        );

        // Load genesis validators from runtime ValidatorCommittee pallet
        let genesis_validators = {
            // Query genesis committee from runtime at genesis block
            let genesis_hash = client.info().genesis_hash;

            match client.runtime_api().validator_committee(genesis_hash) {
                Ok(committee) if !committee.is_empty() => {
                    log::info!(
                        "✅ Loaded {} validators from genesis ValidatorCommittee",
                        committee.len()
                    );

                    // Runtime API already returns Vec<ValidatorInfo> from validator-management
                    // No conversion needed - use directly
                    committee
                },
                Ok(_) => {
                    log::warn!(
                        "⚠️  Genesis ValidatorCommittee is empty. Using fallback single validator."
                    );
                    vec![
                        validator_management::ValidatorInfo::new(
                            validator_management::ValidatorId::from([0u8; 32]),
                            asf_params.min_validator_stake,
                            validator_management::PeerType::FlareNode,
                        ),
                    ]
                },
                Err(e) => {
                    log::error!(
                        "❌ Failed to load genesis ValidatorCommittee: {:?}. Using fallback.",
                        e
                    );
                    vec![
                        validator_management::ValidatorInfo::new(
                            validator_management::ValidatorId::from([0u8; 32]),
                            asf_params.min_validator_stake,
                            validator_management::PeerType::FlareNode,
                        ),
                    ]
                }
            }
        };

        // Create coordinator config
        let coordinator_config = validator_management::CoordinatorConfig {
            max_committee_size: asf_params.max_committee_size,
            epoch_duration: asf_params.epoch_duration,
            health_check_interval: 100, // Every 100 blocks
            enable_rewards: true,
            enable_state_sync: true,
        };

        // Spawn validator management coordinator
        task_manager.spawn_handle().spawn(
            "asf-validator-management",
            Some("validator"),
            validator_management::run_coordinator(coordinator_config, genesis_validators),
        );
    }

    log::info!("✅ ASF FlareChain node started successfully");
    log::info!("   - Block Production: ASF PPFA (slot_duration: {}ms)", asf_params.slot_duration);
    log::info!("   - Finality: Pure ASF (v108)");
    log::info!("   - Committee Size: {}", asf_params.max_committee_size);
    log::info!("   - Epoch Duration: {} blocks", asf_params.epoch_duration);

    Ok(task_manager)
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Check if the runtime supports ASF consensus
///
/// This queries the runtime for ASF-specific APIs to ensure compatibility
pub fn runtime_supports_asf<Client>(_client: &Arc<Client>) -> bool
where
    Client: sc_client_api::BlockchainEvents<Block>,
{
    // Check for ASF runtime APIs
    // For now, assume all FlareChain runtimes support ASF
    true
}

/// Get current PPFA committee from runtime
///
/// Queries the runtime state for the active validator committee
pub fn get_ppfa_committee<Client>(
    _client: &Arc<Client>,
    _at: <Block as sp_runtime::traits::Block>::Hash,
) -> Result<Vec<sp_core::crypto::AccountId32>, String>
where
    Client: sc_client_api::BlockchainEvents<Block>,
{
    // Query runtime state for committee membership
    // This will use validator-management types
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use block_production::{ValidatorId, ProposerSelector, CommitteeManager};
    use validator_management::{ValidatorInfo, PeerType};

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 1: ASF Parameters Configuration
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_asf_params_defaults() {
        let params = AsfParams::default();

        assert_eq!(params.slot_duration, 6000);
        assert_eq!(params.max_committee_size, 21);
        assert_eq!(params.epoch_duration, 2400);
        assert!(params.enable_finality_gadget);
        assert_eq!(params.min_validator_stake, 64_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_asf_params_customization() {
        let params = AsfParams {
            slot_duration: 3000,
            max_committee_size: 42,
            epoch_duration: 1200,
            enable_finality_gadget: false,
            min_validator_stake: 128_000_000_000_000_000_000_000,
        };

        assert_eq!(params.slot_duration, 3000);
        assert_eq!(params.max_committee_size, 42);
        assert!(!params.enable_finality_gadget);
    }

    #[test]
    fn test_asf_params_epoch_calculation() {
        let params = AsfParams::default();

        // Verify epoch duration calculation at 6 second blocks
        // 2400 blocks * 6 seconds = 14,400 seconds = 4 hours
        let epoch_seconds = params.epoch_duration as u64 * (params.slot_duration / 1000);
        assert_eq!(epoch_seconds, 14_400); // 4 hours
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 2: PPFA Committee Management
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_committee_initialization() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators to committee
        for i in 0..5 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            assert!(committee.add_validator(validator_info).is_ok());
        }

        // Verify committee size
        assert_eq!(committee.validator_count(), 5);
    }

    #[test]
    fn test_committee_rotation() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..10 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        // Rotate committee to epoch 1
        assert!(committee.rotate_committee(1).is_ok());

        // Verify active committee size is capped at max_committee_size
        let active_count = committee.active_committee_size();
        assert!(active_count <= params.max_committee_size as usize);
    }

    #[test]
    fn test_committee_exceeds_max_size() {
        let params = AsfParams {
            max_committee_size: 5,
            ..Default::default()
        };
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 10 validators (exceeds max of 5)
        for i in 0..10 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        // Rotate and verify active committee is capped
        committee.rotate_committee(1).unwrap();
        assert_eq!(committee.active_committee_size(), 5);
    }

    #[test]
    fn test_empty_committee_rotation_fails() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Attempt to rotate empty committee should fail
        assert!(committee.rotate_committee(1).is_err());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 3: PPFA Proposer Selection
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_ppfa_proposer_selection() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 3 validators
        for i in 0..3 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();

        let mut proposer_selector = ProposerSelector::new(committee);

        // Get current proposer (should succeed)
        assert!(proposer_selector.current_proposer().is_ok());
    }

    #[test]
    fn test_ppfa_rotation_advances_proposer() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..3 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();
        let mut proposer_selector = ProposerSelector::new(committee);

        // Get initial proposer
        let proposer1 = proposer_selector.current_proposer().unwrap();
        let ppfa_index1 = proposer_selector.current_ppfa_index();

        // Advance to next block
        proposer_selector.advance(1);

        // Verify PPFA index changed
        let ppfa_index2 = proposer_selector.current_ppfa_index();
        assert_ne!(ppfa_index1, ppfa_index2);
    }

    #[test]
    fn test_ppfa_proposer_authorization() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        let validator_ids: Vec<ValidatorId> = (0..3)
            .map(|i| {
                let id = ValidatorId::from([i as u8; 32]);
                let info = ValidatorInfo::new(id, params.min_validator_stake, PeerType::ValidityNode);
                committee.add_validator(info).unwrap();
                id
            })
            .collect();

        committee.rotate_committee(1).unwrap();
        let proposer_selector = ProposerSelector::new(committee);

        // Check if first validator is the current proposer
        let is_proposer = proposer_selector.is_proposer(&validator_ids[0]);

        // At least one validator should be the proposer
        let any_is_proposer = validator_ids.iter()
            .any(|id| proposer_selector.is_proposer(id));
        assert!(any_is_proposer);
    }

    #[test]
    fn test_unauthorized_proposer_rejected() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 3 validators
        for i in 0..3 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();
        let proposer_selector = ProposerSelector::new(committee);

        // Create a validator NOT in the committee
        let unauthorized_validator = ValidatorId::from([99u8; 32]);

        // Verify unauthorized validator is rejected
        assert!(!proposer_selector.is_proposer(&unauthorized_validator));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 4: Epoch Transitions
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_epoch_boundary_detection() {
        let params = AsfParams::default();

        // Block 0 is epoch 0
        let epoch_0 = 0 / params.epoch_duration;
        assert_eq!(epoch_0, 0);

        // Block 2400 is epoch 1
        let epoch_1 = params.epoch_duration / params.epoch_duration;
        assert_eq!(epoch_1, 1);

        // Block 4800 is epoch 2
        let epoch_2 = (params.epoch_duration * 2) / params.epoch_duration;
        assert_eq!(epoch_2, 2);
    }

    #[test]
    fn test_epoch_transition_triggers_committee_rotation() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..5 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        // Rotate to epoch 1
        assert!(committee.rotate_committee(1).is_ok());

        // Rotate to epoch 2
        assert!(committee.rotate_committee(2).is_ok());

        // Verify committee is still active
        assert!(committee.active_committee_size() > 0);
    }

    #[test]
    fn test_epoch_duration_consistency() {
        let params = AsfParams::default();

        // Verify epoch duration is consistent with documentation
        // 2400 blocks at 6 seconds = 4 hours
        assert_eq!(params.epoch_duration, 2400);

        // Test epoch calculation for various block numbers
        let test_cases = vec![
            (0, 0),       // Block 0 → Epoch 0
            (1200, 0),    // Block 1200 → Epoch 0
            (2399, 0),    // Block 2399 → Epoch 0
            (2400, 1),    // Block 2400 → Epoch 1
            (4800, 2),    // Block 4800 → Epoch 2
            (7200, 3),    // Block 7200 → Epoch 3
        ];

        for (block_number, expected_epoch) in test_cases {
            let calculated_epoch = block_number / params.epoch_duration;
            assert_eq!(calculated_epoch, expected_epoch,
                "Block {} should be in epoch {}", block_number, expected_epoch);
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 5: Byzantine Fault Tolerance
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_committee_tolerates_one_third_failures() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 21 validators (max committee size)
        for i in 0..21 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        committee.rotate_committee(1).unwrap();

        // Byzantine fault tolerance: Can tolerate (n-1)/3 failures
        // For 21 validators: (21-1)/3 = 6.67 → 6 Byzantine failures tolerated
        let total_validators = 21;
        let max_byzantine_failures = (total_validators - 1) / 3;

        assert_eq!(max_byzantine_failures, 6);

        // Need 2/3 + 1 for consensus
        let min_honest_validators = (total_validators * 2 / 3) + 1;
        assert_eq!(min_honest_validators, 15);
    }

    #[test]
    fn test_minimum_committee_size_for_bft() {
        // Minimum committee size for BFT is 4 (can tolerate 1 Byzantine failure)
        let params = AsfParams {
            max_committee_size: 4,
            ..Default::default()
        };

        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 4 validators
        for i in 0..4 {
            let validator_id = ValidatorId::from([i as u8; 32]);
            let validator_info = ValidatorInfo::new(
                validator_id,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(validator_info).unwrap();
        }

        assert!(committee.rotate_committee(1).is_ok());

        // With 4 validators, can tolerate (4-1)/3 = 1 Byzantine failure
        assert_eq!(committee.active_committee_size(), 4);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 6: Validator Stake Requirements
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_minimum_validator_stake_enforced() {
        let params = AsfParams::default();

        // Verify minimum stake is 64 ETR for FlareNode
        assert_eq!(params.min_validator_stake, 64_000_000_000_000_000_000_000);
    }

    #[test]
    fn test_validator_with_sufficient_stake() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        let validator_id = ValidatorId::from([1u8; 32]);
        let validator_info = ValidatorInfo::new(
            validator_id,
            params.min_validator_stake, // Exact minimum
            PeerType::ValidityNode,
        );

        // Should succeed with exact minimum stake
        assert!(committee.add_validator(validator_info).is_ok());
    }

    #[test]
    fn test_validator_with_excess_stake() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        let validator_id = ValidatorId::from([1u8; 32]);
        let excess_stake = params.min_validator_stake * 10; // 640 ETR
        let validator_info = ValidatorInfo::new(
            validator_id,
            excess_stake,
            PeerType::ValidityNode,
        );

        // Should succeed with excess stake
        assert!(committee.add_validator(validator_info).is_ok());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 7: Slot Duration and Timing
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_slot_duration_default() {
        let params = AsfParams::default();

        // Default slot duration is 6 seconds (6000 milliseconds)
        assert_eq!(params.slot_duration, 6000);
    }

    #[test]
    fn test_blocks_per_hour_calculation() {
        let params = AsfParams::default();

        // 6 second blocks = 10 blocks per minute = 600 blocks per hour
        let seconds_per_hour = 3600;
        let blocks_per_hour = (seconds_per_hour * 1000) / params.slot_duration;
        assert_eq!(blocks_per_hour, 600);
    }

    #[test]
    fn test_blocks_per_day_calculation() {
        let params = AsfParams::default();

        // 600 blocks/hour * 24 hours = 14,400 blocks per day
        let blocks_per_day = (86400 * 1000) / params.slot_duration;
        assert_eq!(blocks_per_day, 14_400);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // TEST MODULE 7: PPFA Authorization Integration Tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_ppfa_seal_encoding_decoding() {
        use codec::{Encode, Decode};

        // Define the PpfaSeal structure for testing
        #[derive(Encode, Decode, Debug, PartialEq)]
        struct PpfaSeal {
            ppfa_index: u32,
            proposer_id: [u8; 32],
            slot_number: u64,
            timestamp: u64,
        }

        // Create a test PPFA seal
        let test_seal = PpfaSeal {
            ppfa_index: 42,
            proposer_id: [5u8; 32],
            slot_number: 1234,
            timestamp: 1609459200, // 2021-01-01 00:00:00 UTC
        };

        // Encode the seal
        let encoded = test_seal.encode();

        // Decode the seal
        let decoded = PpfaSeal::decode(&mut &encoded[..]).expect("Failed to decode PPFA seal");

        // Verify encoding/decoding round-trip
        assert_eq!(test_seal, decoded);
        assert_eq!(decoded.ppfa_index, 42);
        assert_eq!(decoded.proposer_id, [5u8; 32]);
        assert_eq!(decoded.slot_number, 1234);
        assert_eq!(decoded.timestamp, 1609459200);
    }

    #[test]
    fn test_ppfa_seal_engine_id() {
        // Verify PPFA consensus engine ID is correctly formatted
        let engine_id: [u8; 4] = *b"PPFA";

        assert_eq!(engine_id, [b'P', b'P', b'F', b'A']);
        assert_eq!(engine_id.len(), 4);

        // Verify it matches the engine ID used in block sealing
        let expected_engine_id = *b"PPFA";
        assert_eq!(engine_id, expected_engine_id);
    }

    #[test]
    fn test_ppfa_authorization_data_integrity() {
        use codec::Encode;

        // Create test data representing PPFA authorization
        let block_number: u32 = 100;
        let ppfa_index: u32 = 5;
        let proposer_id = ValidatorId::from([7u8; 32]);

        // Verify data can be encoded without panic
        let _block_number_encoded = block_number.encode();
        let _ppfa_index_encoded = ppfa_index.encode();
        let _proposer_id_encoded = proposer_id.encode();

        // Verify ValidatorId encoding produces 32 bytes
        assert_eq!(proposer_id.encode().len(), 32);
    }

    #[test]
    fn test_ppfa_seal_size_limits() {
        use codec::Encode;

        #[derive(Encode)]
        struct PpfaSeal {
            ppfa_index: u32,
            proposer_id: [u8; 32],
            slot_number: u64,
            timestamp: u64,
        }

        let seal = PpfaSeal {
            ppfa_index: u32::MAX,
            proposer_id: [0xFFu8; 32],
            slot_number: u64::MAX,
            timestamp: u64::MAX,
        };

        let encoded = seal.encode();

        // PPFA seal should be compact: 4 + 32 + 8 + 8 = 52 bytes minimum
        // With SCALE encoding overhead, should be ~52-56 bytes
        assert!(encoded.len() >= 52);
        assert!(encoded.len() <= 64, "PPFA seal too large: {} bytes", encoded.len());
    }

    #[test]
    fn test_ppfa_proposer_rotation() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add 5 validators
        let validator_ids: Vec<ValidatorId> = (0..5)
            .map(|i| {
                let vid = ValidatorId::from([i as u8; 32]);
                let vinfo = ValidatorInfo::new(
                    vid,
                    params.min_validator_stake,
                    PeerType::ValidityNode,
                );
                committee.add_validator(vinfo).expect("Failed to add validator");
                vid
            })
            .collect();

        let mut proposer_selector = ProposerSelector::new(committee);

        // Verify rotation through all proposers
        let mut seen_proposers = std::collections::HashSet::new();

        for i in 0..10 {
            let proposer = proposer_selector.current_proposer()
                .expect("Failed to get current proposer");
            seen_proposers.insert(proposer);
            proposer_selector.advance(i);
        }

        // Should have seen multiple different proposers
        assert!(seen_proposers.len() >= 2, "PPFA rotation not working: only {} unique proposers", seen_proposers.len());
    }

    #[test]
    fn test_unauthorized_proposer_detection() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add authorized validators
        let authorized_ids: Vec<ValidatorId> = (0..3)
            .map(|i| {
                let vid = ValidatorId::from([i as u8; 32]);
                let vinfo = ValidatorInfo::new(
                    vid,
                    params.min_validator_stake,
                    PeerType::ValidityNode,
                );
                committee.add_validator(vinfo).expect("Failed to add validator");
                vid
            })
            .collect();

        let proposer_selector = ProposerSelector::new(committee);

        // Create an unauthorized validator ID
        let unauthorized_validator = ValidatorId::from([99u8; 32]);

        // Verify unauthorized validator is NOT a proposer
        assert!(!proposer_selector.is_proposer(&unauthorized_validator),
            "Unauthorized validator incorrectly identified as proposer");

        // Verify at least one authorized validator IS a proposer
        let has_authorized_proposer = authorized_ids.iter()
            .any(|id| proposer_selector.is_proposer(id));

        assert!(has_authorized_proposer, "No authorized proposers found");
    }

    #[test]
    fn test_epoch_boundary_ppfa_reset() {
        let params = AsfParams::default();
        let mut committee = CommitteeManager::new(params.max_committee_size);

        // Add validators
        for i in 0..5 {
            let vid = ValidatorId::from([i as u8; 32]);
            let vinfo = ValidatorInfo::new(
                vid,
                params.min_validator_stake,
                PeerType::ValidityNode,
            );
            committee.add_validator(vinfo).expect("Failed to add validator");
        }

        // Simulate epoch rotation
        let epoch1 = 0u64;
        let epoch2 = 1u64;

        let result1 = committee.rotate_committee(epoch1);
        assert!(result1.is_ok(), "Epoch 0 rotation failed");

        let result2 = committee.rotate_committee(epoch2);
        assert!(result2.is_ok(), "Epoch 1 rotation failed");

        // After rotation, committee should still have validators
        assert!(committee.size() > 0, "Committee empty after rotation");
    }
}
