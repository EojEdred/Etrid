# Consensus Runtime Integration

## Folder Structure
- `contracts/`: Contains Rust smart contracts related to consensus and distribution.
- `consensus_vote_orchestrationv2.rs`: Main contract logic for coordinating consensus voting events.
- `distribution/`: Execution logic for payout and schedule post-vote.
- `runtime/`: Manages time-gated operations and runtime configurations.

## Key Modules
- `vote_storage.rs`: Handles proposal/vote state management.
- `role_validation.rs`: Role-based permissions (e.g., common peer, staking peer).
- `runtime_config.rs`: Loads configs, time windows, parameters.

## Usage
1. Compile with `cargo build`.
2. Deploy with custom CLI or compatible Rust chain tools.
3. Interact via public query endpoints exposed in `queries/public_query.rs`.

