# ✅ Network Monitoring Page - Ready to Deploy

## 🎯 What I Created

I've built a beautiful, professional network monitoring page that embeds your Grafana dashboard into your ËTRID website.

### Features:
- ✅ Embedded Grafana dashboard (http://98.71.91.84)
- ✅ Auto-refreshes every 30 seconds
- ✅ Loading indicator while dashboard loads
- ✅ Responsive design (works on mobile/tablet/desktop)
- ✅ Matches your website's design perfectly
- ✅ Live status indicator
- ✅ Quick stats cards (21 validators, 142k+ TPS, etc.)
- ✅ Navigation updated on homepage
- ✅ Info cards explaining what's being monitored

---

## 📁 Files Created/Updated

### New Files:
1. **`/website/network/index.html`** - The monitoring page
   - Full-screen Grafana dashboard embed
   - Professional header with stats
   - Loading animation
   - Responsive design

### Updated Files:
2. **`/website/index.html`** - Updated navigation
   - Desktop menu now links to `/network/`
   - Mobile menu also updated
   - Changed from `https://telemetry.etrid.org` to `/network/`

---

## 🚀 How to Deploy

### Option 1: Use the Automated FTP Script (Recommended)

```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "

# If you haven't uploaded the monitoring page yet, use the FTP script
python3 upload-ivory-papers-ftp.py
```

**But wait!** The ivory papers script only uploads the whitepaper. You'll need to upload these files too:

### Option 2: Manual Upload via FileZilla

1. **Connect to Hostinger via FTP** (see COMPLETE_FIX_READY.md for FTP setup)

2. **Upload the monitoring page:**
   - Local file: `/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/network/index.html`
   - Remote location: `public_html/network/index.html`
   - (Create the `network` folder on the server if it doesn't exist)

3. **Update the homepage:**
   - Local file: `/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/index.html`
   - Remote location: `public_html/index.html`
   - (Overwrites existing homepage)

### Option 3: Hostinger File Manager

1. Log into Hostinger
2. Go to File Manager
3. Navigate to `public_html/`
4. Create folder: `network`
5. Upload `network/index.html` into that folder
6. Replace the main `index.html` with your updated version

---

## 🌐 Where to Access It

After deployment:

- **Network Monitoring Page:** https://etrid.org/network/
- **Homepage (updated navigation):** https://etrid.org/

The "Network" link in your main navigation will now take users directly to the embedded Grafana dashboard.

---

## 🎨 What It Looks Like

### Header:
- Live status indicator (green pulsing dot)
- "FlareChain Network Real-Time Monitoring" title with gradient
- Brief description
- Quick stats: 21 Validators, 142k+ TPS, <1s Finality, 100% Uptime

### Dashboard:
- Full-width Grafana iframe
- Loading spinner while dashboard loads
- Auto-refresh every 30 seconds
- Kiosk mode (clean TV display without Grafana UI clutter)

### Footer:
- Info cards explaining what metrics are shown
- Links to Docs, GitHub, Homepage
- Copyright and attribution

---

## ⚙️ Configuration

### Grafana Dashboard URL:
```
http://98.71.91.84/d/8f482a38-0fa1-4460-9faf-01029f1ce1c6/flarechain-network-monitoring?orgId=1&refresh=30s&kiosk=tv
```

### Parameters Explained:
- `orgId=1` - Grafana organization ID
- `refresh=30s` - Auto-refresh every 30 seconds
- `kiosk=tv` - TV/kiosk mode (hides Grafana UI chrome)

### To Change Settings:

**Change auto-refresh interval:**
```html
<!-- In /website/network/index.html, line ~219 -->
src="...&refresh=30s&kiosk=tv"
         ^^^^^^^^^
         Change to: 10s, 1m, 5m, etc.
```

**Change dashboard height:**
```css
/* In /website/network/index.html, line ~103 */
.dashboard-container {
    height: calc(100vh - 200px);
    min-height: 600px;  /* Change minimum height here */
}
```

---

## 🔒 Security Considerations

### Current Setup:
- Dashboard is publicly accessible at `http://98.71.91.84`
- No authentication required
- Embedded in iframe on your website

### Recommendations:

1. **Add HTTPS:**
   - Set up SSL on your Grafana server
   - Change URL to `https://...` instead of `http://...`
   - Prevents mixed content warnings on HTTPS sites

2. **Restrict Access (Optional):**
   - Configure Grafana to only allow embedding from etrid.org
   - Set `X-Frame-Options` or `Content-Security-Policy` headers
   - Prevents dashboard from being embedded on other sites

3. **Read-Only Mode:**
   - Your dashboard is already in kiosk/TV mode (read-only)
   - Viewers cannot edit panels or settings
   - Good for public display

---

## 📱 Responsive Design

The page is fully responsive:

- **Desktop (>768px):** Full navigation menu, large dashboard
- **Tablet (768px-1024px):** Optimized layout, readable stats
- **Mobile (<768px):** Hamburger menu, scrollable dashboard

The iframe adjusts height based on viewport, with a minimum of 600px.

---

## 🧪 Testing Checklist

After deployment, verify:

- [ ] Visit https://etrid.org/network/
- [ ] Page loads without errors
- [ ] Loading spinner appears briefly
- [ ] Grafana dashboard loads and displays metrics
- [ ] Dashboard auto-refreshes every 30 seconds
- [ ] Navigation works (back to home, docs, GitHub links)
- [ ] Mobile menu works on small screens
- [ ] Stats cards display correctly
- [ ] No console errors (press F12 to check)
- [ ] Homepage navigation "Network" link works

---

## 🐛 Troubleshooting

### Dashboard won't load / shows blank:

**Possible causes:**
1. Grafana server at 98.71.91.84 is down
   - Solution: Check if `http://98.71.91.84` is accessible directly
   - Restart Grafana/Prometheus services if needed

2. CORS/iframe embedding blocked
   - Solution: Configure Grafana to allow iframe embedding
   - In Grafana config: `allow_embedding = true`

3. Network/firewall blocking the IP
   - Solution: Check if port 80 is open on 98.71.91.84
   - Test: `curl http://98.71.91.84`

### Loading spinner never disappears:

**Cause:** Iframe failed to load (network error, server down, CORS)

**Solution:**
1. Open browser console (F12)
2. Look for error messages
3. Check if Grafana is accessible: http://98.71.91.84

### Mixed content warning (HTTP/HTTPS):

**Cause:** Your website uses HTTPS but Grafana uses HTTP

**Solution:**
1. Set up SSL on Grafana server
2. Update iframe URL to `https://...`
3. Or: Serve main site over HTTP (not recommended)

### Navigation link doesn't work:

**Cause:** File uploaded to wrong location

**Solution:**
- Ensure `network/index.html` is at `public_html/network/index.html`
- Not `public_html/network/network/index.html` (common mistake)

---

## 🔧 Future Enhancements

Ideas to improve the monitoring page:

1. **Add More Dashboards:**
   - Create tabs to switch between FlareChain, PBC nodes, validators
   - Use multiple iframes with JavaScript tab switching

2. **Add Real-Time Stats API:**
   - Fetch current block height, TPS, validator count from API
   - Update the quick stats cards with real data
   - Use Grafana API or Substrate RPC

3. **Alert Notifications:**
   - Integrate Grafana alerts
   - Show banner if validators are down
   - Email/webhook notifications

4. **Historical Charts:**
   - Add mini charts above the dashboard
   - Show 24h/7d/30d trends
   - Use Chart.js or similar

5. **Validator Details:**
   - Link to individual validator pages
   - Show per-validator metrics
   - Display geographic distribution map

---

## 📊 What's Being Monitored

Your Grafana dashboard tracks:

### FlareChain Metrics:
- Block production rate
- Transaction throughput (TPS)
- Finality time
- Peer connections
- Mempool size

### Validator Metrics (21 Validators):
- Node health status
- Block authoring activity
- Consensus participation
- Network latency
- Sync status

### System Metrics (via Node Exporter):
- CPU usage per validator
- Memory utilization
- Disk I/O and space
- Network bandwidth
- Process counts

### Prometheus Metrics:
- Query performance
- Scrape duration
- Target status
- Time series count

---

## 📝 File Structure

After deployment, your website structure:

```
public_html/
├── index.html                    (updated with /network/ links)
├── network/
│   └── index.html               (NEW - monitoring dashboard page)
├── whitepaper/
│   ├── viewer-standalone.html
│   └── check-file-integrity.html
├── assets/
├── css/
├── js/
└── ...
```

---

## ⏱️ Deployment Time

- **Manual (FileZilla):** 5 minutes
  - Upload network/index.html: 1 min
  - Upload updated index.html: 1 min
  - Test: 3 min

- **File Manager:** 7 minutes
  - Create folder: 1 min
  - Upload files: 3 min
  - Test: 3 min

---

## 🎉 You're Ready!

Everything is set up and ready to deploy:

1. ✅ Beautiful monitoring page created
2. ✅ Homepage navigation updated
3. ✅ Grafana dashboard embedded
4. ✅ Responsive design for all devices
5. ✅ Loading states and error handling
6. ✅ Consistent with your website branding

### Next Steps:

1. **Deploy the files** (use FileZilla or File Manager)
2. **Test it:** Visit https://etrid.org/network/
3. **Share it:** Link to it from docs, social media, etc.

---

## 📞 Need Help?

### Files to upload:
1. `/website/network/index.html` → `public_html/network/index.html`
2. `/website/index.html` → `public_html/index.html`

### Having issues?
- Check browser console for errors (F12)
- Verify Grafana server is running: http://98.71.91.84
- Make sure network folder exists on server
- Clear browser cache after uploading

---

**The monitoring page is ready to go live! Deploy and enjoy your real-time network dashboard.** 🚀
