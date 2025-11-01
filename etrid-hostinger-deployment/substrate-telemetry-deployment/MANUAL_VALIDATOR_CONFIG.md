# Manual Validator Telemetry Configuration

**Quick Guide - Add 1 Line to Each Validator**

---

## 🎯 What You Need to Do

For each of your 21 validators, add this single line to the flarechain service file:

```
--telemetry-url 'ws://98.71.91.84/submit 0' \
```

That's it! Takes about 30 seconds per validator.

---

## 📋 Step-by-Step Instructions

### 1. Connect to Validator
Use whatever method you normally use:
- Azure portal console
- Your existing SSH session
- Cloud shell
- etc.

### 2. Edit the Service File
```bash
sudo nano /etc/systemd/system/flarechain.service
```

### 3. Find the --validator Line
Look for the line that says `--validator` (usually near the bottom of the ExecStart section)

### 4. Add Telemetry Line Right After It
Add this line **immediately after** the `--validator \` line:

```
  --telemetry-url 'ws://98.71.91.84/submit 0' \
```

**Important:**
- Keep the same indentation (2 spaces at start)
- Keep the backslash `\` at the end
- Use single quotes around the URL

### 5. Save and Exit
- Press `Ctrl + O` to save
- Press `Enter` to confirm
- Press `Ctrl + X` to exit

### 6. Reload and Restart
```bash
sudo systemctl daemon-reload
sudo systemctl restart flarechain
```

### 7. Verify It's Working
```bash
# Check service is running
sudo systemctl status flarechain

# Look for telemetry in logs (wait 10-20 seconds)
sudo journalctl -u flarechain -n 50 | grep -i telemetry
```

You should see messages like:
- "Connected to telemetry"
- "Telemetry endpoint: ws://98.71.91.84/submit"

---

## 📄 Example - Before and After

### BEFORE:
```
ExecStart=/usr/local/bin/flarechain-node \
  --base-path /var/lib/flarechain \
  --chain mainnet \
  --name "Validator-01" \
  --validator \
  --port 30333 \
  --rpc-port 9933
```

### AFTER:
```
ExecStart=/usr/local/bin/flarechain-node \
  --base-path /var/lib/flarechain \
  --chain mainnet \
  --name "Validator-01" \
  --validator \
  --telemetry-url 'ws://98.71.91.84/submit 0' \
  --port 30333 \
  --rpc-port 9933
```

---

## 🔍 Verification

### Check Individual Validator
After configuring a validator, check if it appears:

**Option 1: Web UI**
Open in browser: http://98.71.91.84/

**Option 2: Website**
Open in browser: https://etrid.org/telemetry/

You should see your validator's name appear in the node list within 30 seconds.

### Check All Validators
After configuring all 21, you should see:
- **Total Validators:** 21 (or 20 if you skip the BUILD VM)
- **Online Validators:** Should match the number running

---

## ⚡ Quick Copy-Paste Commands

For each validator, run these commands in order:

```bash
# 1. Stop validator
sudo systemctl stop flarechain

# 2. Backup service file
sudo cp /etc/systemd/system/flarechain.service /etc/systemd/system/flarechain.service.backup

# 3. Add telemetry line (if not already there)
sudo sed -i '/--validator/a \  --telemetry-url '"'"'ws://98.71.91.84/submit 0'"'"' \\' /etc/systemd/system/flarechain.service

# 4. Reload and restart
sudo systemctl daemon-reload
sudo systemctl start flarechain

# 5. Check it worked
sleep 5
sudo journalctl -u flarechain -n 50 | grep -i telemetry
```

**Note:** This automated version does the same thing as the manual edit above.

---

## 🚨 Troubleshooting

### Validator Won't Start
```bash
# Check for syntax errors
sudo systemctl status flarechain

# View full logs
sudo journalctl -u flarechain -n 100
```

Common issues:
- Missing backslash `\` at end of line
- Wrong quotes (use single quotes: `'` not `"`)
- Missing space at start of line

**Fix:** Restore backup and try again:
```bash
sudo cp /etc/systemd/system/flarechain.service.backup /etc/systemd/system/flarechain.service
sudo systemctl daemon-reload
sudo systemctl start flarechain
```

### Not Showing in Telemetry
Wait 30-60 seconds, then check:

```bash
# Is validator running?
sudo systemctl status flarechain

# Is telemetry in the config?
grep telemetry /etc/systemd/system/flarechain.service

# Can it reach telemetry server?
curl -I http://98.71.91.84/
```

### Wrong Name Showing
The name in telemetry comes from the `--name` parameter in your service file. If you want to change it, edit that parameter.

---

## 📊 Validator List Reference

Configure these 21 validators (or 20, skipping #10 BUILD VM):

1. Gizzi (AI Overseer) - 64.181.215.19
2. EojEdred (Founder) - 20.69.26.209
3. Audit Dev - 129.80.122.34
4. Governance Dev - 20.186.91.207
5. Consensus Dev (Secondary) - 129.80.122.34
6. Runtime Dev (Primary) - 20.224.104.239
7. Runtime Dev (Secondary) - 108.142.205.177
8. Compiler Dev (Primary) - 4.180.238.67
9. Compiler Dev (Secondary) - 4.180.59.25
10. Multichain Dev (Primary) - 98.71.91.84 ⚠️ SKIP (BUILD VM)
11. Multichain Dev (Secondary) - 68.219.230.63
12. Oracle Dev - 98.71.219.106
13. EDSC Dev (Primary) - 172.167.8.217
14. EDSC Dev (Secondary) - 51.142.203.160
15. Economics Dev (Primary) - 172.166.164.19
16. Economics Dev (Secondary) - 172.166.187.180
17. Ethics Dev (Primary) - 172.166.210.244
18. Ethics Dev (Secondary) - 4.251.115.186
19. Docs Dev (Primary) - 52.143.191.232
20. Docs Dev (Secondary) - 4.211.206.210
21. Docs Dev (Tertiary) - 4.178.181.122

---

## ⏱️ Time Estimate

- **Per validator:** 30-60 seconds
- **All 20 validators:** 10-20 minutes total
- **Test on one first:** Recommended!

---

## 🎯 Recommended Approach

1. **Start with one validator** (pick any one you want)
2. Configure it following the steps above
3. Wait 30 seconds
4. Check if it appears at http://98.71.91.84/
5. If it works, proceed with the rest
6. If it doesn't work, let me know and we'll troubleshoot

---

## ✅ Success Checklist

- [ ] Backed up service file
- [ ] Added telemetry line with correct spacing/backslash
- [ ] Saved file correctly
- [ ] Ran daemon-reload
- [ ] Restarted service
- [ ] Service is running (systemctl status)
- [ ] Validator appears in telemetry UI within 60 seconds

---

## 🆘 Need Help?

If you run into issues:

1. Share the error message
2. Share output of: `sudo systemctl status flarechain`
3. Share the line you added
4. Let me know which validator you're configuring

---

**This is definitely the easier approach given the SSH access issues!**

**Once you configure a few, the rest will go very quickly.**
