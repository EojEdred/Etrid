# Ëtrid AI Devs - Quick Start Guide

**Get AI Devs running in 5 minutes**

---

## Prerequisites

- Docker Desktop installed and running
- API key for Claude or GPT (choose one)

---

## Step 1: Configure API Keys (2 minutes)

```bash
cd ai-devs

# Copy environment template
cp .env.example .env

# Edit and add your API key
nano .env
```

**Add EITHER:**
```env
ANTHROPIC_API_KEY=sk-ant-api03-your-key-here
```

**OR:**
```env
OPENAI_API_KEY=sk-your-openai-key-here
```

Save and exit (Ctrl+X, Y, Enter)

---

## Step 2: Start Services (1 minute)

```bash
docker compose up -d
```

This starts 6 containers:
- ✅ **etrid-node** - Blockchain node
- ✅ **ai-devs** - MCP orchestrator
- ✅ **vectordb** - AI memory (Qdrant)
- ✅ **notion-sync** - Governance docs
- ✅ **grafana** - Monitoring dashboard
- ✅ **prometheus** - Metrics collection

---

## Step 3: Verify (1 minute)

```bash
# Check containers are running
docker compose ps

# Check orchestrator health
curl http://localhost:4000/health

# List active AI agents
curl http://localhost:4000/agents
```

**Expected output:**
```json
{
  "status": "healthy",
  "agents": {
    "compiler": { "status": "started", "skills_count": 4 },
    "governance": { "status": "started", "skills_count": 8 },
    "runtime": { "status": "started", "skills_count": 3 },
    "economics": { "status": "started", "skills_count": 5 },
    "security": { "status": "started", "skills_count": 5 },
    "oracle": { "status": "started", "skills_count": 2 }
  }
}
```

---

## Step 4: Test Execution (1 minute)

### Trigger a Compilation

```bash
curl -X POST http://localhost:4000/trigger/compile
```

### Check Compiler AI Memory

```bash
curl http://localhost:4000/memory/compiler?limit=5
```

### View Metrics

```bash
curl http://localhost:4000/metrics
```

---

## Step 5: Access Dashboards

Open in browser:

- **AI Devs API:** http://localhost:4000
- **Grafana:** http://localhost:3000 (login: admin/admin)
- **Prometheus:** http://localhost:9090
- **Qdrant Dashboard:** http://localhost:6333/dashboard

---

## 🎉 You're Done!

Your 6 AI agents are now operational and monitoring the Ëtrid Protocol.

---

## What's Happening Now?

### Compiler AI
- ✅ Monitoring for code changes
- ✅ Auto-fixing compilation errors
- ✅ Running tests

### Governance AI
- ✅ Monitoring blockchain for proposals
- ✅ Checking compliance
- ✅ Preparing for Consensus Day

### Runtime AI
- ✅ Monitoring node health
- ✅ Ready to perform upgrades

### Economics AI
- ✅ Tracking ËDSC reserve ratio
- ✅ Monitoring cross-chain bridges
- ✅ Analyzing token economics

### Security AI
- ✅ Auditing code changes
- ✅ Monitoring for threats
- ✅ Watching for slashing events

### Oracle AI
- ✅ Monitoring price feeds
- ✅ Detecting anomalies

---

## Next Steps

### 1. Explore the API

Visit http://localhost:4000/docs for interactive API documentation.

### 2. View Logs

```bash
# All logs
docker compose logs -f

# Specific agent
docker compose logs -f ai-devs | grep "Compiler AI"
```

### 3. Execute Skills Manually

```bash
curl -X POST http://localhost:4000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "agent_name": "governance",
    "skill_name": "proposal-generator",
    "parameters": {"type": "treasury"}
  }'
```

### 4. Set Up Notion Integration (Optional)

If you want governance docs synced to Notion:

1. Create a Notion integration at https://www.notion.so/my-integrations
2. Get API key and database ID
3. Add to `.env`:
   ```env
   NOTION_API_KEY=secret_your-key
   NOTION_DATABASE_ID=your-database-id
   ```
4. Restart services:
   ```bash
   docker compose restart
   ```

---

## Troubleshooting

### Containers won't start

```bash
# Check logs
docker compose logs

# Rebuild containers
docker compose down
docker compose build --no-cache
docker compose up -d
```

### API key errors

- Make sure API key is valid
- Check `.env` file format (no spaces around =)
- Restart services: `docker compose restart ai-devs`

### Can't connect to blockchain

```bash
# Check if node is running
curl http://localhost:9944

# Restart node
docker compose restart etrid-node
```

### Out of memory

```bash
# Increase Docker memory limit
# Docker Desktop → Settings → Resources → Memory
# Set to at least 4GB
```

---

## Learn More

- **Full Documentation:** See `README.md`
- **Global Context:** Read `config/GLOBAL_MEMORY.md`
- **Skills Reference:** Browse `skills/` directory
- **Project Roadmap:** See `/workspace/LIVING_ROADMAP.md`

---

## Stop Services

```bash
docker compose down
```

To also remove volumes (clears AI memory):
```bash
docker compose down -v
```

---

**Need help? Check `README.md` or open an issue on GitHub.**
