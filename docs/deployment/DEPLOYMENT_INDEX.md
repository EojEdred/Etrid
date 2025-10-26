# Ëtrid Protocol - Vercel Deployment Documentation Index

**Complete guide to deploying all UI applications to Vercel**

---

## Quick Links

| Document | Purpose | Size | Path |
|----------|---------|------|------|
| **Quick Start** | Fast deployment commands | 2.0K | [DEPLOYMENT_QUICK_START.md](./DEPLOYMENT_QUICK_START.md) |
| **Deployment Report** | Current status and summary | 13K | [VERCEL_DEPLOYMENT_REPORT.md](./VERCEL_DEPLOYMENT_REPORT.md) |
| **Deployment Guide** | Full step-by-step guide | 12K | [docs/VERCEL_DEPLOYMENT_GUIDE.md](./docs/VERCEL_DEPLOYMENT_GUIDE.md) |
| **Architecture** | System architecture diagrams | 31K | [DEPLOYMENT_ARCHITECTURE.md](./DEPLOYMENT_ARCHITECTURE.md) |
| **Checklist** | Deployment checklist | 12K | [DEPLOYMENT_CHECKLIST.md](./DEPLOYMENT_CHECKLIST.md) |
| **Deployment Script** | Automated deployment | 7.8K | [scripts/deploy-ui.sh](./scripts/deploy-ui.sh) |

---

## Where to Start

### New to Vercel Deployment?
Start here: **[DEPLOYMENT_QUICK_START.md](./DEPLOYMENT_QUICK_START.md)**
- Fastest path to deployment
- Essential commands only
- 5-minute read

### Need Detailed Instructions?
Read: **[docs/VERCEL_DEPLOYMENT_GUIDE.md](./docs/VERCEL_DEPLOYMENT_GUIDE.md)**
- Complete step-by-step guide
- Troubleshooting section
- Environment variable configuration
- Custom domain setup
- 30-minute read

### Want Current Status?
Check: **[VERCEL_DEPLOYMENT_REPORT.md](./VERCEL_DEPLOYMENT_REPORT.md)**
- Build status for all apps
- Known issues and fixes
- Deployment readiness
- Next steps
- 15-minute read

### Ready to Deploy?
Use: **[DEPLOYMENT_CHECKLIST.md](./DEPLOYMENT_CHECKLIST.md)**
- Step-by-step checklist
- Pre-deployment checks
- Post-deployment verification
- Printable format
- Track your progress

### Understanding the System?
Study: **[DEPLOYMENT_ARCHITECTURE.md](./DEPLOYMENT_ARCHITECTURE.md)**
- System architecture diagrams
- Deployment flow
- Network topology
- Security configuration
- 20-minute read

### Prefer Automation?
Run: **[scripts/deploy-ui.sh](./scripts/deploy-ui.sh)**
- Automated deployment script
- Handles all three apps
- Production/staging modes
- Error handling

---

## Deployment Workflow

```
1. Read Quick Start
   └─▶ DEPLOYMENT_QUICK_START.md

2. Check Current Status
   └─▶ VERCEL_DEPLOYMENT_REPORT.md

3. Authenticate with Vercel
   └─▶ vercel login

4. Deploy Applications
   ├─▶ Option A: Use script (./scripts/deploy-ui.sh)
   └─▶ Option B: Manual (follow DEPLOYMENT_GUIDE.md)

5. Verify Deployments
   └─▶ Use DEPLOYMENT_CHECKLIST.md

6. Configure Domains (Optional)
   └─▶ Follow guide in VERCEL_DEPLOYMENT_GUIDE.md

7. Monitor & Optimize
   └─▶ Vercel Dashboard + Analytics
```

---

## Document Summaries

### 1. DEPLOYMENT_QUICK_START.md
**Quick reference for immediate deployment**

**Contents:**
- Authentication commands
- Deployment commands for each app
- Basic troubleshooting
- Links to detailed docs

**Use When:**
- You need to deploy quickly
- You're familiar with Vercel
- You just need the commands

---

### 2. VERCEL_DEPLOYMENT_REPORT.md
**Comprehensive deployment status report**

**Contents:**
- Build status for all applications
- Configuration verification
- Known issues and solutions
- Environment variables
- Deployment URLs template
- Next steps

**Use When:**
- Starting a new deployment session
- Need to know what's ready
- Checking for known issues
- Reviewing configurations

---

### 3. docs/VERCEL_DEPLOYMENT_GUIDE.md
**Complete deployment guide with all details**

**Contents:**
- Detailed prerequisites
- Full build status
- Step-by-step deployment instructions
- Environment variable configuration
- Post-deployment verification
- Troubleshooting guide
- Custom domain setup
- CI/CD integration
- Support resources

**Use When:**
- First time deploying
- Need comprehensive instructions
- Troubleshooting issues
- Setting up custom domains
- Configuring advanced features

---

### 4. DEPLOYMENT_ARCHITECTURE.md
**Visual system architecture and flow diagrams**

**Contents:**
- System architecture diagrams
- Application deployment flow
- Network architecture
- Build and deploy process
- Environment configuration
- Security headers
- API routes and rewrites
- Deployment regions
- Function configuration
- Custom domain flow
- Monitoring and analytics
- CI/CD integration plans

**Use When:**
- Understanding the system
- Planning infrastructure
- Troubleshooting network issues
- Presenting to stakeholders
- Onboarding new team members

---

### 5. DEPLOYMENT_CHECKLIST.md
**Interactive checklist for deployment process**

**Contents:**
- Pre-deployment checklist
- Step-by-step deployment tasks
- Post-deployment verification
- Custom domain setup checklist
- Analytics and monitoring setup
- CI/CD configuration
- Documentation and communication
- Troubleshooting log
- Sign-off section

**Use When:**
- Actually performing deployment
- Ensuring nothing is missed
- Tracking deployment progress
- Creating deployment records
- Training new operators

---

### 6. scripts/deploy-ui.sh
**Automated deployment script**

**Contents:**
- Pre-flight checks
- Authentication verification
- Automated deployment for all apps
- Production/preview mode support
- Error handling
- Deployment summary

**Use When:**
- Want automated deployment
- Deploying multiple apps
- Setting up CI/CD
- Need consistent process

---

## Application Status

### Wallet Web
- **Status:** ✅ READY FOR PRODUCTION
- **Build:** ✅ Passing
- **Path:** `apps/wallet-web/etrid-crypto-website/`
- **Framework:** Next.js 15.2.4
- **Deploy Command:** `vercel --prod`

### Validator Dashboard
- **Status:** ✅ READY FOR STAGING
- **Build:** ✅ Passing (with warnings)
- **Path:** `apps/validator-dashboard/`
- **Framework:** Next.js 14.0.4
- **Deploy Command:** `vercel`

### Watchtower Monitor
- **Status:** ⚠️ NEEDS FIX
- **Build:** ❌ Failing (Tailwind CSS issue)
- **Path:** `apps/watchtower-monitor/`
- **Framework:** Next.js 15.2.4
- **Deploy Command:** `vercel` (after fix)
- **Fix:** See VERCEL_DEPLOYMENT_REPORT.md

---

## Common Commands

```bash
# Authenticate
vercel login

# Deploy production
cd [app-directory]
vercel --prod

# Deploy staging/preview
cd [app-directory]
vercel

# Using script - deploy all
./scripts/deploy-ui.sh

# Using script - production
./scripts/deploy-ui.sh all --production

# Check status
vercel ls

# View logs
vercel logs [deployment-url]

# Rollback
vercel rollback [deployment-url]

# Add domain
vercel domains add [domain] --project=[project-name]
```

---

## Environment Variables

All apps use these core variables:
```env
NEXT_PUBLIC_WS_PROVIDER=wss://rpc.etrid.network
NEXT_PUBLIC_NETWORK_NAME=Ëtrid MainNet
NEXT_PUBLIC_CHAIN_ID=etrid-mainnet
NODE_ENV=production
```

Configured in each app's `vercel.json` file.

---

## Support Resources

### Documentation
- Vercel Docs: https://vercel.com/docs
- CLI Reference: https://vercel.com/docs/cli
- Next.js on Vercel: https://vercel.com/docs/frameworks/nextjs

### Internal Resources
- Architecture: [docs/architecture.md](./docs/architecture.md)
- Developer Guide: [docs/DEVELOPER_GUIDE.md](./docs/DEVELOPER_GUIDE.md)
- API Reference: [docs/API_REFERENCE.md](./docs/API_REFERENCE.md)

---

## Troubleshooting Quick Reference

| Issue | Solution | Document |
|-------|----------|----------|
| Build fails | Check build logs, fix errors locally | VERCEL_DEPLOYMENT_GUIDE.md |
| Watchtower CSS error | Downgrade to Tailwind v3 or migrate to v4 | VERCEL_DEPLOYMENT_REPORT.md |
| Auth failed | Run `vercel login` | DEPLOYMENT_QUICK_START.md |
| Env vars not working | Set in Vercel Dashboard | VERCEL_DEPLOYMENT_GUIDE.md |
| Domain not working | Check DNS, wait for propagation | VERCEL_DEPLOYMENT_GUIDE.md |
| Slow page load | Check bundle size, enable analytics | DEPLOYMENT_ARCHITECTURE.md |

---

## Next Steps After Deployment

1. **Immediate (Required)**
   - [ ] Test all deployed apps
   - [ ] Verify RPC connections
   - [ ] Check for errors

2. **Short-term (Recommended)**
   - [ ] Configure custom domains
   - [ ] Enable analytics
   - [ ] Set up error tracking

3. **Long-term (Optional)**
   - [ ] Configure CI/CD
   - [ ] Optimize performance
   - [ ] Set up monitoring

---

## Document Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-10-22 | Initial deployment documentation created |

---

## Contributing

To update this documentation:

1. Edit relevant markdown files
2. Update version numbers and dates
3. Test all commands
4. Update this index if adding new docs

---

## Feedback

If you encounter issues not covered in these docs:
1. Check troubleshooting sections
2. Review Vercel documentation
3. Check application build logs
4. Document the issue and solution

---

**Document Index Version:** 1.0
**Last Updated:** 2025-10-22
**Total Documentation Size:** ~70KB
**Prepared by:** Claude Code Deployment Agent

---

## License

This documentation is part of the Ëtrid Protocol project.

---

**Happy Deploying!**
