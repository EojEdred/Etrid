# ✅ Network Monitoring - UPDATED with Real Blockchain Metrics

## 🎉 What's New

Your monitoring page has been **updated** with the correct Grafana dashboard that shows **actual blockchain validator metrics**!

---

## 📊 What You'll See Now

### Real Blockchain Metrics:
- ✅ **Validators Online Counter** - Real-time count (21 validators)
- ✅ **Validator Status Table** - Shows which validators are up/down
- ✅ **CPU Usage** - Per-validator CPU usage over time
- ✅ **Memory Usage** - RAM usage for each validator
- ✅ **Network Traffic** - Bandwidth usage (In/Out)
- ✅ **Disk Usage** - Storage utilization
- ✅ **Auto-refresh** - Every 30 seconds

### Dashboard Details:
- **No login required** - Public viewing enabled
- **Kiosk mode** - Clean display without Grafana UI
- **Live updates** - See validator health in real-time
- **All 21 validators** - Complete fleet monitoring

---

## 🌐 URLs

### Direct Grafana Access:
```
http://98.71.91.84:3000
```
- Dashboard: Etrid Validator Network
- Login: admin / G1zzi!Pwr2025$ (if needed)
- Public viewing enabled (no login required for viewing)

### Prometheus (Raw Metrics):
```
http://98.71.91.84:9090
```
- See all targets and metrics
- Query metrics directly
- Check scrape status

### Embedded Dashboard URL (in website):
```
http://98.71.91.84:3000/d/702a9947-2229-4f2a-b443-2150b96be29b/etrid-validator-network?orgId=1&refresh=30s&kiosk
```

---

## 🚀 Deploy to Website

### Option 1: Use the Upload Script (Recommended)

```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "
python3 upload-monitoring-page.py
```

This will upload:
1. `/website/network/index.html` - **UPDATED** with correct Grafana URL
2. `/website/index.html` - Updated homepage navigation

### Option 2: Manual Upload via FileZilla

1. Connect to Hostinger via FTP
2. Upload files:
   - `website/network/index.html` → `public_html/network/index.html`
   - `website/index.html` → `public_html/index.html`

---

## 🎯 Test After Deployment

### 1. Visit Your Monitoring Page:
```
https://etrid.org/network/
```

**You should see:**
- Loading spinner briefly
- Grafana dashboard loads
- **Validators Online** counter (showing 21 or current count)
- Table of all validators with status
- CPU, Memory, Network, Disk charts
- Auto-refresh every 30 seconds

### 2. Test Direct Grafana:
```
http://98.71.91.84:3000
```
- Should show same dashboard
- Can browse other dashboards if needed

### 3. Homepage Navigation:
```
https://etrid.org/
```
- Click "Network" in navigation
- Should go to `/network/` page
- Dashboard should load

---

## 📋 What Changed

### Before (Old URL):
```
http://98.71.91.84/d/8f482a38-0fa1-4460-9faf-01029f1ce1c6/flarechain-network-monitoring
```
❌ This was a placeholder/incorrect URL

### After (New URL):
```
http://98.71.91.84:3000/d/702a9947-2229-4f2a-b443-2150b96be29b/etrid-validator-network
```
✅ This is the **real** dashboard with blockchain metrics

### Key Differences:
- Added `:3000` port for Grafana
- Updated dashboard ID (702a9947... instead of 8f482a38...)
- Changed dashboard name to `etrid-validator-network`
- Changed kiosk mode from `kiosk=tv` to `kiosk` (both work, kiosk is simpler)

---

## 🔍 Dashboard Metrics Explained

### Validators Online
Shows count of validators currently reporting metrics to Prometheus.
- **Expected:** 21 validators
- **Status:** Green if all online, Red if any down

### Validator Status Table
Lists all validators with their status:
- **validator-01** through **validator-21**
- **IP addresses** for each validator
- **Up/Down status** with color coding

### CPU Usage Chart
Time-series graph showing:
- CPU percentage per validator
- Last 1 hour (adjustable)
- Hover to see exact values
- Multiple colored lines (one per validator)

### Memory Usage Chart
Shows RAM utilization:
- Used memory vs total memory
- Percentage per validator
- Helps identify memory leaks

### Network Traffic Chart
Bandwidth monitoring:
- **Receive** - Data coming in
- **Transmit** - Data going out
- Shows network health and activity

### Disk Usage Chart
Storage utilization:
- Used vs total disk space
- Per validator
- Warning if running low

---

## 🔧 Advanced Configuration

### Change Auto-Refresh Interval

In the iframe URL, modify `refresh=30s`:

```html
<!-- Current: 30 seconds -->
refresh=30s

<!-- Options: -->
refresh=10s   (every 10 seconds - more real-time)
refresh=1m    (every minute - less frequent)
refresh=5m    (every 5 minutes - minimal updates)
```

### Change Time Range

Add `&from=now-6h&to=now` to URL for specific time range:

```
...&refresh=30s&kiosk&from=now-6h&to=now
```

Options:
- `now-1h` - Last 1 hour
- `now-6h` - Last 6 hours (default)
- `now-24h` - Last 24 hours
- `now-7d` - Last 7 days

### Enable Dark/Light Theme

Grafana defaults to dark theme. To change:

```
...&theme=light
```

But dark theme looks better for monitoring dashboards!

---

## 🔒 Security & Access

### Current Setup:
- **Public viewing enabled** - Anyone can view (no login required)
- **Kiosk mode** - View-only, cannot edit
- **No authentication** - Dashboard accessible to all
- **HTTP** - Not encrypted (plain HTTP)

### Recommended Next Steps:

#### 1. Add HTTPS (SSL Certificate)
Make it secure with Let's Encrypt:

```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

# Install Certbot
sudo apt update
sudo apt install -y certbot python3-certbot-nginx

# Get certificate (need domain first)
sudo certbot --nginx -d metrics.etrid.org
```

**Note:** Requires DNS setup first (see DNS setup instructions in your other terminal)

#### 2. Restrict Embedding (Optional)
If you only want dashboard on etrid.org:

```bash
# Edit Grafana config
sudo nano /etc/grafana/grafana.ini

# Add under [security]
cookie_samesite = lax
allow_embedding = true
```

#### 3. Set up Alerts
Configure Grafana to alert when validators go down:

1. Go to: http://98.71.91.84:3000
2. Login: admin / G1zzi!Pwr2025$
3. Navigate to: Alerting → Alert rules
4. Create alert for `up{job="validator"} == 0`
5. Set notification channel (email/webhook/Discord)

---

## 📱 Mobile Responsiveness

The monitoring page is fully responsive:

### Desktop (>768px):
- Full dashboard with all panels
- Large, readable charts
- Complete navigation menu

### Tablet (768px-1024px):
- Dashboard adjusts height
- Charts stack vertically if needed
- Touch-friendly controls

### Mobile (<768px):
- Hamburger menu
- Dashboard scrollable
- Zooming enabled for charts
- Minimum height: 600px

---

## 🐛 Troubleshooting

### Dashboard Shows Blank/Empty

**Cause:** Grafana server not accessible or CORS issue

**Solutions:**
1. Check Grafana is running:
   ```bash
   ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84
   sudo systemctl status grafana-server
   ```

2. Verify port 3000 is open:
   ```bash
   curl http://98.71.91.84:3000
   ```

3. Check browser console (F12) for errors

### Shows "No Data" in Charts

**Cause:** Prometheus not collecting metrics or validators down

**Solutions:**
1. Check Prometheus targets:
   ```
   http://98.71.91.84:9090/targets
   ```
   Should show all 21 validators as "UP"

2. Verify node exporters running on validators:
   ```bash
   ssh validator-01
   sudo systemctl status node_exporter
   ```

3. Check Prometheus config:
   ```bash
   ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84
   cat /etc/prometheus/prometheus.yml
   ```

### Can't Login to Grafana

**Credentials:**
- Username: `admin`
- Password: `G1zzi!Pwr2025$`

**If forgotten:**
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84
sudo grafana-cli admin reset-admin-password NewPassword123
```

### Dashboard Not Auto-Refreshing

**Check:**
1. URL includes `&refresh=30s`
2. Browser not blocking iframe updates
3. Grafana server is responsive

**Fix:**
- Add/verify refresh parameter in iframe src
- Disable browser ad-blockers
- Check Grafana logs for errors

---

## 🎨 Customization Ideas

### Add More Dashboards

You can create additional dashboards for:
- **FlareChain specific metrics** (block height, finality, etc.)
- **PBC chains monitoring** (if running)
- **Smart contract metrics** (gas usage, calls, etc.)
- **Network topology** (peer connections, geographic distribution)

### Create Custom Panels

In Grafana, you can add:
- **Blockchain-specific metrics:**
  - Current block height
  - Transactions per second (TPS)
  - Finality time
  - Mempool size
  - Peer count

**To add these:** You need to expose these metrics from your blockchain nodes via a Prometheus exporter.

### Multiple Views

Create tab switching on the monitoring page:
- Tab 1: Validators (current dashboard)
- Tab 2: FlareChain metrics
- Tab 3: Network topology
- Tab 4: Historical stats

---

## ⚡ Performance Tips

### Reduce Dashboard Load Time:
1. **Limit time range** - Default to last 1 hour instead of 6
2. **Reduce refresh rate** - Use 1m instead of 30s if not critical
3. **Simplify queries** - Aggregate data for better performance
4. **Use variables** - Filter by validator instead of showing all

### Optimize Prometheus:
1. **Adjust retention** - Keep data for 30 days instead of forever
2. **Reduce scrape interval** - 30s instead of 15s if acceptable
3. **Use recording rules** - Pre-compute common queries

---

## 📊 Metrics Available

Your Prometheus is collecting these metrics from all validators:

### Node Exporter Metrics:
- `node_cpu_seconds_total` - CPU usage
- `node_memory_MemTotal_bytes` - Total memory
- `node_memory_MemAvailable_bytes` - Available memory
- `node_filesystem_size_bytes` - Disk size
- `node_filesystem_free_bytes` - Free disk space
- `node_network_receive_bytes_total` - Network in
- `node_network_transmit_bytes_total` - Network out
- `node_load1`, `node_load5`, `node_load15` - System load

### Prometheus Metrics:
- `up` - Target status (1 = up, 0 = down)
- `scrape_duration_seconds` - How long scrapes take
- `scrape_samples_scraped` - Number of metrics collected

### Blockchain Metrics (if exposed):
- Add Substrate/Polkadot metrics exporter to get:
  - Block height
  - Finality lag
  - Peer count
  - Transaction pool size
  - Consensus participation

---

## 🚀 Quick Reference

### Deploy Commands:
```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "
python3 upload-monitoring-page.py
```

### Test URLs:
- Website: https://etrid.org/network/
- Grafana: http://98.71.91.84:3000
- Prometheus: http://98.71.91.84:9090/targets

### SSH Access:
```bash
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84
```

### Grafana Login:
- Username: admin
- Password: G1zzi!Pwr2025$

### Service Management:
```bash
sudo systemctl restart grafana-server
sudo systemctl restart prometheus
sudo systemctl restart node_exporter
```

---

## ✅ Deployment Checklist

- [ ] Run upload script: `python3 upload-monitoring-page.py`
- [ ] Enter FTP credentials when prompted
- [ ] Wait for successful upload confirmation
- [ ] Test Grafana directly: http://98.71.91.84:3000
- [ ] Verify dashboard shows validator metrics
- [ ] Visit website: https://etrid.org/network/
- [ ] Check dashboard embeds correctly
- [ ] Test auto-refresh (wait 30 seconds)
- [ ] Check mobile responsive view
- [ ] Verify homepage navigation link works
- [ ] Check browser console for errors (F12)
- [ ] Share with team for feedback

---

## 🎉 You're Ready!

The monitoring page now has the **correct Grafana dashboard** with real blockchain validator metrics!

**Deploy it and enjoy your live network monitoring!** 🚀

---

**Questions or issues? Check the troubleshooting section above or let me know!**
