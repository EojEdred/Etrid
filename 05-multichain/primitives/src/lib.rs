//! Ëtrid PBC Bridge Pallet - Cross-chain Token Transfer
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, traits::{Currency, ReservableCurrency}};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;
    use codec::{Encode, Decode};
    use scale_info::TypeInfo;

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum TransferStatus {
        Locked,
        Verified,
        Minted,
        Burned,
        Unlocked,
    }

    #[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct CrossChainTransfer<T: Config> {
        pub source_chain: BoundedVec<u8, ConstU32<64>>,
        pub dest_chain: BoundedVec<u8, ConstU32<64>>,
        pub sender: T::AccountId,
        pub recipient: T::AccountId,
        pub amount: BalanceOf<T>,
        pub proof: BoundedVec<u8, ConstU32<256>>,
        pub status: TransferStatus,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: ReservableCurrency<Self::AccountId>;
        type MaxLen: Get<u32>;
    }

    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::storage]
    #[pallet::getter(fn transfers)]
    pub type Transfers<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, CrossChainTransfer<T>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TokensLocked(T::AccountId, T::Hash),
        LockVerified(T::Hash),
        WrappedMinted(T::AccountId, T::Hash),
        WrappedBurned(T::AccountId, T::Hash),
        TokensUnlocked(T::AccountId, T::Hash),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidProof,
        TransferNotFound,
        AlreadyProcessed,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn lock_tokens(
            origin: OriginFor<T>,
            dest_chain: Vec<u8>,
            recipient: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            T::Currency::reserve(&sender, amount)?;

            let source_chain = BoundedVec::<u8, ConstU32<64>>::try_from(b"Etrid".to_vec())
                .map_err(|_| Error::<T>::InvalidProof)?;
            let dest_chain_bounded = BoundedVec::<u8, ConstU32<64>>::try_from(dest_chain)
                .map_err(|_| Error::<T>::InvalidProof)?;

            let transfer = CrossChainTransfer {
                source_chain,
                dest_chain: dest_chain_bounded,
                sender: sender.clone(),
                recipient,
                amount,
                proof: BoundedVec::default(),
                status: TransferStatus::Locked,
            };

            let transfer_bytes = transfer.encode();
            let hash_bytes = sp_io::hashing::blake2_256(&transfer_bytes);
            let hash = T::Hash::decode(&mut &hash_bytes[..])
                .map_err(|_| Error::<T>::InvalidProof)?;
            Transfers::<T>::insert(hash, transfer);

            Self::deposit_event(Event::TokensLocked(sender, hash));
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn verify_lock_proof(
            origin: OriginFor<T>,
            hash: T::Hash,
            proof: Vec<u8>,
        ) -> DispatchResult{
            let _ = ensure_signed(origin)?;

            Transfers::<T>::try_mutate(hash, |maybe_transfer| {
                let transfer = maybe_transfer.as_mut().ok_or(Error::<T>::TransferNotFound)?;
                ensure!(transfer.status == TransferStatus::Locked, Error::<T>::AlreadyProcessed);

                let proof_bounded = BoundedVec::<u8, ConstU32<256>>::try_from(proof)
                    .map_err(|_| Error::<T>::InvalidProof)?;

                // TODO: Merkle proof validation logic
                // For now, we accept the proof as valid
                transfer.proof = proof_bounded;
                transfer.status = TransferStatus::Verified;

                Self::deposit_event(Event::LockVerified(hash));
                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn burn_wrapped_tokens(
            origin: OriginFor<T>,
            hash: T::Hash,
        ) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            Transfers::<T>::try_mutate(hash, |maybe_transfer| {
                let transfer = maybe_transfer.as_mut().ok_or(Error::<T>::TransferNotFound)?;
                ensure!(transfer.recipient == caller, Error::<T>::AlreadyProcessed);
                ensure!(transfer.status == TransferStatus::Minted, Error::<T>::AlreadyProcessed);

                transfer.status = TransferStatus::Burned;
                Self::deposit_event(Event::WrappedBurned(caller, hash));
                Ok(())
            })
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn unlock_tokens(
            origin: OriginFor<T>,
            hash: T::Hash,
        ) -> DispatchResult {
            let _ = ensure_signed(origin)?;

            Transfers::<T>::try_mutate(hash, |maybe_transfer| {
                let transfer = maybe_transfer.as_mut().ok_or(Error::<T>::TransferNotFound)?;
                ensure!(transfer.status == TransferStatus::Burned || transfer.status == TransferStatus::Verified, Error::<T>::AlreadyProcessed);

                T::Currency::unreserve(&transfer.sender, transfer.amount);
                transfer.status = TransferStatus::Unlocked;

                Self::deposit_event(Event::TokensUnlocked(transfer.sender.clone(), hash));
                Ok(())
            })
        }
    }
}
