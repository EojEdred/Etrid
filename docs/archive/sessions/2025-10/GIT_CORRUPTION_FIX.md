# Git Repository Corruption - Fix Guide

**Date:** October 21, 2025
**Issue:** `remote unpack failed: pack has bad object`
**Status:** ⚠️ Requires Manual Fix

---

## Problem

When attempting to push commits to GitHub, the following error occurred:

```
remote: error: inflate: data stream error (invalid literal/length code)
remote: fatal: pack has bad object at offset 4497000: inflate returned -3
error: remote unpack failed: index-pack failed
```

This indicates **corrupted objects** in the local Git repository.

---

## Fix Steps (In Order)

### Step 1: Verify the Corruption

```bash
cd /Users/macbook/Desktop/etrid
git fsck --full
```

This will list all corrupted objects.

---

### Step 2: Try Aggressive Garbage Collection

```bash
git gc --aggressive --prune=now
```

This attempts to:
- Remove unreferenced objects
- Recompress pack files
- Fix minor corruption

**Then try pushing again:**
```bash
git push origin main
```

---

### Step 3: If GC Doesn't Work, Try Repacking

```bash
git repack -a -d --depth=250 --window=250
```

This creates new pack files from scratch.

**Then push again:**
```bash
git push origin main
```

---

### Step 4: Push in Smaller Batches

If the corruption persists, push commits in smaller chunks:

```bash
# See commits to push
git log --oneline origin/main..main

# Push 10 commits at a time
git push origin HEAD~44:refs/heads/main
git push origin HEAD~34:refs/heads/main
git push origin HEAD~24:refs/heads/main
git push origin HEAD~14:refs/heads/main
git push origin main
```

---

### Step 5: Nuclear Option - Recreate Repository (Last Resort)

**⚠️ Only if all above fails:**

```bash
# 1. Backup current repo
cp -r /Users/macbook/Desktop/etrid /Users/macbook/Desktop/etrid-backup

# 2. Clone fresh from remote
cd /Users/macbook/Desktop
git clone git@github.com:EojEdred/Etrid.git etrid-fresh

# 3. Copy uncommitted work
cd etrid-fresh
git checkout -b fix-branch

# 4. Manually copy changed files from backup
cp /Users/macbook/Desktop/etrid-backup/pallets/pallet-validator-committee/src/lib.rs pallets/pallet-validator-committee/src/lib.rs
# ... (copy other modified files)

# 5. Commit and push
git add .
git commit -m "Add validator committee tests + fixes"
git push origin fix-branch
```

---

## Alternative: Use Different Transport

Sometimes SSH corruption can be bypassed with HTTPS:

```bash
# Temporarily change remote to HTTPS
git remote set-url origin https://github.com/EojEdred/Etrid.git

# Try pushing
git push origin main

# Switch back to SSH
git remote set-url origin git@github.com:EojEdred/Etrid.git
```

---

## Commits Awaiting Push

**Main Branch:**
- 54 commits ahead of origin
- Includes stable2509 migration documentation
- Includes test suite creation

**Testnet-Stable2506 Branch:**
- NEW branch (not yet on remote)
- Based on commit 36391e94

**Files Modified (Uncommitted on testnet-stable2506):**
1. pallets/pallet-validator-committee/src/lib.rs (tests added, partially fixed)
2. pallets/pallet-validator-committee/TEST_COVERAGE_REPORT.md
3. TERMINAL4_STATUS.md
4. TERMINAL4_SESSION_COMPLETE.md
5. GIT_CORRUPTION_FIX.md (this file)

---

## Recommended Immediate Action

1. **First, try Step 2 (aggressive GC)**
   ```bash
   git gc --aggressive --prune=now
   git push origin main
   ```

2. **If that fails, try Step 3 (repack)**
   ```bash
   git repack -a -d --depth=250 --window=250
   git push origin main
   ```

3. **If still failing, use Step 4 (batch push)**

4. **As last resort, use Step 5 (fresh clone)**

---

## Prevention for Future

### Causes of Git Corruption:
1. Disk errors / bad sectors
2. Interrupted push/pull operations
3. Power loss during git operations
4. Large binary files
5. Filesystem issues

### Prevention:
1. Always commit before system sleep/shutdown
2. Use `git push` in stable network conditions
3. Avoid force-interrupting git operations
4. Keep regular backups
5. Use `.gitignore` for large binaries

---

## Current Repository Health

**Before Fix:**
- ❌ Corrupted pack objects
- ❌ Cannot push to remote
- ✅ Local commits intact
- ✅ Working directory intact

**After Fix (Expected):**
- ✅ Clean repository
- ✅ Can push to remote
- ✅ All commits preserved
- ✅ Remote synchronized

---

## Notes for Claude Code Session

**What was being pushed:**
- 54 commits on main branch
- Stable2509 migration work
- Migration documentation (3 files)
- Test suite creation (in progress)

**What needs to be committed after fix:**
- Validator committee test fixes
- Test documentation
- Session reports

**Git Status:**
```
Branch: testnet-stable2506
Modified: 5 files
Uncommitted changes: Yes
Ready to commit: After test verification
```

---

**Prepared by:** Claude Code
**Date:** October 21, 2025
**Priority:** High - blocking git push operations
**Impact:** Medium - local work safe, remote sync blocked

---

*Git corruption is recoverable - stay calm and follow the steps* 🔧
