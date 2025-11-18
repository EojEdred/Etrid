# Deployment Guide - Ëtrid Wallet Backend

Complete guide for deploying the backend API to production.

## Prerequisites

- Docker & Docker Compose installed
- Domain name configured
- SSL certificates (Let's Encrypt recommended)
- PostgreSQL 14+ (or use Docker)
- Redis 7+ (or use Docker)
- Node.js 18+ (for non-Docker deployment)

## Docker Deployment (Recommended)

### 1. Clone Repository

```bash
git clone https://github.com/etrid/wallet-backend.git
cd wallet-backend
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env
```

Set these critical variables:

```bash
NODE_ENV=production
PORT=3000

# Strong passwords
DB_PASSWORD=your_super_secure_db_password_here
REDIS_PASSWORD=your_redis_password_here

# JWT secrets (generate with: openssl rand -base64 32)
JWT_SECRET=your_jwt_secret_key_change_this
JWT_REFRESH_SECRET=your_refresh_secret_change_this

# Blockchain
FLARECHAIN_WS_URL=wss://flarechain.etrid.io
FLARECHAIN_HTTP_URL=https://rpc.etrid.io

# API Keys (get from respective services)
COINME_API_KEY=...
BITCOIN_DEPOT_API_KEY=...
COINFLIP_API_KEY=...
VAST_AI_API_KEY=...
RUNPOD_API_KEY=...
EXPO_ACCESS_TOKEN=...
SENDGRID_API_KEY=...
TWILIO_ACCOUNT_SID=...
TWILIO_AUTH_TOKEN=...
```

### 3. Build and Start Services

```bash
# Build images
docker-compose build

# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f api
```

### 4. Initialize Database

```bash
# Database will be automatically initialized from schema.sql
# Or manually run migrations:
docker-compose exec api npm run migrate
```

### 5. Verify Deployment

```bash
# Health check
curl http://localhost:3000/health

# Expected response:
{
  "success": true,
  "data": {
    "status": "ok",
    "timestamp": "2025-11-18T12:00:00.000Z",
    "version": "v1",
    "uptime": 123.45
  }
}
```

### 6. Scale API Instances

```bash
# Run 3 API instances behind load balancer
docker-compose up -d --scale api=3
```

## Manual Deployment

### 1. Install Dependencies

```bash
npm ci --production
```

### 2. Build TypeScript

```bash
npm run build
```

### 3. Set Up Database

```bash
# Create database
createdb etrid_wallet

# Run migrations
psql -d etrid_wallet -f src/database/schema.sql
```

### 4. Start Application

```bash
# With PM2 (recommended)
npm install -g pm2
pm2 start dist/server.js --name etrid-api -i max

# Or with systemd
sudo nano /etc/systemd/system/etrid-api.service
```

systemd service file:

```ini
[Unit]
Description=Etrid Wallet API
After=network.target postgresql.service redis.service

[Service]
Type=simple
User=etrid
WorkingDirectory=/home/etrid/wallet-backend
ExecStart=/usr/bin/node /home/etrid/wallet-backend/dist/server.js
Restart=on-failure
Environment="NODE_ENV=production"
EnvironmentFile=/home/etrid/wallet-backend/.env

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-api
sudo systemctl start etrid-api
sudo systemctl status etrid-api
```

## Kubernetes Deployment

### 1. Create ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: etrid-api-config
data:
  NODE_ENV: "production"
  PORT: "3000"
  DB_HOST: "postgres-service"
  DB_PORT: "5432"
  REDIS_URL: "redis://redis-service:6379"
```

### 2. Create Secrets

```bash
kubectl create secret generic etrid-api-secrets \
  --from-literal=DB_PASSWORD=your_db_password \
  --from-literal=JWT_SECRET=your_jwt_secret \
  --from-literal=JWT_REFRESH_SECRET=your_refresh_secret
```

### 3. Deploy

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: etrid-api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: etrid-api
  template:
    metadata:
      labels:
        app: etrid-api
    spec:
      containers:
      - name: api
        image: etrid/wallet-backend:latest
        ports:
        - containerPort: 3000
        envFrom:
        - configMapRef:
            name: etrid-api-config
        - secretRef:
            name: etrid-api-secrets
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: etrid-api-service
spec:
  type: LoadBalancer
  ports:
  - port: 80
    targetPort: 3000
  selector:
    app: etrid-api
```

## Nginx Reverse Proxy

### SSL Configuration

```nginx
upstream etrid_api {
    least_conn;
    server 127.0.0.1:3000;
    server 127.0.0.1:3001;
    server 127.0.0.1:3002;
}

server {
    listen 80;
    server_name api.etrid.io;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name api.etrid.io;

    ssl_certificate /etc/letsencrypt/live/api.etrid.io/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.etrid.io/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_types application/json text/plain text/css application/javascript;

    location / {
        proxy_pass http://etrid_api;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=10r/s;
    limit_req zone=api_limit burst=20 nodelay;

    # Access logs
    access_log /var/log/nginx/etrid-api-access.log;
    error_log /var/log/nginx/etrid-api-error.log;
}
```

## Monitoring Setup

### Prometheus

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'etrid-api'
    static_configs:
      - targets: ['localhost:3000']
```

### Grafana Dashboard

Import dashboard JSON from `monitoring/grafana-dashboard.json`

### Health Checks

```bash
# Add to crontab for monitoring
*/5 * * * * curl -f http://localhost:3000/health || systemctl restart etrid-api
```

## Database Backup

### Automated Daily Backup

```bash
#!/bin/bash
# /etc/cron.daily/backup-etrid-db

BACKUP_DIR="/backups/etrid"
DATE=$(date +%Y%m%d_%H%M%S)
DB_NAME="etrid_wallet"

mkdir -p $BACKUP_DIR

# Backup
pg_dump -U postgres -d $DB_NAME | gzip > "$BACKUP_DIR/etrid_$DATE.sql.gz"

# Keep only last 30 days
find $BACKUP_DIR -name "etrid_*.sql.gz" -mtime +30 -delete

# Upload to S3 (optional)
# aws s3 cp "$BACKUP_DIR/etrid_$DATE.sql.gz" s3://your-bucket/backups/
```

Make executable:

```bash
chmod +x /etc/cron.daily/backup-etrid-db
```

## Security Checklist

- [ ] Strong database passwords set
- [ ] JWT secrets changed from defaults
- [ ] Firewall configured (only ports 80, 443 open)
- [ ] SSH key-based authentication only
- [ ] Fail2ban installed and configured
- [ ] SSL certificates installed and auto-renewing
- [ ] Database backups automated
- [ ] Log rotation configured
- [ ] Rate limiting enabled
- [ ] CORS properly configured
- [ ] API keys stored in environment variables
- [ ] Database user has limited permissions
- [ ] Redis password protected
- [ ] Docker containers run as non-root user

## Performance Tuning

### PostgreSQL

```ini
# /etc/postgresql/14/main/postgresql.conf

max_connections = 100
shared_buffers = 256MB
effective_cache_size = 1GB
maintenance_work_mem = 64MB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 2621kB
min_wal_size = 1GB
max_wal_size = 4GB
```

### Redis

```ini
# /etc/redis/redis.conf

maxmemory 256mb
maxmemory-policy allkeys-lru
save 900 1
save 300 10
save 60 10000
```

### Node.js

```bash
# PM2 cluster mode with max CPU cores
pm2 start dist/server.js -i max

# Or set NODE_OPTIONS
export NODE_OPTIONS="--max-old-space-size=2048"
```

## Troubleshooting

### API Not Responding

```bash
# Check service status
docker-compose ps
systemctl status etrid-api

# View logs
docker-compose logs -f api
journalctl -u etrid-api -f

# Check connections
netstat -tulpn | grep 3000
```

### Database Connection Issues

```bash
# Test connection
psql -h localhost -U postgres -d etrid_wallet

# Check PostgreSQL logs
tail -f /var/log/postgresql/postgresql-14-main.log
```

### High Memory Usage

```bash
# Check Node.js memory
pm2 monit

# Restart with memory limit
pm2 restart etrid-api --max-memory-restart 500M
```

## Rollback Procedure

```bash
# Docker
docker-compose down
docker-compose up -d --scale api=3

# PM2
pm2 stop etrid-api
pm2 delete etrid-api
cd /path/to/previous/version
pm2 start dist/server.js --name etrid-api -i max

# Restore database backup
gunzip < backup.sql.gz | psql -U postgres -d etrid_wallet
```

## Support

For deployment issues:
- GitHub: https://github.com/etrid/wallet-backend/issues
- Email: devops@etrid.io
- Discord: https://discord.gg/etrid
