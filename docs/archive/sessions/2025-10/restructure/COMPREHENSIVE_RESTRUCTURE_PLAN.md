# Comprehensive Repository Restructure Plan

> **⚠️ SUPERSEDED:** This plan was executed but the final structure uses parent directories (`infrastructure/`, `development/`) instead of prefixes (`infra-*`, `dev-*`). See `/Users/macbook/Desktop/etrid/docs/REPOSITORY_RESTRUCTURE_COMPLETE.md` for the actual implementation.

**Date:** October 25, 2025
**Scope:** Complete folder reorganization with naming convention

---

## Current Problems Identified

### 1. No Naming Convention for Non-Essential Folders
- Essential components: `01-13` numbered
- Everything else: No structure, hard to navigate
- Inconsistent importance/priority

### 2. Duplicate Folders
- `infra/` + `infrastructure/` (both infrastructure)
- `audit-package/` + `audit-package-2025-10-21/` (same purpose)

### 3. Empty/Test Folders
- `.bridge-test`, `.local-testnet`, `.multichain-test`, `.peering-test`, `.test-network`, `.validator-test`
- `.bfg-report` (BFG cleanup artifact)
- `tools/` (nearly empty, only cli)

### 4. Misplaced Content
- `pallets/` - Custom pallets not integrated into numbered structure
- `vendor/` - Should be in `_reference/`
- Some deployment content duplicated across `deployment/` and `scripts/`

---

## Proposed Naming Convention

### Category A: Core Components (01-14)
**Pattern:** `NN-component-name` where NN = 01-14
**Purpose:** Essential blockchain components in dependency order

```
01-detr-p2p/          ✅ Keep
02-open-did/          ✅ Keep
03-security/          ✅ Keep
04-accounts/          ✅ Keep
05-multichain/        ✅ Keep
06-native-currency/   ✅ Keep
07-transactions/      ✅ Keep
08-etwasm-vm/         ✅ Keep
09-consensus/         ✅ Keep
10-foundation/        ✅ Keep
11-peer-roles/        ✅ Keep
12-consensus-day/     ✅ Keep
13-clients/           ✅ Keep
14-aidevs/            🔄 RENAME from ai-devs/
```

### Category B: Infrastructure (infra-)
**Pattern:** `infra-purpose/`
**Purpose:** Infrastructure, deployment, operations

```
infra-deployment/     🔄 CONSOLIDATE deployment/ + infra/ + infrastructure/
infra-monitoring/     🔄 RENAME from monitoring/
infra-chain-specs/    🔄 RENAME from chain-specs/
infra-config/         🔄 RENAME from config/
```

### Category C: Development (dev-)
**Pattern:** `dev-purpose/`
**Purpose:** Development tools, testing, build artifacts

```
dev-tests/            🔄 RENAME from tests/
dev-coverage/         🔄 RENAME from coverage/
dev-audit/            🔄 CONSOLIDATE audit-package/ + audit-package-2025-10-21/
dev-scripts/          🔄 RENAME from scripts/ (non-deployment scripts)
```

### Category D: Applications (apps-)
**Pattern:** `apps-name/` or keep as `apps/`
**Current:** Already well-organized

```
apps/                 ✅ Keep as-is (wallet, dashboards, etc.)
```

### Category E: Contracts (contracts-)
**Pattern:** Keep as `contracts/`
**Current:** Already well-organized

```
contracts/            ✅ Keep as-is (ethereum, etwasm-examples)
```

### Category F: Documentation (docs-)
**Pattern:** Keep as `docs/`
**Current:** Already cleaned

```
docs/                 ✅ Keep as-is
```

### Category G: Source Code (src/)
**Pattern:** Keep as `src/`
**Current:** Main runtime source

```
src/                  ✅ Keep as-is
pallets/              🔄 MOVE to src/pallets/ (custom pallets belong with source)
```

### Category H: SDKs & Services
**Pattern:** Keep descriptive names

```
sdk/                  ✅ Keep as-is
services/             ✅ Keep as-is
```

### Category I: Reference (._reference-)
**Pattern:** `_reference-name/` (underscore prefix for sorting to bottom)
**Purpose:** External dependencies, submodules

```
_reference/           ✅ Keep existing
_reference-vendor/    🔄 MOVE vendor/substrate-prometheus-endpoint here
```

---

## Detailed Reorganization Actions

### PHASE 1: Clean Test/Temp Directories ✅

**Delete immediately:**
```bash
.bridge-test/
.local-testnet/
.multichain-test/
.peering-test/
.test-network/
.validator-test/
.bfg-report/
```

**Rationale:** These are temporary test artifacts, not needed

---

### PHASE 2: Rename Core Component (ai-devs) ✅

**Action:**
```bash
mv ai-devs/ 14-aidevs/
```

**Contents:** (all current guides - no changes)
- CI_CD_PIPELINE_GUIDE.md
- COMPLETE_FEATURES_IMPLEMENTATION.md
- DEX_DEPLOYMENT_GUIDE.md
- GOVERNANCE_FORUM_GUIDE.md
- MONITORING_INFRASTRUCTURE_GUIDE.md
- PRE_DEPLOYMENT_COMPLETE_SUMMARY.md
- TREASURY_GOVERNANCE_GUIDE.md
- VESTING_GENESIS_GUIDE.md

**New path:** `14-aidevs/` (fits numbering convention)

---

### PHASE 3: Consolidate Duplicate Folders ✅

#### 3A: Infrastructure Consolidation

**Merge:** `infra/` + `infrastructure/` + `deployment/` → `infra-deployment/`

**Current contents:**
- `infra/`: docker/, monitoring/, terraform/
- `infrastructure/`: ansible/
- `deployment/`: ethereum/, services/, substrate/

**New structure:**
```
infra-deployment/
├── docker/              (from infra/)
├── terraform/           (from infra/)
├── ansible/             (from infrastructure/)
├── ethereum/            (from deployment/)
├── services/            (from deployment/)
├── substrate/           (from deployment/)
└── README.md            (consolidated docs)
```

#### 3B: Audit Package Consolidation

**Merge:** `audit-package/` + `audit-package-2025-10-21/` → `dev-audit/`

**Contents:**
```
dev-audit/
├── 2025-10-21/          (dated audit)
│   ├── CI_CD_VALIDATION_SUMMARY.md
│   ├── deployment-production.md
│   ├── KNOWN_ISSUES.md
│   ├── PACKAGE_STATISTICS.md
│   ├── README.md
│   ├── SECURITY_SCAN_SUMMARY.md
│   ├── TEST_COVERAGE_ANALYSIS.md
│   ├── wasm_runtimes/
│   └── ...
├── latest/              (current audit package)
│   ├── AUDIT_DELIVERY_INSTRUCTIONS.md
│   ├── AUDIT_MATERIALS_INDEX.md
│   ├── AUDIT_PACKAGE_DELIVERY_CHECKLIST.md
│   ├── AUDIT_SUMMARY.txt
│   ├── DEPLOYMENT_READINESS_REPORT.md
│   └── ...
└── README.md            (explains structure)
```

---

### PHASE 4: Reorganize Infrastructure Folders ✅

**Actions:**
```bash
mv monitoring/ infra-monitoring/
mv chain-specs/ infra-chain-specs/
mv config/ infra-config/
```

**Result:**
```
infra-deployment/        (deployment infrastructure)
infra-monitoring/        (Prometheus, Grafana, alerts)
infra-chain-specs/       (chain specification files)
infra-config/            (configuration files)
```

---

### PHASE 5: Reorganize Development Folders ✅

**Actions:**
```bash
mv tests/ dev-tests/
mv coverage/ dev-coverage/
mv scripts/ dev-scripts/         # Non-deployment scripts only
```

**Note:** Deployment scripts stay in root `scripts/` for convenience

**Result:**
```
dev-tests/               (integration, e2e, property-based tests)
dev-coverage/            (test coverage reports)
dev-audit/               (audit packages)
dev-scripts/             (development helper scripts)
```

---

### PHASE 6: Move Pallets to Source ✅

**Action:**
```bash
mv pallets/ src/pallets/
```

**Rationale:** Custom pallets are source code, should be with runtime

**Current pallets:**
- pallet-custodian-registry
- pallet-reserve-vault
- pallet-reserve-oracle
- pallet-did-registry
- pallet-circuit-breaker
- pallet-edsc-redemption
- pallet-xcm-bridge
- pallet-validator-committee
- pallet-aidid

**New location:** `src/pallets/*`

---

### PHASE 7: Consolidate Vendor to Reference ✅

**Action:**
```bash
mv vendor/substrate-prometheus-endpoint/ _reference/substrate-prometheus-endpoint/
rmdir vendor/
```

**Rationale:** External dependencies belong in `_reference/`

---

### PHASE 8: Handle Tools Folder ✅

**Option A:** Delete (if cli is unused)
**Option B:** Move to dev-scripts/cli/ (if used)

**Recommendation:** Check if `tools/cli` is actively used:
- If YES → `mv tools/cli/ dev-scripts/cli/`
- If NO → `rm -rf tools/`

---

## Final Directory Structure

```
etrid/
│
├── 📦 ESSENTIAL PROJECT FILES
│   ├── README.md
│   ├── CONTRIBUTING.md
│   ├── CODE_OF_CONDUCT.md
│   ├── SECURITY.md
│   ├── LICENSE
│   ├── CHANGELOG.md
│   ├── LIVING_ROADMAP.md
│   ├── Cargo.toml
│   ├── Makefile
│   ├── Dockerfile
│   └── docker-compose*.yml
│
├── 🔢 CORE COMPONENTS (01-14) - In Dependency Order
│   ├── 01-detr-p2p/
│   ├── 02-open-did/
│   ├── 03-security/
│   ├── 04-accounts/
│   ├── 05-multichain/
│   ├── 06-native-currency/
│   ├── 07-transactions/
│   ├── 08-etwasm-vm/
│   ├── 09-consensus/
│   ├── 10-foundation/
│   ├── 11-peer-roles/
│   ├── 12-consensus-day/
│   ├── 13-clients/
│   └── 14-aidevs/                  (renamed from ai-devs/)
│
├── 🏗️ INFRASTRUCTURE (infra-)
│   ├── infra-deployment/           (consolidated infra/ + infrastructure/ + deployment/)
│   ├── infra-monitoring/           (renamed from monitoring/)
│   ├── infra-chain-specs/          (renamed from chain-specs/)
│   └── infra-config/               (renamed from config/)
│
├── 🔧 DEVELOPMENT (dev-)
│   ├── dev-tests/                  (renamed from tests/)
│   ├── dev-coverage/               (renamed from coverage/)
│   ├── dev-audit/                  (consolidated audit packages)
│   └── dev-scripts/                (dev helper scripts, not deployment)
│
├── 💻 SOURCE CODE
│   ├── src/                        (main runtime)
│   └── src/pallets/                (moved from pallets/)
│
├── 📱 APPLICATIONS
│   └── apps/                       (wallet, dashboards, etc.)
│
├── 📜 SMART CONTRACTS
│   └── contracts/                  (ethereum, etwasm-examples)
│
├── 📚 DOCUMENTATION
│   └── docs/                       (all documentation)
│
├── 🔌 INTEGRATIONS
│   ├── sdk/                        (SDKs for various languages)
│   └── services/                   (microservices)
│
├── 🚀 DEPLOYMENT AUTOMATION
│   └── scripts/                    (deployment scripts - kept at root for convenience)
│       ├── master-deploy.sh
│       ├── pre-deployment-tests.sh
│       ├── setup-forum.sh
│       ├── backup-forum.sh
│       └── restore-forum.sh
│
├── 📖 EXTERNAL REFERENCES (_reference-)
│   ├── _reference/                 (existing submodules)
│   └── _reference-vendor/          (moved from vendor/)
│
└── 🔐 GITHUB/CI
    ├── .github/                    (GitHub Actions)
    └── .claude/                    (Claude Code config)
```

---

## Updated .gitignore

```gitignore
# Test directories (temp artifacts)
.bridge-test/
.local-testnet/
.multichain-test/
.peering-test/
.test-network/
.validator-test/
.bfg-report/

# Build artifacts
target/
node_modules/
*.log
*.profraw
build*.log
test_results.log
coverage/

# Environment files
.env
!.env.example
!.env.*.example

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store

# Chain specs (generated)
*.raw.json
```

---

## Naming Convention Rules

### For Future Additions:

1. **Core Components (01-14):**
   - Add to numbered sequence if fundamental to blockchain operation
   - Use: `NN-component-name/`
   - Next available: `15-*`

2. **Infrastructure:**
   - Anything related to deployment, monitoring, configuration
   - Use: `infra-purpose/`
   - Examples: `infra-cdn/`, `infra-dns/`

3. **Development:**
   - Testing, tooling, audits, development helpers
   - Use: `dev-purpose/`
   - Examples: `dev-benchmarks/`, `dev-profiling/`

4. **Applications:**
   - User-facing apps, dashboards
   - Keep in: `apps/subname/`

5. **External References:**
   - Git submodules, vendored dependencies
   - Use: `_reference-name/` or add to `_reference/`

---

## Migration Checklist

### Immediate Actions (Can do safely):
- [ ] Delete test directories (.bridge-test, .local-testnet, etc.)
- [ ] Delete .bfg-report/
- [ ] Rename ai-devs/ → 14-aidevs/

### Consolidation (Requires careful merging):
- [ ] Merge infra/ + infrastructure/ + deployment/ → infra-deployment/
- [ ] Merge audit packages → dev-audit/
- [ ] Move vendor/ → _reference-vendor/

### Renaming (Update any hardcoded paths):
- [ ] monitoring/ → infra-monitoring/
- [ ] chain-specs/ → infra-chain-specs/
- [ ] config/ → infra-config/
- [ ] tests/ → dev-tests/
- [ ] coverage/ → dev-coverage/

### Source Reorganization (Update Cargo.toml):
- [ ] Move pallets/ → src/pallets/
- [ ] Update Cargo.toml workspace members
- [ ] Test build after move

### Handle tools/:
- [ ] Check if tools/cli is used
- [ ] If yes: move to dev-scripts/cli/
- [ ] If no: delete tools/

---

## Benefits of This Structure

### For Developers:
- **Clear hierarchy:** Core (01-14) > Infrastructure > Development > Apps
- **Easy navigation:** Prefixes indicate purpose instantly
- **Logical grouping:** Related folders together

### For Operations:
- **All infra together:** infra-* folders easy to find
- **All dev tools together:** dev-* folders for development
- **Deployment clarity:** Scripts at root, infra in infra-deployment/

### For New Contributors:
- **Numbered core:** Shows dependency order
- **Descriptive prefixes:** Self-documenting structure
- **Reference material separate:** `_reference-*` clearly external

### For Maintenance:
- **Consistent naming:** Easy to extend with new folders
- **No confusion:** Duplicate folder names impossible
- **Future-proof:** Convention scales with project growth

---

## Execution Order

1. ✅ Delete temp/test directories
2. ✅ Rename ai-devs → 14-aidevs
3. ✅ Consolidate audit packages
4. ✅ Consolidate infrastructure folders
5. ✅ Rename infrastructure folders (infra- prefix)
6. ✅ Rename development folders (dev- prefix)
7. ✅ Move pallets to src/
8. ✅ Move vendor to _reference
9. ✅ Handle tools folder
10. ✅ Update .gitignore
11. ✅ Update any hardcoded paths in code/scripts
12. ✅ Test build
13. ✅ Create migration summary

---

**Status:** Plan ready for execution
**Estimated Time:** 2-3 hours
**Risk Level:** Low (mostly renames and consolidations)

---

*Plan created: October 25, 2025*
*Convention: Category-based prefixes with clear hierarchy*
