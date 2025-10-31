# 🔧 FIX: Oracle Cloud VM Web Server Setup

## Your Server Details
- **IP Address:** 64.181.203.153
- **OS:** Ubuntu 22.04
- **Username:** ubuntu

## The Problem
File works locally but NOT when uploaded to server.

## Solution: Upload to Correct Directory

### Step 1: Connect to Your Server

```bash
ssh ubuntu@64.181.203.153
```

### Step 2: Check if Web Server is Running

```bash
# Check if Apache is installed
systemctl status apache2

# OR check if Nginx is installed
systemctl status nginx

# Check what's listening on port 80
sudo lsof -i :80
```

### Step 3: Find Your Web Root Directory

**If using Apache:**
```bash
# Web root is usually:
/var/www/html/
```

**If using Nginx:**
```bash
# Web root is usually:
/usr/share/nginx/html/
# OR
/var/www/html/
```

### Step 4: Upload Files to Correct Location

**Option A: Using SCP from your Mac:**

```bash
# From your Mac terminal:
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website"

# Upload entire whitepaper folder
scp -r whitepaper/ ubuntu@64.181.203.153:/var/www/html/

# OR upload just the viewer file
scp whitepaper/viewer-standalone.html ubuntu@64.181.203.153:/var/www/html/whitepaper/
```

**Option B: Using SFTP:**

```bash
# From your Mac:
sftp ubuntu@64.181.203.153

# Once connected:
cd /var/www/html
put -r /Users/macbook/Desktop/etrid/etrid-hostinger-deployment\ /website/whitepaper
exit
```

### Step 5: Set Correct Permissions

```bash
# SSH into server first
ssh ubuntu@64.181.203.153

# Then run:
sudo chown -R www-data:www-data /var/www/html/whitepaper/
sudo chmod -R 755 /var/www/html/whitepaper/
sudo chmod 644 /var/www/html/whitepaper/*.html
```

### Step 6: Test the URL

Visit: `http://64.181.203.153/whitepaper/viewer-standalone.html`

---

## Quick Diagnostic Commands

Run these ON THE SERVER (after SSH):

```bash
# 1. Check if web server is running
sudo systemctl status apache2 || sudo systemctl status nginx

# 2. Find web root
if [ -d "/var/www/html" ]; then echo "Web root: /var/www/html"; fi

# 3. Check if whitepaper folder exists
ls -la /var/www/html/whitepaper/

# 4. Check viewer file size
ls -lh /var/www/html/whitepaper/viewer-standalone.html

# 5. Check file permissions
stat /var/www/html/whitepaper/viewer-standalone.html
```

---

## If Web Server Not Installed

### Install Apache:

```bash
sudo apt update
sudo apt install apache2 -y
sudo systemctl start apache2
sudo systemctl enable apache2
```

### Install Nginx (Alternative):

```bash
sudo apt update
sudo apt install nginx -y
sudo systemctl start nginx
sudo systemctl enable nginx
```

---

## Expected File Structure on Server

```
/var/www/html/
├── whitepaper/
│   ├── viewer-standalone.html (219 KB) ← Main file
│   ├── viewer-standalone-debug.html (219 KB) ← Debug version
│   ├── test-simple.html
│   ├── diagnostic.html
│   ├── ivory-paper-vol1-conceptual.md
│   ├── ivory-paper-vol2-technical.md
│   ├── ivory-paper-vol3-governance.md
│   └── ivory-paper.md
```

---

## Test URLs

After upload, test these URLs:

- `http://64.181.203.153/whitepaper/test-simple.html` (should show "SUCCESS!")
- `http://64.181.203.153/whitepaper/viewer-standalone-debug.html` (open console F12)
- `http://64.181.203.153/whitepaper/viewer-standalone.html` (should work!)

---

## Common Issues

### Issue 1: "Connection refused"
**Cause:** Web server not running
**Fix:**
```bash
sudo systemctl start apache2
# OR
sudo systemctl start nginx
```

### Issue 2: "403 Forbidden"
**Cause:** Wrong permissions
**Fix:**
```bash
sudo chown -R www-data:www-data /var/www/html/whitepaper/
sudo chmod -R 755 /var/www/html/whitepaper/
```

### Issue 3: "404 Not Found"
**Cause:** File in wrong directory
**Fix:** Upload to `/var/www/html/whitepaper/` not somewhere else

### Issue 4: File works locally but not on server
**Cause:** You're opening `file://` protocol locally vs `http://` on server
**Fix:** Always test via `http://` - open `http://localhost/whitepaper/viewer-standalone.html` to test locally

---

## One-Command Upload + Fix

Run this from your Mac:

```bash
cd "/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website" && \
scp -r whitepaper/ ubuntu@64.181.203.153:/tmp/ && \
ssh ubuntu@64.181.203.153 "sudo mv /tmp/whitepaper /var/www/html/ && \
sudo chown -R www-data:www-data /var/www/html/whitepaper/ && \
sudo chmod -R 755 /var/www/html/whitepaper/ && \
sudo chmod 644 /var/www/html/whitepaper/*.html && \
echo 'Upload complete! Test at: http://64.181.203.153/whitepaper/viewer-standalone.html'"
```

This will:
1. Upload whitepaper folder
2. Move it to web root
3. Set correct ownership
4. Set correct permissions
5. Show you the test URL

---

## Need More Help?

Run these diagnostic commands and send me the output:

```bash
ssh ubuntu@64.181.203.153 "
echo '=== Web Server Status ==='
sudo systemctl status apache2 || sudo systemctl status nginx
echo ''
echo '=== Web Root Contents ==='
ls -la /var/www/html/
echo ''
echo '=== Whitepaper Folder ==='
ls -lh /var/www/html/whitepaper/ 2>/dev/null || echo 'Folder does not exist'
echo ''
echo '=== Viewer File Check ==='
ls -lh /var/www/html/whitepaper/viewer-standalone.html 2>/dev/null || echo 'File does not exist'
"
```

Send me this output and I'll tell you exactly what to fix!
