# Etrid Operations Center - Public Deployment Guide

## Overview
This guide explains how to deploy the Etrid Operations Center as a public-facing service on **etrid.org**.

## Architecture

```
┌──────────────────────────────────────────────────────┐
│              etrid.org (Public Website)              │
├──────────────────────────────────────────────────────┤
│                                                      │
│  Landing Page (index.html)                          │
│  ↓                                                   │
│  Register with ?autostart=true                       │
│  ↓                                                   │
│  Auto-provision & Configure Validator                │
│  ↓                                                   │
│  Personalized Dashboard                              │
│                                                      │
└──────────────────────────────────────────────────────┘
```

## Deployment Steps

### 1. Server Requirements

**Minimum Specifications:**
- **CPU:** 4 cores
- **RAM:** 8GB
- **Storage:** 100GB SSD
- **OS:** Ubuntu 22.04 LTS or later
- **Node.js:** v18.x or later
- **Network:** Public IP with ports 80/443 open

### 2. Domain Configuration

1. Point **etrid.org** to your server IP:
   ```bash
   A Record: etrid.org → YOUR_SERVER_IP
   A Record: www.etrid.org → YOUR_SERVER_IP
   ```

2. Set up SSL/TLS with Let's Encrypt:
   ```bash
   sudo apt install certbot python3-certbot-nginx
   sudo certbot --nginx -d etrid.org -d www.etrid.org
   ```

### 3. Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Node.js 18
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Install PM2 (process manager)
sudo npm install -g pm2

# Install nginx (reverse proxy)
sudo apt install -y nginx
```

### 4. Clone and Setup

```bash
# Clone repository
cd /opt
sudo git clone https://github.com/EojEdred/Etrid.git
cd Etrid/pinokio-etrid-ops

# Install dependencies
npm install

# Create production environment file
sudo nano .env
```

**Environment Variables (.env):**
```bash
NODE_ENV=production
PORT=8080
JWT_SECRET=<generate-random-secure-key>
SESSION_SECRET=<generate-random-secure-key>

# Database (use PostgreSQL for production)
DATABASE_URL=postgresql://user:password@localhost/etrid_ops

# Optional: External services
TELEGRAM_BOT_TOKEN=
DISCORD_WEBHOOK_URL=
```

### 5. Configure Nginx Reverse Proxy

Create nginx configuration:
```bash
sudo nano /etc/nginx/sites-available/etrid.org
```

```nginx
server {
    listen 80;
    listen [::]:80;
    server_name etrid.org www.etrid.org;

    # Redirect HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name etrid.org www.etrid.org;

    # SSL certificates (managed by certbot)
    ssl_certificate /etc/letsencrypt/live/etrid.org/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/etrid.org/privkey.pem;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Proxy to Node.js app
    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    # WebSocket support for real-time updates
    location /socket.io/ {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }

    # Static file caching
    location ~* \.(css|js|jpg|jpeg|png|gif|ico|svg|woff|woff2|ttf|eot)$ {
        proxy_pass http://localhost:8080;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

Enable the site:
```bash
sudo ln -s /etc/nginx/sites-available/etrid.org /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

### 6. Start the Application

```bash
cd /opt/Etrid/pinokio-etrid-ops/dashboard

# Start with PM2
pm2 start server.js --name etrid-ops

# Save PM2 configuration
pm2 save

# Setup PM2 to start on boot
pm2 startup
# (follow the instructions provided)
```

### 7. Firewall Configuration

```bash
# Allow HTTP, HTTPS, and SSH
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw allow 22/tcp

# Enable firewall
sudo ufw enable
```

## User Flow

### For New Users (from etrid.org)

1. **Landing Page Visit:**
   - User visits `https://etrid.org`
   - Sees professional landing page with features, pricing
   - Clicks "🚀 Launch Your Node Now"

2. **Registration:**
   - Redirected to `/register.html?autostart=true`
   - Fills out registration form
   - Button shows "🚀 Launch My Node"

3. **Auto-Provisioning:**
   - System runs auto-discovery on registration
   - If validator found locally:
     - ✅ Automatically configured
     - User sees success message with node details
     - Redirected to `/validator.html?welcome=true`
   - If no validator found:
     - User directed to manual setup instructions
     - Or provisioned on cloud infrastructure (future)

4. **Dashboard Access:**
   - User lands on personalized dashboard
   - Can see their validator metrics in real-time
   - Access to network overview, finality metrics, alerts

### Auto-Discovery Process

The system automatically:
1. Scans for locally running Etrid validators
2. Detects RPC endpoints and SSH configuration
3. Configures database entries
4. Starts monitoring and health checks
5. Enables all dashboard features

## Monitoring & Maintenance

### View Logs
```bash
# Application logs
pm2 logs etrid-ops

# Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log
```

### Restart Application
```bash
pm2 restart etrid-ops
```

### Update Application
```bash
cd /opt/Etrid
sudo git pull origin main
cd pinokio-etrid-ops/dashboard
npm install
pm2 restart etrid-ops
```

### Database Backups
```bash
# Backup SQLite database
cp /opt/Etrid/pinokio-etrid-ops/api/etrid/etrid.db /backups/etrid-$(date +%Y%m%d).db

# For PostgreSQL (production)
pg_dump etrid_ops > /backups/etrid-$(date +%Y%m%d).sql
```

## Security Best Practices

1. **Keep system updated:**
   ```bash
   sudo apt update && sudo apt upgrade -y
   ```

2. **Use strong JWT secrets:**
   - Generate with: `openssl rand -hex 64`

3. **Rate limiting:**
   - Consider adding rate limiting middleware
   - Use fail2ban for brute force protection

4. **Regular backups:**
   - Schedule daily database backups
   - Store backups off-site

5. **Monitor logs:**
   - Set up log monitoring (e.g., Grafana, ELK stack)
   - Alert on suspicious activity

## Scaling Considerations

### Load Balancing (Multiple Servers)
- Use nginx or HAProxy for load balancing
- Share session storage (Redis)
- Use PostgreSQL for database (not SQLite)

### Database Migration (SQLite → PostgreSQL)
```bash
# Install PostgreSQL
sudo apt install postgresql

# Create database
sudo -u postgres createdb etrid_ops

# Update connection string in .env
DATABASE_URL=postgresql://user:password@localhost/etrid_ops

# Run migrations (create migration scripts)
```

## Troubleshooting

### Application won't start
```bash
# Check logs
pm2 logs etrid-ops

# Check port availability
sudo lsof -i :8080

# Restart
pm2 restart etrid-ops
```

### WebSocket connection fails
- Ensure nginx is configured for WebSocket upgrade
- Check firewall rules
- Verify SSL certificate is valid

### Auto-discovery not working
- Check that validators are running locally
- Verify RPC endpoints are accessible
- Review auto-discovery logs in PM2

## Cost Estimate (Production)

**Monthly Costs:**
- VPS (4 cores, 8GB RAM): ~$40-60/month
- Domain: ~$12/year
- SSL: Free (Let's Encrypt)
- Backups/Storage: ~$10/month

**Total: ~$50-70/month**

## Support

- **Documentation:** https://docs.etrid.org
- **Issues:** https://github.com/EojEdred/Etrid/issues
- **Email:** support@etrid.org

---

## Quick Start (Development)

For local development testing:

```bash
cd pinokio-etrid-ops/dashboard
npm install
npm start
```

Visit `http://localhost:8080`

---

**Last Updated:** 2025-11-19
**Version:** 1.0.0
