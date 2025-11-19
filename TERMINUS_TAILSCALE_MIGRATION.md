# Terminus → Tailscale Migration Guide

## Update Your Existing Terminus SSH Profiles

### Option 1: Edit Hosts in Terminus App

1. **Open Terminus**
2. **Click the "+" button** or go to Settings → Hosts
3. **Find your existing validator hosts**
4. **Update the IP addresses** to Tailscale IPs:

#### Before (Public IPs):
```
Host: validator-01
Address: 64.181.215.19
User: ubuntu
Key: gizzi-validator
```

#### After (Tailscale IPs):
```
Host: validator-01-ts
Address: 100.96.84.69
User: ubuntu
Key: gizzi-validator
```

### Complete IP Mapping

Update your Terminus hosts with these Tailscale IPs:

| Old Public IP | New Tailscale IP | Name | User |
|---------------|------------------|------|------|
| 64.181.215.19 | **100.96.84.69** | Gizzi-Director-1 | ubuntu |
| 129.80.122.34 | **100.70.242.106** | AuditDev-Director-2 | ubuntu |
| 157.173.200.86 | **100.102.128.51** | Director-3 | root |
| 157.173.200.84 | **100.71.242.104** | Director-4 | root |
| 157.173.200.81 | **100.74.84.28** | Director-5 | root |
| 157.173.200.80 | **100.89.102.75** | Director-6 | root |
| 85.239.239.194 | **100.95.0.72** | Validator-6 | root |
| 85.239.239.193 | **100.86.111.37** | Validator-7 | root |
| 85.239.239.190 | **100.125.147.88** | Validator-8 | root |
| 85.239.239.189 | **100.80.84.82** | Validator-9 | root |
| 85.239.239.188 | **100.109.252.56** | Validator-10 | root |
| 80.190.82.186 | **100.117.43.53** | Validator-11 | root |
| 80.190.82.185 | **100.88.104.58** | Validator-12 | root |
| 80.190.82.184 | **100.70.73.10** | Validator-13 | root |
| 80.190.82.183 | **100.68.185.50** | Validator-14 | root |
| 158.220.83.146 | **100.71.127.127** | Validator-15 | root |
| 158.220.83.66 | **100.93.43.18** | Validator-16 | root |
| 154.12.250.18 | **100.124.117.73** | Validator-17 | root |
| 154.12.250.17 | **100.74.204.23** | Validator-18 | root |
| 154.12.250.15 | **100.125.251.60** | Validator-19 | root |
| 154.12.249.223 | **100.114.244.62** | Validator-20 | root |
| 154.12.249.182 | **100.113.226.111** | Validator-21 | root |

---

## Option 2: Use SSH Config (Already Done!)

Your SSH config is already set up. In Terminus, you can now:

1. **Open Terminus**
2. **Type directly in terminal:**
   ```bash
   ssh ts-val-01
   ssh ts-val-02
   # etc...
   ```

3. **Or create new hosts using the aliases:**
   - Host: `ts-val-01`
   - No need to specify IP, user, or key (already in SSH config)

---

## Option 3: Import SSH Config into Terminus

1. **Open Terminus Settings**
2. **Go to SSH → Import from SSH Config**
3. **Select:** `~/.ssh/tailscale-validators-config`
4. **Terminus will auto-create hosts** for all `ts-val-*` entries

---

## Benefits of Using Tailscale IPs

✓ **Works from anywhere** - Coffee shop, home, travel
✓ **Stable IPs** - Never change (100.x.x.x)
✓ **No VPN needed** - Tailscale is the VPN
✓ **Encrypted** - WireGuard by default
✓ **Faster** - Direct peer-to-peer connections

---

## Testing Your Setup

In Terminus terminal:

```bash
# Test connection
ssh ts-val-01 "hostname"

# Check Tailscale is working
ssh ts-val-01 "tailscale status"

# View network info
ssh ts-val-01 "ip addr show tailscale0"
```

---

## Keep Both? (Recommended)

You can keep both public IP and Tailscale hosts:

**Public IP hosts** (existing):
- validator-01 → 64.181.215.19

**Tailscale hosts** (new):
- validator-01-ts → 100.96.84.69

This gives you fallback if Tailscale is down.

---

## Quick Migration Script

Run this to create a Terminus import file:

```bash
cat > ~/Desktop/etrid/terminus-tailscale-import.json <<'EOF'
{
  "hosts": [
    {"name": "ts-gizzi-director-1", "host": "100.96.84.69", "user": "ubuntu", "privateKey": "~/.ssh/gizzi-validator"},
    {"name": "ts-auditdev-director-2", "host": "100.70.242.106", "user": "ubuntu", "privateKey": "~/.ssh/gizzi-validator"},
    {"name": "ts-director-3", "host": "100.102.128.51", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-director-4", "host": "100.71.242.104", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-director-5", "host": "100.74.84.28", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-director-6", "host": "100.89.102.75", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-6", "host": "100.95.0.72", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-7", "host": "100.86.111.37", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-8", "host": "100.125.147.88", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-9", "host": "100.80.84.82", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-10", "host": "100.109.252.56", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-11", "host": "100.117.43.53", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-12", "host": "100.88.104.58", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-13", "host": "100.70.73.10", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-14", "host": "100.68.185.50", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-15", "host": "100.71.127.127", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-16", "host": "100.93.43.18", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-17", "host": "100.124.117.73", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-18", "host": "100.74.204.23", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-19", "host": "100.125.251.60", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-20", "host": "100.114.244.62", "user": "root", "privateKey": "~/.ssh/contabo-validators"},
    {"name": "ts-validator-21", "host": "100.113.226.111", "user": "root", "privateKey": "~/.ssh/contabo-validators"}
  ]
}
EOF

echo "Terminus import file created at: ~/Desktop/etrid/terminus-tailscale-import.json"
```

Then import in Terminus: Settings → SSH → Import → Select file

---

## Summary

**Easiest method**: Use the SSH config (already done)
- Just type `ssh ts-val-01` in Terminus terminal

**Manual method**: Update each Terminus host's IP to Tailscale IP
- Replace public IPs with 100.x.x.x IPs from table above

**Automated method**: Import the JSON file into Terminus
- Run script above, import in Terminus settings
