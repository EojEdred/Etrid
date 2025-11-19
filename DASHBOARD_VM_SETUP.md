# ASF Finality Dashboard - VM Deployment Guide

## 🎯 Perfect Setup for You

Run the finality dashboard on a validator VM, access it via Tailscale from anywhere:
- ✅ Your Mac (any browser)
- ✅ Terminus terminal (built-in browser)
- ✅ Your phone/tablet (with Tailscale app)
- ✅ Any laptop/device on your Tailscale network

---

## 🚀 Quick Deploy (One Command)

```bash
/tmp/deploy-dashboard-to-vm.sh
```

This automatically:
1. Deploys dashboard to Gizzi-Director-1 VM
2. Installs Flask dependencies
3. Creates systemd service (runs on boot)
4. Starts the dashboard
5. Gives you the Tailscale access URL

---

## 📱 How to Access

### From Any Browser (Mac, PC, Terminus)

```
http://100.96.84.69:5000
```

Just paste this URL in any browser!

### From iPhone/iPad

1. **Install Tailscale** (App Store)
2. **Login** with same Tailscale account
3. **Open Safari** or Chrome
4. **Visit:** `http://100.96.84.69:5000`

### From Android

1. **Install Tailscale** (Play Store)
2. **Login** with same Tailscale account
3. **Open Chrome** or any browser
4. **Visit:** `http://100.96.84.69:5000`

### From Terminus Terminal

Terminus has a built-in browser:
1. **Press Ctrl+Shift+P** (or Cmd+Shift+P on Mac)
2. **Type:** "Open in Browser"
3. **Enter URL:** `http://100.96.84.69:5000`

Or just open any browser on your Mac and visit the URL!

---

## 🖥️ What You'll See

### Dashboard Overview

```
┌─────────────────────────────────────────────────────────┐
│         ASF Finality Monitoring Dashboard               │
│         Last Updated: 2025-11-18 16:45:23              │
└─────────────────────────────────────────────────────────┘

┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│ Validators  │ │ NewView     │ │ Finality    │ │ View        │
│ Online      │ │ Broadcast   │ │ Status      │ │ Consensus   │
│   22/22     │ │   15/22     │ │  ✅ Active  │ │  View(3)    │
│   100%      │ │   68%       │ │             │ │  Synced     │
└─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘

Progress to Quorum (15/22 needed):
[████████████████████░░░░] 68%

┌─────────────────────────────────────────────────────────┐
│ Validator Details                                        │
├─────────────────────────────────────────────────────────┤
│ 🟢 Gizzi-Director-1        View(3)  Finalized: #1,245  │
│    100.96.84.69            📤 Broadcasting  18 peers    │
│    QUORUM ✓                                             │
│                                                          │
│ 🟢 AuditDev-Director-2     View(3)  Finalized: #1,245  │
│    100.70.242.106          📤 Broadcasting  21 peers    │
│    QUORUM ✓                                             │
│                                                          │
│ 🟢 Director-3              View(3)  Finalized: #1,245  │
│    100.102.128.51          📤 Broadcasting  19 peers    │
│                                                          │
│ ... (all 22 validators)                                 │
└─────────────────────────────────────────────────────────┘
```

### Color Coding

- 🟢 **Green** - Validator online and healthy
- 🔴 **Red** - Validator offline or issues
- 📤 **Broadcasting** - Sending NewView messages
- ✅ **Active Finality** - Blocks finalizing beyond #0
- **QUORUM ✓** - View change quorum achieved

---

## 🔧 Management Commands

### Check Dashboard Status

```bash
ssh ts-val-01 'sudo systemctl status finality-dashboard'
```

### View Live Logs

```bash
ssh ts-val-01 'sudo journalctl -u finality-dashboard -f'
```

### Restart Dashboard

```bash
ssh ts-val-01 'sudo systemctl restart finality-dashboard'
```

### Stop Dashboard

```bash
ssh ts-val-01 'sudo systemctl stop finality-dashboard'
```

### Update Dashboard Code

```bash
# Edit /tmp/finality-dashboard.py locally, then:
scp /tmp/finality-dashboard.py ts-val-01:/tmp/
ssh ts-val-01 'sudo systemctl restart finality-dashboard'
```

---

## 🎨 Dashboard Features

### Auto-Refresh
- Updates every 30 seconds automatically
- No need to refresh browser manually
- See real-time finality progress

### Responsive Design
- Works on desktop (large monitors)
- Works on tablets (iPad, Android)
- Works on phones (iPhone, Android)
- Adapts to screen size

### Visual Indicators

**Network Overview Cards:**
- Quick glance at overall health
- Progress bars for quorum
- Color-coded status

**Individual Validator Cards:**
- Detailed status per validator
- Tailscale IP addresses
- View numbers and finalized blocks
- P2P peer counts
- Broadcasting indicators

### Real-Time Monitoring

Watch live as:
1. Validators come online
2. NewView broadcast count increases
3. Quorum progress bar fills
4. Views advance (1 → 2 → 3)
5. Finality activates
6. Blocks finalize (increasing numbers)

---

## 🌍 Access From Anywhere

The dashboard runs on the VM 24/7 and is accessible via Tailscale from:

- ✅ **Home WiFi**
- ✅ **Coffee shop**
- ✅ **Office**
- ✅ **Traveling** (airplane WiFi, hotel, etc.)
- ✅ **Mobile hotspot**
- ✅ **Any network worldwide**

As long as you have:
1. Tailscale installed on your device
2. Logged into the same Tailscale account
3. Internet connection

You can access the dashboard!

---

## 📊 Use Cases

### 1. Monitor Deployment Progress

Watch as validators get view transitions deployed:
```
NewView Broadcast: 2/22 → 5/22 → 10/22 → 15/22 ✓ QUORUM!
```

### 2. Verify Finality Activation

See when finality starts working:
```
Finality Status: ❌ Stuck at #0 → ✅ Active (#1,234)
```

### 3. Track View Advancement

Monitor view progression:
```
View(1) → View(2) → View(3) → View(4)...
```

### 4. Debug Network Issues

Identify problems:
- Low peer counts (< 10 peers)
- Validators offline (red dots)
- View consensus issues (different views)
- Broadcast failures (no 📤 icon)

### 5. Share with Team

Send the Tailscale URL to anyone on your Tailscale network:
```
"Check finality status: http://100.96.84.69:5000"
```

---

## 🔒 Security

**Tailscale Security:**
- Encrypted WireGuard VPN
- Only accessible to devices on your Tailscale network
- Not exposed to public internet
- No need to open firewall ports

**Dashboard Security:**
- Read-only (queries only, no writes)
- SSH keys already configured
- Runs on validator VM (internal access)
- Systemd service (managed by OS)

---

## 🚀 Next Steps After Deployment

1. **Deploy the dashboard:**
   ```bash
   /tmp/deploy-dashboard-to-vm.sh
   ```

2. **Open in browser:**
   ```
   http://100.96.84.69:5000
   ```

3. **Watch deployment progress:**
   - NewView broadcast count increasing
   - Quorum progress bar filling
   - Views advancing

4. **Verify finality:**
   - Wait for 15/22 quorum
   - Check finality status turns ✅
   - Watch finalized blocks increase

5. **Share with team:**
   - Send URL to anyone on Tailscale
   - Monitor together in real-time

---

## 📱 Mobile App Alternative (Bonus)

If you prefer mobile apps over browsers:

### iOS
- **SSH Apps:** Terminus, Blink, Prompt
- **Web Browsers:** Safari, Chrome (visit dashboard URL)
- **Terminal + tmux:** SSH to validator, run monitoring scripts

### Android
- **SSH Apps:** Termux, JuiceSSH, ConnectBot
- **Web Browsers:** Chrome, Firefox (visit dashboard URL)
- **Terminal + tmux:** SSH to validator, run monitoring scripts

---

## 🆘 Troubleshooting

### Dashboard won't load

1. **Check service is running:**
   ```bash
   ssh ts-val-01 'sudo systemctl status finality-dashboard'
   ```

2. **Check logs for errors:**
   ```bash
   ssh ts-val-01 'sudo journalctl -u finality-dashboard -n 50'
   ```

3. **Restart service:**
   ```bash
   ssh ts-val-01 'sudo systemctl restart finality-dashboard'
   ```

### Can't access via Tailscale

1. **Check Tailscale is running on VM:**
   ```bash
   ssh ts-val-01 'sudo tailscale status'
   ```

2. **Check Tailscale on your device:**
   ```bash
   tailscale status  # On your Mac
   # Or check Tailscale app on phone
   ```

3. **Verify IP is correct:**
   ```bash
   ssh ts-val-01 'tailscale ip -4'
   # Should show: 100.96.84.69
   ```

### Dashboard shows stale data

1. **Check SSH keys work:**
   ```bash
   ssh ts-val-02 'echo test'  # Test connectivity
   ```

2. **Check validators are online:**
   ```bash
   /tmp/live-p2p-monitor.sh
   ```

3. **Increase refresh interval:**
   - Edit `/tmp/finality-dashboard.py`
   - Change `setTimeout(updateDashboard, 30000);` to longer interval

---

## 📚 Related Files

- `/tmp/deploy-dashboard-to-vm.sh` - Deployment script
- `/tmp/finality-dashboard.py` - Dashboard code
- `~/Desktop/etrid/FINALITY_DASHBOARD_GUIDE.md` - Full guide
- `~/Desktop/etrid/FINALITY_MONITORING_GUIDE.md` - P2P monitoring guide
- `~/Desktop/etrid/P2P_NETWORK_MAPPING_GUIDE.md` - P2P mapping guide

---

**Your finality dashboard is ready to deploy!** 🚀

Run `/tmp/deploy-dashboard-to-vm.sh` and access it from anywhere via Tailscale!
