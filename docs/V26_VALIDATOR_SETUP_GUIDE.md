# V26 Validator Setup Guide: ASF SessionKeys Integration

This guide provides comprehensive instructions for validators to generate, register, and verify ASF (Autonomous State Finality) keys for V26's SessionKeys integration.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Key Generation](#key-generation)
4. [Key Registration](#key-registration)
5. [Verification](#verification)
6. [Batch Operations](#batch-operations)
7. [Security Best Practices](#security-best-practices)
8. [Troubleshooting](#troubleshooting)
9. [FAQ](#faq)

---

## Overview

### What Are ASF Keys?

ASF (Autonomous State Finality) keys are sr25519 keypairs used by validators to sign checkpoint attestations. In V26, these keys are integrated with Substrate's session pallet, allowing validators to publish their ASF public keys on-chain.

### Key Components

- **ASF Keys**: sr25519 keypairs stored in validator keystores
- **Session Pallet**: Substrate's built-in session management
- **SessionKeys Structure**: Contains the ASF public key
- **Runtime APIs**: Query ASF keys for checkpoint signing

### Architecture

```
┌─────────────────┐      ┌──────────────────┐      ┌─────────────────┐
│   Validator     │      │  Session Pallet  │      │  ASF Runtime    │
│   Keystore      │─────▶│  (NextKeys)      │─────▶│  (Checkpoints)  │
│   (asfk...)     │      │  {asf: 0x...}    │      │                 │
└─────────────────┘      └──────────────────┘      └─────────────────┘
```

---

## Prerequisites

### Software Requirements

1. **subkey** - Key generation tool
   ```bash
   cargo install --force subkey --git https://github.com/paritytech/polkadot-sdk --locked
   ```

2. **Node.js** (v16 or higher)
   ```bash
   node --version  # Should be v16+
   ```

3. **@polkadot/api** - Substrate API library
   ```bash
   npm install @polkadot/api
   ```

4. **etrid Scripts** - Clone the etrid repository
   ```bash
   cd ~/Desktop/etrid
   git pull origin main
   chmod +x scripts/v26-*.sh
   ```

### Validator Requirements

- Running validator node with synced chain
- Validator account with sufficient balance (≥10 ETR for transaction fees)
- SSH access to validator VM (for batch operations)
- Validator stash account seed phrase or dev account URI

### Network Requirements

- RPC endpoint accessible (default: `ws://localhost:9944`)
- Port 9944 open for WebSocket connections
- Stable internet connection

---

## Key Generation

### Option 1: Automated Script (Recommended)

The automated script generates keys, backs them up, and inserts them into the keystore.

```bash
cd ~/Desktop/etrid
./scripts/v26-generate-asf-keys.sh
```

**Expected Output:**
```
========================================
   V26 ASF Key Generation Script
========================================

Step 1: Generating sr25519 keypair...

Generated ASF Keypair:
  Public Key:   0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
  SS58 Address: 5CRZoFgJs4zLzCCAGoCUUs2MRmuD5BKAh17pWtb62LMoCi9h
  Account ID:   0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef

Step 2: Backing up keypair...
  Saved to: /Users/username/.etrid-asf-keys-backup/asf-key-20250321_143022.json

Step 3: Inserting key into validator keystore...
  Keystore file: /Users/username/.local/share/primearc/chains/primearc_mainnet/keystore/asfk1234567890abcdef...

Step 4: Exporting public key for registration...
  Public key exported to: /Users/username/.etrid-asf-keys-backup/asf-public-key.txt

========================================
   Key Generation Complete!
========================================
```

**Custom Keystore Path:**
```bash
./scripts/v26-generate-asf-keys.sh --keystore-path /custom/path/to/keystore
```

### Option 2: Manual Generation

If you prefer manual control or need to generate keys offline:

#### Step 1: Generate Keypair

```bash
subkey generate --scheme sr25519 --output-type json
```

**Sample Output:**
```json
{
  "secretPhrase": "word1 word2 word3 ... word12",
  "secretSeed": "0xabcdef1234567890...",
  "publicKey": "0x1234567890abcdef...",
  "ss58Address": "5CRZoFgJs4zLzCCAGoCUUs2MRmuD5BKAh17pWtb62LMoCi9h",
  "accountId": "0x1234567890abcdef..."
}
```

#### Step 2: Backup the Key

**CRITICAL:** Save the complete JSON output to secure offline storage. You will need the `secretSeed` for keystore insertion and the `publicKey` for registration.

```bash
# Save to encrypted USB drive or password manager
echo '<JSON_OUTPUT>' > asf-key-backup.json
chmod 600 asf-key-backup.json
```

#### Step 3: Insert Key into Keystore

```bash
# Set your keystore path
KEYSTORE_PATH="${HOME}/.local/share/primearc/chains/primearc_mainnet/keystore"

# Extract values from your generated key
PUBLIC_KEY="0x1234567890abcdef..."  # From JSON output
SECRET_SEED="0xabcdef1234567890..."   # From JSON output

# Create keystore file
# Format: asfk<public-key-without-0x>
PUBLIC_KEY_CLEAN="${PUBLIC_KEY#0x}"
SECRET_SEED_CLEAN="${SECRET_SEED#0x}"

echo -n "\"$SECRET_SEED_CLEAN\"" > "$KEYSTORE_PATH/asfk$PUBLIC_KEY_CLEAN"
chmod 600 "$KEYSTORE_PATH/asfk$PUBLIC_KEY_CLEAN"
```

#### Step 4: Verify Insertion

```bash
ls -la $KEYSTORE_PATH/asfk*
```

You should see a file like: `asfk1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef`

---

## Key Registration

### Option 1: Automated Script (Recommended)

Register your ASF public key on-chain using the registration script.

```bash
cd ~/Desktop/etrid

# With validator seed phrase
node scripts/v26-register-asf-keys.js \
  --public-key 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef \
  --validator-uri "word1 word2 word3 ... word12"

# With dev account (testing only)
node scripts/v26-register-asf-keys.js \
  --public-key 0x1234567890abcdef... \
  --validator-uri //Alice
```

**Expected Output:**
```
========================================
   V26 ASF Key Registration
========================================

Step 1: Validating inputs...
  ASF Public Key: 0x1234567890abcdef...
  RPC Endpoint: ws://localhost:9944
  Dry Run: false

Step 2: Initializing crypto...

Step 3: Connecting to node...
  Connected to chain: Primearc Core Mainnet
  Runtime version: 26

Step 4: Loading validator account...
  Validator address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY

Validator Balance Check:
  Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Free Balance: 1000000000000000000 plancks
  Required: ~100000000000 plancks (minimum)

Step 5: Preparing session keys structure...
  Session Keys: {
    "asf": "0x1234567890abcdef..."
  }

Step 6: Submitting setKeys transaction...
  Estimated fee: 125000000 plancks

Submitting transaction...

Transaction status: Ready

Transaction status: InBlock
  Included in block: 0xabcd...1234

Transaction status: Finalized
  Finalized in block: 0xabcd...1234

========================================
   Registration Successful!
========================================

Summary:
  Validator: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  ASF Public Key: 0x1234567890abcdef...
  Block: 0xabcd...1234
  Transaction Hash: 0x5678...efgh

Next Steps:
  1. Verify registration: node scripts/v26-verify-asf-keys.js --validator 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  2. Restart validator node to load the key from keystore
  3. Monitor node logs for ASF checkpoint signing
```

### Option 2: Dry Run (Testing)

Test the registration without submitting the transaction:

```bash
node scripts/v26-register-asf-keys.js \
  --public-key 0x1234... \
  --validator-uri //Alice \
  --dry-run
```

### Option 3: Custom RPC Endpoint

Register using a remote node:

```bash
node scripts/v26-register-asf-keys.js \
  --public-key 0x1234... \
  --validator-uri "seed phrase" \
  --rpc-endpoint wss://rpc.primearc.network
```

### Option 4: Manual Registration (Advanced)

Using Polkadot.js Apps interface:

1. Navigate to https://polkadot.js.org/apps
2. Connect to your node (Settings → RPC endpoint)
3. Go to Developer → Extrinsics
4. Select your validator account
5. Choose extrinsic: `session.setKeys(keys, proof)`
6. Fill in parameters:
   - `keys`: `{"asf": "0x1234..."}`
   - `proof`: `0x` (empty)
7. Submit transaction

---

## Verification

### Verify Single Validator

Check if your ASF key is correctly registered:

```bash
node scripts/v26-verify-asf-keys.js \
  --validator 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
```

**Expected Output:**
```
========================================
   V26 ASF Key Verification
========================================

Connecting to node...
  Connected to chain: Primearc Core Mainnet
  Runtime version: 26

Verifying validator: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Status: Keys registered
  ASF Key: 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef
  Validation: ✓ Valid format

Next Steps:
  3. Monitor checkpoint signing in validator logs
  4. Verify ASF signatures are being generated
```

### Verify All Validators

Check the status of all validators in the network:

```bash
node scripts/v26-verify-asf-keys.js --all
```

**Sample Output:**
```
========================================
   V26 ASF Key Verification
========================================

Querying all validators...
  Found 20 validators in current session

============================================================
Verifying 20 validators...
============================================================

[1/20]
Verifying validator: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
  Status: Keys registered
  ASF Key: 0x1234...
  Validation: ✓ Valid format

[2/20]
Verifying validator: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
  Status: No session keys registered

...

============================================================
Verification Summary
============================================================

Total Validators:     20
Valid ASF Keys:       18
No Keys Registered:   2
Invalid Format:       0
Errors:               0

Validators without ASF keys:
  5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
  5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy

Health Status:
  90.00% validators ready

⚠ Most validators ready, but some need attention

Next Steps:
  1. Register missing ASF keys: node scripts/v26-register-asf-keys.js
  2. Or batch register: ./scripts/v26-batch-register-validators.sh
  3. Monitor checkpoint signing in validator logs
  4. Verify ASF signatures are being generated
```

### Export Verification Results

Save verification results to JSON for analysis:

```bash
node scripts/v26-verify-asf-keys.js --all --export-json results.json
```

---

## Batch Operations

### Batch Generate and Register All Validators

For network operators managing multiple validators:

#### Step 1: Configure Validator Information

Edit `/Users/macbook/Desktop/etrid/scripts/v26-batch-register-validators.sh`:

```bash
# Update these arrays with your actual validator information
VALIDATOR_VMS=(
    "vmi2896906" "vmi2896907" "vmi2896908" # ... add all VMs
)

declare -A VALIDATOR_ADDRESSES=(
    ["vmi2896906"]="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    ["vmi2896907"]="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
    # ... add all validators
)

declare -A VALIDATOR_SEEDS=(
    ["vmi2896906"]="word1 word2 ... word12"
    ["vmi2896907"]="word1 word2 ... word12"
    # ... add all validators
)
```

**SECURITY WARNING:** Never commit seed phrases to git. Use environment variables or encrypted storage in production.

#### Step 2: Run Batch Registration

```bash
cd ~/Desktop/etrid
./scripts/v26-batch-register-validators.sh
```

**Sample Output:**
```
========================================
   V26 Batch ASF Key Registration
========================================

Checking prerequisites...
Prerequisites OK

Starting batch registration for 20 validators...
Results will be saved to: /Users/username/.etrid-v26-registration/registration-results-20250321_143500.log

----------------------------------------
Processing: vmi2896906
----------------------------------------
  Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
Extracting ASF public key from vmi2896906...
  Public key: 0x1234567890abcdef...
  Registering ASF key on-chain...
SUCCESS: vmi2896906 registered

----------------------------------------
Processing: vmi2896907
----------------------------------------
...

========================================
   Registration Summary
========================================

Total Validators: 20
Successful:       18
Failed:           0
Skipped:          2

Successful Registrations:
  ✓ vmi2896906 - 0x1234...
  ✓ vmi2896907 - 0x5678...
  ...

Skipped Validators:
  ○ vmi2896924 - No ASF key in keystore
  ○ vmi2896925 - No ASF key in keystore

Full log saved to: /Users/username/.etrid-v26-registration/registration-results-20250321_143500.log

Next Steps:
  1. Review the registration results above
  2. For failed validators, check logs and retry manually
  3. Run verification script: node scripts/v26-verify-asf-keys.js
  4. Restart all validator nodes to load ASF keys

All registrations completed successfully!
```

#### Step 3: Restart All Validators

After registration, restart validator nodes to load the ASF keys:

```bash
# SSH to each validator and restart
ssh -i ~/.ssh/contabo-validators root@vmi2896906.contabo.host \
  "systemctl restart primearc-validator"
```

---

## Security Best Practices

### Key Generation Security

1. **Generate Keys on Secure Machine**
   - Use air-gapped machine for production keys
   - Never generate keys on shared or compromised systems
   - Verify subkey binary integrity before use

2. **Backup Strategy**
   - Store backup JSON files on encrypted offline storage
   - Use multiple backup locations (USB drives, hardware wallets, paper)
   - Never store backups in cloud services unencrypted
   - Test backup restoration process

3. **Keystore Protection**
   ```bash
   # Ensure keystore has correct permissions
   chmod 700 ~/.local/share/primearc/chains/primearc_mainnet/keystore
   chmod 600 ~/.local/share/primearc/chains/primearc_mainnet/keystore/*
   ```

### Registration Security

1. **Protect Validator Seeds**
   - Never commit seed phrases to version control
   - Use environment variables or encrypted vaults
   - Rotate seeds if compromised
   - Use hardware wallets for production

2. **Transaction Safety**
   - Always use `--dry-run` first to test
   - Verify transaction fees are reasonable
   - Confirm correct public key before submitting
   - Monitor transaction status until finalization

3. **Network Security**
   - Use SSH key authentication (no passwords)
   - Restrict SSH access by IP address
   - Keep SSH keys encrypted with strong passphrases
   - Use VPN for remote management

### Operational Security

1. **Access Control**
   - Limit who can access validator machines
   - Use separate accounts for different operators
   - Log all access and operations
   - Implement 2FA for critical systems

2. **Monitoring**
   - Monitor validator logs for suspicious activity
   - Alert on unexpected key usage
   - Track failed authentication attempts
   - Regular security audits

3. **Incident Response**
   - Have key rotation procedure ready
   - Document emergency contacts
   - Test disaster recovery process
   - Keep backup of validator configuration

---

## Troubleshooting

### Issue: "subkey command not found"

**Solution:**
```bash
cargo install --force subkey --git https://github.com/paritytech/polkadot-sdk --locked
```

Ensure `~/.cargo/bin` is in your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Issue: "Insufficient balance"

**Error:**
```
Error: Insufficient balance. Need at least 100000000000 plancks for transaction fees
```

**Solution:**
1. Check validator balance:
   ```bash
   node -e "
   const { ApiPromise, WsProvider } = require('@polkadot/api');
   (async () => {
     const api = await ApiPromise.create({ provider: new WsProvider('ws://localhost:9944') });
     const account = await api.query.system.account('YOUR_ADDRESS');
     console.log('Balance:', account.data.free.toString());
     await api.disconnect();
   })();
   "
   ```

2. Transfer funds to validator account if needed

### Issue: "Invalid public key format"

**Error:**
```
Error: Invalid public key format. Expected 64 hex characters (32 bytes)
```

**Solution:**
- Ensure public key is 64 hex characters (not 66 with 0x prefix)
- Remove any whitespace or newlines
- Verify key was copied completely

```bash
# Correct format
PUBLIC_KEY="0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"
# Length check (should be 66 with 0x, or 64 without)
echo ${#PUBLIC_KEY}
```

### Issue: "Connection refused"

**Error:**
```
Error: connect ECONNREFUSED 127.0.0.1:9944
```

**Solution:**
1. Verify node is running:
   ```bash
   systemctl status primearc-validator
   ```

2. Check WebSocket port is open:
   ```bash
   netstat -tulpn | grep 9944
   ```

3. Update RPC endpoint if using remote node:
   ```bash
   --rpc-endpoint wss://remote-node.example.com:9944
   ```

### Issue: "Key file already exists"

**Error:**
```
Warning: ASF key already exists in keystore
```

**Solution:**
1. Check existing key:
   ```bash
   ls -la ~/.local/share/primearc/chains/primearc_mainnet/keystore/asfk*
   ```

2. If you want to replace it (CAREFUL - this deletes the old key):
   ```bash
   rm ~/.local/share/primearc/chains/primearc_mainnet/keystore/asfk*
   ./scripts/v26-generate-asf-keys.sh
   ```

3. Or use the existing key - extract public key from filename

### Issue: "Session keys not updating"

**Problem:** Registered keys but verification shows no keys.

**Solution:**
1. Wait for next session (can take several minutes)
2. Check transaction was finalized:
   ```bash
   # Query transaction by hash in Polkadot.js Apps
   ```
3. Verify correct validator address was used
4. Check node logs for errors

### Issue: SSH connection fails in batch script

**Error:**
```
ssh: connect to host vmi2896906.contabo.host port 22: Connection refused
```

**Solution:**
1. Verify SSH key exists:
   ```bash
   ls -la ~/.ssh/contabo-validators
   ```

2. Test SSH connection manually:
   ```bash
   ssh -i ~/.ssh/contabo-validators root@vmi2896906.contabo.host
   ```

3. Update VM hostnames in script if using different domain
4. Check firewall rules allow SSH from your IP

---

## FAQ

### Q: What happens if I lose my ASF key backup?

**A:** If you lose the backup but the key is still in your keystore, you can regenerate a new key and register it. The old key will be replaced in the next session. However, for security audit and disaster recovery, always maintain backups.

### Q: Can I use the same ASF key for multiple validators?

**A:** No. Each validator must have a unique ASF key. Using the same key across validators will cause signature conflicts and potential slashing.

### Q: How often do I need to rotate ASF keys?

**A:** There's no enforced rotation period, but we recommend rotating keys:
- Every 6 months as a security best practice
- Immediately if you suspect key compromise
- When changing validator operators

### Q: Do I need to restart my node after registration?

**A:** Yes. After registering your ASF key on-chain, restart your validator node so it loads the key from the keystore. The node needs the private key to sign checkpoints.

```bash
systemctl restart primearc-validator
```

### Q: What's the difference between session keys and ASF keys?

**A:** ASF keys are part of the session keys structure in V26. Previously, ASF keys were managed separately. Now they're integrated with Substrate's session pallet for better key management and synchronization.

### Q: Can I register keys for next session in advance?

**A:** Yes. Keys registered via `session.setKeys()` are stored in `NextKeys` and take effect in the next session. This allows validators to prepare for upcoming sessions without downtime.

### Q: What happens if my validator doesn't have ASF keys registered?

**A:** Your validator will:
- Not be able to sign ASF checkpoints
- Miss out on checkpoint rewards
- Not contribute to cross-chain finality
- Still participate in normal block production (if you have other session keys)

### Q: How do I verify my validator is signing checkpoints?

**A:** Check your validator logs:

```bash
journalctl -u primearc-validator -f | grep -i "asf"
```

Look for messages like:
```
ASF checkpoint signed: height=12345, hash=0xabcd...
```

### Q: Can I use hardware wallets for ASF keys?

**A:** ASF keys must be in the node keystore for signing. Hardware wallets can be used to secure the validator controller account that submits the registration transaction, but the ASF signing key itself needs to be accessible to the node.

### Q: What's the estimated cost to register ASF keys?

**A:** Transaction fees are typically very low:
- Estimated: ~0.0000001 ETR (~125,000 plancks)
- Varies based on network congestion
- Use `--dry-run` to see fee estimate

### Q: Can I update my ASF key after registration?

**A:** Yes. Simply register a new ASF key using `session.setKeys()`. The new key will replace the old one in the next session. Make sure the new key is in your keystore before the session change.

---

## Rollback Procedures

### Emergency Key Rotation

If you need to rotate your ASF key immediately:

1. **Generate new key:**
   ```bash
   ./scripts/v26-generate-asf-keys.sh
   ```

2. **Register new key (replaces old):**
   ```bash
   node scripts/v26-register-asf-keys.js --public-key <NEW_KEY> --validator-uri "seed"
   ```

3. **Restart node:**
   ```bash
   systemctl restart primearc-validator
   ```

4. **Verify new key active:**
   ```bash
   node scripts/v26-verify-asf-keys.js --validator <YOUR_ADDRESS>
   ```

### Restore from Backup

If you need to restore a key from backup:

1. **Locate backup file:**
   ```bash
   ls -lah ~/.etrid-asf-keys-backup/
   ```

2. **Extract secret seed from backup JSON:**
   ```bash
   cat ~/.etrid-asf-keys-backup/asf-key-TIMESTAMP.json | grep secretSeed
   ```

3. **Restore to keystore:**
   ```bash
   PUBLIC_KEY="0x..."  # From backup
   SECRET_SEED="0x..."  # From backup
   KEYSTORE_PATH="${HOME}/.local/share/primearc/chains/primearc_mainnet/keystore"

   PUBLIC_KEY_CLEAN="${PUBLIC_KEY#0x}"
   SECRET_SEED_CLEAN="${SECRET_SEED#0x}"

   echo -n "\"$SECRET_SEED_CLEAN\"" > "$KEYSTORE_PATH/asfk$PUBLIC_KEY_CLEAN"
   chmod 600 "$KEYSTORE_PATH/asfk$PUBLIC_KEY_CLEAN"
   ```

4. **Restart node:**
   ```bash
   systemctl restart primearc-validator
   ```

---

## Support and Resources

### Documentation

- **V26 Technical Spec:** `/Users/macbook/Desktop/etrid/docs/V26_SESSION_KEYS_SPEC.md`
- **Architecture Overview:** `/Users/macbook/Desktop/etrid/docs/architecture.md`
- **API Reference:** `/Users/macbook/Desktop/etrid/docs/API_REFERENCE.md`

### Community

- **Discord:** https://discord.gg/etrid
- **Forum:** https://forum.primearc.network
- **GitHub:** https://github.com/etrid/etrid

### Getting Help

If you encounter issues not covered in this guide:

1. Check the troubleshooting section above
2. Search GitHub issues: https://github.com/etrid/etrid/issues
3. Ask in Discord #validator-support channel
4. Open a GitHub issue with:
   - Detailed error messages
   - Steps to reproduce
   - System information
   - Logs (redact sensitive info)

---

## Appendix: Script Reference

### Available Scripts

| Script | Purpose | Location |
|--------|---------|----------|
| `v26-generate-asf-keys.sh` | Generate ASF keypair and insert into keystore | `/Users/macbook/Desktop/etrid/scripts/` |
| `v26-register-asf-keys.js` | Register ASF public key on-chain | `/Users/macbook/Desktop/etrid/scripts/` |
| `v26-batch-register-validators.sh` | Batch register all validators | `/Users/macbook/Desktop/etrid/scripts/` |
| `v26-verify-asf-keys.js` | Verify ASF keys are correctly registered | `/Users/macbook/Desktop/etrid/scripts/` |

### Quick Command Reference

```bash
# Generate key
./scripts/v26-generate-asf-keys.sh

# Register key
node scripts/v26-register-asf-keys.js \
  --public-key 0x... \
  --validator-uri "seed phrase"

# Verify single validator
node scripts/v26-verify-asf-keys.js --validator 5Grw...

# Verify all validators
node scripts/v26-verify-asf-keys.js --all

# Batch register (network operators)
./scripts/v26-batch-register-validators.sh

# Dry run registration
node scripts/v26-register-asf-keys.js \
  --public-key 0x... \
  --validator-uri //Alice \
  --dry-run
```

---

## Changelog

### Version 1.0 (2025-03-21)

- Initial release for V26 SessionKeys integration
- Added comprehensive scripts for key generation, registration, and verification
- Included batch operations for network operators
- Security best practices and troubleshooting guide

---

**Document Version:** 1.0
**Last Updated:** 2025-03-21
**Maintainers:** Etrid Core Team
**License:** Apache 2.0
