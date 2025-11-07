# Secrets Directory

This directory contains **SENSITIVE** information that should **NEVER** be committed to git.

⚠️ **CRITICAL**: Before any mainnet deployment, review [docs/SECURITY_AUDIT_REPORT.md](../docs/SECURITY_AUDIT_REPORT.md)

## Directory Structure

```
secrets/
├── mainnet/                  # Mainnet genesis files and validator keys
│   ├── flarechain_mainnet_genesis.json
│   ├── flarechain_mainnet_genesis_backup.json
│   └── validator-keys-complete.json
│
├── aidevs-keys/              # AI agents cryptographic keypairs
│   ├── keypairs.json         # Private/public keypairs for AI agents
│   ├── public_keys.json      # Public keys only
│   └── ai-monitoring-keypairs.json
│
├── oracle-keys/              # Oracle service credentials
│
├── validator-keys/           # Validator deployment keys and docs
│   ├── generated-keys/
│   └── docs/
│
└── genesis-accounts/         # Generated genesis account data
    └── genesis-accounts-*/   # Timestamped genesis account sets

```

**Note**: Deployment `.env` files are stored in `deployment/dex/dex-deployment/*/` directories, NOT in this secrets folder. This keeps configuration close to the deployment scripts that use them.

## Security Best Practices

1. **NEVER commit this directory to git** - Already protected by .gitignore
2. **Backup these files securely** - Use encrypted storage (e.g., password manager, HSM)
3. **Rotate keys regularly** - Especially for production validators
4. **Limit access** - Only team members who need these keys should have access
5. **Use environment variables** - For deployment, load .env files at runtime
6. **Generate unique keys per chain** - Use `scripts/generate-deployment-keys.sh`
7. **Hardware wallets for mainnet** - Never use software keys for production
8. **Multi-sig for critical operations** - Implement 4-of-7 or similar
9. **Regular security audits** - Review access logs and key usage
10. **Incident response plan** - Document key compromise procedures

## File Types Protected

- Genesis files (`*_genesis.json`)
- Validator keys (`validator-keys*.json`)
- Private keypairs (`keypairs.json`)
- Environment files (`.env`)
- PEM/Key files (`*.pem`, `*.key`)
- Encrypted files (`*.enc`, `*.gpg`)

## Moving Files Here

When adding new sensitive files:

```bash
# Move the file
mv path/to/sensitive-file.json secrets/appropriate-subdirectory/

# Ensure it's gitignored
git check-ignore secrets/appropriate-subdirectory/sensitive-file.json

# If not ignored, update .gitignore
```

## Emergency Key Rotation

If keys are accidentally committed:

1. **Immediately rotate all exposed keys**
2. **Remove from git history:**
   ```bash
   git filter-branch --force --index-filter \
     "git rm --cached --ignore-unmatch path/to/exposed/file" \
     --prune-empty --tag-name-filter cat -- --all
   ```
3. **Force push** (coordinate with team)
4. **Regenerate all exposed credentials**

## Deployment Keys

Deployment private keys are **NOT** stored in this directory. They are stored in:

```
deployment/dex/dex-deployment/
├── arbitrum/.env          # Arbitrum deployment key (gitignored)
├── base/.env              # Base deployment key (gitignored)
├── bsc/.env               # BSC deployment key (gitignored)
├── ethereum/.env          # Ethereum deployment key (gitignored)
└── polygon/.env           # Polygon deployment key (gitignored)
```

**To generate new deployment keys**:
```bash
./scripts/generate-deployment-keys.sh
```

See [docs/SECURITY_AUDIT_REPORT.md](../docs/SECURITY_AUDIT_REPORT.md) for critical security information about deployment keys.

## Key Generation Guide

### For New Validators

1. **Generate validator session keys**:
   ```bash
   # Using subkey (Substrate tool)
   subkey generate --scheme Sr25519
   ```

2. **Generate staking account**:
   ```bash
   subkey generate --scheme Sr25519
   ```

3. **Store securely**:
   - Save to password manager
   - Create encrypted backup
   - Document in validator inventory

### For Deployment Keys

1. **Run key generation script**:
   ```bash
   ./scripts/generate-deployment-keys.sh
   ```

2. **Review generated keys**:
   - Check `generated-keys-*/MASTER_KEY_LIST.md`
   - Verify unique key per chain

3. **Update .env files**:
   ```bash
   # Example for Ethereum
   cd deployment/dex/dex-deployment/ethereum
   cp .env.example .env
   # Edit .env and paste generated key
   ```

4. **Secure the generated files**:
   - Store keys in password manager
   - Delete generated-keys-*/ directory
   - Verify .env files are gitignored

### For AI Agent Keys

AI agent DID keypairs are generated automatically by the AI monitoring system. They are stored in:
- `secrets/aidevs-keys/keypairs.json` - Full keypairs
- `secrets/aidevs-keys/public_keys.json` - Public keys only

## Contact

For questions about key management or security concerns, contact the security team immediately.

**In case of key compromise**: See "Emergency Key Rotation" section above and [docs/SECURITY_AUDIT_REPORT.md](../docs/SECURITY_AUDIT_REPORT.md).
