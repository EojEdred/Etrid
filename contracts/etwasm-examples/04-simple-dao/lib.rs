#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_dao {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    /// Proposal status
    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum ProposalStatus {
        Pending,     // Not yet active (waiting for start time)
        Active,      // Voting in progress
        Approved,    // Reached threshold
        Rejected,    // Did not reach threshold or voting ended
        Executed,    // Approved and executed
    }

    /// Proposal data structure
    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Proposal {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub proposer: AccountId,
        pub yes_votes: u32,
        pub no_votes: u32,
        pub status: ProposalStatus,
        pub created_at: Timestamp,
        pub voting_ends_at: Timestamp,
    }

    /// Vote type
    #[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum VoteType {
        Yes,
        No,
    }

    /// Custom errors
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotMember,
        NotOwner,
        AlreadyMember,
        NotAMember,
        ProposalNotFound,
        ProposalNotActive,
        AlreadyVoted,
        VotingEnded,
        VotingNotEnded,
        ProposalAlreadyExecuted,
        ThresholdNotReached,
        EmptyTitle,
        EmptyDescription,
        InvalidVotingPeriod,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    /// Simple DAO contract storage
    #[ink(storage)]
    pub struct SimpleDao {
        /// Contract owner
        owner: AccountId,
        /// DAO members (address => is_member)
        members: Mapping<AccountId, bool>,
        /// Total member count
        member_count: u32,
        /// Proposals (id => Proposal)
        proposals: Mapping<u32, Proposal>,
        /// Next proposal ID
        next_proposal_id: u32,
        /// Votes (proposal_id => (voter => vote_type))
        votes: Mapping<(u32, AccountId), VoteType>,
        /// Approval threshold percentage (e.g., 51 = 51%)
        threshold_percentage: u8,
        /// Default voting period in milliseconds
        voting_period: u64,
    }

    /// Events
    #[ink(event)]
    pub struct MemberAdded {
        #[ink(topic)]
        member: AccountId,
    }

    #[ink(event)]
    pub struct MemberRemoved {
        #[ink(topic)]
        member: AccountId,
    }

    #[ink(event)]
    pub struct ProposalCreated {
        #[ink(topic)]
        proposal_id: u32,
        #[ink(topic)]
        proposer: AccountId,
        title: String,
    }

    #[ink(event)]
    pub struct VoteCast {
        #[ink(topic)]
        proposal_id: u32,
        #[ink(topic)]
        voter: AccountId,
        vote: VoteType,
    }

    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        proposal_id: u32,
        yes_votes: u32,
        no_votes: u32,
    }

    #[ink(event)]
    pub struct ProposalRejected {
        #[ink(topic)]
        proposal_id: u32,
        yes_votes: u32,
        no_votes: u32,
    }

    impl SimpleDao {
        /// Constructor - creates a new DAO with the caller as owner and first member
        #[ink(constructor)]
        pub fn new(threshold_percentage: u8, voting_period_days: u32) -> Self {
            let caller = Self::env().caller();
            let mut members = Mapping::default();
            members.insert(&caller, &true);

            let voting_period = voting_period_days as u64 * 24 * 60 * 60 * 1000; // Convert days to milliseconds

            Self {
                owner: caller,
                members,
                member_count: 1,
                proposals: Mapping::default(),
                next_proposal_id: 0,
                votes: Mapping::default(),
                threshold_percentage: threshold_percentage.min(100),
                voting_period,
            }
        }

        /// Get contract owner
        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.owner
        }

        /// Get member count
        #[ink(message)]
        pub fn member_count(&self) -> u32 {
            self.member_count
        }

        /// Get threshold percentage
        #[ink(message)]
        pub fn threshold_percentage(&self) -> u8 {
            self.threshold_percentage
        }

        /// Get voting period in milliseconds
        #[ink(message)]
        pub fn voting_period(&self) -> u64 {
            self.voting_period
        }

        /// Check if an address is a member
        #[ink(message)]
        pub fn is_member(&self, account: AccountId) -> bool {
            self.members.get(&account).unwrap_or(false)
        }

        /// Add a new member (owner only)
        #[ink(message)]
        pub fn add_member(&mut self, member: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if self.is_member(member) {
                return Err(Error::AlreadyMember);
            }

            self.members.insert(&member, &true);
            self.member_count += 1;

            self.env().emit_event(MemberAdded { member });
            Ok(())
        }

        /// Remove a member (owner only)
        #[ink(message)]
        pub fn remove_member(&mut self, member: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if !self.is_member(member) {
                return Err(Error::NotAMember);
            }

            self.members.insert(&member, &false);
            self.member_count = self.member_count.saturating_sub(1);

            self.env().emit_event(MemberRemoved { member });
            Ok(())
        }

        /// Create a new proposal (members only)
        #[ink(message)]
        pub fn create_proposal(
            &mut self,
            title: String,
            description: String,
        ) -> Result<u32> {
            let caller = self.env().caller();
            if !self.is_member(caller) {
                return Err(Error::NotMember);
            }

            if title.is_empty() {
                return Err(Error::EmptyTitle);
            }

            if description.is_empty() {
                return Err(Error::EmptyDescription);
            }

            let proposal_id = self.next_proposal_id;
            let now = self.env().block_timestamp();
            let voting_ends_at = now + self.voting_period;

            let proposal = Proposal {
                id: proposal_id,
                title: title.clone(),
                description,
                proposer: caller,
                yes_votes: 0,
                no_votes: 0,
                status: ProposalStatus::Active,
                created_at: now,
                voting_ends_at,
            };

            self.proposals.insert(&proposal_id, &proposal);
            self.next_proposal_id += 1;

            self.env().emit_event(ProposalCreated {
                proposal_id,
                proposer: caller,
                title,
            });

            Ok(proposal_id)
        }

        /// Get a proposal by ID
        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<Proposal> {
            self.proposals.get(&proposal_id)
        }

        /// Get total proposal count
        #[ink(message)]
        pub fn proposal_count(&self) -> u32 {
            self.next_proposal_id
        }

        /// Cast a vote on a proposal (members only)
        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32, vote: VoteType) -> Result<()> {
            let caller = self.env().caller();
            if !self.is_member(caller) {
                return Err(Error::NotMember);
            }

            let mut proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            // Check if proposal is active
            if proposal.status != ProposalStatus::Active {
                return Err(Error::ProposalNotActive);
            }

            // Check if voting period has ended
            let now = self.env().block_timestamp();
            if now > proposal.voting_ends_at {
                return Err(Error::VotingEnded);
            }

            // Check if already voted
            if self.votes.get(&(proposal_id, caller)).is_some() {
                return Err(Error::AlreadyVoted);
            }

            // Record the vote
            self.votes.insert(&(proposal_id, caller), &vote);

            // Update vote count
            match vote {
                VoteType::Yes => proposal.yes_votes += 1,
                VoteType::No => proposal.no_votes += 1,
            }

            self.proposals.insert(&proposal_id, &proposal);

            self.env().emit_event(VoteCast {
                proposal_id,
                voter: caller,
                vote,
            });

            Ok(())
        }

        /// Get a member's vote on a proposal
        #[ink(message)]
        pub fn get_vote(&self, proposal_id: u32, voter: AccountId) -> Option<VoteType> {
            self.votes.get(&(proposal_id, voter))
        }

        /// Execute a proposal after voting period ends
        #[ink(message)]
        pub fn execute_proposal(&mut self, proposal_id: u32) -> Result<()> {
            let mut proposal = self
                .proposals
                .get(&proposal_id)
                .ok_or(Error::ProposalNotFound)?;

            // Check if already executed
            if proposal.status == ProposalStatus::Executed {
                return Err(Error::ProposalAlreadyExecuted);
            }

            // Check if voting period has ended
            let now = self.env().block_timestamp();
            if now <= proposal.voting_ends_at {
                return Err(Error::VotingNotEnded);
            }

            // Calculate if threshold is reached
            let total_votes = proposal.yes_votes + proposal.no_votes;
            let threshold_reached = if total_votes > 0 {
                (proposal.yes_votes * 100) / total_votes >= self.threshold_percentage as u32
            } else {
                false
            };

            if threshold_reached {
                proposal.status = ProposalStatus::Executed;
                self.proposals.insert(&proposal_id, &proposal);

                self.env().emit_event(ProposalExecuted {
                    proposal_id,
                    yes_votes: proposal.yes_votes,
                    no_votes: proposal.no_votes,
                });

                Ok(())
            } else {
                proposal.status = ProposalStatus::Rejected;
                self.proposals.insert(&proposal_id, &proposal);

                self.env().emit_event(ProposalRejected {
                    proposal_id,
                    yes_votes: proposal.yes_votes,
                    no_votes: proposal.no_votes,
                });

                Err(Error::ThresholdNotReached)
            }
        }

        /// Change threshold percentage (owner only)
        #[ink(message)]
        pub fn set_threshold(&mut self, new_threshold: u8) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.threshold_percentage = new_threshold.min(100);
            Ok(())
        }

        /// Change voting period (owner only)
        #[ink(message)]
        pub fn set_voting_period(&mut self, days: u32) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            if days == 0 {
                return Err(Error::InvalidVotingPeriod);
            }

            self.voting_period = days as u64 * 24 * 60 * 60 * 1000;
            Ok(())
        }

        /// Transfer ownership (owner only)
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) -> Result<()> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(Error::NotOwner);
            }

            self.owner = new_owner;

            // Ensure new owner is a member
            if !self.is_member(new_owner) {
                self.members.insert(&new_owner, &true);
                self.member_count += 1;
                self.env().emit_event(MemberAdded { member: new_owner });
            }

            Ok(())
        }
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn new_works() {
            let dao = SimpleDao::new(51, 7);
            assert_eq!(dao.threshold_percentage(), 51);
            assert_eq!(dao.voting_period(), 7 * 24 * 60 * 60 * 1000);
            assert_eq!(dao.member_count(), 1);
            assert!(dao.is_member(dao.owner()));
        }

        #[ink::test]
        fn add_member_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            assert!(!dao.is_member(accounts.bob));
            assert!(dao.add_member(accounts.bob).is_ok());
            assert!(dao.is_member(accounts.bob));
            assert_eq!(dao.member_count(), 2);
        }

        #[ink::test]
        fn add_member_already_member_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            assert!(dao.add_member(accounts.bob).is_ok());
            assert_eq!(dao.add_member(accounts.bob), Err(Error::AlreadyMember));
        }

        #[ink::test]
        fn add_member_not_owner_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            // Change caller to Bob (not owner)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(dao.add_member(accounts.charlie), Err(Error::NotOwner));
        }

        #[ink::test]
        fn remove_member_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            assert!(dao.add_member(accounts.bob).is_ok());
            assert_eq!(dao.member_count(), 2);

            assert!(dao.remove_member(accounts.bob).is_ok());
            assert!(!dao.is_member(accounts.bob));
            assert_eq!(dao.member_count(), 1);
        }

        #[ink::test]
        fn remove_member_not_a_member_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            assert_eq!(dao.remove_member(accounts.bob), Err(Error::NotAMember));
        }

        #[ink::test]
        fn create_proposal_works() {
            let mut dao = SimpleDao::new(51, 7);

            let proposal_id = dao
                .create_proposal("Test Proposal".into(), "Test Description".into())
                .unwrap();

            assert_eq!(proposal_id, 0);
            assert_eq!(dao.proposal_count(), 1);

            let proposal = dao.get_proposal(proposal_id).unwrap();
            assert_eq!(proposal.title, "Test Proposal");
            assert_eq!(proposal.status, ProposalStatus::Active);
            assert_eq!(proposal.yes_votes, 0);
            assert_eq!(proposal.no_votes, 0);
        }

        #[ink::test]
        fn create_proposal_not_member_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            // Change caller to Bob (not a member)
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

            let result = dao.create_proposal("Test".into(), "Description".into());
            assert_eq!(result, Err(Error::NotMember));
        }

        #[ink::test]
        fn create_proposal_empty_title_fails() {
            let mut dao = SimpleDao::new(51, 7);

            let result = dao.create_proposal("".into(), "Description".into());
            assert_eq!(result, Err(Error::EmptyTitle));
        }

        #[ink::test]
        fn vote_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            // Add Bob as member
            dao.add_member(accounts.bob).unwrap();

            // Create proposal
            let proposal_id = dao
                .create_proposal("Test".into(), "Description".into())
                .unwrap();

            // Alice votes yes
            assert!(dao.vote(proposal_id, VoteType::Yes).is_ok());

            // Bob votes no
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert!(dao.vote(proposal_id, VoteType::No).is_ok());

            // Check votes
            let proposal = dao.get_proposal(proposal_id).unwrap();
            assert_eq!(proposal.yes_votes, 1);
            assert_eq!(proposal.no_votes, 1);
        }

        #[ink::test]
        fn vote_already_voted_fails() {
            let mut dao = SimpleDao::new(51, 7);

            let proposal_id = dao
                .create_proposal("Test".into(), "Description".into())
                .unwrap();

            assert!(dao.vote(proposal_id, VoteType::Yes).is_ok());
            assert_eq!(dao.vote(proposal_id, VoteType::Yes), Err(Error::AlreadyVoted));
        }

        #[ink::test]
        fn vote_not_member_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            let proposal_id = dao
                .create_proposal("Test".into(), "Description".into())
                .unwrap();

            // Bob is not a member
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(dao.vote(proposal_id, VoteType::Yes), Err(Error::NotMember));
        }

        #[ink::test]
        fn execute_proposal_threshold_reached() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            // Add members
            dao.add_member(accounts.bob).unwrap();
            dao.add_member(accounts.charlie).unwrap();

            // Create proposal
            let proposal_id = dao
                .create_proposal("Test".into(), "Description".into())
                .unwrap();

            // Vote: 2 yes, 1 no (66% yes)
            dao.vote(proposal_id, VoteType::Yes).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            dao.vote(proposal_id, VoteType::Yes).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            dao.vote(proposal_id, VoteType::No).unwrap();

            // Advance time past voting period
            ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(8 * 24 * 60 * 60 * 1000);

            // Execute
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.execute_proposal(proposal_id).is_ok());

            let proposal = dao.get_proposal(proposal_id).unwrap();
            assert_eq!(proposal.status, ProposalStatus::Executed);
        }

        #[ink::test]
        fn execute_proposal_threshold_not_reached() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            // Add members
            dao.add_member(accounts.bob).unwrap();
            dao.add_member(accounts.charlie).unwrap();

            // Create proposal
            let proposal_id = dao
                .create_proposal("Test".into(), "Description".into())
                .unwrap();

            // Vote: 1 yes, 2 no (33% yes)
            dao.vote(proposal_id, VoteType::Yes).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            dao.vote(proposal_id, VoteType::No).unwrap();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
            dao.vote(proposal_id, VoteType::No).unwrap();

            // Advance time
            ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
            ink::env::test::set_block_timestamp::<ink::env::DefaultEnvironment>(8 * 24 * 60 * 60 * 1000);

            // Execute should fail
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert_eq!(dao.execute_proposal(proposal_id), Err(Error::ThresholdNotReached));

            let proposal = dao.get_proposal(proposal_id).unwrap();
            assert_eq!(proposal.status, ProposalStatus::Rejected);
        }

        #[ink::test]
        fn set_threshold_works() {
            let mut dao = SimpleDao::new(51, 7);
            assert_eq!(dao.threshold_percentage(), 51);

            assert!(dao.set_threshold(75).is_ok());
            assert_eq!(dao.threshold_percentage(), 75);
        }

        #[ink::test]
        fn set_threshold_not_owner_fails() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            assert_eq!(dao.set_threshold(75), Err(Error::NotOwner));
        }

        #[ink::test]
        fn set_voting_period_works() {
            let mut dao = SimpleDao::new(51, 7);
            assert_eq!(dao.voting_period(), 7 * 24 * 60 * 60 * 1000);

            assert!(dao.set_voting_period(14).is_ok());
            assert_eq!(dao.voting_period(), 14 * 24 * 60 * 60 * 1000);
        }

        #[ink::test]
        fn transfer_ownership_works() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut dao = SimpleDao::new(51, 7);

            assert_eq!(dao.owner(), accounts.alice);
            assert!(dao.transfer_ownership(accounts.bob).is_ok());
            assert_eq!(dao.owner(), accounts.bob);
            assert!(dao.is_member(accounts.bob));
        }
    }

    /// E2E tests
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_dao_lifecycle(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Deploy contract
            let constructor = SimpleDaoRef::new(51, 7);
            let contract_account_id = client
                .instantiate("simple_dao", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Create proposal
            let create = build_message::<SimpleDaoRef>(contract_account_id.clone())
                .call(|dao| dao.create_proposal("Test Proposal".into(), "Test Description".into()));

            let create_result = client
                .call(&ink_e2e::alice(), create, 0, None)
                .await
                .expect("create_proposal failed");

            assert!(create_result.return_value().is_ok());

            // Vote on proposal
            let vote = build_message::<SimpleDaoRef>(contract_account_id.clone())
                .call(|dao| dao.vote(0, VoteType::Yes));

            let vote_result = client
                .call(&ink_e2e::alice(), vote, 0, None)
                .await
                .expect("vote failed");

            assert!(vote_result.return_value().is_ok());

            Ok(())
        }
    }
}
