# 13-Clients & SDK Consolidation Plan

**Date:** October 25, 2025
**Issue:** Duplicate and conflicting SDK/app structures

---

## Problems Identified

### 1. SDK Duplication
**Two SDK locations:**
- `13-clients/sdk/` - Individual language SDKs (js, python, rust, swift)
- `sdk/` (root) - Unified SDK with bindings/

**Status:** Different organization, possibly different versions

### 2. Redundant Symlinks in 13-clients
```
13-clients/mobile-wallet → apps/wallet-mobile/etrid-wallet
13-clients/web-wallet → ../apps/wallet-web/etrid-crypto-website
```

**Problem:** These are just symlinks to apps/ - redundant and confusing

### 3. Unclear Separation
- Are wallets "clients"? (currently in `apps/`)
- Are SDKs "clients"? (currently split between `13-clients/sdk/` and `sdk/`)
- Are CLI tools "clients"? (currently in `13-clients/cli/`)

---

## Analysis

### Current Structure

**13-clients/:**
```
13-clients/
├── ARCHITECTURE.md
├── cli/
│   ├── etrcpp-console/
│   ├── etrust-console/
│   └── pye-console/
├── sdk/
│   ├── js-etrid-sdk/
│   ├── python-etrid-sdk/
│   ├── rust-etrid-sdk/
│   ├── swift-etrid-sdk/
│   ├── IMPLEMENTATION_SUMMARY.md
│   └── README.md
├── mobile-wallet → apps/wallet-mobile/etrid-wallet (symlink)
└── web-wallet → ../apps/wallet-web/etrid-crypto-website (symlink)
```

**Root sdk/:**
```
sdk/
├── Cargo.toml
├── README.md
├── bindings/
│   ├── js-etrid-sdk/
│   ├── python-etrid-sdk/
│   ├── rust-etrid-sdk/
│   └── swift-etrid-sdk/
└── src/
```

**Root apps/:**
```
apps/
├── wallet-mobile/etrid-wallet/
├── wallet-web/etrid-crypto-website/
├── governance-ui/
├── validator-dashboard/
├── watchtower-monitor/
└── masterchef-dashboard/
```

---

## Decision: What is a "Client"?

**Definition:** A client is software that interacts with the Ëtrid blockchain.

**Types of Clients:**
1. **SDKs** - Libraries for developers (js, python, rust, swift)
2. **CLI Tools** - Command-line interfaces (console applications)
3. **Wallets** - User applications (mobile, web)
4. **Dashboards** - User interfaces (governance, validator, etc.)

**Current categorization:**
- SDKs: Split between `13-clients/sdk/` and `sdk/`
- CLI: In `13-clients/cli/`
- Wallets: In `apps/`
- Dashboards: In `apps/`

---

## Proposed Solution

### Option 1: Everything in 13-clients (Strict Definition)

**Rationale:** All client software belongs together

```
13-clients/
├── ARCHITECTURE.md
├── cli/                    # Command-line clients
│   ├── etrcpp-console/
│   ├── etrust-console/
│   └── pye-console/
├── sdk/                    # Developer SDKs (consolidate both)
│   ├── js-etrid-sdk/
│   ├── python-etrid-sdk/
│   ├── rust-etrid-sdk/
│   ├── swift-etrid-sdk/
│   ├── bindings/ (if needed)
│   └── core/ (if root sdk/ has unique code)
├── wallets/                # Move from apps/
│   ├── mobile/            (from apps/wallet-mobile/)
│   └── web/               (from apps/wallet-web/)
└── dashboards/             # Move from apps/
    ├── governance/
    ├── validator/
    ├── watchtower/
    └── masterchef/
```

**Actions:**
- ✅ Consolidate `sdk/` → `13-clients/sdk/`
- ✅ Move `apps/wallet-*` → `13-clients/wallets/`
- ✅ Move `apps/*-dashboard` → `13-clients/dashboards/`
- ✅ Delete `apps/` folder
- ✅ Delete symlinks in `13-clients/`

---

### Option 2: 13-clients for SDKs/CLI, Apps Separate (Practical)

**Rationale:** Apps are user-facing, SDKs/CLI are developer-facing

```
13-clients/
├── ARCHITECTURE.md
├── cli/                    # Developer CLI tools
│   ├── etrcpp-console/
│   ├── etrust-console/
│   └── pye-console/
└── sdk/                    # Developer SDKs (consolidate both)
    ├── js-etrid-sdk/
    ├── python-etrid-sdk/
    ├── rust-etrid-sdk/
    ├── swift-etrid-sdk/
    ├── bindings/ (if needed)
    └── core/ (if root sdk/ has unique code)

apps/                       # User-facing applications (keep separate)
├── wallet-mobile/
├── wallet-web/
├── governance-ui/
├── validator-dashboard/
├── watchtower-monitor/
└── masterchef-dashboard/
```

**Actions:**
- ✅ Consolidate `sdk/` → `13-clients/sdk/`
- ✅ Keep `apps/` separate (user apps ≠ developer clients)
- ✅ Delete symlinks in `13-clients/`

---

### Option 3: Rename to Reflect Purpose (Recommended)

**Rationale:** "13-clients" is ambiguous, rename for clarity

```
13-developer-tools/         # or 13-dev-sdk/
├── ARCHITECTURE.md
├── cli/                    # CLI tools for developers
│   ├── etrcpp-console/
│   ├── etrust-console/
│   └── pye-console/
└── sdk/                    # SDKs for all languages
    ├── js-etrid-sdk/
    ├── python-etrid-sdk/
    ├── rust-etrid-sdk/
    ├── swift-etrid-sdk/
    └── README.md

apps/                       # User-facing applications
├── wallet-mobile/
├── wallet-web/
├── governance-ui/
├── validator-dashboard/
├── watchtower-monitor/
└── masterchef-dashboard/
```

**Actions:**
- ✅ Rename `13-clients/` → `13-developer-tools/`
- ✅ Consolidate `sdk/` → `13-developer-tools/sdk/`
- ✅ Keep `apps/` separate
- ✅ Delete symlinks in `13-developer-tools/`

---

## Recommended Approach: Option 3

**Why:**
1. **Clear separation:** Developer tools vs User apps
2. **Intuitive:** Name reflects purpose
3. **Scalable:** Easy to add more dev tools or apps
4. **Clean:** No symlinks, no duplication

**Implementation:**

### Step 1: Analyze SDK Content
Determine which SDK is canonical:
- Check modification dates
- Check completeness
- Merge if both have unique content

### Step 2: Consolidate SDKs
```bash
# If 13-clients/sdk/ is newer/better:
rm -rf sdk/

# If root sdk/ is newer/better:
rm -rf 13-clients/sdk/
mv sdk/ 13-clients/

# If both have unique content:
# Manually merge into 13-clients/sdk/
```

### Step 3: Remove Symlinks
```bash
cd 13-clients/
rm mobile-wallet web-wallet
```

### Step 4: Rename Directory
```bash
mv 13-clients/ 13-developer-tools/
```

### Step 5: Update References
- Update `README.md`
- Update documentation
- Update any hardcoded paths

---

## Alternative Names for 13-clients

If not "13-developer-tools", consider:

1. **13-dev-sdk** - Developer SDK
2. **13-sdk-cli** - SDK and CLI tools
3. **13-tooling** - Development tooling
4. **13-libraries** - Client libraries
5. **13-integrations** - Integration tools

**Recommendation:** `13-developer-tools` (most descriptive)

---

## CLI Tools Analysis

**Current CLI tools in 13-clients/cli/:**
- `etrcpp-console` - C++ console
- `etrust-console` - Rust console
- `pye-console` - Python console

**Question:** Are these:
1. Developer debugging tools? → Keep in `13-developer-tools/cli/`
2. End-user applications? → Move to `apps/cli/`

**Likely answer:** Developer tools (based on names) → Keep in `13-developer-tools/cli/`

---

## Final Proposed Structure

```
etrid/
├── 01-12/                  # Other core components
├── 13-developer-tools/     # RENAMED from 13-clients
│   ├── ARCHITECTURE.md
│   ├── cli/                # Developer CLI tools
│   │   ├── etrcpp-console/
│   │   ├── etrust-console/
│   │   └── pye-console/
│   └── sdk/                # CONSOLIDATED from root sdk/
│       ├── js-etrid-sdk/
│       ├── python-etrid-sdk/
│       ├── rust-etrid-sdk/
│       ├── swift-etrid-sdk/
│       └── README.md
├── 14-aidevs/              # AI development guides
├── apps/                   # User-facing applications (KEEP SEPARATE)
│   ├── wallet-mobile/
│   ├── wallet-web/
│   ├── governance-ui/
│   ├── validator-dashboard/
│   ├── watchtower-monitor/
│   └── masterchef-dashboard/
├── [other folders...]
```

**Result:**
- ✅ No duplication (one SDK location)
- ✅ No symlinks (removed)
- ✅ Clear separation (dev tools vs user apps)
- ✅ Intuitive naming (developer-tools)
- ✅ Follows numbering convention (13-)

---

## Execution Checklist

### Phase 1: Investigate
- [ ] Compare `sdk/` vs `13-clients/sdk/` content
- [ ] Determine which is canonical
- [ ] Identify unique content in each

### Phase 2: Consolidate SDKs
- [ ] Merge SDK content into `13-clients/sdk/`
- [ ] Delete root `sdk/` folder
- [ ] Verify no broken imports

### Phase 3: Clean Symlinks
- [ ] Delete `13-clients/mobile-wallet` symlink
- [ ] Delete `13-clients/web-wallet` symlink

### Phase 4: Rename
- [ ] Rename `13-clients/` → `13-developer-tools/`
- [ ] Update documentation references
- [ ] Update README.md

### Phase 5: Verify
- [ ] Test build
- [ ] Verify SDK imports work
- [ ] Update COMPREHENSIVE_RESTRUCTURE_PLAN.md

---

## Impact Assessment

**Files affected:**
- SDKs: ~4-8 language bindings
- Symlinks: 2 (deleted)
- Folder name: 1 (renamed)

**Code changes needed:**
- Import paths referencing `13-clients/sdk/`
- Documentation references
- README updates

**Risk level:** Low-Medium
- No source code deletion
- Mainly reorganization
- Test after completion

---

**Status:** Analysis complete, ready for decision
**Recommendation:** Execute Option 3 (rename + consolidate)

---

*Analysis completed: October 25, 2025*
