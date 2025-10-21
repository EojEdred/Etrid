//! Common pallet configurations and runtime construction macros
//!
//! This module provides macros to generate all the boilerplate configuration
//! that is identical across all PBCs.

/// Common parameter types used across all PBCs
#[macro_export]
macro_rules! pbc_parameter_types {
    ($runtime:ty, $version:expr) => {
        parameter_types! {
            pub const BlockHashCount: BlockNumber = 2400;
            pub const Version: RuntimeVersion = $version;
            pub BlockWeights: frame_system::limits::BlockWeights =
                frame_system::limits::BlockWeights::simple_max(
                    Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2), u64::MAX),
                );
            pub BlockLength: frame_system::limits::BlockLength =
                frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, 75);
            pub const SS58Prefix: u8 = 42;
            pub const ExistentialDeposit: Balance = 1_000_000_000; // 0.001 ÉTR
            pub const MaxLocks: u32 = 50;
            pub const MaxReserves: u32 = 50;
        }
    };
}

/// Implement all common pallet configurations
///
/// This macro generates `Config` implementations for all standard pallets
/// that are identical across PBCs.
#[macro_export]
macro_rules! impl_common_pallet_configs {
    ($runtime:ty) => {
        // System
        impl frame_system::Config for $runtime {
            type BaseCallFilter = frame_support::traits::Everything;
            type BlockWeights = BlockWeights;
            type BlockLength = BlockLength;
            type RuntimeOrigin = RuntimeOrigin;
            type RuntimeCall = RuntimeCall;
            type Nonce = Nonce;
            type Hash = Hash;
            type Hashing = BlakeTwo256;
            type AccountId = AccountId;
            type Lookup = AccountIdLookup<AccountId, ()>;
            type Block = Block;
            type RuntimeEvent = RuntimeEvent;
            type BlockHashCount = BlockHashCount;
            type DbWeight = RocksDbWeight;
            type Version = Version;
            type PalletInfo = PalletInfo;
            type AccountData = pallet_balances::AccountData<Balance>;
            type OnNewAccount = ();
            type OnKilledAccount = ();
            type SystemWeightInfo = ();
            type SS58Prefix = SS58Prefix;
            type OnSetCode = ();
            type MaxConsumers = ConstU32<16>;
        }

        // Timestamp
        impl pallet_timestamp::Config for $runtime {
            type Moment = Moment;
            type OnTimestampSet = ();
            type MinimumPeriod = ConstU64<3000>;
            type WeightInfo = ();
        }

        // Randomness
        impl pallet_insecure_randomness_collective_flip::Config for $runtime {}

        // Grandpa (finality)
        impl pallet_grandpa::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type WeightInfo = ();
            type MaxAuthorities = ConstU32<32>;
            type MaxNominators = ConstU32<0>;
            type MaxSetIdSessionEntries = ConstU64<0>;
            type KeyOwnerProof = sp_core::Void;
            type EquivocationReportSystem = ();
        }

        // Balances
        impl pallet_balances::Config for $runtime {
            type MaxLocks = MaxLocks;
            type MaxReserves = MaxReserves;
            type ReserveIdentifier = [u8; 8];
            type Balance = Balance;
            type RuntimeEvent = RuntimeEvent;
            type DustRemoval = ();
            type ExistentialDeposit = ExistentialDeposit;
            type AccountStore = System;
            type WeightInfo = ();
            type RuntimeHoldReason = ();
            type FreezeIdentifier = ();
            type MaxHolds = ConstU32<0>;
            type MaxFreezes = ConstU32<0>;
        }

        // Transaction Payment
        impl pallet_transaction_payment::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
            type OperationalFeeMultiplier = ConstU8<5>;
            type WeightToFee = IdentityFee<Balance>;
            type LengthToFee = IdentityFee<Balance>;
            type FeeMultiplierUpdate = ConstFeeMultiplier<Multiplier>;
        }

        // Sudo
        impl pallet_sudo::Config for $runtime {
            type RuntimeEvent = RuntimeEvent;
            type RuntimeCall = RuntimeCall;
            type WeightInfo = ();
        }
    };
}

/// Construct PBC runtime with common pallets + bridge pallet
///
/// # Example
/// ```rust,ignore
/// construct_pbc_runtime!(
///     Runtime,
///     BitcoinBridge: pallet_bitcoin_bridge
/// );
/// ```
#[macro_export]
macro_rules! construct_pbc_runtime {
    (
        $runtime_name:ident,
        $bridge_pallet_name:ident: $bridge_pallet_path:path
    ) => {
        frame_support::construct_runtime!(
            pub enum $runtime_name {
                // Core FRAME pallets
                System: frame_system,
                RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip,
                Timestamp: pallet_timestamp,
                Grandpa: pallet_grandpa,
                Balances: pallet_balances,
                TransactionPayment: pallet_transaction_payment,
                Sudo: pallet_sudo,

                // Ëtrid pallets
                Consensus: pallet_consensus,

                // Bridge pallet (PBC-specific)
                $bridge_pallet_name: $bridge_pallet_path,
                LightningChannels: pallet_lightning_channels,
            }
        );
    };
}

/// Implement all common runtime APIs
///
/// This generates implementations for all standard Substrate runtime APIs
/// that are identical across PBCs.
#[macro_export]
macro_rules! impl_pbc_runtime_apis {
    ($runtime:ty, $block:ty) => {
        impl_runtime_apis! {
            impl sp_api::Core<$block> for $runtime {
                fn version() -> RuntimeVersion {
                    VERSION
                }

                fn execute_block(block: $block) {
                    Executive::execute_block(block);
                }

                fn initialize_block(header: &<$block as BlockT>::Header) {
                    Executive::initialize_block(header)
                }
            }

            impl sp_api::Metadata<$block> for $runtime {
                fn metadata() -> OpaqueMetadata {
                    OpaqueMetadata::new(Runtime::metadata().into())
                }

                fn metadata_at_version(version: u32) -> Option<OpaqueMetadata> {
                    Runtime::metadata_at_version(version)
                }

                fn metadata_versions() -> sp_std::vec::Vec<u32> {
                    Runtime::metadata_versions()
                }
            }

            impl sp_block_builder::BlockBuilder<$block> for $runtime {
                fn apply_extrinsic(extrinsic: <$block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
                    Executive::apply_extrinsic(extrinsic)
                }

                fn finalize_block() -> <$block as BlockT>::Header {
                    Executive::finalize_block()
                }

                fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<$block as BlockT>::Extrinsic> {
                    data.create_extrinsics()
                }

                fn check_inherents(
                    block: $block,
                    data: sp_inherents::InherentData,
                ) -> sp_inherents::CheckInherentsResult {
                    data.check_extrinsics(&block)
                }
            }

            impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<$block> for $runtime {
                fn validate_transaction(
                    source: TransactionSource,
                    tx: <$block as BlockT>::Extrinsic,
                    block_hash: <$block as BlockT>::Hash,
                ) -> TransactionValidity {
                    Executive::validate_transaction(source, tx, block_hash)
                }
            }

            impl sp_offchain::OffchainWorkerApi<$block> for $runtime {
                fn offchain_worker(header: &<$block as BlockT>::Header) {
                    Executive::offchain_worker(header)
                }
            }

            impl sp_session::SessionKeys<$block> for $runtime {
                fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
                    opaque::SessionKeys::generate(seed)
                }

                fn decode_session_keys(
                    encoded: Vec<u8>,
                ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
                    opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
                }
            }

            impl sp_consensus_asf::AsfApi<$block, AccountId> for $runtime {
                fn committee() -> Vec<AccountId> {
                    Consensus::committee()
                }

                fn ppfa_index() -> u32 {
                    Consensus::ppfa_index()
                }

                fn slot_duration() -> sp_consensus_asf::SlotDuration {
                    sp_consensus_asf::SlotDuration::from_millis(Consensus::slot_duration())
                }

                fn should_propose(validator: AccountId) -> bool {
                    Consensus::should_propose(validator)
                }

                fn current_epoch() -> u32 {
                    Consensus::current_epoch()
                }

                fn active_validators() -> Vec<AccountId> {
                    Consensus::active_validators()
                }
            }

            impl sp_consensus_grandpa::GrandpaApi<$block> for $runtime {
                fn grandpa_authorities() -> sp_consensus_grandpa::AuthorityList {
                    Grandpa::grandpa_authorities()
                }

                fn current_set_id() -> sp_consensus_grandpa::SetId {
                    Grandpa::current_set_id()
                }

                fn submit_report_equivocation_unsigned_extrinsic(
                    _equivocation_proof: sp_consensus_grandpa::EquivocationProof<
                        <$block as BlockT>::Hash,
                        NumberFor<$block>,
                    >,
                    _key_owner_proof: sp_consensus_grandpa::OpaqueKeyOwnershipProof,
                ) -> Option<()> {
                    None
                }

                fn generate_key_ownership_proof(
                    _set_id: sp_consensus_grandpa::SetId,
                    _authority_id: sp_consensus_grandpa::AuthorityId,
                ) -> Option<sp_consensus_grandpa::OpaqueKeyOwnershipProof> {
                    None
                }
            }

            impl frame_system_rpc_runtime_api::AccountNonceApi<$block, AccountId, Nonce> for $runtime {
                fn account_nonce(account: AccountId) -> Nonce {
                    System::account_nonce(account)
                }
            }

            impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<$block, Balance> for $runtime {
                fn query_info(
                    uxt: <$block as BlockT>::Extrinsic,
                    len: u32,
                ) -> RuntimeDispatchInfo<Balance> {
                    TransactionPayment::query_info(uxt, len)
                }

                fn query_fee_details(
                    uxt: <$block as BlockT>::Extrinsic,
                    len: u32,
                ) -> FeeDetails<Balance> {
                    TransactionPayment::query_fee_details(uxt, len)
                }

                fn query_weight_to_fee(weight: Weight) -> Balance {
                    TransactionPayment::weight_to_fee(weight)
                }

                fn query_length_to_fee(length: u32) -> Balance {
                    TransactionPayment::length_to_fee(length)
                }
            }
        }
    };
}
