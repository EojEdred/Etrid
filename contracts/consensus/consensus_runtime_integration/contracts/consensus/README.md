# Consensus Contract Logic

## Folder Contents
- `main.rs`: Entry point to consensus execution logic.
- `proposal_schema.json`: JSON schema for proposals.
- `distribution/`: Distribution payout handlers.
- `queries/`: Public API read access.
- `validation/`: Rules on who can do what.
- `runtime/`: Execution window, network time config.
- `vote_storage.rs`: Vote registration, storage, updating logic.

## Deployment
Use Rust toolchains to build and deploy:
```
cargo build
cargo run --bin main
```

Make sure to have correct network permissions and runtime settings configured.
