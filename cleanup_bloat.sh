#!/bin/bash
# Etrid Project Memory Bloat Cleanup Script
# PRESERVES: npm dependencies (node_modules)
# REMOVES: Rust build artifacts, caches, temp files
# ALWAYS BACKUP IMPORTANT DATA BEFORE RUNNING

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Etrid Bloat Cleanup (NPM-Safe)${NC}"
echo -e "${BLUE}================================${NC}"
echo ""
echo -e "${YELLOW}This will remove:${NC}"
echo "  ✓ All Rust target/ directories"
echo "  ✓ All .rlib and .rmeta intermediate files"
echo "  ✓ Cargo cache (registry & git)"
echo "  ✓ npm cache (NOT node_modules)"
echo "  ✓ Gradle, Android caches"
echo "  ✓ Archive directories"
echo "  ✓ Git cleanup & optimization"
echo ""
echo -e "${GREEN}This will PRESERVE:${NC}"
echo "  ✓ All node_modules directories"
echo "  ✓ All npm installed packages"
echo "  ✓ Source code & configs"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

# Function to safely remove directories
safe_remove() {
    local path="$1"
    local desc="$2"
    if [ -d "$path" ] || [ -f "$path" ]; then
        echo -e "${YELLOW}Removing: $desc${NC}"
        du -sh "$path" 2>/dev/null || echo "  (calculating size...)"
        rm -rf "$path"
        echo -e "${GREEN}✓ Removed${NC}"
    else
        echo -e "${BLUE}✓ Not found: $desc${NC}"
    fi
}

# Track space freed
BEFORE=$(df -h / | tail -1 | awk '{print $3}')

echo ""
echo -e "${GREEN}=== Phase 1: Rust Build Artifacts ===${NC}"
echo "Removing all target/ directories (can rebuild with cargo build)"

# Main etrid target directory
safe_remove "$HOME/Desktop/etrid/target" "Main Rust build artifacts"

# Find all nested target directories in multichain modules
echo "Scanning for nested target/ directories..."
find "$HOME/Desktop/etrid" -type d -name "target" -path "*/target" 2>/dev/null | while read target_dir; do
    safe_remove "$target_dir" "$(basename $(dirname $target_dir))/target"
done

# Clean up intermediate Rust files
echo "Removing intermediate .rlib and .rmeta files..."
find "$HOME/Desktop/etrid" \( -name "*.rlib" -o -name "*.rmeta" \) -type f -delete 2>/dev/null || true
echo -e "${GREEN}✓ Intermediate files removed${NC}"

echo ""
echo -e "${GREEN}=== Phase 2: Package Manager Caches ===${NC}"
echo "Cleaning caches (NOT node_modules)"

# Cargo cache
safe_remove "$HOME/.cargo/git" "Cargo git checkouts"
safe_remove "$HOME/.cargo/registry/cache" "Cargo registry cache"

# npm cache (NOT node_modules - just the cache)
echo -e "${YELLOW}Cleaning npm cache (preserving node_modules)...${NC}"
npm cache clean --force 2>/dev/null || echo "npm cache clean skipped"

# Gradle cache (Android builds)
safe_remove "$HOME/.gradle/caches" "Gradle build cache"

# Android build cache
safe_remove "$HOME/.android/build-cache" "Android build cache"

# General cache directories
safe_remove "$HOME/.cache" "System cache"

echo ""
echo -e "${GREEN}=== Phase 3: Archive Directories ===${NC}"

safe_remove "$HOME/Desktop/_CACHE_ARCHIVE" "Cache archive"
safe_remove "$HOME/Desktop/ARCHIVE" "General archive"

echo ""
echo -e "${GREEN}=== Phase 4: Git Optimization ===${NC}"
echo "Running git garbage collection and optimization..."

cd "$HOME/Desktop/etrid"
git gc --aggressive --prune=now 2>/dev/null || echo "Git gc skipped"
git reflog expire --expire=now --all 2>/dev/null || echo "Reflog cleanup skipped"

echo ""
echo -e "${GREEN}=== Phase 5: Claude Code Temp Files ===${NC}"

# Clean up any Claude-generated temp files
safe_remove "$HOME/.claude/cache" "Claude cache"
safe_remove "/tmp/claude-*" "Claude temp files"

echo ""
echo -e "${GREEN}=== Cleanup Complete ===${NC}"

AFTER=$(df -h / | tail -1 | awk '{print $3}')
echo ""
echo "Before: $BEFORE used"
echo "After:  $AFTER used"
echo ""
echo -e "${BLUE}Summary:${NC}"
echo "  ✓ Rust build artifacts removed"
echo "  ✓ Package manager caches cleared"
echo "  ✓ Archive directories removed"
echo "  ✓ Git repository optimized"
echo "  ✓ node_modules PRESERVED"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo "  1. Run: bash ~/Desktop/etrid/map_binaries_to_drive.sh"
echo "  2. To rebuild: cargo build --release"
echo "  3. To reinstall any missing tools: cargo install <tool>"
echo ""
