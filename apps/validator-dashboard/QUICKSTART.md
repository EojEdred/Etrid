# Ëtrid Validator Dashboard - Quick Start Guide

## Prerequisites Check

Before you begin, ensure you have:

- ✅ Node.js 18.0.0 or higher
- ✅ npm 9.0.0 or higher
- ✅ Running Ëtrid node (local or remote access)
- ✅ Validator stash address

Check your versions:
```bash
node --version  # Should be v18.0.0 or higher
npm --version   # Should be 9.0.0 or higher
```

---

## 5-Minute Setup

### Step 1: Navigate to the Dashboard
```bash
cd /Users/macbook/Desktop/etrid/apps/validator-dashboard
```

### Step 2: Install Dependencies
```bash
npm install
```

This will install:
- Next.js 14
- React 18
- Polkadot.js API
- TailwindCSS
- Recharts
- And all other dependencies

**Expected time**: 2-3 minutes

### Step 3: Configure Environment
```bash
cp .env.example .env
```

Edit `.env` with your settings:
```bash
# Open in your editor
nano .env
# or
code .env
```

**Minimum required configuration**:
```env
NEXT_PUBLIC_WS_PROVIDER=ws://localhost:9944
NEXT_PUBLIC_VALIDATOR_ADDRESS=your_validator_stash_address_here
```

**Optional configuration**:
```env
NEXT_PUBLIC_NETWORK_NAME=Ëtrid
NEXT_PUBLIC_CHAIN_DECIMALS=18
NEXT_PUBLIC_CHAIN_TOKEN=ETRID
NEXT_PUBLIC_ALERT_EMAIL=your-email@example.com
NEXT_PUBLIC_DISCORD_WEBHOOK=https://discord.com/api/webhooks/...
```

### Step 4: Start Development Server
```bash
npm run dev
```

You should see:
```
ready - started server on 0.0.0.0:3002
event - compiled client and server successfully
```

### Step 5: Open in Browser
```
http://localhost:3002
```

---

## First-Time Setup Checklist

After opening the dashboard, verify:

- [ ] Connection status shows "Connected" (green)
- [ ] Session/Era information displays in banner
- [ ] Validator stats cards show your data
- [ ] Charts render without errors
- [ ] Nominator list populates (if you have nominators)
- [ ] Alerts panel shows mock data
- [ ] Navigation works (Dashboard, Performance, Settings)

---

## Common Issues & Solutions

### Issue: "Cannot connect to node"

**Symptoms**: Red "Disconnected" indicator, no data loading

**Solutions**:
1. Verify your node is running:
   ```bash
   curl http://localhost:9944
   ```

2. Check node WebSocket is enabled in node config

3. If using remote node, ensure firewall allows connection

4. Try alternative endpoint:
   ```env
   NEXT_PUBLIC_WS_PROVIDER=wss://rpc.etrid.network
   ```

### Issue: "Module not found" errors

**Solution**:
```bash
rm -rf node_modules package-lock.json
npm install
```

### Issue: Port 3002 already in use

**Solution**:
```bash
# Use a different port
npm run dev -- -p 3003
```

Or kill the process using port 3002:
```bash
lsof -ti:3002 | xargs kill -9
```

### Issue: TypeScript errors

**Solution**:
```bash
npm run type-check
```

Fix any reported errors, then:
```bash
npm run dev
```

---

## Production Build

When ready for production:

### Step 1: Build
```bash
npm run build
```

This creates an optimized production build in `.next/` directory.

### Step 2: Start Production Server
```bash
npm start
```

### Step 3: Verify
Open `http://localhost:3002` and test all features.

---

## Environment-Specific Configurations

### Local Development
```env
NEXT_PUBLIC_WS_PROVIDER=ws://localhost:9944
```

### Testnet
```env
NEXT_PUBLIC_WS_PROVIDER=wss://testnet-rpc.etrid.network
NEXT_PUBLIC_NETWORK_NAME=Ëtrid Testnet
```

### Mainnet
```env
NEXT_PUBLIC_WS_PROVIDER=wss://rpc.etrid.network
NEXT_PUBLIC_NETWORK_NAME=Ëtrid
```

---

## Testing Your Setup

### Test 1: Connection
Visit the dashboard. The header should show:
- ✅ Green WiFi icon
- ✅ "Connected" text

### Test 2: Data Loading
Check that all sections show data:
- ✅ Session banner with era/session info
- ✅ Validator stats (4 cards)
- ✅ Reward history chart
- ✅ Nominator list (if applicable)

### Test 3: Navigation
Click through pages:
- ✅ Dashboard (/)
- ✅ Performance (/performance)
- ✅ Settings (/settings)

### Test 4: Interactions
Try these actions:
- ✅ Click "Refresh" button
- ✅ Change chart type in Reward History
- ✅ Sort nominator table
- ✅ Search nominators
- ✅ Adjust time range in Performance page
- ✅ Move commission slider in Settings

---

## Development Workflow

### Hot Reload
Next.js automatically reloads when you save files. Make changes to any component and see them instantly.

### File Structure
```
src/
├── components/     # Edit UI components here
├── pages/         # Edit pages here
├── hooks/         # Edit data hooks here
├── types/         # Edit TypeScript types here
└── utils/         # Edit helper functions here
```

### Adding Features
1. Create new component in `src/components/`
2. Import and use in page
3. Save and see changes instantly

### Debugging
Open browser DevTools (F12):
- **Console**: View API calls and errors
- **Network**: Monitor WebSocket connection
- **React DevTools**: Inspect component state

---

## Updating the Dashboard

### Update Dependencies
```bash
npm update
```

### Update to Latest
```bash
npm install next@latest react@latest react-dom@latest
```

### Check for Security Issues
```bash
npm audit
npm audit fix
```

---

## Deployment Options

### Option 1: Vercel (Recommended)
```bash
npm install -g vercel
vercel
```

### Option 2: Docker
```bash
docker build -t etrid-validator-dashboard .
docker run -p 3002:3002 etrid-validator-dashboard
```

### Option 3: Traditional Server
```bash
npm run build
npm start
```

Use a process manager like PM2:
```bash
npm install -g pm2
pm2 start npm --name "validator-dashboard" -- start
pm2 save
```

---

## Getting Help

### Documentation
- Full README: [README.md](./README.md)
- Feature Guide: [FEATURES.md](./FEATURES.md)

### Community Support
- Discord: https://discord.gg/etrid
- GitHub Issues: Submit bug reports or feature requests
- Email: support@etrid.network

### Logs
Check console output for errors:
```bash
npm run dev 2>&1 | tee dashboard.log
```

---

## Next Steps

After successful setup:

1. **Customize Branding**: Update colors in `tailwind.config.js`
2. **Add Your Logo**: Place in `public/` and update `Layout.tsx`
3. **Configure Alerts**: Set up email/Discord in Settings page
4. **Enable Analytics**: Add tracking (Google Analytics, etc.)
5. **Secure Deployment**: Use HTTPS, set up authentication if needed

---

## Performance Tips

### Reduce Data Refresh Rate
In `src/hooks/useValidatorStats.ts`, line 295:
```typescript
const interval = setInterval(refreshData, 60000); // Change to 60 seconds
```

### Limit Historical Data
In `src/hooks/useValidatorStats.ts`, line 184:
```typescript
for (let i = 0; i < 15 && eraNumber - i >= 0; i++) { // Reduce to 15 eras
```

### Optimize Charts
In chart components, reduce data points:
```typescript
const uptimeData = Array.from({ length: 7 }, ...); // Show only 7 days
```

---

## Maintenance

### Regular Tasks
- ✅ Update dependencies monthly
- ✅ Check connection to node daily
- ✅ Review alerts weekly
- ✅ Backup configuration files
- ✅ Monitor performance metrics

### Health Checks
```bash
# Check if server is running
curl http://localhost:3002

# Check API response time
curl -w "@curl-format.txt" -o /dev/null -s http://localhost:3002/api/health

# Monitor logs
pm2 logs validator-dashboard
```

---

**Ready to launch your validator dashboard!** 🚀

For detailed documentation, see [README.md](./README.md)
