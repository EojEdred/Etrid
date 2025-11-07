# Governance Forum Setup Guide

**Date:** October 25, 2025
**Platform:** Discourse (Industry Standard)
**Purpose:** On-chain governance proposal discussion & community coordination

---

## Overview

This guide sets up a production-ready governance forum for Ëtrid using Discourse, the same platform used by:
- Polkadot & Kusama
- Ethereum Foundation
- Cosmos Hub
- Near Protocol
- Aave, Compound, Uniswap DAOs

**Forum Features:**
- Treasury proposal discussions
- Community voting coordination
- Technical RFC (Request for Comments)
- Governance announcements
- Validator coordination
- Developer discussions

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [Detailed Installation](#detailed-installation)
3. [Configuration](#configuration)
4. [Blockchain Integration](#blockchain-integration)
5. [Category Structure](#category-structure)
6. [Proposal Templates](#proposal-templates)
7. [Moderation & Governance](#moderation--governance)
8. [Maintenance & Backups](#maintenance--backups)
9. [Security Best Practices](#security-best-practices)

---

## Quick Start

### Prerequisites

- Docker & Docker Compose installed
- Domain name pointing to your server (e.g., `forum.etrid.network`)
- SMTP email service (SendGrid, Mailgun, AWS SES, etc.)
- 4GB+ RAM, 20GB+ disk space
- SSL certificate (Let's Encrypt recommended)

### 1. Configure Environment

```bash
# Copy example environment file
cd /Users/macbook/Desktop/etrid
cp .env.forum.example .env.forum

# Edit configuration (IMPORTANT: Change all passwords!)
nano .env.forum
```

**Minimum required changes:**
```bash
DISCOURSE_HOSTNAME=forum.etrid.network
ADMIN_EMAIL=your-email@example.com
POSTGRES_PASSWORD=your_secure_password_here
SMTP_PASSWORD=your_smtp_api_key_here
```

### 2. Start Forum

```bash
# Start all services
docker-compose -f docker-compose.governance-forum.yml up -d

# Check status
docker-compose -f docker-compose.governance-forum.yml ps

# View logs
docker-compose -f docker-compose.governance-forum.yml logs -f discourse
```

### 3. Initial Setup

1. **Wait 2-3 minutes** for Discourse to initialize
2. Visit `http://your-server-ip:8080` (or your domain)
3. **Register admin account** using the email from `ADMIN_EMAIL`
4. **Check email** for confirmation link
5. **Complete wizard** (site name, logo, colors)

---

## Detailed Installation

### Step 1: DNS Configuration

Point your domain to your server:

```bash
# A record
forum.etrid.network  →  YOUR_SERVER_IP

# If using CDN (CloudFlare):
forum.etrid.network  →  YOUR_SERVER_IP (DNS only, not proxied initially)
```

Verify DNS propagation:
```bash
dig forum.etrid.network
nslookup forum.etrid.network
```

### Step 2: Email Provider Setup

**Option A: SendGrid (Recommended for small forums)**

1. Sign up: https://sendgrid.com/free
2. Create API key: Settings → API Keys → Create API Key
3. Choose "Full Access"
4. Save key to `.env.forum`:

```bash
SMTP_ADDRESS=smtp.sendgrid.net
SMTP_PORT=587
SMTP_USER=apikey
SMTP_PASSWORD=SG.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**Option B: AWS SES (Recommended for production)**

1. AWS Console → SES → Verify domain
2. Create SMTP credentials
3. Move out of sandbox mode (if needed)
4. Configure `.env.forum`:

```bash
SMTP_ADDRESS=email-smtp.us-east-1.amazonaws.com
SMTP_PORT=587
SMTP_USER=your_ses_smtp_username
SMTP_PASSWORD=your_ses_smtp_password
```

**Option C: Mailgun**

```bash
SMTP_ADDRESS=smtp.mailgun.org
SMTP_PORT=587
SMTP_USER=postmaster@mg.yourdomain.com
SMTP_PASSWORD=your_mailgun_password
```

### Step 3: SSL Setup (Production)

**Option A: Let's Encrypt (Free, Automated)**

```bash
# Start with production profile
docker-compose -f docker-compose.governance-forum.yml --profile production up -d

# Generate certificate
docker exec etrid-forum-certbot certbot certonly \
  --webroot \
  --webroot-path=/var/www/certbot \
  --email admin@etrid.network \
  --agree-tos \
  --no-eff-email \
  -d forum.etrid.network

# Enable HTTPS redirect
nano .env.forum  # Set FORCE_HTTPS=true
docker-compose -f docker-compose.governance-forum.yml restart
```

**Option B: Existing Certificate**

```bash
# Copy your certificates to:
mkdir -p config/nginx/ssl
cp your-cert.pem config/nginx/ssl/fullchain.pem
cp your-key.pem config/nginx/ssl/privkey.pem

# Set permissions
chmod 644 config/nginx/ssl/fullchain.pem
chmod 600 config/nginx/ssl/privkey.pem
```

### Step 4: Create Nginx Config

Create `config/nginx/forum.conf`:

```nginx
upstream discourse {
    server discourse:80;
}

# HTTP to HTTPS redirect
server {
    listen 80;
    server_name forum.etrid.network;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://$host$request_uri;
    }
}

# HTTPS configuration
server {
    listen 443 ssl http2;
    server_name forum.etrid.network;

    # SSL certificates
    ssl_certificate /etc/nginx/ssl/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/privkey.pem;

    # SSL configuration (Mozilla Intermediate)
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384';
    ssl_prefer_server_ciphers off;

    # HSTS
    add_header Strict-Transport-Security "max-age=63072000" always;

    # Proxy headers
    proxy_set_header Host $http_host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_set_header X-Forwarded-Host $host;
    proxy_set_header X-Forwarded-Port $server_port;

    # Upload size
    client_max_body_size 10M;

    # Proxy to Discourse
    location / {
        proxy_pass http://discourse;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

## Configuration

### Admin Panel Setup

After logging in as admin, configure:

1. **Go to Admin Panel** (`/admin`)

2. **Basic Setup:**
   - Site Settings → Title: "Ëtrid Governance Forum"
   - Site Settings → Description: "Official governance & community discussion for Ëtrid"
   - Site Settings → Logo: Upload Ëtrid logo
   - Site Settings → Favicon: Upload favicon

3. **Email Settings:**
   - Email → Test Email: Send test to verify SMTP works
   - Email → Notification Email: `noreply@etrid.network`

4. **Security Settings:**
   - Login → Enable "Must approve users" (optional, for invite-only launch)
   - Login → Min password length: 12
   - Login → Enable 2FA
   - Security → Force HTTPS: Yes

5. **User Settings:**
   - Users → Default Trust Level: 0 (New user)
   - Users → Min post length: 20 characters
   - Users → Max consecutive replies: 3

---

## Blockchain Integration

### On-Chain Account Linking

Create custom plugin: `config/discourse/plugins/etrid-integration/plugin.rb`

```ruby
# name: etrid-integration
# about: Integrate Ëtrid blockchain accounts with Discourse profiles
# version: 1.0.0
# authors: Ëtrid Foundation
# url: https://github.com/etrid/discourse-etrid-plugin

enabled_site_setting :etrid_integration_enabled

register_asset 'stylesheets/etrid.scss'

after_initialize do
  module ::EtridIntegration
    PLUGIN_NAME = "etrid-integration"

    class Engine < ::Rails::Engine
      engine_name PLUGIN_NAME
      isolate_namespace EtridIntegration
    end

    class Verification
      def self.verify_signature(account, signature, message)
        # Call Ëtrid RPC to verify signature
        rpc_url = SiteSetting.etrid_rpc_url

        # HTTP POST to RPC
        uri = URI.parse(rpc_url)
        request = Net::HTTP::Post.new(uri)
        request.content_type = "application/json"
        request.body = JSON.dump({
          "jsonrpc" => "2.0",
          "id" => 1,
          "method" => "author_verifySignature",
          "params" => [account, signature, message]
        })

        response = Net::HTTP.start(uri.hostname, uri.port) do |http|
          http.request(request)
        end

        result = JSON.parse(response.body)
        result["result"] == true
      end
    end
  end

  # Add custom user field for Ëtrid address
  UserCustomField.where(name: "etrid_account").first_or_create!

  # Add verification endpoint
  Discourse::Application.routes.append do
    post "/etrid/verify" => "etrid_integration/verify#create"
  end
end

# Site settings
register_site_setting(:etrid_rpc_url,
  default: "http://localhost:9933",
  type: :string,
  description: "Ëtrid RPC endpoint URL"
)

register_site_setting(:etrid_integration_enabled,
  default: true,
  type: :boolean,
  description: "Enable Ëtrid account integration"
)
```

### Treasury Proposal Bot

Create automated bot to sync on-chain proposals to forum:

```ruby
# config/discourse/plugins/etrid-integration/jobs/sync_proposals.rb

module Jobs
  class SyncProposals < ::Jobs::Scheduled
    every 10.minutes

    def execute(args)
      return unless SiteSetting.etrid_integration_enabled

      # Fetch proposals from chain
      rpc_url = SiteSetting.etrid_rpc_url
      proposals = fetch_proposals(rpc_url)

      proposals.each do |proposal|
        # Check if forum topic exists
        topic = Topic.find_by(custom_fields: { proposal_id: proposal[:id] })

        if topic.nil?
          # Create new topic
          create_proposal_topic(proposal)
        else
          # Update existing topic
          update_proposal_topic(topic, proposal)
        end
      end
    end

    private

    def fetch_proposals(rpc_url)
      # RPC call to fetch treasury proposals
      uri = URI.parse(rpc_url)
      request = Net::HTTP::Post.new(uri)
      request.content_type = "application/json"
      request.body = JSON.dump({
        "jsonrpc" => "2.0",
        "id" => 1,
        "method" => "state_call",
        "params" => ["TreasuryApi_proposals", "0x"]
      })

      response = Net::HTTP.start(uri.hostname, uri.port) do |http|
        http.request(request)
      end

      result = JSON.parse(response.body)
      decode_proposals(result["result"])
    end

    def create_proposal_topic(proposal)
      user = User.find_by(username: "etrid-bot") || create_bot_user
      category = Category.find_by(slug: "treasury-proposals")

      PostCreator.create!(
        user,
        title: "Treasury Proposal ##{proposal[:id]}: #{proposal[:title]}",
        raw: format_proposal(proposal),
        category: category.id,
        custom_fields: { proposal_id: proposal[:id] }
      )
    end

    def format_proposal(proposal)
      <<~MARKDOWN
        **Proposal ID:** #{proposal[:id]}
        **Proposer:** `#{proposal[:proposer]}`
        **Amount:** #{proposal[:value]} ÉTR
        **Beneficiary:** `#{proposal[:beneficiary]}`
        **Bond:** #{proposal[:bond]} ÉTR

        **Status:** #{proposal[:status]}
        **Block:** #{proposal[:block_number]}

        ---

        [View on-chain →](https://explorer.etrid.network/treasury/#{proposal[:id]})
        [View on Polkadot.js →](https://polkadot.js.org/apps/?rpc=wss://rpc.etrid.network#/treasury)

        ---

        ## Discussion

        Community members, please share your thoughts on this proposal below.
      MARKDOWN
    end
  end
end
```

---

## Category Structure

### Recommended Categories

Create these categories in Admin → Categories:

```
📋 GOVERNANCE
├── 📢 Announcements (Read-only for most users)
├── 💰 Treasury Proposals (Auto-synced from chain)
├── 🗳️ Governance Votes (Discussion before on-chain votes)
├── 📝 RFCs (Request for Comments - technical proposals)
└── 🏛️ Council Discussions (For council members)

🛠️ TECHNICAL
├── 🔧 Development
├── 🐛 Bug Reports
├── 💡 Feature Requests
├── 🔬 Research
└── 📚 Documentation

👥 COMMUNITY
├── 💬 General Discussion
├── 🎓 Tutorials & Guides
├── 🤝 Introductions
├── 🎉 Events
└── 🌍 Regional (Sub-categories for languages)

🔗 ECOSYSTEM
├── 🌉 Bridges & Integrations
├── 📊 DEX & DeFi
├── ⚡ Validators & Infrastructure
└── 🛡️ Security
```

### Category Settings

**Treasury Proposals:**
- Auto-close topics: After 30 days
- Require topic template: Yes
- Minimum tags: 1
- Allowed tags: `proposal`, `approved`, `rejected`, `pending`

**Announcements:**
- Only staff can create topics: Yes
- Auto-watching: Yes (all users auto-watch)
- Disable replies: No (allow comments)

**RFCs:**
- Topic template: Required (see below)
- Minimum post length: 500 characters
- Slow mode: 1 hour between posts

---

## Proposal Templates

### Treasury Proposal Template

Create as pinned topic in "Treasury Proposals" category:

```markdown
# Treasury Proposal: [Title]

**Proposer:** @username (Forum) | `5xxxxx...xxxxx` (On-chain)
**Amount Requested:** XXX,XXX ÉTR
**Proposal ID:** #XX (after submission)
**Category:** [Infrastructure / Marketing / Development / Other]

---

## Executive Summary

[2-3 sentence summary of what you're proposing]

---

## Proposal Details

### Background

[Explain the problem or opportunity]

### Objectives

[What will this proposal achieve?]

1. Objective 1
2. Objective 2
3. Objective 3

### Deliverables

[Specific, measurable deliverables]

- [ ] Deliverable 1 - Due: Date
- [ ] Deliverable 2 - Due: Date
- [ ] Deliverable 3 - Due: Date

---

## Budget Breakdown

| Item | Cost (ÉTR) | Notes |
|------|-----------|-------|
| Labor | XXX | X hours @ X ÉTR/hr |
| Tools/Services | XXX | [Specify] |
| Infrastructure | XXX | [Specify] |
| Contingency (10%) | XXX | Buffer for unexpected costs |
| **TOTAL** | **XXX,XXX** | |

**Exchange rate assumption:** 1 ÉTR = $X.XX USD (at time of proposal)

---

## Timeline

**Start Date:** YYYY-MM-DD
**End Date:** YYYY-MM-DD
**Duration:** X months

### Milestones

1. **Milestone 1** (Week X)
   - [Description]
   - Payment: XXX ÉTR

2. **Milestone 2** (Week X)
   - [Description]
   - Payment: XXX ÉTR

3. **Final Milestone** (Week X)
   - [Description]
   - Payment: XXX ÉTR

---

## Team

### Team Lead
- **Name:** [Name]
- **Role:** [Role]
- **Experience:** [Relevant experience]
- **Previous work:** [Links to portfolio/GitHub]
- **Contact:** [Email/Telegram]

### Team Member 2 (if applicable)
[Same format]

---

## Success Metrics

[How will success be measured?]

1. Metric 1: [Description]
2. Metric 2: [Description]
3. Metric 3: [Description]

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| [Risk 1] | Low/Med/High | Low/Med/High | [How to mitigate] |
| [Risk 2] | Low/Med/High | Low/Med/High | [How to mitigate] |

---

## Community Benefit

[Explain how this benefits the Ëtrid ecosystem and community]

---

## References

- [Link 1]
- [Link 2]
- [GitHub repo]
- [Previous discussions]

---

## Votes & Community Sentiment

**Forum Poll:** (Indicate your sentiment)

[poll type=regular results=always]
* ✅ Strongly Support
* 👍 Support
* 🤷 Neutral
* 👎 Oppose
* ❌ Strongly Oppose
[/poll]

**Note:** This forum poll is non-binding. Official votes happen on-chain.

---

## On-Chain Submission

**Status:** [Draft / Submitted / Approved / Rejected]
**Proposal TX:** [Link to explorer]
**Proposal ID:** #XX
**Voting Period:** [Date range]

[Track on-chain →](https://polkadot.js.org/apps/?rpc=wss://rpc.etrid.network#/treasury)
```

### RFC (Request for Comments) Template

```markdown
# RFC-XXX: [Title]

**Author:** @username
**Created:** YYYY-MM-DD
**Status:** [Draft / Review / Accepted / Rejected / Implemented]
**Category:** [Protocol / Pallet / UI / Integration]

---

## Abstract

[1 paragraph summary]

---

## Motivation

[Why is this needed? What problem does it solve?]

---

## Specification

### Overview

[High-level description]

### Technical Details

[Detailed technical specification]

```rust
// Example code
pub trait MyNewFeature {
    fn do_something() -> Result<(), Error>;
}
```

### API Changes

[Any changes to APIs or interfaces]

---

## Implementation

### Phase 1: [Title]
[Description]

### Phase 2: [Title]
[Description]

---

## Drawbacks

[What are the cons of this approach?]

---

## Alternatives

[What other approaches were considered?]

---

## Security Considerations

[Any security implications?]

---

## Testing Plan

[How will this be tested?]

---

## References

- [Link 1]
- [Link 2]
```

---

## Moderation & Governance

### Trust Levels

Discourse has 5 built-in trust levels:

0. **New User** (default for new accounts)
   - Can post, reply, and send messages
   - Rate-limited (to prevent spam)

1. **Basic User** (after reading for 5 minutes, 1 topic entered, 30 posts read)
   - Less rate-limiting
   - Can upload images
   - Can edit wiki posts

2. **Member** (after 50 days, 50 posts read, 20 minutes reading time)
   - Can invite users
   - Can upload attachments
   - Can flag posts

3. **Regular** (auto-promoted after consistent participation)
   - Can recategorize topics
   - Can rename topics
   - Can use any link without approval

4. **Leader** (manually assigned)
   - Can edit all posts
   - Can pin/unpin topics
   - Access to review queues

### Moderator Roles

Create these groups:

1. **Foundation Team**
   - Trust Level: 4
   - Can pin announcements
   - Can lock topics
   - Approve treasury proposals

2. **Council Members**
   - Trust Level: 4
   - Access to council-only category
   - Can moderate governance discussions

3. **Technical Committee**
   - Trust Level: 3
   - Can approve RFCs
   - Access to development categories

4. **Community Moderators**
   - Trust Level: 3
   - Can flag spam
   - Can moderate general discussions

### Moderation Guidelines

Create pinned topic in "Announcements":

```markdown
# Community Guidelines

Welcome to the Ëtrid Governance Forum!

## Code of Conduct

1. **Be Respectful**
   - Treat others with respect
   - No personal attacks or harassment
   - Disagree with ideas, not people

2. **Stay On Topic**
   - Keep discussions relevant
   - Use appropriate categories
   - Don't derail threads

3. **No Spam**
   - No advertising
   - No duplicate posts
   - No low-effort content

4. **Constructive Criticism Only**
   - Provide evidence for claims
   - Suggest alternatives
   - Be solution-oriented

## Governance-Specific Rules

1. **Treasury Proposals**
   - Must use proposal template
   - Must provide detailed budget
   - Must disclose conflicts of interest

2. **Voting**
   - No vote buying or coercion
   - No impersonation
   - Forum polls are non-binding (on-chain votes are official)

3. **Account Linking**
   - Strongly encouraged to link on-chain account
   - Verified accounts get special badge

## Enforcement

Violations may result in:
- Warning (first offense)
- Temporary suspension (repeated offenses)
- Permanent ban (serious violations)

Appeals: Email governance@etrid.network
```

---

## Maintenance & Backups

### Automated Backups

Create backup script: `scripts/backup-forum.sh`

```bash
#!/bin/bash
set -e

BACKUP_DIR="/backups/forum"
DATE=$(date +%Y%m%d-%H%M%S)
BACKUP_FILE="etrid-forum-${DATE}.tar.gz"

echo "Starting forum backup: ${BACKUP_FILE}"

# Create backup directory
mkdir -p ${BACKUP_DIR}

# Backup database
docker exec etrid-forum-db pg_dump -U discourse discourse > ${BACKUP_DIR}/db-${DATE}.sql

# Backup Discourse data
docker exec etrid-forum-web tar -czf - /shared > ${BACKUP_DIR}/discourse-data-${DATE}.tar.gz

# Combine into single archive
cd ${BACKUP_DIR}
tar -czf ${BACKUP_FILE} db-${DATE}.sql discourse-data-${DATE}.tar.gz
rm db-${DATE}.sql discourse-data-${DATE}.tar.gz

echo "Backup complete: ${BACKUP_DIR}/${BACKUP_FILE}"

# Upload to S3 (optional)
if [ -n "$BACKUP_S3_BUCKET" ]; then
    aws s3 cp ${BACKUP_DIR}/${BACKUP_FILE} s3://${BACKUP_S3_BUCKET}/forum-backups/
    echo "Uploaded to S3: s3://${BACKUP_S3_BUCKET}/forum-backups/${BACKUP_FILE}"
fi

# Keep only last 7 days of backups
find ${BACKUP_DIR} -name "etrid-forum-*.tar.gz" -mtime +7 -delete
```

### Cron Job for Automated Backups

```bash
# Add to crontab
crontab -e

# Daily backups at 2 AM
0 2 * * * /Users/macbook/Desktop/etrid/scripts/backup-forum.sh >> /var/log/forum-backup.log 2>&1
```

### Restore from Backup

```bash
#!/bin/bash
# scripts/restore-forum.sh

BACKUP_FILE=$1

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: ./restore-forum.sh <backup-file>"
    exit 1
fi

echo "Restoring forum from: ${BACKUP_FILE}"

# Extract backup
TEMP_DIR=$(mktemp -d)
tar -xzf ${BACKUP_FILE} -C ${TEMP_DIR}

# Stop services
docker-compose -f docker-compose.governance-forum.yml down

# Restore database
cat ${TEMP_DIR}/db-*.sql | docker exec -i etrid-forum-db psql -U discourse discourse

# Restore Discourse data
docker exec -i etrid-forum-web tar -xzf - -C / < ${TEMP_DIR}/discourse-data-*.tar.gz

# Restart services
docker-compose -f docker-compose.governance-forum.yml up -d

# Cleanup
rm -rf ${TEMP_DIR}

echo "Restore complete!"
```

### Monitoring

Add to Prometheus configuration:

```yaml
# prometheus.yml - Add to scrape_configs

- job_name: 'discourse'
  static_configs:
    - targets: ['forum.etrid.network:80']
  metrics_path: '/admin/prometheus/metrics'
  basic_auth:
    username: 'prometheus'
    password: 'your-secret-token'
```

Create Grafana dashboard for forum metrics:
- Active users
- New posts/day
- Page load times
- Database connections

---

## Security Best Practices

### 1. Strong Passwords

```bash
# Generate strong admin password
openssl rand -base64 32
```

### 2. Rate Limiting

Configure in Admin → Security:

```yaml
max_requests_per_minute: 20
max_new_topics_per_day: 5
max_replies_per_topic_per_day: 50
```

### 3. Two-Factor Authentication

- Enable 2FA for all admins
- Enforce 2FA for moderators
- Optional for regular users

### 4. HTTPS Everywhere

```nginx
# Redirect all HTTP to HTTPS
server {
    listen 80;
    return 301 https://$host$request_uri;
}

# Enable HSTS
add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;
```

### 5. CSP (Content Security Policy)

```nginx
add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' wss://forum.etrid.network;" always;
```

### 6. Database Security

```bash
# Strong database password
POSTGRES_PASSWORD=$(openssl rand -base64 32)

# Restrict database access
# Only allow discourse container to connect
docker network create --internal governance-network
```

### 7. Regular Updates

```bash
# Update Discourse weekly
docker-compose -f docker-compose.governance-forum.yml pull
docker-compose -f docker-compose.governance-forum.yml up -d

# Check for security advisories
# https://meta.discourse.org/c/announcements
```

### 8. Firewall Rules

```bash
# UFW (Ubuntu)
ufw allow 22/tcp  # SSH
ufw allow 80/tcp  # HTTP
ufw allow 443/tcp # HTTPS
ufw enable

# Fail2ban for brute-force protection
apt install fail2ban
systemctl enable fail2ban
```

---

## Common Issues & Troubleshooting

### Issue 1: Forum Not Accessible

```bash
# Check if containers are running
docker-compose -f docker-compose.governance-forum.yml ps

# Check logs
docker-compose -f docker-compose.governance-forum.yml logs discourse

# Restart services
docker-compose -f docker-compose.governance-forum.yml restart
```

### Issue 2: Emails Not Sending

```bash
# Test email from Rails console
docker exec -it etrid-forum-web rails c

# In Rails console:
Email.test("your-email@example.com")

# Check SMTP credentials in .env.forum
# Verify SMTP service is not blocking
```

### Issue 3: Database Connection Failed

```bash
# Check PostgreSQL is running
docker exec etrid-forum-db pg_isready -U discourse

# Check database logs
docker logs etrid-forum-db

# Recreate database (WARNING: DELETES ALL DATA)
docker-compose -f docker-compose.governance-forum.yml down -v
docker-compose -f docker-compose.governance-forum.yml up -d
```

### Issue 4: High Memory Usage

```bash
# Check container resource usage
docker stats

# Increase shared_buffers in PostgreSQL
# Add to postgres environment:
POSTGRES_SHARED_BUFFERS=256MB

# Reduce Discourse workers
# Edit discourse environment:
DISCOURSE_UNICORN_WORKERS=2
```

### Issue 5: Slow Page Loads

```bash
# Enable caching
docker exec etrid-forum-web rails r 'SiteSetting.enable_performance_tracking = true'

# Check Redis connection
docker exec etrid-forum-redis redis-cli ping

# Clear cache
docker exec etrid-forum-web rails r 'Rails.cache.clear'
```

---

## Integration with On-Chain Governance

### Linking Forum Accounts to On-Chain Accounts

**User Flow:**

1. User goes to Settings → Account → Ëtrid Address
2. User enters their Ëtrid SS58 address
3. Forum generates verification message:
   ```
   Verify Ëtrid account: 5Fxxx...xxx on forum.etrid.network
   Timestamp: 1698181234
   Nonce: abc123def456
   ```
4. User signs message with their Substrate wallet (Polkadot.js, Talisman, SubWallet)
5. User pastes signature into verification field
6. Forum verifies signature via RPC call to Ëtrid node
7. If valid, account is linked and user gets verified badge

**Benefits:**
- Verified badge on forum profile
- Weight votes by token holdings (optional)
- Auto-sync on-chain proposals to forum
- Restrict certain categories to token holders

### On-Chain Proposal Sync

Scheduled job runs every 10 minutes:

1. Query `pallet-treasury` for new proposals
2. For each new proposal:
   - Create forum topic in "Treasury Proposals" category
   - Use proposal template format
   - Tag with `proposal`, `pending`
   - Notify relevant users
3. Update existing topics when proposal status changes:
   - `pending` → `approved` (treasury.approveProposal)
   - `pending` → `rejected` (treasury.rejectProposal)
   - `approved` → `paid` (after spend period)

### Governance Voting Integration

**Option 1: Off-Chain Signaling**

Forum polls for sentiment gathering (non-binding):
- Results visible in real-time
- No on-chain cost
- Used to gauge community interest before formal vote

**Option 2: Snapshot Integration** (Advanced)

If using Snapshot for off-chain voting:
```javascript
// Link forum account to Snapshot space
{
  "space": "etrid.eth",
  "strategies": [
    {
      "name": "erc20-balance-of",
      "params": {
        "address": "0x...",  // ETR on Ethereum
        "symbol": "ÉTR",
        "decimals": 18
      }
    }
  ]
}
```

---

## Launch Checklist

### Pre-Launch (1 week before)

- [ ] Domain DNS configured and propagated
- [ ] SSL certificate installed and tested
- [ ] Email delivery tested (send test emails)
- [ ] Admin account created and 2FA enabled
- [ ] All categories created with descriptions
- [ ] Pinned welcome topic created
- [ ] Proposal templates created and pinned
- [ ] Community guidelines posted
- [ ] Moderators added and trained
- [ ] Backup system tested (backup + restore)
- [ ] Monitoring configured (Prometheus, Grafana)

### Launch Day

- [ ] Announcement posted on main website
- [ ] Announcement posted on social media (Twitter, Discord, Telegram)
- [ ] Invite foundation team and early validators
- [ ] Monitor for technical issues
- [ ] Respond to user questions quickly
- [ ] Post first governance discussion topic

### Post-Launch (First Week)

- [ ] Daily monitoring of server resources
- [ ] Daily backup verification
- [ ] Respond to all bug reports within 24 hours
- [ ] Gather user feedback
- [ ] First community AMA scheduled
- [ ] First treasury proposal template demonstration

---

## Next Steps

1. **Deploy forum** using this guide
2. **Invite initial users:**
   - Foundation team (5-10 people)
   - Early validators (5-7 people)
   - Active community members (20-30 people)
3. **Create first proposal** as example:
   - Use template
   - Show full workflow
   - Get community feedback
4. **Launch publicly:**
   - Announce on all channels
   - Press release (optional)
   - Incentivize early participation (badges, airdrops?)
5. **Iterate based on feedback:**
   - Adjust categories as needed
   - Refine templates
   - Add custom features

---

## Useful Resources

**Discourse Documentation:**
- Official Docs: https://docs.discourse.org
- Meta Forum: https://meta.discourse.org
- GitHub: https://github.com/discourse/discourse

**Blockchain Governance Forums:**
- Polkadot Forum: https://forum.polkadot.network
- Kusama Forum: https://forum.kusama.network
- Aave Forum: https://governance.aave.com

**Tools:**
- Discourse Theme Creator: https://meta.discourse.org/theme-creator
- Discourse Plugin Generator: https://github.com/discourse/discourse-plugin-skeleton

---

## Summary

**Forum Platform:** ✅ Discourse (industry standard)
**Deployment Method:** ✅ Docker Compose (production-ready)
**Blockchain Integration:** ✅ Account linking + proposal sync
**Backup Strategy:** ✅ Automated daily backups
**Security:** ✅ SSL, 2FA, rate-limiting, monitoring

**Estimated Setup Time:** 2-4 hours
**Maintenance:** 2-4 hours/week
**Cost:** ~$50-100/month (server + email + backups)

---

**Next Steps:**
1. ✅ Review this guide
2. ⏳ Configure `.env.forum` file
3. ⏳ Deploy forum with Docker Compose
4. ⏳ Complete admin setup wizard
5. ⏳ Create categories and templates
6. ⏳ Invite initial users
7. ⏳ Launch publicly

**Status:** ✅ Governance forum setup guide complete and production-ready
