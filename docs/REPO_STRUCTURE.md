# ËTRID Repository Structure

## Root Directory Files

### 📋 Start Here
- **START_HERE.md** - Main entry point for deployment
- **README.md** - Project overview and getting started
- **EXECUTIVE_SUMMARY.md** - High-level deployment overview

### 🚀 Deployment Documentation
- **AGENT_DEPLOYMENT_GUIDE.md** - Complete agent deployment guide
- **DEPLOYMENT_CHECKLIST.md** - Deployment verification checklist
- **DEPLOYMENT_PACKAGE_README.md** - Package overview
- **DEPLOYMENT_PACKAGE_CONTENTS.txt** - Full inventory
- **QUICK_DEPLOYMENT_REFERENCE.md** - Quick command reference

### 📜 Project Documentation
- **CHANGELOG.md** - Version history
- **CODE_OF_CONDUCT.md** - Community guidelines
- **CONTRIBUTING.md** - Contribution guidelines
- **SECURITY.md** - Security policy

### 🔧 Deployment Scripts
- **deploy-monitoring-agents-parallel.sh** - Main deployment script
- **deploy-complete-ai-system.sh** - AI system deployment
- **deploy-monitoring-infrastructure.sh** - Prometheus/Grafana deployment
- **deploy-node-exporters-fixed.sh** - Node exporter deployment
- **insert-validator-keys-accessible.sh** - Key insertion script
- **install-etrid-monitoring.sh** - Universal installer

## Directory Structure

```
etrid/
├── START_HERE.md                           ⭐ Read this first
├── README.md                               📖 Project overview
├── EXECUTIVE_SUMMARY.md                    📊 Deployment overview
│
├── Deployment Documentation/
│   ├── AGENT_DEPLOYMENT_GUIDE.md          🔍 Complete guide
│   ├── DEPLOYMENT_CHECKLIST.md            ✅ Verification steps
│   ├── DEPLOYMENT_PACKAGE_README.md       📦 Package overview
│   ├── DEPLOYMENT_PACKAGE_CONTENTS.txt    📋 Full inventory
│   └── QUICK_DEPLOYMENT_REFERENCE.md      ⚡ Quick reference
│
├── Deployment Scripts/
│   ├── deploy-monitoring-agents-parallel.sh    🚀 Main deployment
│   ├── deploy-complete-ai-system.sh            🤖 AI system
│   ├── deploy-monitoring-infrastructure.sh      📊 Monitoring
│   ├── deploy-node-exporters-fixed.sh           📈 Metrics
│   ├── insert-validator-keys-accessible.sh      🔑 Keys
│   └── install-etrid-monitoring.sh              💾 Installer
│
├── ai-monitoring/                          🧠 AI system code
│   ├── orchestrator.py                     Main coordinator
│   ├── ai_dev_workers.py                   AI workers
│   ├── validator_monitor.py                Monitoring logic
│   ├── ai_router.py                        Multi-AI routing
│   ├── .env.example                        Config template
│   ├── skills/                             AI dev skills
│   └── dids/                               DID documents
│
├── docs/                                   📚 Documentation
│   ├── README.md                           Main docs
│   ├── architecture.md                     Architecture
│   ├── devnet/                             🧪 DevNet docs
│   │   ├── README.md
│   │   ├── DEVNET_QUICK_START.md
│   │   ├── DEVNET_DEPLOYMENT_GUIDE.md
│   │   ├── DEVNET_DEPLOYMENT_SUMMARY.md
│   │   ├── DEVNET_TOOLS_README.md
│   │   └── README_DEVNET_TOOLKIT.md
│   ├── deployment-archive/                 📦 Historical docs
│   │   ├── DEPLOYMENT_MASTER_PLAN.md
│   │   ├── DEPLOYMENT_REPORT_2025-11-01.md
│   │   ├── FINAL_DEPLOYMENT_SUMMARY.md
│   │   └── MAINNET_DEPLOYMENT_HANDOFF.md
│   └── [other docs]
│
├── scripts/                                🔧 Utility scripts
│   ├── archive/                            Old script versions
│   ├── devnet/                             DevNet scripts
│   │   ├── deploy-devnet-test-keys.sh
│   │   ├── manage-devnet-nodes.sh
│   │   └── verify-devnet-nodes.sh
│   └── calculate-multisig.js               Foundation multisig
│
├── 01-detr-p2p/                            Network layer
├── 02-open-did/                            Identity
├── 05-multichain/                          Bridge protocols
├── 06-native-currency/                     ETR token
├── 09-consensus/                           Consensus
└── [other components]
```

## Quick Navigation

### First Time Here?
1. Read **START_HERE.md**
2. Read **EXECUTIVE_SUMMARY.md**
3. Follow **QUICK_DEPLOYMENT_REFERENCE.md**

### Deploying?
1. **AGENT_DEPLOYMENT_GUIDE.md** - Complete instructions
2. **DEPLOYMENT_CHECKLIST.md** - Verify each step
3. Run `deploy-monitoring-agents-parallel.sh`

### Contributing?
1. **CONTRIBUTING.md** - How to contribute
2. **CODE_OF_CONDUCT.md** - Community guidelines
3. **README.md** - Project overview

### Need Details?
- Historical deployment docs: `docs/deployment-archive/`
- Technical architecture: `docs/architecture.md`
- API reference: `docs/API_REFERENCE.md`

## File Count Summary

- **Root documentation:** 12 files (essential only)
- **Deployment scripts:** 6 files (all functional)
- **AI monitoring code:** 6 Python files + configs
- **Archived docs:** 5 files (historical reference)

## What Was Cleaned Up

### Phase 1 (Initial Cleanup)
**Removed (redundant/temporary):**
- CLEANUP_SUMMARY.md (old cleanup notes)
- CONTINUE_IN_NEW_TERMINAL.txt (temporary file)
- COPY_THIS_TO_OTHER_VMS.md (replaced by agent docs)
- PROMPT_FOR_OTHER_VMS.txt (replaced by agent docs)
- QUICK_DEPLOY_INSTRUCTIONS.md (replaced by QUICK_DEPLOYMENT_REFERENCE.md)
- SAFE_TO_COMMIT.md (covered in CONTRIBUTING.md)
- WHATS_SAVED_WHERE.md (covered in README.md)

**Archived (historical reference):**
- DEPLOYMENT_MASTER_PLAN.md → docs/deployment-archive/
- DEPLOYMENT_REPORT_2025-11-01.md → docs/deployment-archive/
- FINAL_DEPLOYMENT_SUMMARY.md → docs/deployment-archive/
- DEPLOYMENT_STATUS.md → docs/deployment-archive/
- MAINNET_DEPLOYMENT_HANDOFF.md → docs/deployment-archive/

### Phase 2 (Deep Cleanup - 11.5+ GB Freed)
**Build Artifacts Removed:**
- target/ directory (7.7 GB - Rust build artifacts)
- node_modules/ in dex-deployment (3.8 GB - NPM dependencies)
- binaries/ directory (78 MB - compiled binaries)
- etrid-monitoring-package.tar.gz (46 KB - package artifact)

**System Files Removed:**
- All .DS_Store files (macOS metadata)
- _archive/ directory (old configs/docs/scripts)
- _reference/ directory (cosmos-sdk, substrate references)
- ..bfg-report/ directory (BFG cleanup report)

**Redundant Configs Removed:**
- .env.forum.example (old forum config)
- .gitignore.multi-node (old multi-node config)

**Files Reorganized:**
- DEVNET_*.md (5 files) → docs/devnet/
- README_DEVNET_TOOLKIT.md → docs/devnet/
- deploy-devnet-test-keys.sh → scripts/devnet/
- manage-devnet-nodes.sh → scripts/devnet/
- verify-devnet-nodes.sh → scripts/devnet/
- calculate-multisig.js → scripts/
- Old node-exporter scripts → scripts/archive/

## Repository Size

**Before Phase 1 cleanup:**
- ~25 documentation files in root
- ~200 KB of docs

**After Phase 1 cleanup:**
- 12 essential documentation files
- ~100 KB of docs
- 5 archived files in docs/deployment-archive/

**Before Phase 2 cleanup:**
- ~36.5 GB total repository size
- 11.5+ GB of build artifacts and bloat
- ~90 items in root directory

**After Phase 2 cleanup:**
- ~25 GB total repository size
- 0 GB of build artifacts (properly excluded)
- ~75 items in root directory
- Better organization with docs/devnet/ and scripts/devnet/

**Total Result:**
- 50% reduction in root documentation clutter
- 11.5+ GB disk space freed
- 15 fewer items in root directory
- Improved organization and structure

---

**Last Updated:** 2025-11-01
