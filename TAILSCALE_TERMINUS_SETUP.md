# Tailscale Setup for Terminus

## ✓ Installation Complete

All 22 validators are now accessible via Tailscale from your Mac, Terminus, or any terminal.

## Quick Start Guide

### 1. Basic SSH Access

```bash
# Connect to any validator using the ts-* prefix
ssh ts-val-01          # Gizzi-Director-1
ssh ts-director-01     # Same as above (alias)
ssh ts-validator-6     # Contabo Validator-6
ssh ts-val-07          # Same as above

# Or use direct Tailscale IPs
ssh ubuntu@100.96.84.69   # Gizzi-Director-1
ssh root@100.95.0.72      # Validator-6
```

### 2. Terminus Configuration

Your SSH config is now set up at:
- `~/.ssh/tailscale-validators-config`
- Automatically included in `~/.ssh/config`

**No additional Terminus configuration needed!** Just open Terminus and use the commands above.

### 3. All Available Aliases

#### Directors (Oracle Cloud)
```bash
ssh ts-val-01   # or ts-director-01, ts-gizzi-director-1
ssh ts-val-02   # or ts-director-02, ts-auditdev-director-2
ssh ts-val-03   # or ts-director-03
ssh ts-val-04   # or ts-director-04
ssh ts-val-05   # or ts-director-05
ssh ts-val-06   # or ts-director-06
```

#### Validators (Contabo)
```bash
ssh ts-val-07   # or ts-validator-6
ssh ts-val-08   # or ts-validator-7
ssh ts-val-09   # or ts-validator-8
# ... through ...
ssh ts-val-22   # or ts-validator-21
```

### 4. Test Your Setup

```bash
# Test connection to first validator
ssh ts-val-01 "hostname && tailscale ip -4"

# Should output:
# [hostname]
# 100.96.84.69
```

### 5. Advantages in Terminus

✓ **Works from anywhere**: Coffee shop, home, travel - no VPN needed
✓ **Stable IPs**: Tailscale IPs never change (100.x.x.x)
✓ **Fast**: Direct peer-to-peer encrypted connections
✓ **Secure**: WireGuard encryption by default
✓ **Easy**: Simple aliases like `ts-val-01` instead of long IPs

### 6. Useful Commands

```bash
# Check your Mac's Tailscale IP
tailscale ip -4
# Output: 100.70.117.105

# Check Tailscale status
tailscale status

# List all Tailscale peers
tailscale status | grep "100\."

# Copy files via Tailscale
scp file.txt ts-val-01:/tmp/
scp ts-val-01:/var/log/flarechain.log ./

# Run commands on all validators
for i in {1..22}; do
  echo "=== Validator $i ==="
  ssh ts-val-$(printf "%02d" $i) "hostname && uptime"
done
```

### 7. Network Map File Locations

All Tailscale IPs saved in:
- `~/Desktop/etrid/tailscale-ips-complete.txt` - Full network map
- `~/.ssh/tailscale-validators-config` - SSH configuration

### 8. Troubleshooting

**If SSH fails:**
```bash
# Check if Tailscale is running on your Mac
tailscale status

# Check if validator is online
tailscale ping 100.96.84.69

# Test direct SSH
ssh -v ts-val-01
```

**If Tailscale is not running:**
```bash
# Start Tailscale
sudo brew services start tailscale
sudo tailscale up
```

### 9. Mobile Access (Bonus)

Install Tailscale on your phone/tablet:
1. Download Tailscale from App Store / Play Store
2. Login with the same account
3. Use SSH apps like Terminus, Termius, or Blink
4. Connect using the same `100.x.x.x` IPs

### 10. Next Steps

Your Tailscale network is ready! You can now:
- Access all 22 validators from anywhere
- Deploy updates remotely
- Monitor logs in real-time
- Manage the network on-the-go

**No additional configuration needed for Terminus!** Just use the `ts-val-XX` aliases.

---

**Complete Network Map:**

| Validator | Tailscale IP | Alias | User |
|-----------|--------------|-------|------|
| Gizzi-Director-1 | 100.96.84.69 | ts-val-01 | ubuntu |
| AuditDev-Director-2 | 100.70.242.106 | ts-val-02 | ubuntu |
| Director-3 | 100.102.128.51 | ts-val-03 | root |
| Director-4 | 100.71.242.104 | ts-val-04 | root |
| Director-5 | 100.74.84.28 | ts-val-05 | root |
| Director-6 | 100.89.102.75 | ts-val-06 | root |
| Validator-6 | 100.95.0.72 | ts-val-07 | root |
| Validator-7 | 100.86.111.37 | ts-val-08 | root |
| Validator-8 | 100.125.147.88 | ts-val-09 | root |
| Validator-9 | 100.80.84.82 | ts-val-10 | root |
| Validator-10 | 100.109.252.56 | ts-val-11 | root |
| Validator-11 | 100.117.43.53 | ts-val-12 | root |
| Validator-12 | 100.88.104.58 | ts-val-13 | root |
| Validator-13 | 100.70.73.10 | ts-val-14 | root |
| Validator-14 | 100.68.185.50 | ts-val-15 | root |
| Validator-15 | 100.71.127.127 | ts-val-16 | root |
| Validator-16 | 100.93.43.18 | ts-val-17 | root |
| Validator-17 | 100.124.117.73 | ts-val-18 | root |
| Validator-18 | 100.74.204.23 | ts-val-19 | root |
| Validator-19 | 100.125.251.60 | ts-val-20 | root |
| Validator-20 | 100.114.244.62 | ts-val-21 | root |
| Validator-21 | 100.113.226.111 | ts-val-22 | root |

---

**Your Mac Tailscale IP:** 100.70.117.105

Ready to use! 🚀
