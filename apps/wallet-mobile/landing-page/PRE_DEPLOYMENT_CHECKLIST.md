# Pre-Deployment Checklist

Complete this checklist before deploying to production.

## Assets Preparation

### Images & Icons
- [ ] Add app screenshots to `/public/screenshots/`
  - screenshot1.png (540x720)
  - screenshot2.png (540x720)
  - screenshot3.png (540x720)

- [ ] Add app icons to `/public/icons/`
  - icon-72x72.png
  - icon-96x96.png
  - icon-128x128.png
  - icon-144x144.png
  - icon-152x152.png
  - icon-192x192.png
  - icon-384x384.png
  - icon-512x512.png

- [ ] Add OG image for social sharing
  - /public/og-image.jpg (1200x630)

- [ ] Add favicon files
  - /public/favicon.ico
  - /public/apple-touch-icon.png

- [ ] Add logo images
  - /public/logo.png
  - /public/logo-white.png

### Content Review
- [ ] Review all text for typos
- [ ] Verify feature descriptions are accurate
- [ ] Check pricing is up to date
- [ ] Verify stats and metrics
- [ ] Update testimonials (if using real ones)
- [ ] Review legal links (Privacy, Terms, Cookies)

## Configuration

### Environment Variables
- [ ] Create `.env.local` from `.env.example`
- [ ] Set `NEXT_PUBLIC_SITE_URL=https://wallet.etrid.com`
- [ ] Add analytics keys (if using)
- [ ] Add API keys (if needed)

### SEO
- [ ] Verify metadata in `app/layout.tsx`
- [ ] Update OG image URLs
- [ ] Check sitemap.ts paths
- [ ] Review robots.txt rules
- [ ] Add structured data (if needed)

### Analytics (Optional)
- [ ] Set up Google Analytics
- [ ] Add Mixpanel tracking
- [ ] Configure Facebook Pixel
- [ ] Set up conversion tracking

## Testing

### Functionality
- [ ] Test all navigation links
- [ ] Test all CTAs
- [ ] Test download buttons
- [ ] Test mobile menu
- [ ] Test form submissions (if any)
- [ ] Test external links

### Responsive Design
- [ ] Test on iPhone (Safari)
- [ ] Test on Android (Chrome)
- [ ] Test on iPad
- [ ] Test on desktop (1920x1080)
- [ ] Test on laptop (1440x900)
- [ ] Test on small mobile (375x667)

### Browser Testing
- [ ] Chrome (latest)
- [ ] Safari (latest)
- [ ] Firefox (latest)
- [ ] Edge (latest)
- [ ] Mobile Safari
- [ ] Chrome Mobile

### Performance
- [ ] Run Lighthouse audit (aim for 90+)
- [ ] Check page load speed
- [ ] Test with slow 3G
- [ ] Verify image optimization
- [ ] Check bundle size

### Accessibility
- [ ] Test keyboard navigation
- [ ] Check color contrast
- [ ] Verify alt text on images
- [ ] Test screen reader compatibility
- [ ] Check focus indicators

## Build & Deploy

### Pre-Build
- [ ] Run `npm run type-check` (no errors)
- [ ] Run `npm run lint` (no errors)
- [ ] Run `npm run build` (successful)
- [ ] Test production build locally (`npm start`)

### Domain & DNS
- [ ] Domain purchased (wallet.etrid.com)
- [ ] DNS configured
- [ ] SSL certificate ready
- [ ] WWW redirect configured
- [ ] Verify DNS propagation

### Deployment Platform
- [ ] Choose platform (Vercel/Netlify/Other)
- [ ] Account created
- [ ] Project connected to Git
- [ ] Environment variables set
- [ ] Build settings configured
- [ ] Deploy preview tested

## Post-Deployment

### Verification
- [ ] Site loads correctly
- [ ] All pages accessible
- [ ] SSL working (https://)
- [ ] Sitemap accessible (/sitemap.xml)
- [ ] Robots.txt accessible (/robots.txt)
- [ ] PWA manifest loads (/manifest.json)

### SEO Setup
- [ ] Submit sitemap to Google Search Console
- [ ] Verify site ownership
- [ ] Check Google indexing
- [ ] Submit to Bing Webmaster Tools
- [ ] Update social media links

### Monitoring
- [ ] Set up uptime monitoring
- [ ] Configure error tracking (Sentry)
- [ ] Set up performance monitoring
- [ ] Configure analytics
- [ ] Set up alerts

### Marketing
- [ ] Share on social media
- [ ] Update main site links
- [ ] Send to marketing team
- [ ] Prepare launch announcement
- [ ] Update app store listings (when ready)

## Security

- [ ] Security headers configured
- [ ] No sensitive data exposed
- [ ] HTTPS enforced
- [ ] CORS configured correctly
- [ ] Rate limiting (if needed)
- [ ] CSP headers (if needed)

## Documentation

- [ ] README.md updated
- [ ] API documentation (if applicable)
- [ ] Internal wiki updated
- [ ] Team notified
- [ ] Training materials prepared

## Backup & Rollback Plan

- [ ] Code committed to Git
- [ ] Backup of current live site
- [ ] Rollback procedure documented
- [ ] Emergency contacts listed
- [ ] On-call schedule set

## Legal & Compliance

- [ ] Privacy policy link works
- [ ] Terms of service link works
- [ ] Cookie policy (if needed)
- [ ] GDPR compliance (if EU users)
- [ ] Legal team review completed

## Final Checks

- [ ] All checklist items completed
- [ ] Stakeholder approval received
- [ ] Launch date confirmed
- [ ] Team notified of launch
- [ ] Support team ready

## Launch Day

- [ ] Deploy to production
- [ ] Verify live site
- [ ] Monitor error logs
- [ ] Watch analytics
- [ ] Respond to issues quickly
- [ ] Celebrate! 🎉

---

## Quick Commands

```bash
# Pre-deployment checks
npm run type-check
npm run lint
npm run build

# Test production build
npm start

# Deploy to Vercel
vercel deploy --prod

# Check site is live
curl -I https://wallet.etrid.com
```

## Emergency Contacts

- **Technical Lead**: [Name/Email]
- **DevOps**: [Name/Email]
- **Marketing**: [Name/Email]
- **Support**: hello@etrid.com

## Notes

- Keep this checklist updated
- Mark items as completed during deployment
- Document any issues encountered
- Share learnings with team

**Last Updated**: 2025-11-19
