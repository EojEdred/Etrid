// ═══════════════════════════════════════════════════════════════════════════════
// Consensus Commands (Governance & Consensus Day)
// ═══════════════════════════════════════════════════════════════════════════════

use anyhow::Result;
use colored::Colorize;

use crate::{cli::ConsensusCommands, rpc_client::EtridRpcClient};

pub async fn execute(command: ConsensusCommands, endpoint: &str) -> Result<()> {
    match command {
        ConsensusCommands::ProposeSubmit {
            title,
            description,
            from,
        } => submit_proposal(title, description, from, endpoint).await,
        ConsensusCommands::ProposeList => list_proposals(endpoint).await,
        ConsensusCommands::Vote {
            proposal_id,
            vote,
            from,
        } => vote_on_proposal(proposal_id, vote, from, endpoint).await,
        ConsensusCommands::ProposalInfo { proposal_id } => show_proposal_info(proposal_id, endpoint).await,
        ConsensusCommands::Status => show_consensus_status(endpoint).await,
        ConsensusCommands::Distribution => show_distribution_schedule(endpoint).await,
    }
}

async fn submit_proposal(
    title: String,
    description: String,
    from: String,
    endpoint: &str,
) -> Result<()> {
    println!("{}", "Submitting Governance Proposal".bright_green().bold());
    println!();
    println!("  {}: {}", "Title".bold(), title.bright_white());
    println!("  {}: {}", "Description".bold(), description.bright_white());
    println!("  {}: {}", "Proposer".bold(), from.bright_white());
    println!();

    // Connect to RPC
    let _client = EtridRpcClient::new(endpoint).await?;
    println!("  {}: {}", "RPC".bold(), "Connected".bright_green());
    println!();

    println!("{}", "Proposal Submission Process:".bright_cyan().bold());
    println!("  {} Validate proposal format", "○".bright_yellow());
    println!("  {} Check proposer eligibility", "○".bright_yellow());
    println!("  {} Create proposal extrinsic", "○".bright_yellow());
    println!("  {} Submit to governance pallet", "○".bright_yellow());
    println!("  {} Proposal enters voting period", "○".bright_yellow());
    println!();

    println!("{}", "Note: Proposal submission requires:".bright_yellow());
    println!("      - Governance pallet integration");
    println!("      - Minimum stake requirement check");
    println!("      - Proposal bond deposit");
    println!();
    println!("  On ËTRID, proposals are voted on during Consensus Day.");
    println!();

    Ok(())
}

async fn list_proposals(endpoint: &str) -> Result<()> {
    println!("{}", "Active Governance Proposals".bright_green().bold());
    println!();

    // Connect to RPC
    match EtridRpcClient::new(endpoint).await {
        Ok(_client) => {
            println!("  {}: {}", "Status".bold(), "Connected".bright_green());
            println!();
            println!("  {} Querying governance proposals...", "→".bright_cyan());
            println!();

            println!("{}", "Note: Proposal queries require:".bright_yellow());
            println!("      - Governance pallet storage iteration");
            println!("      - Proposal status tracking");
            println!("      - Vote tallying");
            println!();
            println!("  Use: {} to view specific proposal", "etrust consensus proposal-info <ID>".bright_cyan());
        }
        Err(e) => {
            println!("  {}: {}", "Status".bold(), "Connection failed".bright_red());
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}

async fn vote_on_proposal(
    proposal_id: u32,
    vote: String,
    from: String,
    endpoint: &str,
) -> Result<()> {
    println!("{}", "Voting on Proposal".bright_green().bold());
    println!();
    println!("  {}: {}", "Proposal ID".bold(), proposal_id.to_string().bright_white());
    println!("  {}: {}", "Vote".bold(), vote.bright_white());
    println!("  {}: {}", "Voter".bold(), from.bright_white());
    println!();

    // Validate vote
    let vote_normalized = vote.to_lowercase();
    let vote_display = match vote_normalized.as_str() {
        "yes" | "aye" => "YES".bright_green(),
        "no" | "nay" => "NO".bright_red(),
        "abstain" => "ABSTAIN".bright_yellow(),
        _ => {
            println!("{}", "Error: Invalid vote. Use: yes, no, or abstain".bright_red());
            return Ok(());
        }
    };

    // Connect to RPC
    let _client = EtridRpcClient::new(endpoint).await?;
    println!("  {}: {}", "RPC".bold(), "Connected".bright_green());
    println!("  {}: {}", "Vote Type".bold(), vote_display);
    println!();

    println!("{}", "Voting Process:".bright_cyan().bold());
    println!("  {} Verify proposal exists and is active", "○".bright_yellow());
    println!("  {} Check voter eligibility (staking)", "○".bright_yellow());
    println!("  {} Calculate vote weight", "○".bright_yellow());
    println!("  {} Create vote extrinsic", "○".bright_yellow());
    println!("  {} Submit vote to governance", "○".bright_yellow());
    println!();

    println!("{}", "Note: On ËTRID:".bright_yellow());
    println!("      - Voting power is based on staked ETR");
    println!("      - Votes are cast during Consensus Day");
    println!("      - Validator votes carry additional weight");
    println!();

    Ok(())
}

async fn show_proposal_info(proposal_id: u32, endpoint: &str) -> Result<()> {
    println!("{}", "Proposal Details".bright_green().bold());
    println!();
    println!("  {}: {}", "Proposal ID".bold(), proposal_id.to_string().bright_white());
    println!();

    // Connect to RPC
    match EtridRpcClient::new(endpoint).await {
        Ok(_client) => {
            println!("  {}: {}", "Status".bold(), "Connected".bright_green());
            println!();
            println!("  {} Fetching proposal details...", "→".bright_cyan());
            println!();

            // Mock data for demonstration
            println!("  {}", "Proposal Information:".bright_cyan().bold());
            println!("    {}: {}", "Status".bold(), "Active".bright_green());
            println!("    {}: {}", "Proposer".bold(), "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
            println!("    {}: {}", "Title".bold(), "[Requires storage query]");
            println!("    {}: {}", "Voting Period".bold(), "Block 100-200");
            println!();

            println!("{}", "Note: Full proposal details require governance pallet queries".bright_yellow());
        }
        Err(e) => {
            println!("  {}: {}", "Status".bold(), "Connection failed".bright_red());
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}

async fn show_consensus_status(endpoint: &str) -> Result<()> {
    println!("{}", "Consensus Day Status".bright_green().bold());
    println!();

    // Connect to RPC
    match EtridRpcClient::new(endpoint).await {
        Ok(client) => {
            println!("  {}: {}", "Status".bold(), "Connected".bright_green());
            println!();

            // Get current block
            match client.get_block_number().await {
                Ok(block_number) => {
                    println!("  {}: {}", "Current Block".bold(), block_number.to_string().bright_white());

                    // Calculate consensus day info (assuming 28800 blocks per day)
                    let blocks_per_day = 28800_u64;
                    let current_day = block_number / blocks_per_day;
                    let blocks_until_next = blocks_per_day - (block_number % blocks_per_day);

                    println!("  {}: {}", "Current Day".bold(), current_day.to_string().bright_white());
                    println!(
                        "  {}: {}",
                        "Blocks Until Next".bold(),
                        blocks_until_next.to_string().bright_white()
                    );
                }
                Err(e) => {
                    println!("  {}: {}", "Block Query Error".bold(), e.to_string().bright_red());
                }
            }

            println!();
            println!("{}", "Note: Full consensus status requires:".bright_yellow());
            println!("      - Consensus day pallet queries");
            println!("      - Active proposal count");
            println!("      - Participation metrics");
        }
        Err(e) => {
            println!("  {}: {}", "Status".bold(), "Connection failed".bright_red());
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}

async fn show_distribution_schedule(endpoint: &str) -> Result<()> {
    println!("{}", "Fiscal Distribution Schedule".bright_green().bold());
    println!();

    // Connect to RPC
    match EtridRpcClient::new(endpoint).await {
        Ok(_client) => {
            println!("  {}: {}", "Status".bold(), "Connected".bright_green());
            println!();
            println!("  {} Querying distribution data...", "→".bright_cyan());
            println!();

            println!("{}", "Distribution Allocations:".bright_cyan().bold());
            println!("  {}: {}%", "Foundation".bold(), "20".bright_white());
            println!("  {}: {}%", "Flare Nodes".bold(), "25".bright_white());
            println!("  {}: {}%", "Validity Nodes".bold(), "20".bright_white());
            println!("  {}: {}%", "Directors".bold(), "15".bright_white());
            println!("  {}: {}%", "Treasury".bold(), "10".bright_white());
            println!("  {}: {}%", "Development".bold(), "10".bright_white());
            println!();

            println!("{}", "Note: Actual distribution requires:".bright_yellow());
            println!("      - Distribution pallet queries");
            println!("      - Current period information");
            println!("      - Beneficiary calculations");
        }
        Err(e) => {
            println!("  {}: {}", "Status".bold(), "Connection failed".bright_red());
            println!("  {}: {}", "Error".bold(), e.to_string().bright_red());
        }
    }

    println!();
    Ok(())
}
