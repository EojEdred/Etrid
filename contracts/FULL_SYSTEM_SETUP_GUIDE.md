# Full System Setup & Deployment Guide

## 1. Overview

This document outlines the necessary steps for developers to configure, compile, and deploy the entire Etrid multichain system to a production-ready state. This includes both the internal Substrate-based components and the external cross-chain bridge contracts/programs.

**Assumptions:**
*   You have a working development environment with administrative privileges.
*   Basic familiarity with Rust, TypeScript, Hardhat, Solana/Anchor, Plutus, XRP Hooks, and Soroban.
*   Access to funded deployer accounts on all target networks (EVM chains, Solana, Cardano, XRP Ledger, Stellar).

## 2. Global Prerequisites

Ensure the following tools are installed and up-to-date:

*   **Rust Toolchain**: `rustup` managed (stable channel recommended).
*   **Node.js & Yarn/NPM**: LTS version.
*   **`jq`**: Command-line JSON processor.
*   **Docker & Docker Compose**: For running local blockchain nodes.
*   **Substrate Development Environment**:
    *   `cargo install subkey --locked`
    *   Rust nightly toolchain for `wasm32-unknown-unknown` target.
    *   Clone or ensure access to `polkadot-sdk` (matching the version in `Cargo.toml` of Substrate pallets).

## 3. EVM Bridge Setup & Deployment (Solidity & TypeScript Script)

This section covers deploying the Ethereum-compatible bridge contracts using Hardhat and orchestrating it with our `bridge_bootstrap.ts` script.

### 3.1 Prerequisites
*   **Hardhat**: `npm install -g hardhat`
*   **Local EVM Networks**: For local development, run Hardhat Network, Anvil, or Ganache. For testnets, ensure RPC URLs are accessible.
*   **EVM Wallets**: Deployer address funded with native token (ETH, BNB, MATIC).

### 3.2 Compilation
Navigate to the Ethereum contracts directory and compile:
```bash
cd contracts/ethereum
npm install # Install hardhat dependencies
npx hardhat compile
```
This will generate `artifacts/` containing the ABI and bytecode for `EtridBridge`, `TokenMessenger`, `WrappedETR`, etc.

### 3.3 Configuration
The `scripts/deployment/bridge_bootstrap.ts` script uses environment variables.
*   `ETHEREUM_RPC_URL`: RPC endpoint for Ethereum (e.g., `https://eth-sepolia.g.alchemy.com/v2/...`).
*   `BSC_RPC_URL`: RPC endpoint for Binance Smart Chain.
*   `POLYGON_RPC_URL`: RPC endpoint for Polygon.
*   `DEPLOYER_PRIVATE_KEY`: Private key of the deployer wallet (e.g., `0x...`). **Keep this secure.**
*   **Initial Validators/Threshold**: Review `scripts/deployment/bridge_bootstrap.ts` for the initial `initialValidators` and `initialThreshold` passed to the `EtridBridge` constructor. Adjust as needed.

### 3.4 Deployment
Ensure environment variables are set and run the bootstrap script:
```bash
cd scripts/deployment
npm install # if ts-node is not globally installed
ETHEREUM_RPC_URL=... BSC_RPC_URL=... POLYGON_RPC_URL=... DEPLOYER_PRIVATE_KEY=... ts-node bridge_bootstrap.ts
```
Verify `contracts/addresses.json` is updated with deployed contract addresses for each chain.

## 4. Solana Bridge Setup & Deployment (Anchor)

This section covers deploying the Solana Message Transmitter program.

### 4.1 Prerequisites
*   **Solana CLI**: `sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`
*   **Anchor CLI**: `cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked --force`
*   **Rust Toolchain**: Ensure `rustup default stable` or a specific compatible version.
*   **Solana Validator**: `solana-test-validator` for local testing or access to Devnet/Testnet.
*   **Solana Wallet**: Keypair (`~/.config/solana/id.json`) funded with SOL.

### 4.2 Compilation
Navigate to the Solana program directory and build:
```bash
cd 05-multichain/bridges/protocols/solana-bridge/program
anchor build
```

### 4.3 Configuration
*   **`Anchor.toml`**: Configure `cluster` (e.g., `Localnet`, `Devnet`) and `wallet` path.
*   **Program ID**: Update `declare_id!` in `src/lib.rs` with the deployed Program ID after initial deployment.
*   **`initialize` Parameters**: `admin` (deployer pubkey), `threshold` (minimum signatures for attestation), `supported_validators` (Pubkeys of Etrid validators). These are passed during the `initialize` instruction.

### 4.4 Deployment
```bash
cd 05-multichain/bridges/protocols/solana-bridge/program
anchor deploy
anchor idl init -f target/idl/message_transmitter.json Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS # Replace ID with actual
# Run initialize instruction
anchor run initialize --provider.cluster localnet --provider.wallet ~/.config/solana/id.json \
    --program Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS -- admin <ADMIN_PUBKEY> threshold <THRESHOLD>
```
Update `addresses.json` (manually or via a script) with the deployed program ID and `GlobalState` account address.

## 5. Cardano Bridge Setup & Deployment (Plutus)

This section covers deploying the Cardano Plutus script.

### 5.1 Prerequisites
*   **Haskell Toolchain**: `cabal` (recommended) or `stack`.
*   **`ghc`**: Compatible Haskell compiler.
*   **`cardano-node` & `cardano-cli`**: Synchronized Cardano node and CLI.
*   **`plutus-apps`**: Relevant version for compiling Plutus scripts.
*   **Cardano Wallet**: Funded address for transactions.

### 5.2 Compilation
Navigate to the Plutus script directory and build:
```bash
cd 05-multichain/bridges/protocols/cardano-bridge/plutus
cabal build # or stack build
# This will generate the Plutus script in a format usable by cardano-cli (e.g., .plutus or .json)
```

### 5.3 Configuration
*   **`BridgeDatum`**: Define the initial bridge state with `bdBridgeValidators` (list of `PubKeyHash`es for Etrid validators), `bdMinSignaturesThreshold`, and `bdProcessedAttestations` (initial empty Merkle root hash). This Datum needs to be converted to a `DatumHash`.
*   **Message Format**: Standardize the cross-chain message format that `urAttestationHash` will represent.

### 5.4 Deployment
Deployment on Cardano involves creating a UTXO locked at the script address with the initial `BridgeDatum`.
```bash
# 1. Generate Plutus script hash and address
cardano-cli address build --payment-script-file ./BridgeValidator.plutus --mainnet --out-file bridge-script.addr
cardano-cli transaction policyid --script-file ./BridgeValidator.plutus > bridge-script.hash

# 2. Create initial Datum (e.g., bridge-datum.json)
# This datum must contain the initial validator set (PubKeyHashes) and threshold.
# Example: cardano-cli transaction build-raw ... --tx-out "bridge-script.addr + 1000000 lovelace + DatumHash <HASH_OF_INITIAL_DATUM>" ...

# 3. Fund the script address with initial UTXO holding the datum
# This requires a transaction from a regular wallet to the script address.
# A relayer will then interact with this UTXO.
```
Update `addresses.json` with the `bridge-script.addr`.

## 6. XRP Bridge Setup & Deployment (Hooks C)

This section covers deploying the XRP Ledger Hook.

### 6.1 Prerequisites
*   **XRP Ledger Node**: Local `rippled` instance or access to an XRPL Testnet/Devnet.
*   **XRP Hooks SDK / Builder**: A C compiler configured for XRPL Hooks (often a custom WASM toolchain).
*   **XRP Account**: Funded account to deploy the Hook.

### 6.2 Compilation
Navigate to the Hook directory and compile:
```bash
cd 05-multichain/bridges/protocols/xrp-bridge/hooks
# Compile bridge_hook.c into a WASM file (e.g., using xrp-hooks-builder or a custom toolchain)
# Example: xrp-hooks-builder build bridge_hook.c -o bridge_hook.wasm
```

### 6.3 Configuration
*   **`HOOK_ACCID_BUF_SIZE`, `ETRID_BRIDGE_ISSUER_SIZE`**: Replace placeholder comments with actual byte arrays for relevant account IDs.
*   **Relayer Monitoring**: The relayer service needs to be configured to monitor the XRPL for `trace` messages emitted by this Hook.
*   **Message Format**: Standardize the canonical message hash generation.

### 6.4 Deployment
Deployment involves submitting a `SetHook` transaction to the XRPL.
```bash
# 1. Get the WASM binary from compilation.
# 2. Submit a SetHook transaction via `rippled` or a client library.
# Example using `xrp-hooks-builder deploy`:
# xrp-hooks-builder deploy bridge_hook.wasm --keypair "s..." --rippled-server "wss://s.altnet.rippletest.net:51233"
```
Update `addresses.json` with the deployed Hook account address.

## 7. Stellar Bridge Setup & Deployment (Soroban)

This section covers deploying the Stellar Soroban contract.

### 7.1 Prerequisites
*   **Soroban CLI**: `cargo install soroban-cli --locked`
*   **Rust Toolchain**: `rustup target add wasm32-unknown-unknown`
*   **Soroban Network**: Local standalone network or access to Testnet/Futurenet.
*   **Stellar Account**: Funded account for deployment and interactions.

### 7.2 Compilation
Navigate to the Soroban contract directory and build:
```bash
cd 05-multichain/bridges/protocols/stellar-bridge/soroban
soroban contract build
```

### 7.3 Configuration
*   **`initialize` Parameters**:
    *   `admin`: The initial admin `Address`.
    *   `validators`: `SVec<Address>` of Etrid validators (Stellar Pubkeys).
    *   `threshold`: Minimum number of validator signatures.
*   **Token IDs**: The `token_id` used in `lock` and `unlock` should refer to actual Stellar Asset IDs or contract IDs.
*   **Message Format**: Standardize the `message_body` content that validators sign.

### 7.4 Deployment
```bash
cd 05-multichain/bridges/protocols/stellar-bridge/soroban
# 1. Deploy the WASM contract
soroban contract deploy --source <DEPLOYER_ACCOUNT_KEY> --network <NETWORK> --wasm target/wasm32-unknown-unknown/release/etrid_stellar_bridge.wasm

# 2. Initialize the contract
# Replace with actual admin, validators, and threshold
soroban contract invoke \
    --id <DEPLOYED_CONTRACT_ID> \
    --source <ADMIN_ACCOUNT_KEY> \
    --network <NETWORK> \
    -- initialize \
    --admin <ADMIN_ADDRESS> \
    --validators "['<VALIDATOR1_ADDRESS>', '<VALIDATOR2_ADDRESS>']" \
    --threshold <THRESHOLD_VALUE>
```
Update `addresses.json` with the deployed contract ID.

## 8. General Notes on Cross-Chain Integration

*   **Message Format Standardization**: A critical aspect is ensuring a consistent and canonical message format (`message_hash`, `attestation_hash`, `message_body`) that is signed by validators on the Etrid chain and verified on all target chains.
*   **Relayer Network**: The `services/bridge-monitor-service` needs to be fully configured, compiled, and run to actively monitor all connected chains, observe events/traces, aggregate attestations, and submit transactions to destination chains.
*   **Off-Chain Validator Orchestration**: There must be an off-chain process for Etrid validators to collectively sign cross-chain messages in response to events from connected chains.
*   **Security Audits**: All smart contracts, Substrate pallets, and off-chain services must undergo rigorous security audits before production deployment.
*   **Monitoring & Alerting**: Comprehensive monitoring and alerting systems are essential for detecting anomalies and potential bridge security incidents.
*   **Incident Response Plan**: A clear plan for handling bridge failures, attacks, or discrepancies.
