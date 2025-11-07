# Server Deployment Guide

Complete guide to deploying Etrid Operations Center on a dedicated server for 24/7 monitoring.

## Why Deploy to a Server?

### Local Setup vs Server: Comparison

| Feature | Local (Laptop/Desktop) | Dedicated Server |
|---------|----------------------|------------------|
| **Uptime** | Only when machine is on | 24/7 continuous |
| **Reliability** | Home internet, power issues | Datacenter-grade SLA |
| **Access** | Limited to local network | Accessible anywhere |
| **Monitoring** | Manual, when you're available | Automated, always active |
| **Alerts** | Miss alerts when offline | Never miss critical alerts |
| **Performance** | Shared resources | Dedicated resources |
| **Team Access** | Difficult | Easy for multiple users |
| **Scalability** | Limited | Easily upgradeable |
| **Cost** | $0 (existing hardware) | $5-50/month |

### Server Benefits in Detail

**1. Always-On Monitoring**
- Continuous health checks every 5 minutes
- Immediate alerts for any issues
- No gaps in monitoring when you sleep/travel
- Historical data collection never stops

**2. Better Uptime**
- Datacenter reliability (99.9%+ uptime)
- Redundant power and internet
- Professional infrastructure
- Automatic failover

**3. Professional Operations**
- Predictable, reliable environment
- Team can access simultaneously
- Proper backups and disaster recovery
- Audit trails and compliance

**4. Lower Latency**
- Deploy in same region as your nodes
- Faster SSH connections
- Quicker health checks
- Reduced API response times

**5. Scalability**
- Start small, upgrade as needed
- Handle more nodes easily
- Add team members without issues
- Integrate with other tools

---

## Server Requirements

### Minimum Specs

For monitoring up to 20 nodes:
- **CPU**: 1 vCPU
- **RAM**: 1GB
- **Storage**: 20GB SSD
- **Network**: 1TB bandwidth/month
- **Cost**: ~$5-10/month

### Recommended Specs

For production with 50+ nodes:
- **CPU**: 2 vCPUs
- **RAM**: 2-4GB
- **Storage**: 40GB SSD
- **Network**: 2TB bandwidth/month
- **Cost**: ~$10-20/month

### Providers We Recommend

1. **DigitalOcean** - Simple, reliable, good docs
   - Droplets from $6/month
   - Excellent for beginners
   - 1-click marketplace apps

2. **Hetzner** - Best price/performance
   - VPS from €4/month
   - European datacenters
   - Great value

3. **Linode** - Reliable, good support
   - Linodes from $5/month
   - Multiple regions
   - 24/7 support

4. **AWS Lightsail** - If already on AWS
   - Instances from $5/month
   - Integrates with AWS services
   - Predictable pricing

5. **Vultr** - Good global coverage
   - Instances from $5/month
   - Many locations
   - Hourly billing

---

## Deployment Methods

We provide 3 deployment methods. Choose based on your preference:

### Method 1: Docker (Recommended)

**Best for**: Most users, easiest setup

**Pros**:
- ✅ Isolated environment
- ✅ Easy updates
- ✅ Portable across servers
- ✅ Consistent behavior

**Cons**:
- Slightly more resource usage
- Requires Docker knowledge

### Method 2: Docker Compose (Production)

**Best for**: Production deployments with SSL

**Pros**:
- ✅ Includes Nginx reverse proxy
- ✅ Automatic SSL with Let's Encrypt
- ✅ Professional setup
- ✅ Easy scaling

**Cons**:
- More complex configuration
- Requires domain name

### Method 3: Systemd Service (Native)

**Best for**: Maximum performance, minimal overhead

**Pros**:
- ✅ Native Linux performance
- ✅ Lower resource usage
- ✅ Systemd integration
- ✅ Fine-grained control

**Cons**:
- More manual setup
- OS-specific

---

## Method 1: Docker Deployment

### Prerequisites

```bash
# Install Docker
curl -fsSL https://get.docker.com | sh

# Install Docker Compose
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

### Setup

```bash
# 1. Clone repository
git clone https://github.com/EojEdred/Etrid.git
cd Etrid/pinokio-etrid-ops

# 2. Copy and configure
cp api/etrid/config.json config.json
nano config.json  # Add your nodes

# 3. Set environment variables (optional)
cp .env.example .env
nano .env  # Add alert credentials

# 4. Build and run
docker-compose up -d

# 5. Check logs
docker-compose logs -f etrid-ops
```

### Access Dashboard

```bash
# Get server IP
curl ifconfig.me

# Access at: http://YOUR_SERVER_IP:8080
```

### Updates

```bash
# Pull latest changes
git pull

# Rebuild and restart
docker-compose down
docker-compose up -d --build
```

---

## Method 2: Production Docker Compose (with SSL)

### Prerequisites

- Domain name pointing to your server
- Ports 80 and 443 open on firewall

### Setup

```bash
# 1. Clone and configure
git clone https://github.com/EojEdred/Etrid.git
cd Etrid/pinokio-etrid-ops

# 2. Configure nginx
nano nginx.conf
# Change "ops.etrid.io" to your domain

# 3. Configure docker-compose
nano docker-compose.prod.yml
# Update domain references

# 4. Set environment variables
nano .env
# Add all credentials

# 5. Get SSL certificate
docker-compose -f docker-compose.prod.yml run --rm certbot certonly \
  --webroot --webroot-path /var/www/certbot \
  -d ops.yourdomain.com \
  --email your@email.com \
  --agree-tos --no-eff-email

# 6. Start services
docker-compose -f docker-compose.prod.yml up -d

# 7. Verify
curl https://ops.yourdomain.com/health
```

### Access Dashboard

```
https://ops.yourdomain.com
```

### Automatic SSL Renewal

Certbot container automatically renews certificates every 12 hours.

---

## Method 3: Systemd Service

### Prerequisites

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y nodejs npm git sqlite3

# CentOS/RHEL
sudo yum install -y nodejs npm git sqlite
```

### Setup

```bash
# 1. Create user
sudo useradd -r -s /bin/false etrid

# 2. Clone repository
sudo mkdir -p /opt/etrid-ops
sudo git clone https://github.com/EojEdred/Etrid.git /tmp/etrid
sudo cp -r /tmp/etrid/pinokio-etrid-ops/* /opt/etrid-ops/

# 3. Install dependencies
cd /opt/etrid-ops/dashboard
sudo npm install --production
cd /opt/etrid-ops/api/etrid
sudo npm install --production

# 4. Configure
sudo nano /opt/etrid-ops/api/etrid/config.json

# 5. Create data directory
sudo mkdir -p /var/lib/etrid-ops /var/log/etrid-ops
sudo chown -R etrid:etrid /var/lib/etrid-ops /var/log/etrid-ops
sudo chown -R etrid:etrid /opt/etrid-ops

# 6. Install service
sudo cp /opt/etrid-ops/etrid-ops.service /etc/systemd/system/
sudo systemctl daemon-reload

# 7. Start service
sudo systemctl enable etrid-ops
sudo systemctl start etrid-ops

# 8. Check status
sudo systemctl status etrid-ops
sudo journalctl -u etrid-ops -f
```

### Nginx Reverse Proxy (Optional)

```bash
# Install Nginx
sudo apt install -y nginx certbot python3-certbot-nginx

# Configure
sudo nano /etc/nginx/sites-available/etrid-ops
# Paste nginx.conf content, update domain

# Enable
sudo ln -s /etc/nginx/sites-available/etrid-ops /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx

# Get SSL certificate
sudo certbot --nginx -d ops.yourdomain.com
```

---

## Configuration

### SSH Keys

Your server needs SSH keys to connect to nodes:

```bash
# Copy SSH keys to server
scp -r ~/.ssh/aws-etrid.pem user@server:/root/.ssh/
scp -r ~/.ssh/gcp-etrid user@server:/root/.ssh/

# Docker: Mount in docker-compose.yml
volumes:
  - ~/.ssh:/root/.ssh:ro

# Systemd: Copy to /home/etrid/.ssh/
```

### Alert Configuration

Edit `config.json` or use environment variables:

```json
{
  "alerts": {
    "telegram": {
      "enabled": true,
      "botToken": "YOUR_BOT_TOKEN",
      "chatId": "YOUR_CHAT_ID"
    },
    "discord": {
      "enabled": true,
      "webhookUrl": "YOUR_WEBHOOK_URL"
    },
    "email": {
      "enabled": true,
      "smtp": {
        "host": "smtp.gmail.com",
        "port": 587,
        "user": "your@email.com",
        "pass": "your_app_password"
      },
      "to": ["alerts@yourcompany.com"]
    }
  }
}
```

### Scheduler Configuration

Enable automated health checks:

```json
{
  "scheduler": {
    "enabled": true,
    "healthCheckInterval": "*/5 * * * *",
    "metricsInterval": "*/10 * * * *"
  }
}
```

---

## Security Best Practices

### Firewall

```bash
# Ubuntu/Debian (UFW)
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable

# CentOS/RHEL (firewalld)
sudo firewall-cmd --permanent --add-service=ssh
sudo firewall-cmd --permanent --add-service=http
sudo firewall-cmd --permanent --add-service=https
sudo firewall-cmd --reload
```

### SSH Hardening

```bash
# Disable password authentication
sudo nano /etc/ssh/sshd_config
# Set: PasswordAuthentication no
# Set: PubkeyAuthentication yes

sudo systemctl restart sshd
```

### Fail2Ban

```bash
# Install
sudo apt install -y fail2ban

# Configure
sudo cp /etc/fail2ban/jail.conf /etc/fail2ban/jail.local
sudo nano /etc/fail2ban/jail.local

# Enable
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### Auto-Updates (Ubuntu)

```bash
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```

---

## Monitoring the Monitor

Even the monitoring system needs monitoring!

### Health Checks

Use UptimeRobot or similar to monitor your dashboard:

```
https://uptimerobot.com

Add monitor:
- Type: HTTP(s)
- URL: https://ops.yourdomain.com/health
- Interval: 5 minutes
- Alert: Email/SMS when down
```

### Backup Strategy

```bash
# Automated backups (add to crontab)
0 2 * * * docker exec etrid-ops node -e "require('./api/etrid/database').backup('/backups/$(date +\%Y\%m\%d).db')"

# Or for systemd
0 2 * * * sqlite3 /var/lib/etrid-ops/etrid-data.db ".backup '/backups/$(date +\%Y\%m\%d).db'"
```

### Log Rotation

Docker handles this automatically. For systemd:

```bash
sudo nano /etc/logrotate.d/etrid-ops

/var/log/etrid-ops/*.log {
    daily
    rotate 7
    compress
    delaycompress
    notifempty
    missingok
}
```

---

## Cost Optimization

### Small Setup (1-10 nodes)

**Provider**: Hetzner CX11
- **Cost**: €4.15/month (~$4.50)
- **Specs**: 1 vCPU, 2GB RAM, 20GB SSD
- **Perfect for**: Personal validator

### Medium Setup (10-50 nodes)

**Provider**: DigitalOcean Basic Droplet
- **Cost**: $12/month
- **Specs**: 2 vCPUs, 2GB RAM, 50GB SSD
- **Perfect for**: Professional validator

### Large Setup (50+ nodes)

**Provider**: Hetzner CPX21
- **Cost**: €9.50/month (~$10)
- **Specs**: 3 vCPUs, 4GB RAM, 80GB SSD
- **Perfect for**: Validator service provider

---

## Troubleshooting

### Dashboard won't start

```bash
# Check logs
docker-compose logs etrid-ops
# or
sudo journalctl -u etrid-ops -n 100

# Common issues:
# 1. Port already in use
sudo lsof -i :8080

# 2. Missing dependencies
cd dashboard && npm install

# 3. Permission issues
sudo chown -R etrid:etrid /opt/etrid-ops
```

### Can't access from browser

```bash
# Check if running
curl http://localhost:8080/health

# Check firewall
sudo ufw status
sudo iptables -L

# Check Nginx
sudo nginx -t
sudo systemctl status nginx
```

### Alerts not sending

```bash
# Test alerts
curl -X POST http://localhost:8080/api/test-alert

# Check config
cat api/etrid/config.json | grep -A 10 "alerts"

# Check environment variables
docker-compose config
```

---

## Performance Tuning

### Database Optimization

```bash
# Schedule periodic cleanup
0 3 * * * sqlite3 /data/etrid/etrid-data.db "VACUUM"
```

### Nginx Caching

Add to nginx.conf:

```nginx
proxy_cache_path /var/cache/nginx levels=1:2 keys_zone=etrid_cache:10m max_size=100m inactive=60m;
proxy_cache etrid_cache;
```

### Resource Limits

In docker-compose.yml:

```yaml
services:
  etrid-ops:
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 1G
        reservations:
          cpus: '0.5'
          memory: 512M
```

---

## Maintenance

### Regular Tasks

**Weekly**:
- Check logs for errors
- Review alert history
- Verify backups

**Monthly**:
- Update server packages
- Review disk space
- Check for application updates
- Review security advisories

**Quarterly**:
- Audit access logs
- Review and update documentation
- Test disaster recovery plan

### Update Procedure

```bash
# 1. Backup first
docker exec etrid-ops node -e "require('./api/etrid/database').backup('/backups/pre-update.db')"

# 2. Pull updates
git pull

# 3. Rebuild and restart
docker-compose down
docker-compose up -d --build

# 4. Verify
curl http://localhost:8080/health

# 5. Check logs
docker-compose logs -f etrid-ops
```

---

## Migration from Local to Server

### Step-by-Step Migration

```bash
# 1. Export config from local
cd ~/pinokio/api/etrid
cp config.json /tmp/etrid-config.json

# 2. Export database (if any)
cp ~/pinokio/etrid-data.db /tmp/etrid-data.db

# 3. Copy to server
scp /tmp/etrid-config.json user@server:/tmp/
scp /tmp/etrid-data.db user@server:/tmp/

# 4. On server, import
docker cp /tmp/etrid-config.json etrid-ops:/app/api/etrid/config.json
docker cp /tmp/etrid-data.db etrid-ops:/data/etrid/etrid-data.db

# 5. Restart
docker-compose restart etrid-ops

# 6. Verify all nodes appear
curl http://localhost:8080/api/nodes
```

---

## Conclusion

**Server deployment provides**:
- 24/7 reliable monitoring
- Professional infrastructure
- Team collaboration
- Peace of mind

**Recommended approach**:
1. Start with Docker on DigitalOcean ($6/month)
2. Add custom domain + SSL when ready
3. Scale up as you add more nodes

**Total cost for small validator**:
- Server: $6/month (DigitalOcean)
- Domain: $12/year (Namecheap)
- **Total: ~$7/month**

**ROI**: One prevented outage pays for months of hosting!

Need help? Join our Discord or open a GitHub issue.

**Deploy your operations center today and sleep better tonight! 🚀**
