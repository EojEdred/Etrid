# ⚡ QUICK START - Deploy ËTRID Website Updates

## 🚀 Deploy Everything (10 Minutes Total)

```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment "

# 1. Upload Ivory Papers Fix (5 min)
python3 upload-ivory-papers-ftp.py

# 2. Upload Network Monitoring (5 min)
python3 upload-monitoring-page.py
```

**Both scripts will ask for your Hostinger FTP credentials.**

---

## 🔑 Get FTP Credentials

1. Log into **Hostinger**
2. Go to: **Hosting → Manage → FTP Accounts**
3. Copy:
   - FTP Host (e.g., ftp.etrid.org)
   - Username
   - Password

---

## ✅ What You'll Get

### 1. Ivory Papers Viewer - Fixed
- **URL:** https://etrid.org/whitepaper/viewer-standalone.html
- **What:** Complete ivory papers (all 4 volumes)
- **Fix:** File now uploads completely (was truncated before)

### 2. Network Monitoring - NEW!
- **URL:** https://etrid.org/network/
- **What:** Live blockchain validator metrics
- **Shows:**
  - 21 validators status (up/down)
  - CPU, Memory, Network, Disk usage
  - Real-time updates every 30 seconds
- **Grafana:** http://98.71.91.84:3000

### 3. Homepage - Updated
- **URL:** https://etrid.org/
- **What:** Navigation now includes "Network" link
- **Goes to:** /network/ monitoring dashboard

---

## 🧪 Test After Deployment

### Test Ivory Papers:
```
https://etrid.org/whitepaper/viewer-standalone.html
```
✅ Should load instantly with full content
✅ All volume buttons work
✅ Download buttons work

### Test Monitoring:
```
https://etrid.org/network/
```
✅ Dashboard loads (may take 5-10 seconds)
✅ Shows 21 validators
✅ Charts display metrics
✅ Auto-refreshes every 30 seconds

### Test Navigation:
```
https://etrid.org/
```
✅ Click "Network" in menu
✅ Goes to monitoring page

---

## 🆘 Quick Troubleshooting

### FTP Connection Failed
- Check host is correct (try without "ftp." prefix)
- Verify username/password
- Try different network if blocked

### Files Upload But Site Doesn't Update
- Clear browser cache: **Cmd+Shift+R** (Mac) or **Ctrl+F5** (Windows)
- Try incognito/private window
- Wait 2-3 minutes for CDN to update

### Monitoring Dashboard Blank
- Check Grafana is accessible: http://98.71.91.84:3000
- Check browser console (F12) for errors
- Verify port 3000 is not blocked by firewall

---

## 📚 Full Documentation

For detailed instructions, see:

- **MONITORING_UPDATED.md** - Complete monitoring guide with correct Grafana URL
- **COMPLETE_FIX_READY.md** - Ivory papers detailed guide
- **DEPLOYMENT_COMPLETE_SUMMARY.md** - Comprehensive overview of both

---

## ⏱️ Time Required

- Reading this: **2 minutes**
- Getting FTP credentials: **1 minute**
- Uploading ivory papers: **3 minutes**
- Uploading monitoring: **3 minutes**
- Testing: **3 minutes**
- **Total: 12 minutes**

---

## 🎯 Ready?

Run the commands above and you're done!

**Both updates will be live in ~10 minutes.** 🚀
