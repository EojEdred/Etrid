# 🔧 Fix Whitepaper Viewer & Connect to Real Node Data

## 🎯 Two Issues to Fix

### Issue 1: Whitepaper showing "Loading content..."
- **Problem:** Content Security Policy (CSP) blocking AOS library from unpkg.com
- **Solution:** Remove AOS dependency or add to CSP whitelist

### Issue 2: Telemetry/Explorer not showing real data
- **Problem:** Apps not configured with your actual RPC endpoints
- **Solution:** Update apps to connect to your running FlareChain nodes

---

## 🔧 FIX 1: Whitepaper Viewer

### Problem Details:

The whitepaper file is uploaded correctly (461,512 bytes = 451 KB), but Hostinger's CSP blocks:
```
script-src 'self' 'unsafe-inline' https://cdn.tailwindcss.com
```

This blocks: `https://unpkg.com/aos@2.3.1/dist/aos.js` (Animate On Scroll library)

### Solution A: Remove AOS (Simplest - Recommended)

AOS is just for scroll animations - not essential for functionality.

**Create this file to upload:**

`/website/whitepaper/viewer-standalone-fixed.html`

Remove this line (around line ~12):
```html
<script src="https://unpkg.com/aos@2.3.1/dist/aos.js"></script>
```

And remove all `data-aos` attributes and the AOS init script at the bottom.

I'll create the fixed version for you.

### Solution B: Update .htaccess CSP (Advanced)

Add to `/website/whitepaper/.htaccess`:
```apache
<IfModule mod_headers.c>
Header set Content-Security-Policy "default-src 'self' https:; script-src 'self' 'unsafe-inline' https://cdn.tailwindcss.com https://unpkg.com; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com https://unpkg.com; font-src 'self' https://fonts.gstatic.com; img-src 'self' data: https:;"
</IfModule>
```

---

## 🔧 FIX 2: Connect Apps to Real Blockchain Data

### What You Need:

**Your FlareChain RPC endpoints:**
1. **HTTP RPC:** `http://YOUR_NODE_IP:9933`
2. **WebSocket RPC:** `ws://YOUR_NODE_IP:9944`
3. **Node IP:** (your validator node IP address)

### Apps That Need Configuration:

#### 1. Block Explorer (`/apps/explorer/`)
Needs to connect to your Substrate node RPC

#### 2. Telemetry (`/apps/telemetry/`)
Needs to connect to your Polkadot telemetry or validator metrics

#### 3. Network Monitoring (`/website/network/`)
Already configured with Grafana: http://98.71.91.84:3000

#### 4. Validator Dashboard (`/apps/validator/`)
Needs RPC endpoint to fetch validator data

---

## 📝 What I Need From You:

### For Blockchain Data Connection:

1. **What's your FlareChain node IP address?**
   - Example: `98.71.91.84` or `node.etrid.org`

2. **What ports are your RPC endpoints on?**
   - HTTP RPC (usually 9933): `______`
   - WebSocket RPC (usually 9944): `______`

3. **Are these ports publicly accessible?**
   - Can external websites connect to them?
   - Or do they need to be on same server?

4. **Do you have Polkadot.js telemetry running?**
   - If yes, what's the telemetry URL?

### Example Answers:
```
Node IP: 98.71.91.84
HTTP RPC: http://98.71.91.84:9933
WebSocket RPC: ws://98.71.91.84:9944
Public access: Yes (ports 9933, 9944 open in firewall)
Telemetry: Not configured yet
```

---

## 🚀 Quick Fixes I Can Do Now:

### Fix 1: Whitepaper (Remove AOS)

I'll create a version without AOS dependencies that will load immediately.

### Fix 2: Default RPC Configuration

I'll configure apps with common endpoints, you can update with your actual IPs.

---

## 🔍 Test Your RPC Endpoints

Run these to verify your nodes are accessible:

```bash
# Test HTTP RPC
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://98.71.91.84:9933

# Test WebSocket (if wscat installed)
wscat -c ws://98.71.91.84:9944

# Check if ports are open
nc -zv 98.71.91.84 9933
nc -zv 98.71.91.84 9944
```

If these work, your endpoints are ready!

---

## 📋 Summary

**To fix both issues, I need:**

1. **For whitepaper:** Nothing - I'll remove AOS dependency
2. **For real data:** Your RPC endpoint information (IP, ports, accessibility)

**Once you provide the RPC info, I'll:**
1. ✅ Create fixed whitepaper viewer (no AOS)
2. ✅ Update explorer config with your RPC
3. ✅ Update telemetry config with your endpoints
4. ✅ Update validator dashboard config
5. ✅ Create upload instructions

---

**Let me know your RPC endpoint details and I'll configure everything!** 🚀
