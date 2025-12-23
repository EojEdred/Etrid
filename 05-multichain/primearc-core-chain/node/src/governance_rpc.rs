//! Governance RPC Endpoints
//!
//! Provides RPC methods for querying ËTRID governance and Consensus Day state.
//! Used by wallets, block explorers, and governance UIs.

use jsonrpsee::{
    core::{async_trait, RpcResult},
    proc_macros::rpc,
    types::error::{ErrorCode, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;
#[allow(unused_imports)]
use codec::{Decode, Encode};
use sp_core::crypto::Ss58Codec;
use serde::{Deserialize, Serialize};

pub use governance_runtime_api::GovernanceApi as GovernanceRuntimeApi;

// ═══════════════════════════════════════════════════════════════════════════════
// RPC TRAIT DEFINITION
// ═══════════════════════════════════════════════════════════════════════════════

/// Governance RPC API
#[rpc(server, client, namespace = "governance")]
pub trait GovernanceApi<BlockHash, AccountId> {
    /// Get all governance proposals
    #[method(name = "getProposals")]
    async fn get_proposals(&self) -> RpcResult<Vec<RpcProposal>>;

    /// Get a specific proposal by ID
    #[method(name = "getProposal")]
    async fn get_proposal(&self, id: u64) -> RpcResult<Option<RpcProposal>>;

    /// Get current Consensus Day phase
    #[method(name = "getPhase")]
    async fn get_phase(&self) -> RpcResult<RpcConsensusDayPhase>;

    /// Check if Consensus Day is currently active
    #[method(name = "isActive")]
    async fn is_active(&self) -> RpcResult<bool>;

    /// Get voting power for an account
    #[method(name = "getVotingPower")]
    async fn get_voting_power(&self, account: String) -> RpcResult<RpcVotingPower>;

    /// Get vote counts for a proposal
    #[method(name = "getVoteCounts")]
    async fn get_vote_counts(&self, proposal_id: u64) -> RpcResult<RpcVoteCounts>;

    /// Get all votes for a proposal
    #[method(name = "getVotes")]
    async fn get_votes(&self, proposal_id: u64) -> RpcResult<Vec<RpcVote>>;

    /// Get elected directors
    #[method(name = "getDirectors")]
    async fn get_directors(&self) -> RpcResult<Vec<String>>;

    /// Get director candidates
    #[method(name = "getCandidates")]
    async fn get_candidates(&self) -> RpcResult<Vec<RpcDirectorCandidate>>;

    /// Get distribution information
    #[method(name = "getDistribution")]
    async fn get_distribution(&self) -> RpcResult<RpcDistribution>;

    /// Get governance statistics
    #[method(name = "getStats")]
    async fn get_stats(&self) -> RpcResult<RpcGovernanceStats>;

    /// Get pending participation reward for account
    #[method(name = "getReward")]
    async fn get_reward(&self, account: String) -> RpcResult<String>;

    /// Get current inflation rate
    #[method(name = "getInflationRate")]
    async fn get_inflation_rate(&self) -> RpcResult<u32>;
}

// ═══════════════════════════════════════════════════════════════════════════════
// RPC RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

/// Proposal info for RPC responses
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcProposal {
    pub id: u64,
    pub proposer: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub status: String,
    pub votes_for: String,
    pub votes_against: String,
    pub votes_abstain: String,
    pub created_at: u64,
    pub approved: bool,
    pub executed: bool,
}

/// Consensus Day phase info
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcConsensusDayPhase {
    pub phase: String,
    pub phase_code: u8,
    pub phase_start_block: u32,
    pub event_start_block: u32,
    pub year: u32,
    pub blocks_remaining: u32,
    pub is_active: bool,
}

/// Voting power info
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcVotingPower {
    pub staked_amount: String,
    pub voting_power: String,
    pub participation_history: u32,
    pub can_vote: bool,
}

/// Vote counts
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcVoteCounts {
    pub proposal_id: u64,
    pub votes_for: String,
    pub votes_against: String,
    pub votes_abstain: String,
    pub total_voting_power: String,
}

/// Individual vote
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcVote {
    pub voter: String,
    pub proposal_id: u64,
    pub vote_type: String,
    pub voting_power: String,
}

/// Director candidate
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcDirectorCandidate {
    pub account: String,
    pub stake: String,
    pub votes: String,
}

/// Distribution info
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcDistribution {
    pub total_minted: String,
    pub foundation_share: String,
    pub directors_share: String,
    pub validators_share: String,
    pub voters_share: String,
    pub fees_distributed: String,
}

/// Governance statistics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcGovernanceStats {
    pub total_proposals: u64,
    pub active_proposals: u64,
    pub approved_proposals: u64,
    pub rejected_proposals: u64,
    pub total_votes_cast: String,
    pub unique_voters: u32,
    pub circulating_supply: String,
    pub quorum_threshold_percent: u32,
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

fn phase_to_string(phase: u8) -> String {
    match phase {
        0 => "Inactive".to_string(),
        1 => "Registration".to_string(),
        2 => "Voting".to_string(),
        3 => "Minting".to_string(),
        4 => "Distribution".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn category_to_string(category: u8) -> String {
    match category {
        0 => "InflationRate".to_string(),
        1 => "ParameterChange".to_string(),
        2 => "BudgetAllocation".to_string(),
        3 => "ProtocolUpgrade".to_string(),
        4 => "DirectorElection".to_string(),
        5 => "EmergencyAction".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn status_to_string(status: u8) -> String {
    match status {
        0 => "Pending".to_string(),
        1 => "Active".to_string(),
        2 => "Approved".to_string(),
        3 => "Rejected".to_string(),
        4 => "Executed".to_string(),
        5 => "Cancelled".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn vote_type_to_string(vote_type: u8) -> String {
    match vote_type {
        0 => "Yes".to_string(),
        1 => "No".to_string(),
        2 => "Abstain".to_string(),
        _ => "Unknown".to_string(),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// RPC IMPLEMENTATION
// ═══════════════════════════════════════════════════════════════════════════════

/// Governance RPC implementation
pub struct Governance<C, Block, AccountId> {
    client: Arc<C>,
    _phantom: std::marker::PhantomData<(Block, AccountId)>,
}

impl<C, Block, AccountId> Governance<C, Block, AccountId>
where
    Block: BlockT,
    C: HeaderBackend<Block> + Send + Sync + 'static,
    AccountId: codec::Codec + std::fmt::Display + Clone,
{
    /// Create a new Governance RPC handler
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: std::marker::PhantomData,
        }
    }

    fn err(msg: &str) -> ErrorObject<'static> {
        ErrorObject::owned(
            ErrorCode::InternalError.code(),
            msg.to_string(),
            None::<()>,
        )
    }
}

#[async_trait]
impl<C, Block, AccountId> GovernanceApiServer<Block::Hash, AccountId> for Governance<C, Block, AccountId>
where
    Block: BlockT,
    C: HeaderBackend<Block> + ProvideRuntimeApi<Block> + Send + Sync + 'static,
    C::Api: GovernanceRuntimeApi<Block, AccountId>,
    AccountId: codec::Codec + std::fmt::Display + Clone + Send + Sync + 'static,
{
    async fn get_proposals(&self) -> RpcResult<Vec<RpcProposal>> {
        let best = self.client.info().best_hash;

        let proposals = self.client
            .runtime_api()
            .get_proposals(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(proposals
            .into_iter()
            .map(|p| RpcProposal {
                id: p.id,
                proposer: format!("{}", p.proposer),
                title: String::from_utf8_lossy(&p.title).to_string(),
                description: String::from_utf8_lossy(&p.description).to_string(),
                category: category_to_string(p.category),
                status: status_to_string(p.status),
                votes_for: p.votes_for.to_string(),
                votes_against: p.votes_against.to_string(),
                votes_abstain: p.votes_abstain.to_string(),
                created_at: p.created_at,
                approved: p.approved,
                executed: p.executed,
            })
            .collect())
    }

    async fn get_proposal(&self, id: u64) -> RpcResult<Option<RpcProposal>> {
        let best = self.client.info().best_hash;

        let proposal = self.client
            .runtime_api()
            .get_proposal(best, id)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(proposal.map(|p| RpcProposal {
            id: p.id,
            proposer: format!("{}", p.proposer),
            title: String::from_utf8_lossy(&p.title).to_string(),
            description: String::from_utf8_lossy(&p.description).to_string(),
            category: category_to_string(p.category),
            status: status_to_string(p.status),
            votes_for: p.votes_for.to_string(),
            votes_against: p.votes_against.to_string(),
            votes_abstain: p.votes_abstain.to_string(),
            created_at: p.created_at,
            approved: p.approved,
            executed: p.executed,
        }))
    }

    async fn get_phase(&self) -> RpcResult<RpcConsensusDayPhase> {
        let best = self.client.info().best_hash;

        let phase_info = self.client
            .runtime_api()
            .get_consensus_day_phase(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(RpcConsensusDayPhase {
            phase: phase_to_string(phase_info.phase),
            phase_code: phase_info.phase,
            phase_start_block: phase_info.phase_start_block,
            event_start_block: phase_info.event_start_block,
            year: phase_info.year,
            blocks_remaining: phase_info.blocks_remaining,
            is_active: phase_info.phase != 0,
        })
    }

    async fn is_active(&self) -> RpcResult<bool> {
        let best = self.client.info().best_hash;

        self.client
            .runtime_api()
            .is_consensus_day_active(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))
    }

    async fn get_voting_power(&self, account: String) -> RpcResult<RpcVotingPower> {
        let best = self.client.info().best_hash;

        // Parse account from SS58 or hex
        let account_id: AccountId = parse_account(&account)
            .map_err(|e| Self::err(&format!("Invalid account: {}", e)))?;

        let power = self.client
            .runtime_api()
            .get_voting_power(best, account_id)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(RpcVotingPower {
            staked_amount: power.staked_amount.to_string(),
            voting_power: power.voting_power.to_string(),
            participation_history: power.participation_history,
            can_vote: power.can_vote,
        })
    }

    async fn get_vote_counts(&self, proposal_id: u64) -> RpcResult<RpcVoteCounts> {
        let best = self.client.info().best_hash;

        let (yes, no, abstain) = self.client
            .runtime_api()
            .get_vote_counts(best, proposal_id)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        let total = yes.saturating_add(no).saturating_add(abstain);

        Ok(RpcVoteCounts {
            proposal_id,
            votes_for: yes.to_string(),
            votes_against: no.to_string(),
            votes_abstain: abstain.to_string(),
            total_voting_power: total.to_string(),
        })
    }

    async fn get_votes(&self, proposal_id: u64) -> RpcResult<Vec<RpcVote>> {
        let best = self.client.info().best_hash;

        let votes = self.client
            .runtime_api()
            .get_proposal_votes(best, proposal_id)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(votes
            .into_iter()
            .map(|v| RpcVote {
                voter: format!("{}", v.voter),
                proposal_id: v.proposal_id,
                vote_type: vote_type_to_string(v.vote_type),
                voting_power: v.voting_power.to_string(),
            })
            .collect())
    }

    async fn get_directors(&self) -> RpcResult<Vec<String>> {
        let best = self.client.info().best_hash;

        let directors = self.client
            .runtime_api()
            .get_elected_directors(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(directors.into_iter().map(|d| format!("{}", d)).collect())
    }

    async fn get_candidates(&self) -> RpcResult<Vec<RpcDirectorCandidate>> {
        let best = self.client.info().best_hash;

        let candidates = self.client
            .runtime_api()
            .get_director_candidates(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(candidates
            .into_iter()
            .map(|c| RpcDirectorCandidate {
                account: format!("{}", c.account),
                stake: c.stake.to_string(),
                votes: c.votes.to_string(),
            })
            .collect())
    }

    async fn get_distribution(&self) -> RpcResult<RpcDistribution> {
        let best = self.client.info().best_hash;

        let dist = self.client
            .runtime_api()
            .get_distribution_info(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(RpcDistribution {
            total_minted: dist.total_minted.to_string(),
            foundation_share: dist.foundation_share.to_string(),
            directors_share: dist.directors_share.to_string(),
            validators_share: dist.validators_share.to_string(),
            voters_share: dist.voters_share.to_string(),
            fees_distributed: dist.fees_distributed.to_string(),
        })
    }

    async fn get_stats(&self) -> RpcResult<RpcGovernanceStats> {
        let best = self.client.info().best_hash;

        let stats = self.client
            .runtime_api()
            .get_governance_stats(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(RpcGovernanceStats {
            total_proposals: stats.total_proposals,
            active_proposals: stats.active_proposals,
            approved_proposals: stats.approved_proposals,
            rejected_proposals: stats.rejected_proposals,
            total_votes_cast: stats.total_votes_cast.to_string(),
            unique_voters: stats.unique_voters,
            circulating_supply: stats.circulating_supply.to_string(),
            quorum_threshold_percent: stats.quorum_threshold,
        })
    }

    async fn get_reward(&self, account: String) -> RpcResult<String> {
        let best = self.client.info().best_hash;

        let account_id: AccountId = parse_account(&account)
            .map_err(|e| Self::err(&format!("Invalid account: {}", e)))?;

        let reward = self.client
            .runtime_api()
            .get_participation_reward(best, account_id)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))?;

        Ok(reward.to_string())
    }

    async fn get_inflation_rate(&self) -> RpcResult<u32> {
        let best = self.client.info().best_hash;

        self.client
            .runtime_api()
            .get_inflation_rate(best)
            .map_err(|e| Self::err(&format!("Runtime API error: {:?}", e)))
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// ACCOUNT PARSING
// ═══════════════════════════════════════════════════════════════════════════════

/// Parse account from SS58 or hex string
fn parse_account<AccountId: codec::Codec>(s: &str) -> Result<AccountId, String> {
    // Try hex first
    if s.starts_with("0x") {
        let bytes = hex::decode(&s[2..]).map_err(|e| format!("Invalid hex: {}", e))?;
        AccountId::decode(&mut &bytes[..]).map_err(|e| format!("Decode error: {:?}", e))
    } else {
        // Try SS58
        sp_core::crypto::AccountId32::from_ss58check(s)
            .map_err(|e| format!("Invalid SS58: {:?}", e))
            .and_then(|acc| {
                let bytes = acc.encode();
                AccountId::decode(&mut &bytes[..]).map_err(|e| format!("Decode error: {:?}", e))
            })
    }
}

/// Create governance RPC extension
pub fn create_governance_rpc<C, Block, AccountId>(
    client: Arc<C>,
) -> Result<jsonrpsee::RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
    Block: BlockT,
    C: HeaderBackend<Block> + ProvideRuntimeApi<Block> + Send + Sync + 'static,
    C::Api: GovernanceRuntimeApi<Block, AccountId>,
    AccountId: codec::Codec + std::fmt::Display + Clone + Send + Sync + 'static,
{
    use GovernanceApiServer;
    let governance = Governance::<C, Block, AccountId>::new(client);
    let mut module = jsonrpsee::RpcModule::new(());
    module.merge(governance.into_rpc())?;
    Ok(module)
}
