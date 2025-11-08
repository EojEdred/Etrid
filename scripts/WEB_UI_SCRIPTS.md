# ÉTRID Web UI Management Scripts

This directory contains convenient shell scripts for managing all ÉTRID web UI applications.

## Available Scripts

### build-all-web-uis.sh
Builds all web UI applications with their dependencies.

```bash
./scripts/build-all-web-uis.sh
```

**What it does:**
- Installs npm dependencies for each application
- Builds production bundles for all web UIs
- Reports build status for each application
- Handles special flags (e.g., `--legacy-peer-deps` for wallet-web)

**Applications built:**
1. Lightning Landing
2. MasterChef Dashboard
3. Validator Dashboard
4. Watchtower Monitor
5. Wallet Web

---

### start-all-web-uis.sh
Starts all web UI applications on their designated ports.

```bash
./scripts/start-all-web-uis.sh
```

**What it does:**
- Checks for and installs dependencies if missing
- Builds applications if not already built
- Starts each application as a background process
- Creates PID files for process management
- Logs output to `/tmp/etrid-*.log`

**Port assignments:**
- Lightning Landing: `http://localhost:3000`
- MasterChef Dashboard: `http://localhost:3001`
- Validator Dashboard: `http://localhost:3002`
- Watchtower Monitor: `http://localhost:3003`
- Wallet Web: `http://localhost:3004`

---

### stop-all-web-uis.sh
Stops all running web UI applications.

```bash
./scripts/stop-all-web-uis.sh
```

**What it does:**
- Reads PID files from `/tmp/etrid-*.pid`
- Gracefully terminates each running process
- Cleans up PID and log files
- Reports status for each application

---

### status-web-uis.sh
Checks the status of all web UI applications.

```bash
./scripts/status-web-uis.sh
```

**What it does:**
- Checks if processes are running via PID files
- Verifies if ports are listening
- Displays running/stopped status for each app
- Shows URLs for running applications
- Lists available management commands

---

## Quick Start

### First time setup:
```bash
# Build all applications
./scripts/build-all-web-uis.sh

# Start all applications
./scripts/start-all-web-uis.sh

# Check status
./scripts/status-web-uis.sh
```

### Daily usage:
```bash
# Start all apps
./scripts/start-all-web-uis.sh

# Check what's running
./scripts/status-web-uis.sh

# Stop all apps when done
./scripts/stop-all-web-uis.sh
```

---

## Troubleshooting

### Check logs
Application logs are stored in `/tmp/etrid-*.log`:
```bash
# View logs for a specific app
tail -f /tmp/etrid-lightning-landing.log

# View all logs
tail -f /tmp/etrid-*.log
```

### Port conflicts
If a port is already in use:
1. Check what's using the port: `lsof -i:3000`
2. Stop the conflicting process
3. Restart the application

### Build failures
If a build fails:
1. Check the error output
2. Try rebuilding just that application:
   ```bash
   cd apps/[app-name]
   npm install
   npm run build
   ```
3. For wallet-web, use: `npm install --legacy-peer-deps`

### Stale processes
If processes show as running but aren't responding:
```bash
# Force stop all
./scripts/stop-all-web-uis.sh

# Manually clean up if needed
rm /tmp/etrid-*.pid
rm /tmp/etrid-*.log

# Restart
./scripts/start-all-web-uis.sh
```

---

## Development Tips

### Running individual applications
Instead of starting all apps, you can run them individually:

```bash
cd apps/lightning-landing
npm run dev        # Development mode with hot reload
npm start          # Production mode
```

### Watching logs in real-time
```bash
# Watch all logs
tail -f /tmp/etrid-*.log

# Watch specific app
tail -f /tmp/etrid-validator-dashboard.log
```

### Customizing ports
Edit the port assignments in `start-all-web-uis.sh` if you need different ports.

---

## Integration with Pinokio

These scripts are also referenced in the Pinokio configuration (`pinokio.js`).
You can use either:
- Pinokio's UI for installation and management
- These command-line scripts for more control

Both approaches build and manage the same applications.

---

## Requirements

- Node.js 18+ (required for validator-dashboard and others)
- npm
- Bash shell
- Linux/macOS (Windows users: use WSL or Git Bash)

---

## Notes

- All scripts use color-coded output for better readability
- Scripts are designed to be idempotent (safe to run multiple times)
- PID files prevent duplicate processes
- Logs are automatically rotated when you stop/start applications
- Build artifacts are cached in each app's `.next` directory

---

For more information, see the main [PINOKIO_README.md](../PINOKIO_README.md).
