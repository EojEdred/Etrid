#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod nft_erc721 {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    /// NFT contract storage
    #[ink(storage)]
    pub struct NftErc721 {
        /// Token name
        name: String,
        /// Token symbol
        symbol: String,
        /// Mapping from token ID to owner
        token_owner: Mapping<u32, AccountId>,
        /// Mapping from token ID to approved address
        token_approvals: Mapping<u32, AccountId>,
        /// Mapping from owner to number of owned tokens
        owned_tokens_count: Mapping<AccountId, u32>,
        /// Mapping from owner to operator approvals
        operator_approvals: Mapping<(AccountId, AccountId), bool>,
        /// Mapping from token ID to token URI
        token_uris: Mapping<u32, String>,
        /// Total supply of tokens
        total_supply: u32,
        /// Contract owner (can mint)
        owner: AccountId,
    }

    /// Events
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        token_id: u32,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        approved: AccountId,
        #[ink(topic)]
        token_id: u32,
    }

    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    /// Errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        NotTokenOwner,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
        TransferToZeroAddress,
        EmptyTokenUri,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl NftErc721 {
        /// Constructor
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            let caller = Self::env().caller();
            Self {
                name,
                symbol,
                token_owner: Mapping::default(),
                token_approvals: Mapping::default(),
                owned_tokens_count: Mapping::default(),
                operator_approvals: Mapping::default(),
                token_uris: Mapping::default(),
                total_supply: 0,
                owner: caller,
            }
        }

        /// Get contract name
        #[ink(message)]
        pub fn name(&self) -> String {
            self.name.clone()
        }

        /// Get contract symbol
        #[ink(message)]
        pub fn symbol(&self) -> String {
            self.symbol.clone()
        }

        /// Get total supply
        #[ink(message)]
        pub fn total_supply(&self) -> u32 {
            self.total_supply
        }

        /// Get contract owner
        #[ink(message)]
        pub fn contract_owner(&self) -> AccountId {
            self.owner
        }

        /// Get balance of owner
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.owned_tokens_count.get(&owner).unwrap_or(0)
        }

        /// Get owner of token
        #[ink(message)]
        pub fn owner_of(&self, token_id: u32) -> Option<AccountId> {
            self.token_owner.get(&token_id)
        }

        /// Get approved address for token
        #[ink(message)]
        pub fn get_approved(&self, token_id: u32) -> Option<AccountId> {
            self.token_approvals.get(&token_id)
        }

        /// Check if operator is approved for owner
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.operator_approvals.get(&(owner, operator)).unwrap_or(false)
        }

        /// Get token URI
        #[ink(message)]
        pub fn token_uri(&self, token_id: u32) -> Option<String> {
            self.token_uris.get(&token_id)
        }

        /// Approve address to transfer token
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, token_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.owner_of(token_id).ok_or(Error::TokenNotFound)?;

            if caller != owner && !self.is_approved_for_all(owner, caller) {
                return Err(Error::NotAllowed);
            }

            if to == owner {
                return Err(Error::NotAllowed);
            }

            self.token_approvals.insert(&token_id, &to);

            self.env().emit_event(Approval {
                owner,
                approved: to,
                token_id,
            });

            Ok(())
        }

        /// Set approval for all tokens
        #[ink(message)]
        pub fn set_approval_for_all(&mut self, operator: AccountId, approved: bool) -> Result<()> {
            let caller = self.env().caller();

            if caller == operator {
                return Err(Error::NotAllowed);
            }

            self.operator_approvals.insert(&(caller, operator), &approved);

            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator,
                approved,
            });

            Ok(())
        }

        /// Transfer token from one account to another
        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u32) -> Result<()> {
            let caller = self.env().caller();

            if !self.is_approved_or_owner(caller, token_id) {
                return Err(Error::NotApproved);
            }

            self.transfer_token_from(from, to, token_id)?;
            Ok(())
        }

        /// Safe transfer (same as transfer_from for basic implementation)
        #[ink(message)]
        pub fn safe_transfer_from(&mut self, from: AccountId, to: AccountId, token_id: u32) -> Result<()> {
            self.transfer_from(from, to, token_id)
        }

        /// Mint new NFT (owner only)
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, token_id: u32, token_uri: String) -> Result<()> {
            let caller = self.env().caller();

            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if token_uri.is_empty() {
                return Err(Error::EmptyTokenUri);
            }

            self.add_token_to(to, token_id)?;
            self.token_uris.insert(&token_id, &token_uri);
            self.total_supply += 1;

            self.env().emit_event(Transfer {
                from: None,
                to: Some(to),
                token_id,
            });

            Ok(())
        }

        /// Burn NFT
        #[ink(message)]
        pub fn burn(&mut self, token_id: u32) -> Result<()> {
            let caller = self.env().caller();
            let owner = self.owner_of(token_id).ok_or(Error::TokenNotFound)?;

            if caller != owner {
                return Err(Error::NotTokenOwner);
            }

            self.remove_token_from(owner, token_id)?;
            self.token_approvals.remove(&token_id);
            self.token_uris.remove(&token_id);
            self.total_supply -= 1;

            self.env().emit_event(Transfer {
                from: Some(owner),
                to: None,
                token_id,
            });

            Ok(())
        }

        /// Transfer ownership of contract
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();

            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.owner = new_owner;
            Ok(())
        }

        // Internal functions

        fn transfer_token_from(&mut self, from: AccountId, to: AccountId, token_id: u32) -> Result<()> {
            let owner = self.owner_of(token_id).ok_or(Error::TokenNotFound)?;

            if owner != from {
                return Err(Error::NotTokenOwner);
            }

            if to == AccountId::from([0x0; 32]) {
                return Err(Error::TransferToZeroAddress);
            }

            self.clear_approval(token_id);
            self.remove_token_from(from, token_id)?;
            self.add_token_to(to, token_id)?;

            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                token_id,
            });

            Ok(())
        }

        fn clear_approval(&mut self, token_id: u32) {
            self.token_approvals.remove(&token_id);
        }

        fn add_token_to(&mut self, to: AccountId, token_id: u32) -> Result<()> {
            if self.token_owner.contains(&token_id) {
                return Err(Error::TokenExists);
            }

            let count = self.owned_tokens_count.get(&to).unwrap_or(0);
            self.owned_tokens_count.insert(&to, &(count + 1));
            self.token_owner.insert(&token_id, &to);

            Ok(())
        }

        fn remove_token_from(&mut self, from: AccountId, token_id: u32) -> Result<()> {
            if !self.token_owner.contains(&token_id) {
                return Err(Error::TokenNotFound);
            }

            let count = self.owned_tokens_count.get(&from).unwrap_or(0);
            let new_count = count.checked_sub(1).unwrap_or(0);
            self.owned_tokens_count.insert(&from, &new_count);
            self.token_owner.remove(&token_id);

            Ok(())
        }

        fn is_approved_or_owner(&self, spender: AccountId, token_id: u32) -> bool {
            let owner = match self.owner_of(token_id) {
                Some(o) => o,
                None => return false,
            };

            if spender == owner {
                return true;
            }

            if let Some(approved) = self.get_approved(token_id) {
                if approved == spender {
                    return true;
                }
            }

            self.is_approved_for_all(owner, spender)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn get_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<ink::env::DefaultEnvironment>()
        }

        fn set_caller(account: AccountId) {
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(account);
        }

        #[ink::test]
        fn new_works() {
            let nft = NftErc721::new("My NFT".into(), "MNFT".into());
            assert_eq!(nft.name(), "My NFT");
            assert_eq!(nft.symbol(), "MNFT");
            assert_eq!(nft.total_supply(), 0);
        }

        #[ink::test]
        fn mint_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            assert!(nft.mint(accounts.alice, 1, "ipfs://token1".into()).is_ok());
            assert_eq!(nft.owner_of(1), Some(accounts.alice));
            assert_eq!(nft.balance_of(accounts.alice), 1);
            assert_eq!(nft.total_supply(), 1);
            assert_eq!(nft.token_uri(1), Some("ipfs://token1".into()));
        }

        #[ink::test]
        fn mint_fails_for_non_owner() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            set_caller(accounts.bob);
            assert_eq!(nft.mint(accounts.bob, 1, "ipfs://token1".into()), Err(Error::NotOwner));
        }

        #[ink::test]
        fn mint_fails_duplicate_token() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            assert!(nft.mint(accounts.alice, 1, "ipfs://token1".into()).is_ok());
            assert_eq!(nft.mint(accounts.bob, 1, "ipfs://token2".into()), Err(Error::TokenExists));
        }

        #[ink::test]
        fn mint_fails_empty_uri() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            assert_eq!(nft.mint(accounts.alice, 1, "".into()), Err(Error::EmptyTokenUri));
        }

        #[ink::test]
        fn transfer_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();

            assert!(nft.transfer_from(accounts.alice, accounts.bob, 1).is_ok());
            assert_eq!(nft.owner_of(1), Some(accounts.bob));
            assert_eq!(nft.balance_of(accounts.alice), 0);
            assert_eq!(nft.balance_of(accounts.bob), 1);
        }

        #[ink::test]
        fn transfer_fails_not_owner() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();

            set_caller(accounts.bob);
            assert_eq!(
                nft.transfer_from(accounts.alice, accounts.bob, 1),
                Err(Error::NotApproved)
            );
        }

        #[ink::test]
        fn approve_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();

            assert!(nft.approve(accounts.bob, 1).is_ok());
            assert_eq!(nft.get_approved(1), Some(accounts.bob));
        }

        #[ink::test]
        fn approved_transfer_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();
            nft.approve(accounts.bob, 1).unwrap();

            set_caller(accounts.bob);
            assert!(nft.transfer_from(accounts.alice, accounts.charlie, 1).is_ok());
            assert_eq!(nft.owner_of(1), Some(accounts.charlie));
        }

        #[ink::test]
        fn set_approval_for_all_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            assert!(nft.set_approval_for_all(accounts.bob, true).is_ok());
            assert!(nft.is_approved_for_all(accounts.alice, accounts.bob));

            assert!(nft.set_approval_for_all(accounts.bob, false).is_ok());
            assert!(!nft.is_approved_for_all(accounts.alice, accounts.bob));
        }

        #[ink::test]
        fn operator_transfer_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();
            nft.set_approval_for_all(accounts.bob, true).unwrap();

            set_caller(accounts.bob);
            assert!(nft.transfer_from(accounts.alice, accounts.charlie, 1).is_ok());
            assert_eq!(nft.owner_of(1), Some(accounts.charlie));
        }

        #[ink::test]
        fn burn_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();

            assert!(nft.burn(1).is_ok());
            assert_eq!(nft.owner_of(1), None);
            assert_eq!(nft.balance_of(accounts.alice), 0);
            assert_eq!(nft.total_supply(), 0);
        }

        #[ink::test]
        fn burn_fails_not_owner() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();

            set_caller(accounts.bob);
            assert_eq!(nft.burn(1), Err(Error::NotTokenOwner));
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            assert_eq!(nft.contract_owner(), accounts.alice);
            assert!(nft.transfer_ownership(accounts.bob).is_ok());
            assert_eq!(nft.contract_owner(), accounts.bob);
        }

        #[ink::test]
        fn multiple_tokens_per_owner() {
            let accounts = get_accounts();
            let mut nft = NftErc721::new("My NFT".into(), "MNFT".into());

            nft.mint(accounts.alice, 1, "ipfs://token1".into()).unwrap();
            nft.mint(accounts.alice, 2, "ipfs://token2".into()).unwrap();
            nft.mint(accounts.alice, 3, "ipfs://token3".into()).unwrap();

            assert_eq!(nft.balance_of(accounts.alice), 3);
            assert_eq!(nft.total_supply(), 3);
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_nft_lifecycle(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Deploy
            let constructor = NftErc721Ref::new("Test NFT".into(), "TNFT".into());
            let contract_account_id = client
                .instantiate("nft_erc721", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Mint
            let mint = build_message::<NftErc721Ref>(contract_account_id.clone())
                .call(|nft| nft.mint(
                    ink_e2e::account_id(ink_e2e::AccountKeyring::Alice),
                    1,
                    "ipfs://test".into()
                ));

            let mint_result = client
                .call(&ink_e2e::alice(), mint, 0, None)
                .await
                .expect("mint failed");

            assert!(mint_result.return_value().is_ok());

            Ok(())
        }
    }
}
