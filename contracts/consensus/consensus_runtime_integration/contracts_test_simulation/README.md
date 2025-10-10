# Test & Execution Layer

This module provides the test scaffolds, execution logic, and CLI runners for simulating and validating the Êtrid Consensus and Minting Protocol.

## 📁 Structure

- `tests/`: Unit tests for all core modules.
- `cli/`: CLI runners for testing consensus votes, distribution logic, and proposal processing.
- `vote_storage.rs`: On-chain storage handling of proposals and votes.
- `distribution_execution.rs`: Real logic for executing reward schedules based on valid stake and vote commitment.
- `consensus_vote_orchestration.rs`: Master coordinator logic for Consensus Day voting.

## 🛠️ Setup

Ensure you have Rust and Cargo installed:
```bash
rustup install stable
cargo build