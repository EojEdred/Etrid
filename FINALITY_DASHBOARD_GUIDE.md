# ASF Finality Dashboard Guide

## 🖥️ GUI Options for Monitoring Finality

### Option 1: Web Dashboard (Recommended) ⭐

**Beautiful, real-time web interface accessible from anywhere**

#### Quick Start:

```bash
# Install Flask (if not already installed)
pip3 install flask

# Start the dashboard
chmod +x /tmp/finality-dashboard.py
python3 /tmp/finality-dashboard.py
```

**Access it at**: `http://localhost:5000`

#### Features:
- ✅ Real-time monitoring (updates every 30 seconds)
- ✅ Beautiful gradient UI with card layout
- ✅ Quorum progress bar
- ✅ Color-coded status indicators
- ✅ Individual validator cards showing:
  - Online/offline status
  - NewView broadcast status
  - Current view number
  - Finalized block height
  - P2P peer count
- ✅ Network-wide summary statistics
- ✅ Responsive design (works on mobile!)

#### Access From Anywhere:

**Option A: Via Tailscale (Most Secure)**
```bash
# On your Mac (where dashboard runs):
python3 /tmp/finality-dashboard.py

# Access from any device on Tailscale network:
http://<your-mac-tailscale-ip>:5000

# Find your Tailscale IP:
tailscale ip -4
```

**Option B: Via SSH Tunnel**
```bash
# From remote machine:
ssh -L 5000:localhost:5000 user@your-mac-ip

# Then access:
http://localhost:5000
```

**Option C: Cloud VM**
```bash
# Copy dashboard to a cloud VM:
scp /tmp/finality-dashboard.py user@cloud-vm:/home/user/

# Run on cloud VM:
python3 /home/user/finality-dashboard.py

# Access from anywhere:
http://<cloud-vm-ip>:5000
```

---

### Option 2: Terminal Dashboard (tmux)

**Split-screen terminal monitoring**

#### Quick Start:

```bash
# Install tmux (if not installed)
brew install tmux  # macOS

# Start the dashboard
tmux new -s finality
```

**Then inside tmux**, split into panes:

```bash
# Split horizontally: Ctrl+b then "
# Split vertically: Ctrl+b then %

# Navigate panes: Ctrl+b then arrow keys

# In different panes, run:

# Pane 1: Deployment progress
tail -f /tmp/view-transition-deployment.log

# Pane 2: Quick finality check (loops)
watch -n 30 /tmp/quick-finality-check.sh

# Pane 3: Gizzi logs (live NewView messages)
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 \
  'sudo journalctl -u flarechain-validator -f | grep -E "NewView|finalized #"'

# Pane 4: Validator status summary
watch -n 60 'echo "Online validators:" && \
  grep -c "✅ ONLINE" /tmp/finality-reports/finality_report_*.txt 2>/dev/null || echo "Run /tmp/monitor-finality-network.sh first"'
```

**tmux Cheat Sheet**:
- Detach: `Ctrl+b then d`
- Re-attach: `tmux attach -t finality`
- Kill session: `tmux kill-session -t finality`

---

### Option 3: Terminal UI with watch

**Simple, no installation required**

```bash
# Auto-refreshing terminal display
watch -n 30 -c /tmp/quick-finality-check.sh
```

**Features**:
- Updates every 30 seconds
- Color support with `-c` flag
- Shows NewView counts, views, finalized blocks

---

### Option 4: Grafana + Prometheus (Advanced)

**Professional monitoring for production**

If you want enterprise-grade monitoring:

1. **Install Prometheus** on a monitoring server
2. **Configure substrate-telemetry** on each validator
3. **Set up Grafana** dashboards
4. **Add custom metrics** for view transitions

**Pros**: Production-ready, alerts, historical data
**Cons**: Complex setup, requires infrastructure

---

## 🎯 Recommended Setup

### For Development/Testing:
Use **Web Dashboard** - beautiful, easy, accessible

### For Production Operations:
Use **Web Dashboard** + **tmux** combo:
- Web dashboard for overview and remote access
- tmux for deep dive and log monitoring

---

## 📊 Dashboard Screenshots (What You'll See)

### Web Dashboard Layout:

```
┌─────────────────────────────────────────────────────┐
│         🔥 ASF Finality Monitor                     │
│    Real-time View Transition & Finality Status      │
├─────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌────┐ │
│  │Validators│  │ NewView  │  │ Finality │  │View│ │
│  │  Online  │  │Broadcast │  │  Status  │  │ ✅ │ │
│  │  22/22   │  │  15/15   │  │ ✅Active │  │Syn │ │
│  │          │  │ ████░░░  │  │  #1234   │  │ ced│ │
│  └──────────┘  └──────────┘  └──────────┘  └────┘ │
├─────────────────────────────────────────────────────┤
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐     │
│  │Gizzi │ │val-1 │ │val-2 │ │val-3 │ │val-4 │     │
│  │  ● ON│ │  ● ON│ │  ● ON│ │  ● ON│ │  ● ON│     │
│  │📤    │ │📤    │ │📤    │ │📤    │ │📤    │     │
│  │View 5│ │View 5│ │View 5│ │View 5│ │View 5│     │
│  │ #1234│ │ #1234│ │ #1234│ │ #1234│ │ #1234│     │
│  │18 prs│ │20 prs│ │19 prs│ │17 prs│ │21 prs│     │
│  └──────┘ └──────┘ └──────┘ └──────┘ └──────┘     │
│  ... (17 more validators) ...                       │
└─────────────────────────────────────────────────────┘
```

---

## 🚀 Running the Dashboard

### Step 1: Install Dependencies

```bash
# Python 3 should already be installed on macOS
# Install Flask:
pip3 install flask

# Verify:
python3 -c "import flask; print('Flask installed!')"
```

### Step 2: Start Dashboard

```bash
# Make executable
chmod +x /tmp/finality-dashboard.py

# Run it
python3 /tmp/finality-dashboard.py
```

You'll see:
```
============================================================
🔥 ASF Finality Dashboard Starting
============================================================

Dashboard will be available at:
  Local:  http://localhost:5000
  Network: http://<your-ip>:5000

Press Ctrl+C to stop

Updating validator status at 2025-11-18 17:30:00
Status updated: 22/22 online, 15/15 broadcasting
```

### Step 3: Open in Browser

**On your Mac**:
```bash
# Auto-open
open http://localhost:5000
```

**From another device**:
```
http://<your-mac-ip>:5000
```

---

## 🔧 Customization

### Change Update Frequency

Edit `/tmp/finality-dashboard.py`, line ~160:

```python
# Change from 30 seconds to 10 seconds:
time.sleep(10)  # was: time.sleep(30)
```

### Change Port

```python
# Change from 5000 to 8080:
app.run(host='0.0.0.0', port=8080, debug=False)
```

### Add Alerts

Add webhook or email notifications when finality stalls:

```python
if summary['max_finalized_block'] == 0 and summary['quorum_achieved']:
    send_alert("Finality stuck despite quorum!")
```

---

## 🐛 Troubleshooting

### Dashboard won't start

**Error**: `ModuleNotFoundError: No module named 'flask'`

**Fix**:
```bash
pip3 install flask
# or
python3 -m pip install flask
```

### Can't access from other devices

**Issue**: Firewall blocking port 5000

**Fix**:
```bash
# macOS: Allow Flask through firewall
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --add /usr/local/bin/python3
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --unblockapp /usr/local/bin/python3
```

### Validators show as offline

**Issue**: SSH keys not found or permissions

**Fix**:
```bash
# Check SSH keys exist
ls -la ~/.ssh/gizzi-validator
ls -la ~/.ssh/contabo-validators

# Fix permissions if needed
chmod 600 ~/.ssh/gizzi-validator
chmod 600 ~/.ssh/contabo-validators
```

### Dashboard updates slowly

**Issue**: SSH timeouts

**Fix**: Reduce connection timeout in dashboard code:
```python
# Change from 3 seconds to 2 seconds
cmd = f"ssh -i {validator['key']} -o ConnectTimeout=2 ..."
```

---

## 📱 Mobile Access

The web dashboard is fully responsive and works on:
- iPhone/iPad (Safari)
- Android (Chrome)
- Tablets

Just open: `http://<your-mac-ip>:5000` on your mobile browser

---

## 🎬 Next Steps

1. **Start the dashboard**: `python3 /tmp/finality-dashboard.py`
2. **Open in browser**: `http://localhost:5000`
3. **Bookmark it**: Add to favorites for quick access
4. **Monitor deployment**: Watch as validators get view transitions
5. **Celebrate quorum**: When 15/22 validators are broadcasting!
6. **Verify finality**: When finalized blocks start increasing!

---

## 🔗 Integration with Other Tools

### Use with P2P Network Mapping

```bash
# Terminal 1: Run dashboard
python3 /tmp/finality-dashboard.py

# Terminal 2: Generate P2P topology
/tmp/map-p2p-network.sh

# View both:
# - Dashboard for real-time status
# - P2P map for network topology
```

### Use with Deployment Scripts

```bash
# Terminal 1: Run dashboard
python3 /tmp/finality-dashboard.py

# Terminal 2: Deploy view transitions
/tmp/deploy-view-transition-all-validators.sh

# Watch dashboard to see:
# - Validators coming online
# - NewView broadcast count increasing
# - Quorum progress bar filling up
```

---

## 💡 Pro Tips

1. **Keep it running**: Use `tmux` or `screen` to keep dashboard running even when you disconnect

2. **Bookmark the URL**: Add to browser bookmarks for quick access

3. **Share with team**: Give team members the Tailscale URL for collaborative monitoring

4. **Take screenshots**: Use during deployments for documentation

5. **Set up alerts**: Modify code to send Slack/Discord webhooks when milestones reached

---

## 🎯 Success Indicators

**Dashboard shows finality is working when you see**:

✅ 15+ validators broadcasting NewView
✅ View consensus shows "✅ Synced"
✅ Finality status shows "✅ Active"
✅ Max finalized block > 0 and increasing
✅ All validators showing same view number
✅ Green status dots for all validators

**This means ASF finality is successfully advancing!** 🎉
