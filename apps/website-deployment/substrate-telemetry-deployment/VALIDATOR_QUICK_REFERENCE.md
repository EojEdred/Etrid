# 🚀 ËTRID Validator Telemetry - Quick Reference Card

**Telemetry Server:** http://98.71.91.84:30334
**Status:** ✅ Running on port 30334
**Date:** November 1, 2025

---

## 📋 ONE-LINE FIX (Add to Each Validator)

```bash
--telemetry-url 'ws://98.71.91.84:30334/submit 0' \
```

---

## ⚡ 30-Second Setup Per Validator

### Step 1: Edit Service File
```bash
sudo nano /etc/systemd/system/flarechain.service
```

### Step 2: Add Telemetry Line

**Find this section:**
```
--validator \
--name 'ValidatorName' \
--base-path /data/validator \
```

**Add the telemetry line right after `--validator \`:**
```
--validator \
--telemetry-url 'ws://98.71.91.84:30334/submit 0' \
--name 'ValidatorName' \
--base-path /data/validator \
```

### Step 3: Restart
```bash
sudo systemctl daemon-reload
sudo systemctl restart flarechain
```

### Step 4: Verify
```bash
sudo journalctl -u flarechain -n 20 | grep telemetry
```

---

## 📊 Before & After Example

### ❌ BEFORE (No Telemetry)
```ini
[Service]
ExecStart=/usr/local/bin/flarechain-node \
  --validator \
  --name 'Validator-01' \
  --base-path /data/validator \
  --chain flarechain \
  --port 30333
```

### ✅ AFTER (With Telemetry)
```ini
[Service]
ExecStart=/usr/local/bin/flarechain-node \
  --validator \
  --telemetry-url 'ws://98.71.91.84:30334/submit 0' \
  --name 'Validator-01' \
  --base-path /data/validator \
  --chain flarechain \
  --port 30333
```

---

## 🎯 All 21 Validators (Copy-Paste Ready)

### Validator List

| # | IP | Name | Status |
|---|---|---|---|
| 1 | 98.71.91.84 | Validator-01 | ⏳ Pending |
| 2 | 129.80.122.34 | AuditDev-Validator | ✅ Done |
| 3 | [IP] | Validator-03 | ⏳ Pending |
| 4 | [IP] | Validator-04 | ⏳ Pending |
| 5 | [IP] | Validator-05 | ⏳ Pending |
| 6 | [IP] | Validator-06 | ⏳ Pending |
| 7 | [IP] | Validator-07 | ⏳ Pending |
| 8 | [IP] | Validator-08 | ⏳ Pending |
| 9 | [IP] | Validator-09 | ⏳ Pending |
| 10 | [IP] | Validator-10 | ⏳ Pending |
| 11 | [IP] | Validator-11 | ⏳ Pending |
| 12 | [IP] | Validator-12 | ⏳ Pending |
| 13 | [IP] | Validator-13 | ⏳ Pending |
| 14 | [IP] | Validator-14 | ⏳ Pending |
| 15 | [IP] | Validator-15 | ⏳ Pending |
| 16 | [IP] | Validator-16 | ⏳ Pending |
| 17 | [IP] | Validator-17 | ⏳ Pending |
| 18 | [IP] | Validator-18 | ⏳ Pending |
| 19 | [IP] | Validator-19 | ⏳ Pending |
| 20 | [IP] | Validator-20 | ⏳ Pending |
| 21 | [IP] | Validator-21 | ⏳ Pending |

---

## 🔍 Verification Commands

### Check if telemetry is in service file:
```bash
sudo cat /etc/systemd/system/flarechain.service | grep telemetry
```

### Check validator logs for telemetry connection:
```bash
sudo journalctl -u flarechain -n 50 | grep -i telemetry
```

### Check if validator is running:
```bash
sudo systemctl status flarechain
```

### View live on telemetry dashboard:
Open: http://98.71.91.84:30334

---

## ⚠️ Common Issues & Fixes

### Issue 1: "Connection refused"
**Cause:** Telemetry server is down
**Fix:** Check server status on 98.71.91.84
```bash
ssh compiler-dev01@98.71.91.84
sudo systemctl status substrate-telemetry
```

### Issue 2: "No telemetry in logs"
**Cause:** Line not added correctly
**Fix:** Check service file spacing:
```bash
sudo cat /etc/systemd/system/flarechain.service
# Each line should end with space + backslash + newline
```

### Issue 3: "Validator not appearing on dashboard"
**Cause:** Takes 30-60 seconds after restart
**Fix:** Wait 1 minute, then refresh: http://98.71.91.84:30334

### Issue 4: "Service won't start after edit"
**Cause:** Syntax error in service file
**Fix:** Check for missing `\` at end of lines
```bash
sudo systemctl status flarechain
# Look for error messages
```

---

## 📱 Quick Copy-Paste Commands

### All-in-one (run on each validator):
```bash
# Backup current service file
sudo cp /etc/systemd/system/flarechain.service /etc/systemd/system/flarechain.service.backup

# Edit service file (add telemetry line manually)
sudo nano /etc/systemd/system/flarechain.service

# Restart
sudo systemctl daemon-reload && sudo systemctl restart flarechain

# Verify
sudo journalctl -u flarechain -n 20 | grep -E "(telemetry|Telemetry)"
```

---

## 🎯 Success Indicators

You'll know it's working when you see:

1. ✅ In validator logs:
```
Connected to telemetry at ws://98.71.91.84:30334/submit
```

2. ✅ On telemetry dashboard (http://98.71.91.84:30334):
   - Validator name appears in the list
   - Block height is updating
   - "Online" status shown

3. ✅ In telemetry server logs:
```bash
ssh compiler-dev01@98.71.91.84
sudo journalctl -u substrate-telemetry -n 20
# Should show: "✅ Validator connected"
```

---

## 📞 Need Help?

### Check telemetry server status:
```bash
curl http://98.71.91.84:30334
# Should return HTML with "ËTRID Telemetry"
```

### Restart telemetry server (if needed):
```bash
ssh compiler-dev01@98.71.91.84
sudo systemctl restart substrate-telemetry
sudo systemctl status substrate-telemetry
```

### View all connected validators:
```bash
ssh compiler-dev01@98.71.91.84
sudo journalctl -u substrate-telemetry -f
# Watch for "✅ Validator connected" messages
```

---

## ⏱️ Time Estimate

- **Per validator:** 30 seconds - 1 minute
- **All 20 remaining:** 10-20 minutes total
- **Verification:** 5 minutes

**Total:** ~15-25 minutes for complete network visibility

---

## 🎉 What You Get After Setup

Once all 21 validators are configured:

- ✅ **Real-time network dashboard** at http://98.71.91.84:30334
- ✅ **See all validators** reporting simultaneously
- ✅ **Monitor block production** across the network
- ✅ **Detect issues** immediately if a validator goes offline
- ✅ **Network statistics** (total validators, block height, etc.)

---

**Last Updated:** November 1, 2025
**Telemetry Server:** 98.71.91.84:30334
**Status:** ✅ Operational
