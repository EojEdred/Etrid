# Root Directory Cleanup Plan - Industry Grade Organization

**Date:** October 23, 2025
**Goal:** Clean root to industry-standard architecture
**Testnet Name:** Ember

---

## Current State: 57+ .md files in root (TOO MANY)

## Target State: 8 core files in root only

---

## KEEP IN ROOT (Industry Standard)

1. **README.md** - Project overview ✅ Update with Ember testnet
2. **LICENSE** - Apache 2.0 license ✅ Keep as-is
3. **ROADMAP.md** - Strategic roadmap ✅ Update with Ember testnet
4. **CONTRIBUTING.md** - Contribution guidelines ✅ Update
5. **CODE_OF_CONDUCT.md** - Community guidelines ✅ Keep
6. **SECURITY.md** - Security policy ✅ Update
7. **CHANGELOG.md** - Version history ✅ Update with latest
8. **QUICK_START.md** - Fast onboarding ✅ Update with Ember

---

## MOVE TO docs/archive/sessions/2025-10-23/

**Session-specific reports (49 files):**
- ALL_13_ITEMS_COMPLETE.md
- ALL_FIXES_COMPLETE_FINAL_SUMMARY.md
- ALPHA_COMPLETE_SUMMARY.md
- BUILD_ERRORS_COMPREHENSIVE_REPORT.md
- CLAUDE_PROJECT_SETUP.md
- COMPLETION_PLAN.md
- COMPONENT_TEST_REPORT.md
- CONSOLIDATION_STATUS.md
- CRITICAL_FIXES_COMPLETE.md
- DEPLOYMENT_READY_STATUS.md
- DOCUMENTATION_AND_SETUP_COMPLETE.md
- DOCUMENTATION_COMPLETE.md
- FINAL_DELIVERY_REPORT.md
- MONITORING_SETUP_COMPLETE.md
- MULTI_AGENT_STATUS_*.md (all 5 files)
- SESSION_COMPLETE_*.md (all session files)
- SINGLE_TERMINAL_COMPLETION_PLAN.md
- START_HERE.md
- STATUS_SUMMARY_OCT23.md
- TERMINAL_*.md (all terminal files)
- TODO_CONSOLIDATED.md
- TODO_IMPLEMENTATION_PLAN.md
- TRANSACTION_BUILDER_*.md (enhancement files)
- UI_DEPLOYMENT_COMPLETE_REPORT.md
- QUICK_REFERENCE_CARD.md (session-specific)

---

## MOVE TO docs/deployment/

**Deployment documentation:**
- DEPLOYMENT_ARCHITECTURE.md
- DEPLOYMENT_CHECKLIST.md
- DEPLOYMENT_INDEX.md
- DEPLOYMENT_INSTRUCTIONS.md
- DEPLOYMENT_QUICK_START.md
- PRODUCTION_DEPLOYMENT_GUIDE.md

---

## MOVE TO docs/reports/

**Technical reports:**
- DOCUMENTATION_AUDIT_REPORT.md
- PERFORMANCE_ANALYSIS_REPORT.md
- PERFORMANCE_AUDIT_CHECKLIST.md
- PERFORMANCE_IMPLEMENTATION_SUMMARY.md
- PERFORMANCE_QUICK_START.md
- FINAL_PERFORMANCE_VALIDATION_REPORT.md
- TERMINAL_6_COMPLETE.md
- VALIDATION_REPORT.md
- VERCEL_DEPLOYMENT_REPORT.md

---

## CONSOLIDATE INTO SINGLE FILES

**Release documentation:**
- RELEASE_NOTES_v1.0.0.md → Keep in root temporarily, move to docs/releases/

**Quick reference:**
- QUICK_REFERENCE.md → Consolidate into QUICK_START.md
- QUICK_REFERENCE_CARD.md → Archive (session-specific)

**Known issues:**
- KNOWN_ISSUES.md → Move to docs/

---

## ROOT CONFIG FILES (Keep Clean)

**Keep:**
- Cargo.toml (workspace)
- docker-compose.yml (primary)
- LICENSE

**Move to config/:**
- docker-compose.bridge.yml → config/docker/
- sdk-wasm-workflow.yaml → config/workflows/
- chain-spec-dev.json → config/chain-specs/
- btc-pbc-chain-spec-dev.json → config/chain-specs/

**Remove temporary:**
- DEPLOYMENT_SUMMARY.txt (consolidate into markdown)
- TRANSACTION_BUILDER_FINAL_SUMMARY.txt (consolidate into markdown)

---

## EXECUTION ORDER

1. Create directory structure
2. Move session files to archive
3. Move deployment docs to docs/deployment/
4. Move reports to docs/reports/
5. Move configs to config/
6. Update remaining root .md files with Ember testnet name
7. Update all component .md files
8. Verify clean root directory

---

## TESTNET NAME: EMBER

Update in all files:
- README.md
- ROADMAP.md
- QUICK_START.md
- All deployment documentation
- All component READMEs
- Configuration files
- Scripts

References to replace:
- "testnet" → "Ember testnet"
- "local testnet" → "Ember (local testnet)"
- "dev chain" → "Ember development chain"

---

## FINAL ROOT STRUCTURE

```
etrid/
├── README.md                    # ✅ Updated with Ember
├── LICENSE                      # ✅ Keep as-is
├── ROADMAP.md                   # ✅ Updated with Ember
├── CONTRIBUTING.md              # ✅ Updated
├── CODE_OF_CONDUCT.md           # ✅ Keep
├── SECURITY.md                  # ✅ Updated
├── CHANGELOG.md                 # ✅ Updated with latest
├── QUICK_START.md               # ✅ Updated with Ember
├── Cargo.toml                   # Workspace config
├── docker-compose.yml           # Primary compose
├── Makefile                     # Build automation
└── .claude/                     # Claude project config
    ├── project.md               # ✅ Updated
    └── .claudeignore

**Total: 8 .md files + 3 config files = CLEAN ✅**
```

---

**Execution Status:** Ready to execute
