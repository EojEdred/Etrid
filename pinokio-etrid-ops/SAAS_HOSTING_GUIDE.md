# Hosting Etrid Operations Center on etrid.org

Complete guide to deploying the Operations Center as a hosted SaaS platform on etrid.org with user authentication and multi-tenant isolation.

## Vision: ops.etrid.org

Transform the Operations Center into a **managed service** where validators can:
1. **Sign up** with email/password
2. **Add their nodes** securely
3. **Monitor 24/7** without self-hosting
4. **Never expose** sensitive node information publicly
5. **Access from anywhere** via web browser

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│  users.etrid.org (Website)                              │
│  ├─ About Etrid                                         │
│  ├─ Documentation                                       │
│  └─ "Monitor Your Nodes" → ops.etrid.org               │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│  ops.etrid.org (Operations Center - SaaS)               │
│                                                          │
│  Public Routes:                                         │
│  ├─ /                  → Landing page                   │
│  ├─ /login             → Login form                     │
│  ├─ /register          → Sign up form                   │
│  ├─ /reset-password    → Password reset                 │
│  └─ /pricing           → Subscription tiers             │
│                                                          │
│  Protected Routes (Require Login):                      │
│  ├─ /dashboard         → User's node dashboard          │
│  ├─ /nodes             → Manage nodes                   │
│  ├─ /nodes/add         → Add new node                   │
│  ├─ /alerts            → Alert configuration            │
│  ├─ /history           → Historical data                │
│  ├─ /settings          → User settings                  │
│  └─ /api/*             → API endpoints                  │
│                                                          │
│  Backend:                                               │
│  ├─ JWT Authentication                                  │
│  ├─ Multi-tenant Database (SQLite/PostgreSQL)          │
│  ├─ Per-user Node Isolation                            │
│  └─ Subscription Management                             │
└─────────────────────────────────────────────────────────┘
```

---

## Why Hosted SaaS vs Self-Hosted?

### For Validators:

| Self-Hosted | Hosted on ops.etrid.org |
|-------------|------------------------|
| Setup complexity (Docker, server, etc.) | **Sign up and go (2 minutes)** |
| Pay $5-10/month for server | **Free tier available** |
| Maintain server yourself | **We maintain everything** |
| Update software manually | **Auto-updates** |
| Configure backups | **Automatic backups** |
| Secure server yourself | **Enterprise security** |
| Team access is complicated | **Easy multi-user accounts** |

### For Etrid Project:

✅ **Onboard validators faster** - No technical setup required
✅ **Support revenue** - Freemium model (Free → Pro → Enterprise)
✅ **Better UX** - Professional, polished interface
✅ **Network insights** - Aggregate (anonymous) health stats
✅ **Community building** - Validators interact on platform
✅ **Reduce support burden** - Centralized, tested environment

---

## Features & Subscription Tiers

### Free Tier

**Perfect for**: Individual validators with 1-5 nodes

- Monitor up to 5 nodes
- Health checks every 5 minutes
- Email alerts only
- 7 days historical data
- Community support

**Price**: $0/month

###  Pro Tier

**Perfect for**: Professional validators with 5-20 nodes

- Monitor up to 20 nodes
- Health checks every minute
- Email + Telegram + Discord alerts
- 90 days historical data
- API access
- Priority support

**Price**: $19/month

### Enterprise Tier

**Perfect for**: Validator service providers, institutions

- Unlimited nodes
- Custom health check intervals
- All alert channels + webhooks
- Unlimited historical data
- Dedicated support
- SLA guarantees
- White-label option
- Team accounts (multiple users)

**Price**: Custom (starting at $99/month)

---

## Security & Privacy

### Data Isolation

**Each user's data is completely isolated**:

```sql
-- User A can ONLY see their nodes
SELECT * FROM user_nodes WHERE user_id = 'user_a'

-- User B can ONLY see their nodes
SELECT * FROM user_nodes WHERE user_id = 'user_b'

-- No way to access another user's data
```

### What's Stored

**Stored securely**:
- User account info (encrypted passwords)
- Node configurations (IP, SSH keys encrypted)
- Health check results
- Performance metrics
- Alert history

**NOT stored**:
- Private keys (never!)
- Mnemonics (never!)
- Wallet passwords (never!)

### SSH Key Security

When users add nodes, they provide SSH access. We secure this:

1. **Encrypted at rest** - AES-256 encryption in database
2. **Separate key storage** - SSH keys in encrypted vault (HashiCorp Vault or AWS Secrets Manager)
3. **Limited access** - Keys only accessible by monitoring system, not by staff
4. **Automatic rotation** - Optional: keys can be rotated automatically
5. **Revocable** - User can remove access instantly

### Compliance

- **GDPR compliant** - Right to delete, data export
- **SOC 2 ready** - Security auditing
- **ISO 27001 practices** - Information security management
- **Regular security audits** - Penetration testing

---

## Technical Implementation

### Phase 1: Basic Authentication (Current)

**What's built**:
✅ JWT-based authentication system
✅ User registration/login
✅ Password reset flow
✅ API key generation
✅ Multi-tenant database schema
✅ Role-based access control (user, admin)
✅ Tier-based feature gating (free, pro, enterprise)

**What's needed**:
- [ ] Login/registration UI (React/Vue)
- [ ] Dashboard UI with user's nodes only
- [ ] Node management UI (add/edit/delete)
- [ ] Email verification system
- [ ] Session management with refresh tokens

### Phase 2: Node Management

- [ ] Secure SSH key encryption/storage
- [ ] Node configuration wizard
- [ ] Test connection before saving
- [ ] Import from existing config files
- [ ] Bulk node import

### Phase 3: Subscription Management

- [ ] Stripe integration
- [ ] Subscription plans
- [ ] Usage limits enforcement
- [ ] Upgrade/downgrade flow
- [ ] Billing portal

### Phase 4: Advanced Features

- [ ] Team accounts (multiple users per organization)
- [ ] SSO (Single Sign-On) for enterprises
- [ ] API rate limiting
- [ ] Custom alert webhooks
- [ ] Grafana/Prometheus export
- [ ] Mobile app (iOS/Android)

---

## Implementation Steps

### Step 1: Set Up Domain

```bash
# DNS Configuration
ops.etrid.org    A      YOUR_SERVER_IP
ops.etrid.org    AAAA   YOUR_SERVER_IPv6
```

### Step 2: Deploy with Docker Compose

```yaml
# docker-compose.saas.yml
version: '3.8'

services:
  etrid-ops-saas:
    build: .
    image: etrid/operations-center:saas
    environment:
      - NODE_ENV=production
      - DATABASE_URL=postgresql://user:pass@db:5432/etrid_ops
      - JWT_SECRET=${JWT_SECRET}
      - REDIS_URL=redis://redis:6379
      - SMTP_HOST=${SMTP_HOST}
      - STRIPE_SECRET_KEY=${STRIPE_SECRET_KEY}
    depends_on:
      - db
      - redis

  db:
    image: postgres:15
    volumes:
      - postgres-data:/var/lib/postgresql/data
    environment:
      - POSTGRES_DB=etrid_ops
      - POSTGRES_USER=etrid
      - POSTGRES_PASSWORD=${DB_PASSWORD}

  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx-saas.conf:/etc/nginx/nginx.conf
      - certbot-conf:/etc/letsencrypt
    depends_on:
      - etrid-ops-saas

volumes:
  postgres-data:
  redis-data:
  certbot-conf:
```

### Step 3: Configure Authentication

```javascript
// config.json
{
  "auth": {
    "jwtSecret": "your-super-secret-jwt-key-min-64-chars",
    "jwtExpiry": "7d",
    "refreshTokenExpiry": "30d",
    "requireEmailVerification": true,
    "allowedDomains": ["etrid.org"],
    "rateLimiting": {
      "login": {
        "max": 5,
        "window": "15m"
      },
      "register": {
        "max": 3,
        "window": "1h"
      }
    }
  },
  "tiers": {
    "free": {
      "maxNodes": 5,
      "healthCheckInterval": 300,
      "dataRetention": 7,
      "alerts": ["email"]
    },
    "pro": {
      "maxNodes": 20,
      "healthCheckInterval": 60,
      "dataRetention": 90,
      "alerts": ["email", "telegram", "discord"]
    },
    "enterprise": {
      "maxNodes": -1,
      "healthCheckInterval": 30,
      "dataRetention": -1,
      "alerts": ["email", "telegram", "discord", "webhook"]
    }
  }
}
```

### Step 4: Frontend Integration

**Add to existing etrid.org website**:

```html
<!-- In etrid.org header/navigation -->
<nav>
  <a href="/">Home</a>
  <a href="/docs">Documentation</a>
  <a href="https://ops.etrid.org">Operations Center</a> <!-- NEW -->
</nav>

<!-- Call-to-action on homepage -->
<section class="cta">
  <h2>Monitor Your Validator Nodes</h2>
  <p>Professional blockchain operations dashboard</p>
  <a href="https://ops.etrid.org/register" class="btn">Start Free Trial</a>
</section>
```

### Step 5: Email Templates

```html
<!-- Welcome Email -->
Subject: Welcome to Etrid Operations Center

Hi {{name}},

Welcome to Etrid Operations Center! Your account is ready.

Quick Start:
1. Verify your email (click link below)
2. Add your first node
3. Set up alerts

Verify Email: https://ops.etrid.org/verify?token={{token}}

Need help? Reply to this email or visit our docs.

Best,
The Etrid Team
```

---

## API Endpoints

### Public Endpoints

```
POST   /api/auth/register        - Create account
POST   /api/auth/login           - Login
POST   /api/auth/refresh         - Refresh access token
POST   /api/auth/forgot-password - Request password reset
POST   /api/auth/reset-password  - Reset password
GET    /api/auth/verify-email    - Verify email address
```

### Protected Endpoints (Require JWT)

```
GET    /api/user/profile         - Get user profile
PUT    /api/user/profile         - Update profile
POST   /api/user/change-password - Change password
GET    /api/user/api-key         - Get API key
POST   /api/user/regenerate-key  - Regenerate API key

GET    /api/nodes                - List user's nodes
POST   /api/nodes                - Add new node
GET    /api/nodes/:id            - Get node details
PUT    /api/nodes/:id            - Update node
DELETE /api/nodes/:id            - Delete node
POST   /api/nodes/:id/test       - Test node connection

GET    /api/status               - Get status of user's nodes
GET    /api/health               - Run health check
GET    /api/history/:type        - Get historical data
GET    /api/alerts               - Get alert history

POST   /api/subscription/upgrade   - Upgrade tier
POST   /api/subscription/cancel    - Cancel subscription
GET    /api/subscription/invoice   - Get invoices
```

---

## Database Migration

**From single-user to multi-tenant**:

```sql
-- Add user_id to existing tables
ALTER TABLE node_status ADD COLUMN user_id TEXT;
ALTER TABLE metrics ADD COLUMN user_id TEXT;
ALTER TABLE alerts ADD COLUMN user_id TEXT;

-- Create user tables
CREATE TABLE users (...);
CREATE TABLE user_nodes (...);
CREATE TABLE subscriptions (...);

-- Migrate existing data to default user (if migrating from self-hosted)
UPDATE node_status SET user_id = 'default_user';
UPDATE metrics SET user_id = 'default_user';
```

---

## Monitoring & Operations

### Health Checks

```bash
# Application health
curl https://ops.etrid.org/health

# Database health
curl https://ops.etrid.org/health/db

# Auth system health
curl https://ops.etrid.org/health/auth
```

### Metrics to Track

- **User Metrics**:
  - New signups per day
  - Active users
  - Churn rate
  - Subscription conversions

- **System Metrics**:
  - API response times
  - Database query performance
  - Health check success rate
  - Alert delivery rate

- **Business Metrics**:
  - MRR (Monthly Recurring Revenue)
  - Total nodes monitored
  - Average nodes per user
  - Support ticket volume

---

## Monetization Strategy

### Freemium Model

**Free Tier** (80% of users):
- Attracts validators
- Low cost to serve (5 nodes × basic checks)
- Many convert to Pro after growing

**Pro Tier** (15% of users):
- Target: $19/month × 100 users = $1,900/month
- Primary revenue source
- Higher margin (automated service)

**Enterprise Tier** (5% of users):
- Target: $99/month × 10 organizations = $990/month
- Highest margin
- Justifies dedicated support

**Total MRR Target**: $3,000-$5,000/month within 6 months

### Alternative Models

**Pay-per-node**:
- $2/node/month
- More flexible
- Good for varying needs

**Annual discount**:
- 2 months free on annual plans
- Improves cash flow
- Reduces churn

---

## Marketing & Growth

### Launch Strategy

1. **Soft launch** - Invite beta testers from existing validators
2. **Blog post** - "Introducing ops.etrid.org"
3. **Social media** - Twitter, Discord announcements
4. **Partnerships** - Integrate with validator dashboards
5. **Content marketing** - Guides, tutorials, case studies

### Growth Channels

- **Organic**: SEO, documentation, validator forums
- **Partnership**: Listed on validator tooling directories
- **Community**: Active on Discord, Telegram, Reddit
- **Content**: YouTube tutorials, blog posts
- **Referral**: Invite friends, get month free

---

## Security Checklist

Before going live:

- [ ] All passwords hashed with bcrypt (salt rounds ≥ 10)
- [ ] JWT secrets are cryptographically random (64+ chars)
- [ ] SSH keys encrypted at rest (AES-256)
- [ ] HTTPS only (HTTP redirects to HTTPS)
- [ ] Security headers configured (CSP, HSTS, etc.)
- [ ] Rate limiting on all endpoints
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS prevention (input sanitization)
- [ ] CSRF protection (tokens)
- [ ] Regular automated backups
- [ ] Secrets in environment variables (not code)
- [ ] Dependency vulnerability scanning
- [ ] Penetration testing completed
- [ ] Privacy policy and ToS published
- [ ] GDPR compliance (data export, deletion)
- [ ] Audit logging for sensitive operations

---

## Support & Documentation

### Required Documentation

1. **User Guide**
   - How to sign up
   - Adding your first node
   - Setting up alerts
   - Reading the dashboard

2. **API Documentation**
   - Authentication
   - Endpoints
   - Rate limits
   - Examples

3. **FAQ**
   - Pricing questions
   - Security concerns
   - Troubleshooting

4. **Legal**
   - Terms of Service
   - Privacy Policy
   - Acceptable Use Policy

### Support Channels

- **Free tier**: Email support (48-hour response)
- **Pro tier**: Email support (24-hour response)
- **Enterprise**: Dedicated Slack channel, phone support

---

## Launch Checklist

### Pre-Launch

- [ ] Complete user authentication system
- [ ] Build login/registration UI
- [ ] Implement node management UI
- [ ] Set up payment processing (Stripe)
- [ ] Write all documentation
- [ ] Create legal pages (ToS, Privacy)
- [ ] Set up analytics (Google Analytics, Mixpanel)
- [ ] Configure monitoring (UptimeRobot, Datadog)
- [ ] Load testing (can handle 100+ concurrent users)
- [ ] Security audit
- [ ] Beta testing (10+ users)

### Launch Day

- [ ] Deploy to production
- [ ] Configure DNS
- [ ] SSL certificates active
- [ ] Backup systems running
- [ ] Monitoring dashboards set up
- [ ] Blog post published
- [ ] Social media announcements
- [ ] Email existing validators
- [ ] Update etrid.org with links

### Post-Launch (Week 1)

- [ ] Monitor error rates
- [ ] Respond to all support tickets < 24h
- [ ] Track signup funnel
- [ ] Gather user feedback
- [ ] Fix critical bugs
- [ ] Optimize performance bottlenecks

---

## Future Enhancements

**Year 1**:
- Mobile app (iOS/Android)
- Advanced analytics dashboard
- Custom alert rules builder
- Integration marketplace (PagerDuty, Opsgenie, etc.)
- Multi-signature wallet integration
- Governance proposal tracking

**Year 2**:
- White-label offering for other chains
- AI-powered anomaly detection
- Automated remediation (smart fixes)
- Marketplace for validator services
- Community forums
- Training/certification program

---

## Conclusion

Hosting the Operations Center on ops.etrid.org as a SaaS platform provides:

**For Validators**:
- Zero setup time
- Professional monitoring
- Better reliability than self-hosting
- Affordable pricing

**For Etrid Project**:
- Revenue stream
- Better validator experience
- Network health visibility
- Community growth

**Next Steps**:
1. Review this guide with team
2. Prioritize features (MVP vs nice-to-have)
3. Set timeline (3-6 months to launch)
4. Allocate resources (development, design, operations)
5. Begin Phase 1 implementation

**Ready to build the future of validator operations? Let's do this! 🚀**
