# 🌐 DNS and SSL Setup for ËTRID Telemetry

## ✅ Status: Telemetry Server Running

**Server IP:** 98.71.91.84  
**Server Running:** ✅ Node.js telemetry server active  
**Nginx Configured:** ✅ Reverse proxy ready  

---

## 📋 Step 1: Create DNS Record

**Go to your DNS provider** (Hostinger, Cloudflare, or wherever etrid.org is managed)

### Create A Record:

| Type | Name/Host | Value/Points to | TTL |
|------|-----------|-----------------|-----|
| A | telemetry | 98.71.91.84 | 300 |

Or use the fully qualified name:

| Type | Name/Host | Value/Points to | TTL |
|------|-----------|-----------------|-----|
| A | telemetry.etrid.org | 98.71.91.84 | 300 |

---

## ⏱️ Step 2: Wait for DNS Propagation

**Wait 5-10 minutes**, then verify:

```bash
# Test DNS resolution
dig telemetry.etrid.org +short

# Should return: 98.71.91.84
```

Or use online tools:
- https://dnschecker.org/#A/telemetry.etrid.org
- https://www.whatsmydns.net/#A/telemetry.etrid.org

**Do NOT proceed until DNS resolves correctly!**

---

## 🔒 Step 3: Get SSL Certificate

Once DNS resolves, run this command:

```bash
# SSH to server
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

# Stop Nginx temporarily
sudo systemctl stop nginx

# Get SSL certificate
sudo certbot certonly --standalone -d telemetry.etrid.org

# Follow prompts:
# - Enter email: your@email.com
# - Agree to terms: Y
# - Share email: N (optional)

# Start Nginx
sudo systemctl start nginx
```

---

## 🔧 Step 4: Update Nginx for HTTPS

After getting SSL, run:

```bash
# SSH to server (if not already connected)
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84

# Update Nginx config
sudo tee /etc/nginx/sites-available/telemetry > /dev/null << 'EOF'
# Redirect HTTP to HTTPS
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

# HTTPS server
server {
    listen 443 ssl http2;
    server_name telemetry.etrid.org;

    ssl_certificate /etc/letsencrypt/live/telemetry.etrid.org/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/telemetry.etrid.org/privkey.pem;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Validators submit data
    location /submit {
        proxy_pass http://127.0.0.1:8000/submit;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }

    # Website reads feed
    location /feed {
        proxy_pass http://127.0.0.1:8000/feed;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_read_timeout 86400s;
        proxy_send_timeout 86400s;
    }

    # Web UI
    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
    }
}
EOF

# Test and reload
sudo nginx -t
sudo systemctl reload nginx
```

---

## ✅ Step 5: Verify

Test HTTPS:

```bash
curl -I https://telemetry.etrid.org
# Should return: HTTP/2 200

# Visit in browser:
# https://telemetry.etrid.org
```

---

## 📝 Quick Command Summary

```bash
# 1. Create DNS record (do this in your DNS provider web interface)
#    telemetry.etrid.org → 98.71.91.84

# 2. Wait and verify DNS
dig telemetry.etrid.org +short

# 3. Get SSL
ssh -i ~/.ssh/gizzi-validator compiler-dev01@98.71.91.84
sudo systemctl stop nginx
sudo certbot certonly --standalone -d telemetry.etrid.org
sudo systemctl start nginx

# 4. Update Nginx (copy the config above)

# 5. Test
curl -I https://telemetry.etrid.org
```

---

## 🆘 Troubleshooting

**DNS not resolving:**
- Wait longer (up to 1 hour for full propagation)
- Check DNS provider for typos
- Use TTL of 300 for faster updates

**SSL fails:**
- Make sure DNS resolves first!
- Ensure port 80 is open: `sudo ufw allow 80`
- Check certbot logs: `sudo tail -f /var/log/letsencrypt/letsencrypt.log`

**Nginx won't start:**
- Check config: `sudo nginx -t`
- Check logs: `sudo tail -f /var/log/nginx/error.log`
- Verify telemetry service running: `sudo systemctl status etrid-telemetry`

---

**After SSL is working, proceed to configure validators!**
