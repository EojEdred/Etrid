# ⚡ QUICKSTART: Deploy in 10 Minutes

**Status:** ⚠️ Requires SSH access to server 98.71.91.84

---

## 🎯 What You Need

1. SSH access to server: `98.71.91.84`
2. DNS management access (to create `telemetry.etrid.org`)
3. This deployment package (already created ✅)

---

## 🚀 Deployment Commands (Copy & Paste)

### Option A: Automated (If You Have SSH Access)

```bash
# From your local machine:
cd /Users/macbook/Desktop/etrid/substrate-telemetry-deployment

# Transfer tarball to server
scp -i ~/.ssh/YOUR_SSH_KEY \
    etrid-telemetry-deployment.tar.gz \
    ubuntu@98.71.91.84:~/

# SSH to server
ssh -i ~/.ssh/YOUR_SSH_KEY ubuntu@98.71.91.84

# On the server, extract and deploy:
tar -xzf etrid-telemetry-deployment.tar.gz
chmod +x deploy-telemetry.sh
./deploy-telemetry.sh

# Follow the on-screen instructions for:
# - DNS record creation
# - SSL certificate
# - Validator configuration
```

---

### Option B: Manual (Copy Commands to Server)

**Step 1:** SSH to server
```bash
ssh ubuntu@98.71.91.84
```

**Step 2:** Install prerequisites
```bash
# Update system
sudo apt update

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" \
    -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# Install Nginx and Certbot
sudo apt install -y nginx certbot python3-certbot-nginx

# Logout and login again for Docker group to take effect
exit
```

**Step 3:** Create deployment directory
```bash
# SSH back in
ssh ubuntu@98.71.91.84

# Create directory
sudo mkdir -p /opt/substrate-telemetry
sudo chown $USER:$USER /opt/substrate-telemetry
cd /opt/substrate-telemetry
```

**Step 4:** Create docker-compose.yml
```bash
cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  backend:
    image: parity/substrate-telemetry-backend:latest
    container_name: telemetry-backend
    restart: unless-stopped
    ports:
      - "127.0.0.1:8000:8000"
    environment:
      - RUST_LOG=info
    command: ["--listen", "0.0.0.0:8000"]
    networks:
      - telemetry

  frontend:
    image: parity/substrate-telemetry-frontend:latest
    container_name: telemetry-frontend
    restart: unless-stopped
    ports:
      - "127.0.0.1:3001:80"
    environment:
      - SUBSTRATE_TELEMETRY_URL=wss://telemetry.etrid.org/feed
    networks:
      - telemetry

  shard:
    image: parity/substrate-telemetry-shard:latest
    container_name: telemetry-shard
    restart: unless-stopped
    ports:
      - "127.0.0.1:8001:8001"
    environment:
      - RUST_LOG=info
    command: [
      "--listen", "0.0.0.0:8001",
      "--core", "http://backend:8000/shard_submit"
    ]
    depends_on:
      - backend
    networks:
      - telemetry

networks:
  telemetry:
    driver: bridge
EOF
```

**Step 5:** Start containers
```bash
docker-compose pull
docker-compose up -d

# Check status
docker-compose ps

# Check logs
docker-compose logs -f
# (Press Ctrl+C to exit logs)
```

**Step 6:** Configure Nginx
```bash
sudo tee /etc/nginx/sites-available/telemetry > /dev/null << 'EOF'
server {
    listen 80;
    server_name telemetry.etrid.org;
    
    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }
    
    location / {
        return 301 https://$server_name$request_uri;
    }
}

server {
    listen 443 ssl http2;
    server_name telemetry.etrid.org;

    ssl_certificate /etc/letsencrypt/live/telemetry.etrid.org/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/telemetry.etrid.org/privkey.pem;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    location /submit/ {
        proxy_pass http://127.0.0.1:8001;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }

    location /feed/ {
        proxy_pass http://127.0.0.1:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }

    location / {
        proxy_pass http://127.0.0.1:3001;
    }
}
EOF

# Enable site
sudo rm -f /etc/nginx/sites-enabled/default
sudo ln -s /etc/nginx/sites-available/telemetry /etc/nginx/sites-enabled/
sudo nginx -t
```

---

## 🌐 Step 7: Create DNS Record

**Go to your DNS provider (Hostinger/Cloudflare) and create:**

| Type | Name | Value | TTL |
|------|------|-------|-----|
| A | telemetry | 98.71.91.84 | 300 |

Or fully qualified:
```
telemetry.etrid.org → 98.71.91.84
```

**Verify DNS propagation:**
```bash
# Wait 5-10 minutes, then:
dig telemetry.etrid.org +short
# Should return: 98.71.91.84
```

---

## 🔒 Step 8: Get SSL Certificate

**After DNS resolves:**
```bash
# On the server:
sudo systemctl stop nginx
sudo certbot certonly --standalone -d telemetry.etrid.org

# Follow prompts, then:
sudo systemctl start nginx
sudo systemctl reload nginx

# Test HTTPS
curl -I https://telemetry.etrid.org
# Should return: HTTP/2 200
```

---

## ✅ Step 9: Verify Deployment

Visit: https://telemetry.etrid.org

You should see the Substrate Telemetry UI (empty for now, waiting for validators).

---

## 🔧 Step 10: Configure Validators

For EACH of your 21 validators, add telemetry reporting:

```bash
# SSH to each validator
ssh ubuntu@VALIDATOR_IP

# Stop validator
sudo systemctl stop flarechain

# Edit service file
sudo nano /etc/systemd/system/flarechain.service

# Add this flag to ExecStart line:
--telemetry-url 'wss://telemetry.etrid.org/submit/ 0'

# Example full line:
# ExecStart=/usr/local/bin/flarechain-node \
#   --validator \
#   --name "Validator-01" \
#   --telemetry-url 'wss://telemetry.etrid.org/submit/ 0'

# Save and restart
sudo systemctl daemon-reload
sudo systemctl start flarechain

# Check logs
sudo journalctl -u flarechain -f | grep telemetry
# Should see: "Connected to telemetry"
```

---

## 🌐 Step 11: Update Website

```bash
# On your local machine:
cd /Users/macbook/Desktop/etrid

# Copy new telemetry integration
cp substrate-telemetry-deployment/telemetry-feed-integration.js \
   etrid-hostinger-deployment/apps/telemetry/app.js

# Deploy via FTP
cd etrid-hostinger-deployment
python3 << 'PYEOF'
import ftplib

ftp = ftplib.FTP()
ftp.connect('157.173.214.206', 21)
ftp.login('u724092535', 'Fullashit13!')

with open('apps/telemetry/app.js', 'rb') as f:
    ftp.storbinary('STOR /domains/etrid.org/public_html/telemetry/app.js', f)

ftp.quit()
print("✅ Website updated!")
PYEOF
```

---

## 🎉 Done!

**Visit and verify:**
- https://telemetry.etrid.org → Should show all 21 validators
- https://etrid.org/telemetry/ → Should show all 21 validators

---

## 🆘 Troubleshooting

**Containers not starting:**
```bash
docker-compose logs -f
docker-compose restart
```

**Validators not appearing:**
```bash
# Check validator logs
ssh ubuntu@VALIDATOR_IP
sudo journalctl -u flarechain -f | grep telemetry
```

**DNS not resolving:**
```bash
dig telemetry.etrid.org +short
# Wait longer if it returns nothing
```

**SSL fails:**
```bash
# Make sure DNS resolves first!
# Check certbot logs:
sudo tail -f /var/log/letsencrypt/letsencrypt.log
```

---

## 📦 Files Available

All files are in: `/Users/macbook/Desktop/etrid/substrate-telemetry-deployment/`

- `etrid-telemetry-deployment.tar.gz` (15 KB) - Everything in one tarball
- `DEPLOYMENT_GUIDE.md` - Complete detailed guide
- `MANUAL_DEPLOYMENT.sh` - Interactive deployment script

---

**Estimated Time:** 45-60 minutes (including waiting for DNS)

**Need help?** See `DEPLOYMENT_GUIDE.md` for complete troubleshooting.
