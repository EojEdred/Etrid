# 📱 Check Mainnet Health from Your Phone

## Super Simple - Just 2 Steps!

### Option 1: SSH then Run (Easiest)

1. **Open Termius on your phone**
2. **Connect to auditdev**
3. **Type this:**
   ```bash
   check-mainnet-health
   ```

That's it! You'll get the full health report.

---

### Option 2: One-Liner (Fastest)

In Termius, create a saved snippet:

```bash
ssh ubuntu@129.80.122.34 "TERM=xterm check-mainnet-health"
```

Then just tap the snippet to run!

---

## What You'll See

```
╔═══════════════════════════════════════════════════════════════╗
║   ËTRID PRIMEARC CORE CHAIN MAINNET HEALTH CHECK            ║
╚═══════════════════════════════════════════════════════════════╝

🔗 BLOCKCHAIN STATUS
  Current Block: 19,600
  Finalized Block: 19,594
  Finalization Lag: 6 blocks ✅
  Sync Status: ✅ Fully Synced

⚡ CONSENSUS & FINALITY
  Consensus Mode: pure_asf
  Finality Type: ASF
  Block Production: PPFA

🌐 NETWORK & PEERING
  Connected Peers: 8 ✅
  Validator Peers: 8 ✅
  ✅ All peers are validators (strong quorum)

📊 HEALTH SUMMARY
  ✅ OVERALL STATUS: HEALTHY
```

---

## Termius Setup (One-Time)

### Save the Command as a Snippet

1. Open **Termius** app
2. Tap **Snippets** (bottom menu)
3. Tap **+** (top right)
4. Fill in:
   - **Label**: `Check Mainnet`
   - **Command**: `TERM=xterm check-mainnet-health`
5. Tap **Save**

**Now you can:**
1. SSH to `auditdev`
2. Tap **Snippets**
3. Tap **Check Mainnet**
4. Done! ✅

---

## Alternative: Auto-Run on Connect

Make it run automatically when you connect:

1. Go to **Hosts** in Termius
2. Find or create **auditdev** host:
   - Address: `129.80.122.34`
   - Username: `ubuntu`
   - Key: Select your SSH key
3. Scroll down to **Post-login snippet**
4. Select: `Check Mainnet`
5. Save

**Now:** Every time you connect to auditdev, the health check runs automatically! 🚀

---

## Servers You Can Use

Any of these servers (all on Tailscale):

| Server | Tailscale IP | Public IP | Alias |
|--------|--------------|-----------|-------|
| **Auditdev** (Recommended) | 100.70.242.106 | 129.80.122.34 | `auditdev` |
| Gizzi Validator | 100.96.84.69 | 64.181.215.19 | `gizzi` |

**From phone, use:** `auditdev` (most accessible)

---

## Troubleshooting

### "TERM environment variable not set"

Use:
```bash
TERM=xterm check-mainnet-health
```

### "command not found"

Script needs to be installed. From the server run:
```bash
ls /usr/local/bin/check-mainnet-health
```

If not there, reinstall from your laptop.

### Can't connect to auditdev

Try the public IP:
```bash
ssh ubuntu@129.80.122.34
```

---

## Full Documentation

See: `Desktop/etrid/docs/MAINNET_HEALTH_CHECK_GUIDE.md`

---

**That's it! You now have mainnet health monitoring in your pocket! 📱✅**
