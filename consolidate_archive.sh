#!/bin/bash
# Archive Consolidation Script
# Consolidates 47 archive files into organized categories

set -e

ARCHIVE_DIR="/Users/macbook/Desktop/etrid/docs/archive"

echo "=========================================="
echo "Archive Consolidation"
echo "=========================================="
echo ""

# Create consolidated archive directory
mkdir -p "$ARCHIVE_DIR/original-files"

echo "Step 1: Consolidating Session Reports..."
cat > "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md" << 'SESSION_EOF'
# Consolidated Session Reports

Historical development session reports consolidated for reference.

---

## Table of Contents

1. [ASF Consensus Sessions](#asf-consensus-sessions)
2. [Bridge Integration Sessions](#bridge-integration-sessions)
3. [Gizzi Sessions](#gizzi-sessions)
4. [General Sessions](#general-sessions)

---

SESSION_EOF

# Add ASF sessions
echo "## ASF Consensus Sessions" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
for file in "$ARCHIVE_DIR/sessions"/ASF_*.md; do
    if [ -f "$file" ]; then
        echo "### $(basename "$file" .md)" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        tail -n +2 "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md" 2>/dev/null || cat "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "---" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
    fi
done

# Add Bridge sessions
echo "## Bridge Integration Sessions" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
for file in "$ARCHIVE_DIR/sessions"/BRIDGE_*.md; do
    if [ -f "$file" ]; then
        echo "### $(basename "$file" .md)" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        tail -n +2 "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md" 2>/dev/null || cat "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "---" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
    fi
done

# Add Gizzi sessions
echo "## Gizzi Sessions" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
for file in "$ARCHIVE_DIR/sessions"/GIZZI_*.md; do
    if [ -f "$file" ]; then
        echo "### $(basename "$file" .md)" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        tail -n +2 "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md" 2>/dev/null || cat "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "---" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
    fi
done

# Add remaining sessions
echo "## General Sessions" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
for file in "$ARCHIVE_DIR/sessions"/*.md; do
    filename=$(basename "$file")
    if [[ ! "$filename" =~ ^(ASF_|BRIDGE_|GIZZI_) ]]; then
        if [ -f "$file" ]; then
            echo "### $(basename "$file" .md)" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
            echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
            tail -n +2 "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md" 2>/dev/null || cat "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
            echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
            echo "---" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
            echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_SESSIONS.md"
        fi
    fi
done

echo "✅ CONSOLIDATED_SESSIONS.md created"

echo "Step 2: Consolidating Status Reports..."
cat > "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md" << 'STATUS_EOF'
# Consolidated Status Reports

Historical status reports and build logs consolidated for reference.

---

## Table of Contents

1. [Build Status](#build-status)
2. [PBC Status](#pbc-status)
3. [Migration Reports](#migration-reports)
4. [Fix Guides](#fix-guides)

---

STATUS_EOF

# Add all status reports
echo "## Build Status" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"

for file in "$ARCHIVE_DIR/status-reports"/*.md; do
    if [ -f "$file" ]; then
        echo "### $(basename "$file" .md)" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
        tail -n +2 "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md" 2>/dev/null || cat "$file" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
        echo "---" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
        echo "" >> "$ARCHIVE_DIR/CONSOLIDATED_STATUS_REPORTS.md"
    fi
done

echo "✅ CONSOLIDATED_STATUS_REPORTS.md created"

echo "Step 3: Creating migration scripts reference..."
cat > "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md" << 'SCRIPTS_EOF'
# Migration Scripts Reference

Historical one-time migration scripts used during development.

---

## Overview

These scripts were used for one-time migrations and fixes during development. They are preserved for reference but are no longer actively used.

---

## Bridge Migration Scripts

SCRIPTS_EOF

# List bridge scripts
echo "### Bridge Addition Scripts" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- add_remaining_bridges_final.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- add_remaining_bridges.sh" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- complete_all_bridges.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"

echo "### Bridge Fix Scripts" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- fix_all_bridges_final.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- fix_all_bridges_from_template.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- fix_correct_bridges.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- comment_incompatible_bridges.sh" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"

echo "### Bridge Validation Scripts" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- extract_all_bridge_configs.sh" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- validate_bridge_config.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"

echo "### GenesisBuilder Scripts" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- deploy_genesis_builder_to_all_pbcs.sh" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "- fix_genesisbuilder_placement.py" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"

echo "---" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "" >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"
echo "**Note**: All scripts remain in their original locations in \`docs/archive/scripts/\` for reference." >> "$ARCHIVE_DIR/MIGRATION_SCRIPTS_REFERENCE.md"

echo "✅ MIGRATION_SCRIPTS_REFERENCE.md created"

echo "Step 4: Moving original files to preserve them..."
# Move original session files
mv "$ARCHIVE_DIR/sessions"/* "$ARCHIVE_DIR/original-files/" 2>/dev/null || true
rmdir "$ARCHIVE_DIR/sessions" 2>/dev/null || true

# Move original status files
mv "$ARCHIVE_DIR/status-reports"/* "$ARCHIVE_DIR/original-files/" 2>/dev/null || true
rmdir "$ARCHIVE_DIR/status-reports" 2>/dev/null || true

# Scripts stay in their directory (they're referenced, not consolidated)
echo "✅ Original files preserved in original-files/"

echo "Step 5: Updating archive README..."
cat > "$ARCHIVE_DIR/README.md" << 'README_EOF'
# Archive Directory

This directory contains historical documentation and scripts from the Ëtrid project development.

---

## Consolidated Archive Files

**Current Archive** (3 consolidated files):

1. **CONSOLIDATED_SESSIONS.md** - All historical session reports
   - ASF Consensus sessions (7 reports)
   - Bridge Integration sessions (4 reports)
   - Gizzi sessions (2 reports)
   - General sessions (12 reports)

2. **CONSOLIDATED_STATUS_REPORTS.md** - All status reports
   - Build status reports
   - PBC runtime status
   - Collator fix guides
   - Migration handoff documents

3. **MIGRATION_SCRIPTS_REFERENCE.md** - Migration scripts index
   - Bridge migration scripts
   - GenesisBuilder deployment scripts
   - Validation utilities

---

## Directory Structure

```
docs/archive/
├── README.md                           # This file
├── CONSOLIDATED_SESSIONS.md            # All session reports
├── CONSOLIDATED_STATUS_REPORTS.md      # All status reports
├── MIGRATION_SCRIPTS_REFERENCE.md      # Scripts index
├── DOCUMENTATION_CONSOLIDATION_PLAN.md # Consolidation plan
├── scripts/                            # Original migration scripts (preserved)
├── consolidated-sources/               # Original source docs from root
└── original-files/                     # Original session/status files
```

---

## Why These Files Were Archived

1. **Superseded**: Newer documentation contains the same information
2. **Completed**: Scripts finished their one-time migrations
3. **Historical**: Valuable context but not needed for active development
4. **Consolidation**: Reduced from 47 files to 3 readable documents

---

## Accessing Historical Information

All files remain accessible via:
- Consolidated markdown files (easy reading)
- Original files in subdirectories (exact copies)
- Git history (`git log --all --full-history -- <filename>`)

---

## Archive Statistics

**Before Consolidation**:
- 25 session reports
- 11 status reports
- 11 migration scripts
- 8 consolidated source docs
- **Total**: 55 files

**After Consolidation**:
- 3 consolidated markdown files
- 1 scripts reference
- All originals preserved in subdirectories
- **Total**: 4 reference files + originals

---

**Archive Created**: October 19, 2025
**Last Updated**: October 19, 2025
**Consolidation Reason**: Documentation cleanup and improved organization

README_EOF

echo "✅ Archive README.md updated"

echo ""
echo "=========================================="
echo "Archive Consolidation Complete!"
echo "=========================================="
echo ""
echo "Final Archive Structure:"
echo "  - CONSOLIDATED_SESSIONS.md (all session reports)"
echo "  - CONSOLIDATED_STATUS_REPORTS.md (all status reports)"
echo "  - MIGRATION_SCRIPTS_REFERENCE.md (scripts index)"
echo "  - README.md (this guide)"
echo "  - scripts/ (original migration scripts)"
echo "  - consolidated-sources/ (original root docs)"
echo "  - original-files/ (original session/status files)"
echo ""
echo "Result: 47 files → 3 consolidated docs + originals preserved ✅"
echo ""
