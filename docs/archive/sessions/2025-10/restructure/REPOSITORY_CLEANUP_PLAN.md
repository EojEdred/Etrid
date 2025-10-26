# Repository Cleanup & Organization Plan

**Date:** October 25, 2025
**Status:** 🔄 In Progress

---

## Current Problems

### 1. Root Level (32 markdown files)
**Problem:** Too many files at root making navigation difficult

**Current Files:**
- 3x SESSION_SUMMARY files
- 2x HANDOFF files
- 3x QUICK_START files
- 3x MASTER_PLAN files
- 3x *_COMPLETE files
- 2x ROADMAP files
- 2x START_HERE files
- Multiple log files (build.log, build-new.log, build-fixed.log)

### 2. docs/archive (duplicative and bloated)
**Problem:** 318KB CONSOLIDATED files, duplicate sessions, original-files folder

**Current Structure:**
```
docs/archive/
├── CONSOLIDATED_SESSIONS.md (318KB!)
├── CONSOLIDATED_STATUS_REPORTS.md (40KB)
├── sessions/ (40 files - duplicates)
├── original-files/ (40 files - duplicates)
├── consolidated-sources/ (10 files)
├── scripts/ (13 files)
└── development-artifacts/ (5 files)
```

### 3. ai-devs/ directory
**Problem:** Mix of current and outdated guides

---

## Cleanup Actions

### Phase 1: Consolidate Root Markdown Files

**Keep at Root (Essential):**
1. README.md
2. CONTRIBUTING.md
3. CODE_OF_CONDUCT.md
4. SECURITY.md
5. LICENSE
6. CHANGELOG.md
7. LIVING_ROADMAP.md (active roadmap)

**Move to docs/:**
- QUICK_START.md → docs/QUICK_START.md
- FINAL_DEPLOYMENT_CHECKLIST.md → docs/FINAL_DEPLOYMENT_CHECKLIST.md

**Move to docs/archive/sessions/2025-10/:**
- SESSION_SUMMARY_OCT24.md
- SESSION_SUMMARY_OCT24_AIDEVS_TESTING.md
- SESSION_SUMMARY_OCT24_TESTS.md
- HANDOFF_EXCHANGE_EXPANSION.md
- HANDOFF_TO_NEXT_SESSION.md
- PASTE_INTO_NEXT_CLAUDE.txt
- NEXT_SESSION_PRIORITIES.md
- NEXT_STEPS.md

**Consolidate & Archive:**
- AI_DEVS_*.md (4 files) → docs/archive/sessions/2025-10/AI_DEVS_SUMMARY.md
- DEX_*.md (3 files) → docs/archive/sessions/2025-10/DEX_EXPANSION_SUMMARY.md
- EXCHANGE_*.md (4 files) → docs/archive/sessions/2025-10/EXCHANGE_LISTING_SUMMARY.md
- *_COMPLETE.md (3 files) → docs/archive/sessions/2025-10/COMPLETION_REPORTS.md
- START_HERE_*.md (2 files) → docs/archive/sessions/2025-10/START_HERE_ARCHIVE.md
- IMPLEMENTATION_PLAN_2_WEEKS.md → docs/archive/sessions/2025-10/
- CRITICAL_DECISIONS_NEEDED.md → docs/archive/sessions/2025-10/
- ROADMAP.md (old) → DELETE (superseded by LIVING_ROADMAP.md)

### Phase 2: Clean docs/archive/

**Delete Entirely:**
- docs/archive/CONSOLIDATED_SESSIONS.md (318KB, outdated)
- docs/archive/CONSOLIDATED_STATUS_REPORTS.md (40KB, outdated)
- docs/archive/original-files/ (all duplicates)
- docs/archive/collator_test_results.txt
- docs/archive/BRIDGE_CONFIG_TRAITS.txt
- docs/archive/QUICK_START_NEW_SESSION.txt

**Keep & Organize:**
- docs/archive/sessions/ → Rename to docs/archive/sessions/2025-10-pre-launch/
- docs/archive/development-artifacts/ → Keep
- docs/archive/DOCUMENTATION_CONSOLIDATION_PLAN.md → Keep
- docs/archive/README.md → Update

### Phase 3: Clean Build Artifacts

**Delete:**
- build.log (generated file)
- build-new.log (generated file)
- build-fixed.log (generated file)
- test_results.log (generated file)
- build_rs_cov.profraw (coverage file)

**Add to .gitignore:**
```
*.log
*.profraw
build*.log
test_results.log
```

### Phase 4: Organize ai-devs/

**Current State:** Mix of active guides and old plans

**Actions:**
- Keep all recent guides (TREASURY, FORUM, DEX, MONITORING, CI_CD, PRE_DEPLOYMENT)
- Move old summaries to docs/archive/sessions/2025-10/

**Final Structure:**
```
ai-devs/
├── COMPLETE_FEATURES_IMPLEMENTATION.md (keep)
├── CI_CD_PIPELINE_GUIDE.md (keep)
├── DEX_DEPLOYMENT_GUIDE.md (keep)
├── GOVERNANCE_FORUM_GUIDE.md (keep)
├── MONITORING_INFRASTRUCTURE_GUIDE.md (keep)
├── PRE_DEPLOYMENT_COMPLETE_SUMMARY.md (keep)
├── TREASURY_GOVERNANCE_GUIDE.md (keep)
└── VESTING_GENESIS_GUIDE.md (keep)
```

### Phase 5: Final Root Structure

**After Cleanup:**
```
etrid/
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── LICENSE
├── CHANGELOG.md
├── LIVING_ROADMAP.md
├── Cargo.toml
├── Makefile
├── Dockerfile
├── docker-compose.yml
├── docker-compose.governance-forum.yml
├── .gitignore (updated)
├── src/
├── pallets/
├── 01-14/ (component directories)
├── scripts/
├── docs/
├── ai-devs/
├── apps/
├── contracts/
└── ... (other essential directories)
```

---

## Estimated Impact

**Files Deleted:** ~50
**Files Moved:** ~25
**Files Consolidated:** ~15 → 5
**Space Saved:** ~400KB+ documentation bloat

---

## Execution Order

1. Create archive directory structure
2. Consolidate root markdown files
3. Move files to appropriate locations
4. Delete redundant/outdated files
5. Clean build artifacts
6. Update .gitignore
7. Verify no broken links
8. Create summary document

---

**Status:** Ready for execution
