#!/bin/bash
# Ëtrid Protocol - Phase 2: Reorganize Documentation
# Moves 51 session reports from root to docs/archive/sessions/2025-10/
# Consolidates audit materials to audit-package/

set -e

ETRID_ROOT="/Users/macbook/Desktop/etrid"
cd "$ETRID_ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║                                                              ║"
echo "║         ËTRID PROTOCOL - PHASE 2 DOCUMENTATION              ║"
echo "║         Reorganizing Root Directory                         ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo

# Count initial files
INITIAL_COUNT=$(ls -1 *.md *.txt 2>/dev/null | wc -l)
echo -e "${BLUE}Root files before cleanup: $INITIAL_COUNT${NC}"
echo

# Step 1: Create archive directory
echo -e "${YELLOW}Step 1: Creating archive directory structure...${NC}"
mkdir -p docs/archive/sessions/2025-10
echo -e "${GREEN}✓ Created docs/archive/sessions/2025-10/${NC}"
echo

# Step 2: Move Terminal session reports
echo -e "${YELLOW}Step 2: Moving Terminal session reports...${NC}"
MOVED=0
for file in TERMINAL*.md CURRENT_STATUS_TERMINAL*.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/archive/sessions/2025-10/
        echo "  Moved: $file"
        MOVED=$((MOVED + 1))
    fi
done
echo -e "${GREEN}✓ Moved $MOVED terminal reports${NC}"
echo

# Step 3: Move Phase reports
echo -e "${YELLOW}Step 3: Moving Phase completion reports...${NC}"
MOVED=0
for file in PHASE3*.md *_STEP_*.md *SESSION*.md PARALLEL_WORK*.md FINAL_*.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/archive/sessions/2025-10/
        echo "  Moved: $file"
        MOVED=$((MOVED + 1))
    fi
done
echo -e "${GREEN}✓ Moved $MOVED phase reports${NC}"
echo

# Step 4: Move Option/Test reports
echo -e "${YELLOW}Step 4: Moving Option and test reports...${NC}"
MOVED=0
for file in OPTION*.md ORACLE_TEST*.md POLISH_WORK*.md TESTNET_DEPLOYMENT_COMPLETE.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/archive/sessions/2025-10/
        echo "  Moved: $file"
        MOVED=$((MOVED + 1))
    fi
done
echo -e "${GREEN}✓ Moved $MOVED option/test reports${NC}"
echo

# Step 5: Move Technical reports
echo -e "${YELLOW}Step 5: Moving technical reports...${NC}"
MOVED=0
for file in ASF_RUNTIME*.md DOCUMENTATION_RESTRUCTURING*.md GIT_CORRUPTION*.md RESERVE_VAULT*.md RUNTIME_API*.md SC_CONSENSUS*.md TESTNET_DEPLOYMENT_GUIDE.md; do
    if [ -f "$file" ]; then
        mv "$file" docs/archive/sessions/2025-10/
        echo "  Moved: $file"
        MOVED=$((MOVED + 1))
    fi
done
echo -e "${GREEN}✓ Moved $MOVED technical reports${NC}"
echo

# Step 6: Consolidate audit materials
echo -e "${YELLOW}Step 6: Consolidating audit materials...${NC}"
mkdir -p audit-package
MOVED=0
for file in AUDIT_*.md AUDIT_*.txt DEPLOYMENT_READINESS_REPORT.md; do
    if [ -f "$file" ]; then
        mv "$file" audit-package/
        echo "  Moved: $file"
        MOVED=$((MOVED + 1))
    fi
done
echo -e "${GREEN}✓ Moved $MOVED audit files to audit-package/${NC}"
echo

# Step 7: Create session index
echo -e "${YELLOW}Step 7: Creating session archive index...${NC}"
cat > docs/archive/sessions/2025-10/README.md << 'EOF'
# October 2025 Development Sessions

This directory contains session reports and status updates from October 2025 development work.

## Contents

### Terminal Session Reports
- Terminal 1: PPFA Block Sealing
- Terminal 2: Bridge Security
- Terminal 3: Network Layer Polish
- Terminal 4: Property-Based Testing & Documentation
- Terminal 7: Oracle Test Implementation

### Phase 3 Reports
- Phase 3 completion reports
- Parallel work coordination
- Final integration status

### Test & Implementation Reports
- Option B test plan
- Oracle test implementation
- Polish work completion
- Testnet deployment

## Consolidated Reports

For consolidated summaries, see:
- `../CONSOLIDATED_SESSIONS.md` - All session reports merged
- `../CONSOLIDATED_STATUS_REPORTS.md` - All status reports merged

## Current Status

See root directory:
- `KNOWN_ISSUES.md` - Current known issues
- `AUDIT_PACKAGE.md` - Security audit package

---

**Archive Date:** October 2025
**Total Reports:** 51 files
EOF
echo -e "${GREEN}✓ Created archive index${NC}"
echo

# Count final files
FINAL_COUNT=$(ls -1 *.md *.txt 2>/dev/null | wc -l)
FILES_MOVED=$((INITIAL_COUNT - FINAL_COUNT))

echo
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}                 REORGANIZATION COMPLETE                   ${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════════${NC}"
echo
echo -e "${BLUE}Root files before: $INITIAL_COUNT${NC}"
echo -e "${GREEN}Root files after:  $FINAL_COUNT${NC}"
echo -e "${YELLOW}Files moved:       $FILES_MOVED${NC}"
echo
echo -e "${BLUE}Essential files remaining in root:${NC}"
ls -1 *.md 2>/dev/null | head -10
echo
echo -e "${GREEN}✅ Phase 2 Complete - Root directory cleaned${NC}"
echo
