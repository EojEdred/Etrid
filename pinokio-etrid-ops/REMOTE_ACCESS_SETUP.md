# Etrid Operations Center - Remote Access Setup

Complete guide to accessing your Etrid Operations Center from anywhere.

## Overview

Pinokio provides two methods for remote access:
1. **WiFi Sharing** - Access from devices on your local network
2. **Internet Sharing** - Access from anywhere via Cloudflare tunnel

## Method 1: WiFi Sharing (Local Network)

Perfect for accessing from devices on the same network (home, office).

### Setup Steps:

1. **Launch Pinokio** and start the Etrid dashboard
   ```bash
   # Dashboard should be running on http://localhost:8080
   ```

2. **Enable WiFi Sharing**:
   - Click the "Share" icon in Pinokio's top-right corner
   - Select "WiFi"
   - A QR code will appear

3. **Access from Other Devices**:
   - **Mobile/Tablet**: Scan the QR code
   - **Computer**: Note the local IP URL (e.g., `http://192.168.1.100:8080`)
   - Open in any browser

### Benefits:
- ✅ Fast - no internet latency
- ✅ Secure - stays on local network
- ✅ No external dependencies
- ✅ Free - no bandwidth costs

### Use Cases:
- Access from laptop while server runs on desktop
- Monitor from tablet/phone while at home/office
- Team members on same network

---

## Method 2: Internet Sharing (Cloudflare Tunnel)

Access your dashboard from anywhere in the world.

### Setup Steps:

1. **Launch Pinokio** and start the Etrid dashboard

2. **Enable Internet Sharing**:
   - Click "Share" icon in Pinokio's top-right corner
   - Select "Internet"
   - Pinokio creates a Cloudflare tunnel

3. **Set Passcode** (Highly Recommended):
   - When prompted, set a strong passcode
   - This protects your dashboard from unauthorized access
   - Example: `EtRiD-SecUr3-P@ssw0rd!2024`

4. **Save the URL**:
   - Pinokio generates a public URL like:
     `https://random-name-12345.trycloudflare.com`
   - **Save this URL** - you'll use it to access your dashboard
   - The URL remains active as long as Pinokio is running

5. **Access from Anywhere**:
   - Open the URL in any browser
   - Enter your passcode
   - Manage your nodes from anywhere!

### Benefits:
- ✅ Access from anywhere (coffee shop, travel, etc.)
- ✅ Works across any network/firewall
- ✅ HTTPS encrypted connection
- ✅ No port forwarding or router config needed
- ✅ No static IP required

### Security Features:
- 🔐 Passcode protection
- 🔐 HTTPS encryption
- 🔐 Cloudflare DDoS protection
- 🔐 No ports exposed on your network

### Use Cases:
- Check node status while traveling
- Emergency maintenance from anywhere
- Share access with remote team members
- Monitor mainnet 24/7 from any device

---

## Best Practices

### Security Recommendations:

1. **Always Set a Strong Passcode**:
   - Minimum 16 characters
   - Mix uppercase, lowercase, numbers, symbols
   - Use a password manager
   - Don't share passcode insecurely

2. **Rotate Passcodes Regularly**:
   - Change every 30-90 days
   - Change immediately if compromised
   - Use different passcodes for different environments

3. **Monitor Access Logs**:
   - Check Pinokio logs regularly
   - Look for unusual connection attempts
   - Set up alerts for failed login attempts

4. **Use VPN When Possible**:
   - For extra security layer
   - Especially on public WiFi
   - WireGuard or OpenVPN recommended

5. **Restrict Access by IP** (Advanced):
   - Configure firewall rules
   - Whitelist known IPs
   - Use fail2ban for intrusion prevention

### Performance Tips:

1. **Choose Closest Region**:
   - Cloudflare automatically routes to nearest edge
   - Lower latency = better experience

2. **Optimize Dashboard Settings**:
   - Adjust refresh intervals
   - Limit log history shown
   - Use filtering for large node counts

3. **Stable Internet Connection**:
   - Use wired connection when possible
   - Ensure sufficient bandwidth
   - Consider redundant internet (4G/5G backup)

---

## Advanced Configurations

### Custom Domain (Optional)

Want a permanent URL like `ops.etrid.io`?

1. **Set up Cloudflare Tunnel** (cloudflared):
   ```bash
   # Install cloudflared
   brew install cloudflare/cloudflare/cloudflared  # Mac
   # Or download from: https://github.com/cloudflare/cloudflared

   # Login
   cloudflared tunnel login

   # Create tunnel
   cloudflared tunnel create etrid-ops

   # Configure
   cloudflared tunnel route dns etrid-ops ops.etrid.io

   # Run tunnel
   cloudflared tunnel run etrid-ops
   ```

2. **Update Pinokio Config** to use custom tunnel

3. **Access at**: `https://ops.etrid.io`

### Reverse Proxy (Nginx)

For more control, use Nginx as reverse proxy:

```nginx
server {
    listen 443 ssl http2;
    server_name ops.etrid.io;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;

        # WebSocket support
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Rate limiting
    limit_req_zone $binary_remote_addr zone=ops:10m rate=10r/s;
    limit_req zone=ops burst=20;
}
```

### Authentication Layer (Optional)

Add OAuth2 or basic auth:

```bash
# Using OAuth2 Proxy
docker run -d \
  -p 4180:4180 \
  quay.io/oauth2-proxy/oauth2-proxy:latest \
  --upstream=http://localhost:8080 \
  --provider=github \
  --client-id=YOUR_GITHUB_CLIENT_ID \
  --client-secret=YOUR_GITHUB_CLIENT_SECRET
```

---

## Troubleshooting

### Can't Access Dashboard

**Issue**: Dashboard not loading

**Solutions**:
1. Check Pinokio is running
2. Verify dashboard started (green icon)
3. Check port 8080 is not in use:
   ```bash
   lsof -i :8080
   ```
4. Check firewall settings
5. Try restarting Pinokio

### Cloudflare Tunnel Disconnects

**Issue**: Remote URL stops working

**Solutions**:
1. Check internet connection
2. Restart Pinokio
3. Re-enable Internet Sharing
4. Check Cloudflare status: https://www.cloudflarestatus.com/

### Slow Performance

**Issue**: Dashboard is laggy remotely

**Solutions**:
1. Check internet speed (both ends)
2. Reduce dashboard refresh rate
3. Filter nodes shown
4. Use WiFi sharing when on local network
5. Consider dedicated server for dashboard

### Passcode Not Working

**Issue**: Can't access with passcode

**Solutions**:
1. Verify passcode is correct (case-sensitive)
2. Disable and re-enable sharing
3. Set new passcode
4. Check browser isn't caching old auth
5. Try incognito/private browsing

---

## Mobile App Integration

### Progressive Web App (PWA)

The dashboard works as a PWA:

1. **iOS (Safari)**:
   - Open dashboard URL
   - Tap Share button
   - Select "Add to Home Screen"
   - Icon appears like native app

2. **Android (Chrome)**:
   - Open dashboard URL
   - Tap menu (three dots)
   - Select "Add to Home screen"
   - Icon appears in app drawer

### Benefits:
- Launches like native app
- Offline caching
- Push notifications (coming soon)
- Full-screen experience

---

## Backup Access Methods

### Always Have Alternatives

1. **SSH Direct**:
   ```bash
   ssh user@your-server-ip
   cd ~/pinokio
   # Manual commands
   ```

2. **VPN Access**:
   - Set up WireGuard/OpenVPN
   - Access as if on local network

3. **Backup Server**:
   - Run dashboard on multiple machines
   - Failover if primary goes down

4. **Mobile Hotspot**:
   - Use phone as backup internet
   - Access via Cloudflare tunnel

---

## Security Checklist

Before going to production:

- [ ] Strong passcode set (16+ chars)
- [ ] HTTPS only (Cloudflare provides this)
- [ ] Firewall configured
- [ ] SSH keys (not passwords) for nodes
- [ ] 2FA on cloud provider accounts
- [ ] Regular security updates
- [ ] Access logs monitored
- [ ] Backup authentication method
- [ ] Team access documented
- [ ] Emergency contact list ready

---

## Support

Having issues? Check:

1. **Pinokio Docs**: https://docs.pinokio.computer/
2. **Etrid GitHub**: https://github.com/EojEdred/Etrid
3. **Cloudflare Docs**: https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/

## Next Steps

Once remote access is working:

1. ✅ Configure all your nodes in `config.json`
2. ✅ Set up alerts (Telegram, Discord, email)
3. ✅ Create backup and monitoring schedules
4. ✅ Document runbooks for common operations
5. ✅ Train team members on dashboard use

**You're now ready to manage Etrid from anywhere! 🚀**
