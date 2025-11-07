# Deployment Security Checklist

**Last Updated**: November 6, 2025
**Purpose**: Ensure secure deployment of Etrid smart contracts to mainnet
**Related**: [SECURITY_AUDIT_REPORT.md](./SECURITY_AUDIT_REPORT.md)

---

## Pre-Deployment Security Review

Complete ALL items before deploying to ANY mainnet network.

### 🔐 Key Management

- [ ] **Unique keys per chain** - Each blockchain has its own private key
- [ ] **No test keys** - Verified no Hardhat/Ganache default keys in use
- [ ] **Hardware wallet ready** - Ledger or Trezor configured for production
- [ ] **Multi-sig implemented** - Gnosis Safe or similar for contract ownership
- [ ] **Keys stored securely** - Password manager or HSM, not plain text files
- [ ] **Backup created** - Encrypted offline backup of all keys
- [ ] **Key rotation plan** - Schedule and procedures documented

**Verify**:
```bash
# Check no .env files are tracked by git
git ls-files | grep -E "\.env$" | grep -v example

# Should return nothing (empty)

# Verify all .env files are gitignored
git check-ignore deployment/dex/dex-deployment/*/.env

# Should list all .env files
```

### 🔍 Code Audit

- [ ] **Professional audit completed** - Third-party security audit by reputable firm
- [ ] **Audit report reviewed** - All findings addressed or accepted
- [ ] **Test coverage > 90%** - Comprehensive unit and integration tests
- [ ] **Fuzz testing completed** - Invariant testing for edge cases
- [ ] **Dependencies audited** - All npm/cargo packages reviewed for vulnerabilities
- [ ] **No console.log or debug code** - Production code cleaned
- [ ] **Access controls verified** - Only authorized addresses can call admin functions

**Verify**:
```bash
# Check test coverage
cargo tarpaulin --out Stdout

# Check for debug code
grep -r "console.log" contracts/
grep -r "println!" contracts/

# Audit dependencies
cargo audit
npm audit
```

### 🌐 Network Configuration

- [ ] **RPC endpoints verified** - Using reliable, trusted RPC providers
- [ ] **Gas prices configured** - Appropriate max gas price to prevent failed txs
- [ ] **Nonce management** - Sequential nonce tracking implemented
- [ ] **Chain IDs verified** - Correct chain ID for target network
- [ ] **Block confirmations set** - Waiting for sufficient confirmations (12+ for ETH)
- [ ] **Deployment addresses documented** - Pre-computed CREATE2 addresses recorded

**Verify**:
```bash
# Check RPC connectivity
cast block latest --rpc-url $ETHEREUM_MAINNET_RPC

# Verify chain ID
cast chain-id --rpc-url $ETHEREUM_MAINNET_RPC
# Should match expected: 1 for Ethereum mainnet
```

### 💰 Financial Safety

- [ ] **Funding verified** - Sufficient ETH/tokens for gas in deployer address
- [ ] **Initial supply calculated** - Token economics reviewed and approved
- [ ] **Price oracles tested** - Chainlink or backup oracle functioning
- [ ] **Circuit breakers implemented** - Emergency pause functionality
- [ ] **Rate limits configured** - Transaction/withdrawal limits set
- [ ] **Treasury multisig ready** - Foundation multisig address confirmed

**Verify**:
```bash
# Check deployer balance
cast balance $DEPLOYER_ADDRESS --rpc-url $ETHEREUM_MAINNET_RPC

# Verify multisig
cast code $FOUNDATION_MULTISIG --rpc-url $ETHEREUM_MAINNET_RPC
# Should show Gnosis Safe or multisig contract bytecode
```

### 📋 Documentation

- [ ] **Deployment plan documented** - Step-by-step deployment procedure
- [ ] **Contract addresses recorded** - Spreadsheet or database ready
- [ ] **Verification scripts prepared** - Etherscan/BSCScan verification ready
- [ ] **Emergency procedures** - Incident response plan documented
- [ ] **Team roles assigned** - Clear responsibility for each deployment step
- [ ] **Rollback plan** - Steps to take if deployment fails
- [ ] **Communications plan** - Who to notify, when, and how

### 🧪 Testing

- [ ] **Testnet deployment successful** - Deployed to testnet and tested thoroughly
- [ ] **Integration tests passed** - All cross-contract interactions verified
- [ ] **End-to-end tests passed** - Full user workflows tested
- [ ] **Mainnet fork tested** - Deployment tested on mainnet fork (Tenderly/Hardhat)
- [ ] **Deployment script dry-run** - Tested deployment script in safe mode
- [ ] **Gas estimation verified** - Deployment won't fail due to gas issues

**Run**:
```bash
# Deploy to testnet first
npm run deploy:testnet

# Run integration tests
npm run test:integration

# Dry run on mainnet fork
npm run deploy:fork
```

---

## Deployment Day Checklist

### Pre-Deployment (1 hour before)

- [ ] **Team assembled** - All deployment team members available
- [ ] **Communication channels open** - Discord/Slack war room active
- [ ] **Backup plan reviewed** - Everyone knows rollback procedures
- [ ] **Keys accessible** - Hardware wallets connected and unlocked
- [ ] **RPC endpoints healthy** - All RPC providers responding normally
- [ ] **Gas prices reasonable** - Network not congested
- [ ] **Monitoring ready** - Block explorers and dashboards open

### During Deployment

- [ ] **Record start time** - Document when deployment begins
- [ ] **Save all transaction hashes** - Every tx hash recorded
- [ ] **Monitor confirmations** - Wait for sufficient confirmations
- [ ] **Screenshot each step** - Visual record of deployment
- [ ] **Verify contract code** - Check deployed bytecode matches expected
- [ ] **Test basic functionality** - Smoke test critical functions
- [ ] **Transfer ownership** - Move admin rights to multisig

**During deployment, run**:
```bash
# Get transaction receipt
cast receipt $TX_HASH --rpc-url $ETHEREUM_MAINNET_RPC

# Verify deployed bytecode
cast code $CONTRACT_ADDRESS --rpc-url $ETHEREUM_MAINNET_RPC

# Check contract owner
cast call $CONTRACT_ADDRESS "owner()" --rpc-url $ETHEREUM_MAINNET_RPC
```

### Post-Deployment

- [ ] **Verify on block explorer** - Etherscan/BSCScan verification complete
- [ ] **Documentation updated** - Contract addresses in docs/mainnet/
- [ ] **Community notified** - Twitter/Discord announcement prepared
- [ ] **Monitoring enabled** - Grafana/DataDog tracking contract activity
- [ ] **Multisig ownership verified** - Admin functions only callable by multisig
- [ ] **Emergency contacts notified** - Security team knows deployment is live
- [ ] **Post-deployment report** - Summary document created

---

## Security Verification Commands

### Check .env Files Are Gitignored

```bash
# This should return empty (no .env files tracked)
git ls-files | grep "\.env$" | grep -v example

# This should list all your .env files (they are gitignored)
git check-ignore deployment/dex/dex-deployment/*/.env
```

### Verify No Test Keys in Use

```bash
# Check for Hardhat default keys (should return nothing)
grep -r "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80" deployment/

# Check for repeated keys (should show unique keys)
grep -h "PRIVATE_KEY=" deployment/dex/dex-deployment/*/.env | sort | uniq -c
# Each key should appear only ONCE
```

### Verify Unique Keys Per Chain

```bash
# Count unique private keys (should equal number of chains)
grep -h "PRIVATE_KEY=" deployment/dex/dex-deployment/*/.env | sort -u | wc -l

# Example: If you have 5 chains, this should output "5"
```

### Check Contract Ownership

```bash
# After deployment, verify owner is multisig
OWNER=$(cast call $CONTRACT_ADDRESS "owner()" --rpc-url $RPC_URL)
echo $OWNER
# Should match your multisig address
```

### Verify Multi-Sig Configuration

```bash
# Check multisig threshold (e.g., 4 of 7)
cast call $MULTISIG_ADDRESS "getThreshold()" --rpc-url $RPC_URL

# Check multisig owners
cast call $MULTISIG_ADDRESS "getOwners()" --rpc-url $RPC_URL
```

---

## Emergency Response

### If Deployment Fails

1. **Stop immediately** - Do not continue deploying
2. **Analyze failure** - Review transaction revert reason
3. **Check funds** - Ensure deployer hasn't lost funds
4. **Review plan** - Determine what went wrong
5. **Fix and retest** - Update scripts and test on fork
6. **Retry when ready** - Don't rush to retry

### If Wrong Key Used

1. **Assess damage** - What contracts were deployed?
2. **Transfer ownership** - If possible, transfer to correct address
3. **Redeploy if needed** - If ownership can't be transferred
4. **Update documentation** - Record incident and resolution
5. **Rotate compromised key** - Generate new key

### If Key Compromised

1. **Emergency pause** - Call pause() on all contracts immediately
2. **Transfer ownership** - Move to multisig if not already
3. **Assess exposure** - What could attacker do?
4. **Communicate** - Notify team and prepare user announcement
5. **Rotate keys** - Generate new keys for all chains
6. **Post-mortem** - Document how compromise occurred

---

## Chain-Specific Notes

### Ethereum Mainnet
- **Gas**: 30-50 gwei during low congestion
- **Confirmations**: Wait for 12+ confirmations
- **Etherscan API**: Required for verification
- **Estimated cost**: 0.5-1 ETH for full deployment

### BSC (BNB Chain)
- **Gas**: 3-5 gwei typical
- **Confirmations**: Wait for 20+ confirmations (faster blocks)
- **BSCScan API**: Required for verification
- **Estimated cost**: 0.1-0.3 BNB

### Polygon
- **Gas**: 30-50 gwei typical
- **Confirmations**: Wait for 128+ confirmations (reorganization risk)
- **PolygonScan API**: Required for verification
- **Estimated cost**: 0.5-1 MATIC

### Arbitrum
- **Gas**: Similar to Ethereum L1
- **Confirmations**: Wait for L1 finalization
- **Arbiscan API**: Required for verification
- **Estimated cost**: 0.1-0.2 ETH

### Base
- **Gas**: Low, similar to Optimism
- **Confirmations**: Wait for L1 finalization
- **BaseScan API**: Required for verification
- **Estimated cost**: 0.05-0.1 ETH

---

## Final Verification

Before considering deployment complete:

```bash
# 1. All contracts verified on block explorer
# Visit: https://etherscan.io/address/$CONTRACT_ADDRESS#code

# 2. Ownership transferred to multisig
cast call $CONTRACT_ADDRESS "owner()" --rpc-url $RPC_URL
# Should return multisig address

# 3. All addresses documented
cat docs/mainnet/DEPLOYED_ADDRESSES.md
# Should list all contracts with addresses

# 4. Monitoring active
# Check Grafana dashboard shows contract events

# 5. Community notified
# Twitter announcement posted
# Discord announcement posted

# 6. Post-deployment report created
cat docs/mainnet/DEPLOYMENT_REPORT_$(date +%Y%m%d).md
```

---

## Resources

- [Etrid Security Audit Report](./SECURITY_AUDIT_REPORT.md)
- [Etrid Secrets Management](../secrets/README.md)
- [OpenZeppelin Defender](https://defender.openzeppelin.com/)
- [Tenderly Monitoring](https://tenderly.co/)
- [Gnosis Safe](https://gnosis-safe.io/)
- [Etherscan Verification](https://etherscan.io/verifyContract)

---

## Sign-off

Deployment to mainnet requires sign-off from:

- [ ] **Technical Lead**: ___________________  Date: __________
- [ ] **Security Lead**: ___________________  Date: __________
- [ ] **Operations**: ___________________  Date: __________

**Deployment authorized by**: ___________________  Date: __________
