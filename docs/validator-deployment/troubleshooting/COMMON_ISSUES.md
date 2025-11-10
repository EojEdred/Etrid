# Common Validator Issues

## Quick Reference

| Issue | Cause | Fix |
|-------|-------|-----|
| 0 peers | Port 30333 blocked | [Firewall Guide](./FIREWALL_TROUBLESHOOTING.md) |
| NetworkKeyNotFound | Missing network key | [Session Keys Guide](./SESSION_KEY_ISSUES.md) |
| Wrong genesis | Wrong chainspec | Download correct chainspec from existing validator |
| Service won't start | Multiple causes | Check logs: \`journalctl -u flarechain-validator -n 50\` |

## Most Common: 0 Peers

**Symptom:** Validator stuck at 0 peers after 5+ minutes

**Cause:** Port 30333 is blocked by firewall

**Fix:** See [Firewall Troubleshooting Guide](./FIREWALL_TROUBLESHOOTING.md)

**Quick test:**
\`\`\`bash
nc -zv <validator-ip> 30333
\`\`\`

## Second Most Common: NetworkKeyNotFound

**Error:**
\`\`\`
Error: NetworkKeyNotFound("/var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519")
\`\`\`

**Fix:**
\`\`\`bash
NETWORK_KEY=$(openssl rand -hex 32)
mkdir -p /var/lib/etrid/chains/flarechain_mainnet/network
echo -n "$NETWORK_KEY" > /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
chmod 600 /var/lib/etrid/chains/flarechain_mainnet/network/secret_ed25519
systemctl restart flarechain-validator
\`\`\`

## Service Crashes on Startup

**Check logs:**
\`\`\`bash
journalctl -u flarechain-validator -n 50 --no-pager
\`\`\`

**Common causes:**
- Missing network key (see above)
- Corrupt database → Delete \`/var/lib/etrid/chains/flarechain_mainnet/db\` and restart
- Missing chainspec → Re-download from existing validator

## Wrong Genesis Hash

**Symptom:** Syncing to different chain

**Fix:**
\`\`\`bash
# Download correct chainspec
scp -i ~/.ssh/contabo-validators root@85.239.239.194:/var/lib/etrid/chainspec-mainnet-raw-FIXED.json /var/lib/etrid/

# Verify genesis contains: 0xca40...4da8
grep '"genesis":' /var/lib/etrid/chainspec-mainnet-raw-FIXED.json

# Clear database
systemctl stop flarechain-validator
rm -rf /var/lib/etrid/chains/flarechain_mainnet/db
systemctl start flarechain-validator
\`\`\`

For more issues, see specific troubleshooting guides.
