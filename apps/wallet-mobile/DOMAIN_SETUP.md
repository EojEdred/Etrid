# Domain Configuration Guide

## Purchase Domains

1. Buy domains:
   - wallet.etrid.com (for PWA)
   - www.wallet.etrid.com (for landing page)

## Vercel Domain Setup

### PWA (wallet.etrid.com)

1. Go to Vercel Dashboard → etrid-wallet → Settings → Domains
2. Add domain: `wallet.etrid.com`
3. Follow DNS instructions:

#### Cloudflare DNS
```
Type: CNAME
Name: wallet
Value: cname.vercel-dns.com
Proxy: DNS only (click cloud icon to disable)
```

#### Other DNS Providers
```
Type: A
Name: wallet
Value: 76.76.21.21

Type: AAAA
Name: wallet
Value: 2606:4700:3000::6815:1515
```

4. Wait for DNS propagation (up to 48 hours)
5. Vercel will auto-provision SSL certificate

### Landing Page (www.wallet.etrid.com)

1. Go to Vercel Dashboard → etrid-wallet-landing → Settings → Domains
2. Add domain: `www.wallet.etrid.com`
3. Add domain: `wallet.etrid.com` (will redirect to www)
4. Follow same DNS instructions as above

## SSL Certificate

- Vercel automatically provisions Let's Encrypt SSL
- No action needed after DNS is configured
- Certificate auto-renews

## Verify Setup

```bash
# Check DNS propagation
dig wallet.etrid.com
dig www.wallet.etrid.com

# Check SSL
curl -I https://wallet.etrid.com
curl -I https://www.wallet.etrid.com
```

## Custom Domain Setup in Vercel

### Step-by-Step Process

1. **Login to Vercel Dashboard**
   - Visit https://vercel.com/dashboard
   - Select your project (etrid-wallet or etrid-wallet-landing)

2. **Add Custom Domain**
   - Navigate to Settings → Domains
   - Click "Add" button
   - Enter your domain name
   - Click "Add"

3. **Configure DNS**
   - Vercel will show you DNS records to add
   - Go to your DNS provider (e.g., Cloudflare, Namecheap, GoDaddy)
   - Add the records shown by Vercel

4. **Wait for Verification**
   - DNS propagation can take 5 minutes to 48 hours
   - Vercel will automatically verify and provision SSL
   - You'll receive an email when it's ready

### DNS Configuration Examples

#### Cloudflare

```
# For wallet.etrid.com
Type: CNAME
Name: wallet
Target: cname.vercel-dns.com
Proxy status: DNS only (gray cloud)
TTL: Auto

# For www.wallet.etrid.com
Type: CNAME
Name: www.wallet
Target: cname.vercel-dns.com
Proxy status: DNS only (gray cloud)
TTL: Auto
```

#### AWS Route 53

```
# For wallet.etrid.com
Record name: wallet
Record type: A
Value: 76.76.21.21
TTL: 300

# AAAA record for IPv6
Record name: wallet
Record type: AAAA
Value: 2606:4700:3000::6815:1515
TTL: 300
```

#### Google Domains

```
# For wallet.etrid.com
Name: wallet
Type: CNAME
TTL: 3600
Data: cname.vercel-dns.com
```

## Domain Redirect Configuration

If you want `wallet.etrid.com` to redirect to `www.wallet.etrid.com`:

1. Add both domains in Vercel
2. Set `www.wallet.etrid.com` as primary
3. Vercel will automatically redirect `wallet.etrid.com` → `www.wallet.etrid.com`

## Troubleshooting

### Domain Not Connecting

1. **Check DNS Records**
   ```bash
   dig wallet.etrid.com
   nslookup wallet.etrid.com
   ```

2. **Check DNS Propagation**
   - Use https://dnschecker.org
   - Enter your domain
   - Verify records are propagating globally

3. **Common Issues**
   - Cloudflare proxy enabled (must be DNS only)
   - Wrong DNS records
   - DNS cache (wait or flush DNS)
   - CAA records blocking SSL

### SSL Certificate Issues

1. **Check Certificate**
   ```bash
   openssl s_client -connect wallet.etrid.com:443 -servername wallet.etrid.com
   ```

2. **Force SSL Renewal**
   - Remove domain from Vercel
   - Wait 5 minutes
   - Re-add domain

### DNS Propagation Taking Too Long

1. **Flush Local DNS Cache**
   ```bash
   # Linux
   sudo systemd-resolve --flush-caches

   # macOS
   sudo dscacheutil -flushcache

   # Windows
   ipconfig /flushdns
   ```

2. **Use Google DNS for Testing**
   - Change your DNS to 8.8.8.8 and 8.8.4.4
   - Test domain resolution

## Domain Email Setup

If you want to use email with your domain:

1. **Add MX Records** (don't interfere with Vercel)
   ```
   Type: MX
   Name: @
   Priority: 10
   Value: mail.youremailprovider.com
   ```

2. **Add SPF Record**
   ```
   Type: TXT
   Name: @
   Value: v=spf1 include:_spf.youremailprovider.com ~all
   ```

3. **Add DKIM Record**
   - Get from your email provider
   - Usually a TXT record

## Best Practices

1. **Use HTTPS Only**
   - Enable "Force HTTPS" in Vercel settings
   - Redirects HTTP → HTTPS automatically

2. **Enable HSTS**
   - Already configured in vercel.json
   - Tells browsers to always use HTTPS

3. **Use WWW or Apex**
   - Choose one as primary
   - Configure redirect for the other

4. **Monitor DNS**
   - Set up monitoring for DNS changes
   - Use tools like UptimeRobot or Pingdom

5. **Backup DNS Configuration**
   - Document all DNS records
   - Keep in version control or password manager

## Production Checklist

- [ ] Domain purchased and verified
- [ ] DNS records added correctly
- [ ] SSL certificate active (green padlock)
- [ ] Domain accessible via HTTPS
- [ ] Redirect working (if configured)
- [ ] Email DNS records added (if using email)
- [ ] DNSSEC enabled (if supported)
- [ ] Monitoring set up
- [ ] DNS backup documented
