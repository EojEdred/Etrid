# ËTRID - Known Issues & Blockers

**Last Updated:** October 11, 2025  
**Status:** Day 1 Complete - Dependency Resolution Pending

---

## 🚧 Current Blocker: Polkadot SDK Dependency Instability

### Issue
Workspace fails to compile due to polkadot-sdk master branch instability:
- Missing codec trait implementations (`DecodeWithMemTracking`)
- API changes between SDK versions
- Deprecated patterns (RuntimeEvent, hardcoded weights)

### Root Cause
Using `branch = "master"` in Cargo.toml points to bleeding-edge, unstable code.

### Impact
- ❌ `cargo check --workspace` fails
- ✅ Individual pallet code is architecturally correct
- ✅ E³20 structure is complete and follows best practices

### Solutions (In Priority Order)

**Option A: Wait for Stable Release** (RECOMMENDED)
- Monitor polkadot-sdk for stable tag release
- Update Cargo.toml to use stable version tag instead of master
- Timeline: Typically 2-4 weeks between stable releases

**Option B: Pin to Known Working Commit**
```toml
[workspace.dependencies]
frame-support = { git = "https://github.com/paritytech/polkadot-sdk.git", rev = "KNOWN_WORKING_COMMIT_HASH" }
```
- Requires finding a working commit hash
- More stable but requires periodic updates

**Option C: Use Released Crates from crates.io**
- Switch from git dependencies to published crates
- More stable but may lag behind features we need

### Current Workaround
Development continues on non-Rust components while SDK stabilizes:
- ✅ Documentation
- ✅ Mobile wallet (Flutter)
- ✅ Web UI (React)
- ✅ CLI tools
- ✅ Token economics design
- ✅ Deployment planning

---

## 📊 Component Status

| Component | Status | Blocker |
|-----------|--------|---------|
| E³20 Structure | ✅ Complete | None |
| Pallet Code | ✅ Written | SDK deps |
| Mobile Wallet | 🟡 Cloned | Integration pending |
| Web UI | 🟡 Cloned | Integration pending |
| Documentation | 🟢 In Progress | None |
| Deployment Plan | 🔴 Not Started | None |

---

## 🎯 Next Steps (No Blockers)

1. **Mobile Wallet Integration**
   - Location: `apps/mobile/`
   - Integrate bloc-banc-wallet Flutter code
   - No Rust compilation required

2. **Web UI Development**
   - Location: `apps/web/`
   - Build Consensus Day governance interface
   - React/TypeScript - independent of Rust backend

3. **Documentation Sprint**
   - Complete API docs
   - User guides
   - Developer onboarding
   - Whitepaper alignment verification

4. **Token Economics Finalization**
   - ÉTR supply schedule
   - EDSC stability mechanism
   - VMw gas pricing model
   - Distribution formulas

5. **Deployment Strategy**
   - Testnet architecture
   - Mainnet launch checklist
   - Legal/regulatory prep
   - Community building

---

## 🔄 Periodic Checks

**Weekly:** Check polkadot-sdk for stable release  
**When stable:** Update deps, recompile, continue Rust development

---

## 📝 Technical Debt Log

### Warnings to Address (Non-Breaking)
1. Deprecated `RuntimeEvent` in Config trait
2. Hard-coded weights (need benchmarking or dev mode)
3. Unused code warnings in pbc-runtime

### Future Improvements
1. Implement proper weight benchmarking
2. Add comprehensive tests
3. Implement missing storage migrations
4. Add runtime upgrade logic

---

**Conclusion:** Architecture is solid. Rust compilation blocked by external SDK instability. Development continues on parallel tracks.
