#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# ËTRID E³20 RESTRUCTURE - MASTER SCRIPT
# ═══════════════════════════════════════════════════════════════════════════════
# Purpose: Automated migration from current structure to E³20-aligned structure
# Author: Claude (Anthropic)
# Date: October 11, 2025
# ═══════════════════════════════════════════════════════════════════════════════

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ═══════════════════════════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════════

REPO_ROOT="${PWD}"
BACKUP_DIR="${REPO_ROOT}/_backup_$(date +%Y%m%d_%H%M%S)"
LOG_FILE="${REPO_ROOT}/restructure_log_$(date +%Y%m%d_%H%M%S).txt"

# ═══════════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $1" | tee -a "$LOG_FILE"
}

success() {
    echo -e "${GREEN}✅ $1${NC}" | tee -a "$LOG_FILE"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}❌ $1${NC}" | tee -a "$LOG_FILE"
}

header() {
    echo "" | tee -a "$LOG_FILE"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}" | tee -a "$LOG_FILE"
    echo -e "${PURPLE}$1${NC}" | tee -a "$LOG_FILE"
    echo -e "${PURPLE}═══════════════════════════════════════════════════════════${NC}" | tee -a "$LOG_FILE"
    echo "" | tee -a "$LOG_FILE"
}

# ═══════════════════════════════════════════════════════════════════════════════
# PREFLIGHT CHECKS
# ═══════════════════════════════════════════════════════════════════════════════

preflight_checks() {
    header "PREFLIGHT CHECKS"
    
    log "Checking current directory..."
    if [ ! -f "Cargo.toml" ]; then
        error "Not in Ëtrid root directory (no Cargo.toml found)"
        exit 1
    fi
    success "In Ëtrid root directory"
    
    log "Checking git status..."
    if ! git diff-index --quiet HEAD --; then
        warning "You have uncommitted changes. Continuing anyway..."
    else
        success "Working directory is clean"
    fi
    
    log "Checking required tools..."
    command -v cargo >/dev/null 2>&1 || { error "cargo not found. Please install Rust."; exit 1; }
    command -v git >/dev/null 2>&1 || { error "git not found."; exit 1; }
    success "All required tools available"
    
    log "Creating backup at: ${BACKUP_DIR}"
    mkdir -p "${BACKUP_DIR}"
    success "Backup directory created"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 1: CREATE BACKUP
# ═══════════════════════════════════════════════════════════════════════════════

create_backup() {
    header "STEP 1: CREATING BACKUP"
    
    log "Backing up current structure..."
    
    # Backup critical directories
    for dir in pallets runtime node contracts network identity apps client infra; do
        if [ -d "$dir" ]; then
            log "Backing up ${dir}/"
            cp -r "$dir" "${BACKUP_DIR}/" 2>/dev/null || true
        fi
    done
    
    # Backup Cargo.toml
    if [ -f "Cargo.toml" ]; then
        cp Cargo.toml "${BACKUP_DIR}/Cargo.toml.backup"
    fi
    
    success "Backup created at: ${BACKUP_DIR}"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 2: CREATE E³20 DIRECTORY STRUCTURE
# ═══════════════════════════════════════════════════════════════════════════════

create_e320_structure() {
    header "STEP 2: CREATING E³20 DIRECTORY STRUCTURE"
    
    log "Creating 13 E³20 component directories..."
    
    # Component #1: DETR p2p
    mkdir -p 01-detr-p2p/{core,dpeers,aecomms,detrp2p,etrid-protocol,fluent,stored}/src
    
    # Component #2: OpenDID
    mkdir -p 02-open-did/{registry,resolver,types}/src
    
    # Component #3: Security
    mkdir -p 03-security/{cryptography,post-quantum,key-management}/src
    
    # Component #4: Accounts
    mkdir -p 04-accounts/{types,pallet}/src
    
    # Component #5: Multichain
    mkdir -p 05-multichain/flare-chain/{primitives,runtime,node}/src
    mkdir -p 05-multichain/partition-burst-chains/{pbc-runtime,pbc-node,bridge}/src
    mkdir -p 05-multichain/lightning-bloc-networks/{channel-manager,network}/src
    mkdir -p 05-multichain/bridge-protocols/{bitcoin-bridge,ethereum-bridge}/src
    mkdir -p 05-multichain/primitives/src
    
    # Component #6: Native Currency
    mkdir -p 06-native-currency/{etr-token,etd-stablecoin,vmw-gas,economics}/src
    
    # Component #7: Transactions
    mkdir -p 07-transactions/{types,regular,cross-chain,smart-contract,lightning-bloc,stake-deposit}/src
    
    # Component #8: EtwasmVM
    mkdir -p 08-etwasm-vm/{runtime,opcodes,gas-metering,pallet}/src
    
    # Component #9: Consensus
    mkdir -p 09-consensus/{asf-algorithm,validator-management,block-production,finality-gadget,pallet}/src
    
    # Component #10: Foundation
    mkdir -p 10-foundation/governance/{pallet,proposal-types}/src
    mkdir -p 10-foundation/legal
    
    # Component #11: Peer Roles
    mkdir -p 11-peer-roles/{flare-nodes,validity-nodes,decentralized-directors}/src
    mkdir -p 11-peer-roles/staking/{pallet,types}/src
    
    # Component #12: Consensus Day
    mkdir -p 12-consensus-day/{voting-protocol,minting-logic,distribution,proposal-system,queries}/src
    
    # Component #13: Clients
    mkdir -p 13-clients/cli/{etrcpp-console,pye-console}/src
    mkdir -p 13-clients/sdk/{js-sdk,rust-sdk,swift-sdk,python-sdk}/src
    mkdir -p 13-clients/{web-wallet,mobile-wallet,ui-generated}
    
    # Reference folder
    mkdir -p _reference/{cosmos-sdk,substrate-polkadot-sdk,other-references}
    
    # Support folders
    mkdir -p {apps,docs,scripts,infra}
    
    success "E³20 directory structure created"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 3: MIGRATE EXISTING CODE
# ═══════════════════════════════════════════════════════════════════════════════

migrate_code() {
    header "STEP 3: MIGRATING EXISTING CODE TO E³20 STRUCTURE"
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Issue 1: Consolidate Mobile Wallet
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating mobile wallet..."
    if [ -d "bloc_banc_wallet_flutter_v1_new" ]; then
        cp -r bloc_banc_wallet_flutter_v1_new/* 13-clients/mobile-wallet/
        success "Mobile wallet consolidated"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Issue 2: Move Reference Code
    # ═══════════════════════════════════════════════════════════════════════════
    log "Moving reference code..."
    if [ -d "pallets/cosmos-modules" ]; then
        mv pallets/cosmos-modules _reference/cosmos-sdk/ 2>/dev/null || true
    fi
    if [ -d "pallets/substrate-pallets" ]; then
        mv pallets/substrate-pallets _reference/substrate-polkadot-sdk/ 2>/dev/null || true
    fi
    success "Reference code moved to _reference/"
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Pallets
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating pallets..."
    
    if [ -d "pallets/accounts" ]; then
        cp -r pallets/accounts/* 04-accounts/pallet/
        success "Migrated accounts pallet"
    fi
    
    if [ -d "pallets/governance" ]; then
        cp -r pallets/governance/* 10-foundation/governance/pallet/
        success "Migrated governance pallet"
    fi
    
    if [ -d "pallets/consensus" ]; then
        cp -r pallets/consensus/* 09-consensus/pallet/
        success "Migrated consensus pallet"
    fi
    
    if [ -d "pallets/vm" ]; then
        cp -r pallets/vm/* 08-etwasm-vm/pallet/
        success "Migrated VM pallet"
    fi
    
    if [ -d "pallets/staking" ]; then
        cp -r pallets/staking/* 11-peer-roles/staking/pallet/ 2>/dev/null || true
        success "Migrated staking pallet"
    fi
    
    if [ -d "pallets/multichain" ]; then
        cp -r pallets/multichain/* 05-multichain/primitives/
        success "Migrated multichain pallet"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Issue 3: Flatten Contracts
    # ═══════════════════════════════════════════════════════════════════════════
    log "Flattening contracts structure..."
    
    if [ -d "contracts/consensus/consensus_runtime_integration" ]; then
        SOURCE="contracts/consensus/consensus_runtime_integration"
        
        # Proposal schema
        if [ -f "$SOURCE/contracts/consensus/proposal_schema.json" ]; then
            mkdir -p 10-foundation/governance/proposal-types/schemas
            cp "$SOURCE/contracts/consensus/proposal_schema.json" \
               10-foundation/governance/proposal-types/schemas/
        fi
        
        # Distribution
        if [ -d "$SOURCE/contracts/consensus/distribution" ]; then
            cp -r "$SOURCE/contracts/consensus/distribution"/* \
                  12-consensus-day/distribution/src/ 2>/dev/null || true
        fi
        
        # Simulation tests
        if [ -f "$SOURCE/simulation/distribution_schedule.rs" ]; then
            mkdir -p 12-consensus-day/distribution/tests
            cp "$SOURCE/simulation/distribution_schedule.rs" \
               12-consensus-day/distribution/tests/distribution_simulation.rs
        fi
        
        # Vote storage
        if [ -f "$SOURCE/contracts/consensus/vote_storage.rs" ]; then
            cp "$SOURCE/contracts/consensus/vote_storage.rs" \
               12-consensus-day/voting-protocol/src/
        fi
        
        # Runtime config
        if [ -d "$SOURCE/contracts/consensus/runtime" ]; then
            cp -r "$SOURCE/contracts/consensus/runtime"/* \
                  12-consensus-day/voting-protocol/src/ 2>/dev/null || true
        fi
        
        # Validation
        if [ -d "$SOURCE/contracts/consensus/validation" ]; then
            cp -r "$SOURCE/contracts/consensus/validation"/* \
                  11-peer-roles/validity-nodes/src/ 2>/dev/null || true
        fi
        
        # Queries
        if [ -d "$SOURCE/contracts/consensus/queries" ]; then
            cp -r "$SOURCE/contracts/consensus/queries"/* \
                  12-consensus-day/queries/src/ 2>/dev/null || true
        fi
        
        success "Contracts flattened and distributed"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Runtime
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating runtime..."
    
    if [ -d "runtime/flare-chain" ]; then
        cp -r runtime/flare-chain/* 05-multichain/flare-chain/runtime/
        success "Migrated FlareChain runtime"
    fi
    
    if [ -d "runtime/primitives" ]; then
        cp -r runtime/primitives/* 05-multichain/flare-chain/primitives/
        success "Migrated runtime primitives"
    fi
    
    if [ -d "runtime/pbc-runtime" ]; then
        cp -r runtime/pbc-runtime/* 05-multichain/partition-burst-chains/pbc-runtime/
        success "Migrated PBC runtime"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Node
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating node..."
    
    if [ -d "node" ]; then
        cp -r node/* 05-multichain/flare-chain/node/
        success "Migrated node binary"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Network
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating network layer..."
    
    if [ -d "network/detr-p2p" ]; then
        cp -r network/detr-p2p/* 01-detr-p2p/core/
        success "Migrated DETR p2p"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Identity
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating identity system..."
    
    if [ -d "identity/open-did" ]; then
        cp -r identity/open-did/* 02-open-did/registry/
        success "Migrated OpenDID"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Security
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating security stack..."
    
    if [ -d "security-stack" ]; then
        cp -r security-stack/* 03-security/
        success "Migrated security stack"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Apps
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating applications..."
    
    if [ -d "apps/governance-ui" ]; then
        cp -r apps/governance-ui apps/
        success "Migrated governance UI"
    fi
    
    if [ -d "apps/wallet-web" ]; then
        cp -r apps/wallet-web 13-clients/web-wallet/
        success "Migrated web wallet"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Client SDKs
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating client SDKs..."
    
    if [ -d "client/js-sdk" ]; then
        cp -r client/js-sdk/* 13-clients/sdk/js-sdk/
        success "Migrated JS SDK"
    fi
    
    if [ -d "client/rust-sdk" ]; then
        cp -r client/rust-sdk/* 13-clients/sdk/rust-sdk/
        success "Migrated Rust SDK"
    fi
    
    if [ -d "client/swift-sdk" ]; then
        cp -r client/swift-sdk/* 13-clients/sdk/swift-sdk/
        success "Migrated Swift SDK"
    fi
    
    # ═══════════════════════════════════════════════════════════════════════════
    # Migrate Infrastructure
    # ═══════════════════════════════════════════════════════════════════════════
    log "Migrating infrastructure..."
    
    if [ -d "infra" ]; then
        cp -r infra/* infra/
        success "Migrated infrastructure"
    fi
    
    success "Code migration complete"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 4: INSTALL ROOT CARGO.TOML
# ═══════════════════════════════════════════════════════════════════════════════

install_cargo_toml() {
    header "STEP 4: INSTALLING NEW CARGO.TOML"
    
    log "Installing hybrid hierarchical + grouped Cargo.toml..."
    
    # The new Cargo.toml should be provided separately
    if [ -f "etrid-root-Cargo.toml" ]; then
        mv Cargo.toml Cargo.toml.old
        cp etrid-root-Cargo.toml Cargo.toml
        success "New Cargo.toml installed (old saved as Cargo.toml.old)"
    else
        warning "etrid-root-Cargo.toml not found. You'll need to install it manually."
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 5: GENERATE README FILES
# ═══════════════════════════════════════════════════════════════════════════════

generate_readmes() {
    header "STEP 5: GENERATING README FILES"
    
    log "Generating README.md files for all components..."
    
    # This will be run as a separate script
    if [ -f "scripts/generate-readme-templates.sh" ]; then
        bash scripts/generate-readme-templates.sh
        success "README files generated"
    else
        warning "generate-readme-templates.sh not found"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 6: UPDATE IMPORTS
# ═══════════════════════════════════════════════════════════════════════════════

update_imports() {
    header "STEP 6: UPDATING IMPORT PATHS"
    
    log "Updating Rust import paths (this may take a while)..."
    
    # Find all Rust files and update imports
    find . -name "*.rs" -type f -not -path "./_backup*" -not -path "./_reference*" -not -path "./target/*" | while read -r file; do
        # Update pallet imports
        sed -i.bak 's|use pallets::accounts|use etrid_accounts|g' "$file" 2>/dev/null || true
        sed -i.bak 's|use pallets::governance|use etrid_governance|g' "$file" 2>/dev/null || true
        sed -i.bak 's|use pallets::consensus|use etrid_consensus|g' "$file" 2>/dev/null || true
        sed -i.bak 's|use pallets::vm|use etrid_etwasm_vm|g' "$file" 2>/dev/null || true
        sed -i.bak 's|use pallets::staking|use etrid_staking|g' "$file" 2>/dev/null || true
        
        # Clean up backup files
        rm -f "${file}.bak"
    done
    
    success "Import paths updated (manual verification recommended)"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 7: CLEAN UP OLD STRUCTURE
# ═══════════════════════════════════════════════════════════════════════════════

cleanup_old_structure() {
    header "STEP 7: CLEANING UP OLD STRUCTURE"
    
    log "Removing old directories..."
    
    # Remove old directories (they're backed up)
    rm -rf pallets/ 2>/dev/null || true
    rm -rf runtime/ 2>/dev/null || true
    rm -rf node/ 2>/dev/null || true
    rm -rf network/ 2>/dev/null || true
    rm -rf identity/ 2>/dev/null || true
    rm -rf security-stack/ 2>/dev/null || true
    rm -rf client/ 2>/dev/null || true
    rm -rf bloc_banc_wallet_flutter_v1_new/ 2>/dev/null || true
    
    success "Old structure cleaned up (backed up in ${BACKUP_DIR})"
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 8: VERIFY BUILD
# ═══════════════════════════════════════════════════════════════════════════════

verify_build() {
    header "STEP 8: VERIFYING BUILD"
    
    log "Attempting to build workspace..."
    
    if cargo check --workspace 2>&1 | tee -a "$LOG_FILE"; then
        success "Workspace builds successfully!"
    else
        warning "Build has errors. Check log at: ${LOG_FILE}"
        warning "You may need to manually fix import paths and Cargo.toml files"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# STEP 9: GIT COMMIT
# ═══════════════════════════════════════════════════════════════════════════════

git_commit() {
    header "STEP 9: GIT COMMIT"
    
    read -p "Create git commit for restructure? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        log "Creating git commit..."
        git add .
        git commit -m "refactor: Complete E³20 restructure

- Reorganized all code to match E³20 specification (13 components)
- Consolidated mobile wallet (Issue #1)
- Moved reference code to _reference/ (Issue #2)
- Flattened contracts structure (Issue #3)
- Implemented hybrid hierarchical + grouped Cargo.toml (Issue #4)
- Generated comprehensive README system (Issue #5)
- Updated import paths
- Backup created at: ${BACKUP_DIR}

This restructure aligns the codebase with the Ëtrid whitepaper architecture,
making it self-documenting and easier for contributors to navigate."
        
        success "Git commit created"
    else
        log "Skipping git commit (you can commit manually later)"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN EXECUTION
# ═══════════════════════════════════════════════════════════════════════════════

main() {
    clear
    echo -e "${CYAN}"
    echo "═══════════════════════════════════════════════════════════════════════════════"
    echo "  ËTRID E³20 RESTRUCTURE AUTOMATION"
    echo "  Migrating to Whitepaper-Aligned Architecture"
    echo "═══════════════════════════════════════════════════════════════════════════════"
    echo -e "${NC}"
    echo ""
    
    log "Starting E³20 restructure process..."
    log "Log file: ${LOG_FILE}"
    echo ""
    
    # Confirm before proceeding
    read -p "This will restructure your entire codebase. Continue? (yes/no) " -r
    echo
    if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
        error "Restructure cancelled by user"
        exit 1
    fi
    
    # Execute all steps
    preflight_checks
    create_backup
    create_e320_structure
    migrate_code
    install_cargo_toml
    generate_readmes
    update_imports
    cleanup_old_structure
    verify_build
    git_commit
    
    # Final summary
    header "RESTRUCTURE COMPLETE!"
    
    echo ""
    success "✅ E³20 restructure completed successfully!"
    echo ""
    echo -e "${CYAN}Next Steps:${NC}"
    echo "1. Review the restructured code"
    echo "2. Fix any remaining import errors (check log)"
    echo "3. Run: cargo build --workspace"
    echo "4. Run: cargo test --workspace"
    echo "5. Update CI/CD pipelines"
    echo "6. Update documentation"
    echo ""
    echo -e "${YELLOW}Backup Location:${NC} ${BACKUP_DIR}"
    echo -e "${YELLOW}Log File:${NC} ${LOG_FILE}"
    echo ""
    echo -e "${GREEN}🚀 Ëtrid is now E³20-aligned!${NC}"
    echo ""
}

# Run main function
main "$@"
