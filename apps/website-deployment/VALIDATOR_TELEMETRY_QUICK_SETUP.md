# ⚡ ËTRID Validator Telemetry - Quick Setup Guide

**Date:** November 1, 2025
**Status:** Ready to Deploy
**Time Required:** 30 seconds per validator (~10-15 minutes for all 21)

---

## 🎯 What This Does

Connects all 21 validators to the centralized telemetry server so they appear on:
**https://etrid.org/telemetry/**

---

## 📋 Prerequisites

- ✅ Telemetry server running on 98.71.91.84:30334
- ✅ Website updated and deployed
- ✅ SSH access to validators
- ✅ Sudo permissions

---

## 🚀 Quick Setup (Per Validator)

### Step 1: SSH into Validator

```bash
ssh ubuntu@<VALIDATOR_IP>
```

### Step 2: Edit Systemd Service

```bash
sudo nano /etc/systemd/system/flarechain.service
```

### Step 3: Add Telemetry Flag

Find the `ExecStart` line and add this flag:

```bash
--telemetry-url 'ws://98.71.91.84:30334/submit 0'
```

**Example:** Before
```bash
ExecStart=/home/ubuntu/etrid/target/release/flarechain-node \
  --base-path /home/ubuntu/.flarechain \
  --chain flarechain \
  --name "Gizzi-Director-01" \
  --validator \
  --rpc-port 9933 \
  --ws-port 9944 \
  --prometheus-port 9615
```

**After:**
```bash
ExecStart=/home/ubuntu/etrid/target/release/flarechain-node \
  --base-path /home/ubuntu/.flarechain \
  --chain flarechain \
  --name "Gizzi-Director-01" \
  --validator \
  --rpc-port 9933 \
  --ws-port 9944 \
  --prometheus-port 9615 \
  --telemetry-url 'ws://98.71.91.84:30334/submit 0'
```

### Step 4: Reload and Restart

```bash
sudo systemctl daemon-reload
sudo systemctl restart flarechain
```

### Step 5: Verify Connection

```bash
sudo journalctl -u flarechain -f | grep -i telemetry
```

You should see:
```
✅ Connected to telemetry endpoint: ws://98.71.91.84:30334/submit
```

---

## 📊 All 21 Validators

| # | Validator Name | IP Address | Status |
|---|----------------|------------|--------|
| 1 | Gizzi-Director-01 | 64.181.215.19 | ⏳ Pending |
| 2 | EojEdred-Director-02 | 20.69.26.209 | ⏳ Pending |
| 3 | Audit-Director-03 | 129.80.122.34 | ⏳ Pending |
| 4 | FlareNode-Governance | 20.69.26.120 | ⏳ Pending |
| 5 | FlareNode-Consensus | 20.84.44.226 | ⏳ Pending |
| 6 | FlareNode-Runtime | 20.84.48.201 | ⏳ Pending |
| 7 | FlareNode-Compiler | 20.84.49.52 | ⏳ Pending |
| 8 | FlareNode-Multichain | 20.69.29.235 | ⏳ Pending |
| 9 | FlareNode-Oracle | 20.84.49.177 | ⏳ Pending |
| 10 | FlareNode-BTC | 20.84.49.178 | ⏳ Pending |
| 11 | FlareNode-ETH | 20.84.49.179 | ⏳ Pending |
| 12 | FlareNode-SOL | 20.84.49.180 | ⏳ Pending |
| 13 | ValidityNode-EDSC | 20.84.49.181 | ⏳ Pending |
| 14 | ValidityNode-Economics | 20.84.49.182 | ⏳ Pending |
| 15 | ValidityNode-Ethics | 20.84.49.183 | ⏳ Pending |
| 16 | ValidityNode-Docs | 20.84.49.184 | ⏳ Pending |
| 17 | ValidityNode-Testing | 20.84.49.185 | ⏳ Pending |
| 18 | ValidityNode-Security | 20.84.49.186 | ⏳ Pending |
| 19 | ValidityNode-Community | 20.84.49.187 | ⏳ Pending |
| 20 | ValidityNode-Research | 20.84.49.188 | ⏳ Pending |
| 21 | ValidityNode-Archive | 20.84.49.189 | ⏳ Pending |

---

## 🤖 Automated Setup Script

Want to configure all 21 validators at once? Use this script:

```bash
#!/bin/bash

VALIDATORS=(
    "ubuntu@64.181.215.19:Gizzi-Director-01"
    "ubuntu@20.69.26.209:EojEdred-Director-02"
    "ubuntu@129.80.122.34:Audit-Director-03"
    # ... add all 21
)

for val in "${VALIDATORS[@]}"; do
    IFS=':' read -r ssh_addr name <<< "$val"
    echo "🔧 Configuring $name..."

    ssh "$ssh_addr" << 'ENDSSH'
sudo sed -i '/--prometheus-port 9615/a\  --telemetry-url '"'"'ws://98.71.91.84:30334/submit 0'"'"' \\' /etc/systemd/system/flarechain.service
sudo systemctl daemon-reload
sudo systemctl restart flarechain
echo "✅ $name configured"
ENDSSH

done

echo "🎉 All validators configured!"
```

**Save as:** `configure-all-telemetry.sh`

**Run:**
```bash
chmod +x configure-all-telemetry.sh
./configure-all-telemetry.sh
```

---

## ✅ Verification

### Check Telemetry Dashboard

Visit: **https://etrid.org/telemetry/**

You should see all 21 validators appearing with:
- 🟢 Online status
- Current block height
- Finalized block
- Peer count
- Role badge (Director/FlareNode/ValidityNode)
- ASF finality metrics

### Check Telemetry Server

```bash
ssh compiler-dev01@98.71.91.84
sudo journalctl -u substrate-telemetry -f
```

You should see messages like:
```
✅ Validator connected from 64.181.215.19
📝 New node registered: Gizzi-Director-01
```

### Check Individual Validator

```bash
ssh ubuntu@<VALIDATOR_IP>
sudo journalctl -u flarechain | grep -i telemetry
```

Should show:
```
Connected to telemetry endpoint: ws://98.71.91.84:30334/submit
Telemetry: Sending system info
Telemetry: Sending block #142850
```

---

## 🔧 Troubleshooting

### Issue: "Connection refused" in validator logs

**Cause:** Firewall blocking port 30334 or telemetry server offline

**Fix:**
```bash
# Check telemetry server is running
curl http://98.71.91.84:30334/
# Should return 200 OK

# Check validator can reach telemetry server
telnet 98.71.91.84 30334
# Should connect
```

### Issue: "WebSocket error" in validator logs

**Cause:** Wrong WebSocket URL format

**Fix:** Ensure the flag is exactly:
```bash
--telemetry-url 'ws://98.71.91.84:30334/submit 0'
```

(Note the single quotes and space before 0)

### Issue: Validator appears but immediately disconnects

**Cause:** Telemetry server rejecting messages

**Fix:** Check telemetry server logs:
```bash
ssh compiler-dev01@98.71.91.84
sudo journalctl -u substrate-telemetry -f
```

Look for error messages about message parsing.

### Issue: Validator doesn't appear at all

**Checklist:**
1. ✅ Telemetry flag added correctly?
2. ✅ Service restarted: `sudo systemctl restart flarechain`
3. ✅ Daemon reloaded: `sudo systemctl daemon-reload`
4. ✅ No errors in logs: `sudo journalctl -u flarechain -n 50`
5. ✅ Validator is actually running: `systemctl status flarechain`

---

## 📈 Expected Results

### Before Configuration:
- Telemetry dashboard shows: "⏳ Waiting for validators to connect..."
- 0/21 validators online
- Empty validator table

### After Configuration:
- All 21 validators visible
- 21/21 validators online (100% committee participation)
- Real-time block heights updating
- ASF consensus health showing "Healthy"
- PPFA proposer rotation visible
- Finality confidence at 85-99%

---

## 🔐 Security Notes

1. **Telemetry is READ-ONLY**: It only sends metrics, cannot control validators
2. **No sensitive data**: Only public blockchain metrics are sent
3. **Firewall safe**: No inbound ports need to be opened on validators
4. **Unencrypted connection**: Telemetry uses WS not WSS (metrics are public anyway)

To upgrade to WSS (encrypted):
1. Install SSL certificate on 98.71.91.84
2. Configure Nginx for WSS reverse proxy
3. Update telemetry URL to `wss://telemetry.etrid.org/submit`

---

## 🎯 Next Steps

After all validators are configured:

1. ✅ Monitor dashboard for 24 hours
2. ✅ Verify all validators stay connected
3. ✅ Check ASF consensus metrics are accurate
4. ✅ Share dashboard URL with community

**Optional Improvements:**
- Set up Grafana dashboards (Prometheus already enabled on validators)
- Configure alerts for validator downtime
- Enable SSL/WSS for encrypted telemetry
- Add geographic location data to telemetry

---

## 📞 Support

If you encounter issues:

1. **Check telemetry server status:**
   ```bash
   curl http://98.71.91.84:30334/
   ```

2. **Check validator logs:**
   ```bash
   sudo journalctl -u flarechain -f | grep telemetry
   ```

3. **Restart telemetry server (if needed):**
   ```bash
   ssh compiler-dev01@98.71.91.84
   sudo systemctl restart substrate-telemetry
   ```

4. **Hard refresh browser:**
   - Chrome: Ctrl+Shift+R
   - Firefox: Ctrl+F5
   - Safari: Cmd+Shift+R

---

**Ready to configure your validators? Follow the steps above for each validator, or use the automated script for bulk configuration!** 🚀
