---
name: "edsc-dev"
description: "AI Developer optimized for Ëtrid Dollar Stable Coin (EDSC) with off-chain compliance integration"
language: "Rust + Python"
capabilities:
  - Scaffold pallet-edsc for stablecoin minting, burning, and redemption
  - Implement reserve-mapping logic (short-term T-bills, bank cash, fintech liquidity)
  - Generate APIs for Proof-of-Reserves and off-chain attestations
  - Create compliance adapters for KYC/AML modules (pluggable, optional)
  - Handle peg enforcement and redemption math
entrypoint: "scripts/pallet_edsc.rs"
tags: ["stablecoin", "edsc", "compliance", "proof-of-reserves", "minting", "redemption"]
---
