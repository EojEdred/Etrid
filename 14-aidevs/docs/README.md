# Ëtrid AI Devs - MCP Orchestrator

**Autonomous AI agents for Ëtrid Protocol governance and operations**

---

## 🤖 What is AI Devs?

AI Devs is a Model Context Protocol (MCP) orchestrator that manages 6 autonomous AI agents, each with specialized skills to automate Ëtrid Protocol operations:

| Agent | Purpose | Skills | Priority |
|-------|---------|--------|----------|
| **Compiler AI** | Build, test, debug | 4 skills | High |
| **Governance AI** | Proposals, voting, Consensus Day | 8 skills | Critical |
| **Runtime AI** | Upgrades, node management | 3 skills | High |
| **Economics AI** | Reserves, bridges, token economics | 5 skills | Medium |
| **Security AI** | Audits, threat detection, slashing | 5 skills | Critical |
| **Oracle AI** | Price feeds, anomaly detection | 2 skills | Medium |

**Total:** 29 specialized skills across 6 AI agents

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│                  Claude/GPT APIs                    │
│          (Reasoning Layer - Cloud/Local)            │
└───────────────────┬─────────────────────────────────┘
                    │
┌───────────────────▼─────────────────────────────────┐
│         MCP Orchestrator (FastAPI)                  │
│   ┌──────────┬──────────┬──────────┬──────────┐    │
│   │Compiler  │Governance│ Runtime  │Economics │    │
│   │   AI     │    AI    │   AI     │   AI     │    │
│   └──────────┴──────────┴──────────┴──────────┘    │
│   ┌──────────┬──────────────────────────────────┐  │
│   │Security  │        Skills Library            │  │
│   │   AI     │  (29 SKILL.md packages)          │  │
│   └──────────┴──────────────────────────────────┘  │
└───────────────────┬─────────────────────────────────┘
                    │
        ┌───────────┼───────────┬─────────────┐
        │           │           │             │
┌───────▼────┐ ┌───▼────┐ ┌────▼─────┐ ┌─────▼──────┐
│Ëtrid Node  │ │VectorDB│ │  Notion  │ │  Grafana   │
│(Substrate) │ │(Qdrant)│ │   Sync   │ │ Dashboard  │
│WS:9944     │ │:6333   │ │          │ │   :3000    │
└────────────┘ └────────┘ └──────────┘ └────────────┘
```

---

## 🚀 Quick Start

### Prerequisites

- Docker & Docker Compose
- API keys (Anthropic Claude or OpenAI GPT)
- (Optional) Notion API key for governance docs

### 1. Configure Environment

```bash
cd ai-devs
cp .env.example .env

# Edit .env and add your API keys
nano .env
```

Add your keys:
```env
ANTHROPIC_API_KEY=sk-ant-api03-your-key-here
# OR
OPENAI_API_KEY=sk-your-openai-key-here
```

### 2. Start All Services

```bash
docker compose up -d
```

This starts:
- **etrid-node**: FlareChain blockchain node
- **ai-devs**: MCP orchestrator (FastAPI)
- **vectordb**: Qdrant for AI memory
- **notion-sync**: Governance docs sync
- **grafana**: Monitoring dashboard
- **prometheus**: Metrics collection

### 3. Verify Deployment

```bash
# Check all containers
docker compose ps

# Check orchestrator health
curl http://localhost:4000/health

# List active agents
curl http://localhost:4000/agents

# List all skills
curl http://localhost:4000/skills
```

### 4. Access Dashboards

- **AI Devs API:** http://localhost:4000
- **Grafana:** http://localhost:3000 (admin/admin)
- **Prometheus:** http://localhost:9090
- **Qdrant:** http://localhost:6333/dashboard

---

## 📚 Skills Available

### Compiler AI Skills
- `etrid-compile-build` - Full workspace compilation
- `error-debugging` - Parse and fix compile errors
- `workspace-manager` - Cargo workspace orchestration
- `integration-test` - Run test suite with coverage

### Governance AI Skills
- `proposal-generator` - Draft governance proposals
- `vote-simulation` - Simulate voting outcomes
- `committee-rotation` - Validator committee management
- `consensus-day-orchestrator` - Annual Consensus Day automation
- `compliance-dev` - Legal/regulatory checks
- `ethics-dev` - Bias detection, ethical guardrails
- `moderation-dev` - Proposal spam filtering
- `governance-dev` - Core governance operations

### Runtime AI Skills
- `runtime-upgrade` - Package runtime upgrades
- `node-launcher` - Start/stop validators
- `integration-test` - Runtime integration tests

### Economics AI Skills
- `reserve-tracker` - Monitor ËDSC reserves
- `vmw-simulator` - Gas fee modeling
- `bridge-monitor` - Cross-chain bridge health
- `distribution-scheduler` - Token distribution automation
- `edsc-dev` - Stablecoin operations

### Security AI Skills
- `security-hardening` - Security best practices enforcement
- `bridge-monitor` - Multi-sig bridge auditing
- `slashing-verifier` - Validator slashing verification
- `audit-dev` - Continuous compliance checking
- `reputation-dev` - Sybil attack detection

### Oracle AI Skills
- `oracle-dev` - AI-enhanced data attestation
- `bridge-monitor` - Oracle integrity checks

---

## 🎯 Usage Examples

### Execute a Skill

```bash
curl -X POST http://localhost:4000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "agent_name": "compiler",
    "skill_name": "etrid-compile-build",
    "parameters": {}
  }'
```

### Trigger Compilation

```bash
curl -X POST http://localhost:4000/trigger/compile
```

### Generate Governance Proposal

```bash
curl -X POST http://localhost:4000/trigger/governance?proposal_type=treasury
```

### View Agent Memory

```bash
curl http://localhost:4000/memory/compiler?limit=10
```

### Get Metrics

```bash
curl http://localhost:4000/metrics
```

---

## 🔧 Configuration

### MCP Config (`config/mcp_config.yaml`)

Customize:
- Agent auto-start behavior
- LLM backend (Claude, GPT, or local Ollama)
- VectorDB settings
- Blockchain connection
- Skill execution timeouts

### Environment Variables

See `.env.example` for all available options:
- API keys
- LLM model selection
- Logging level
- Timeouts and limits

---

## 📊 Monitoring

### Grafana Dashboards

Access Grafana at http://localhost:3000

Pre-configured dashboards:
- AI Dev Activity
- Skill Execution Metrics
- Blockchain Sync Status
- Memory Usage
- Error Rates

### Logs

```bash
# View orchestrator logs
docker compose logs -f ai-devs

# View specific agent activity
docker compose logs -f ai-devs | grep "Compiler AI"

# View all logs
docker compose logs -f
```

### Prometheus Metrics

Access Prometheus at http://localhost:9090

Metrics available:
- Skill execution counts
- Success/failure rates
- Execution times
- Agent health status

---

## 🆔 AI Dev DIDs

Each AI agent has a Decentralized Identifier (DID) registered on-chain:

- **Compiler AI:** `did:etrid:compiler-01`
- **Governance AI:** `did:etrid:governance-01`
- **Runtime AI:** `did:etrid:runtime-01`
- **Economics AI:** `did:etrid:economics-01`
- **Security AI:** `did:etrid:security-01`
- **Oracle AI:** `did:etrid:oracle-01`

DID documents are in `dids/` directory.

---

## 🔐 Security

### API Keys
- Store in `.env` (never commit to git)
- Use read-only API keys where possible
- Rotate keys regularly

### DID Private Keys
- Stored in encrypted keystore
- Each agent has unique keypair
- Used to sign on-chain actions

### Network Isolation
- Run on private network
- Firewall external access
- VPN access only

### Audit Logging
- All actions logged to VectorDB
- Immutable audit trail
- Query with `/memory/{agent}` endpoint

---

## 🛠️ Development

### Adding a New Skill

1. Create skill directory:
```bash
mkdir -p ai-devs/skills/my-new-skill/scripts
```

2. Create `SKILL.md`:
```markdown
---
name: my-new-skill
category: operations
priority: medium
---

# My New Skill

Description of what this skill does...
```

3. Create executable script:
```bash
cat > ai-devs/skills/my-new-skill/scripts/run.sh << 'EOF'
#!/bin/bash
# Skill implementation here
echo "Skill executed successfully"
EOF

chmod +x ai-devs/skills/my-new-skill/scripts/run.sh
```

4. Restart orchestrator:
```bash
docker compose restart ai-devs
```

### Adding a New Agent

1. Create agent file: `orchestrator/agents/my_agent.py`
2. Inherit from `BaseAgent`
3. Implement `_start_monitoring()` method
4. Update `mcp_config.yaml`
5. Rebuild container

---

## 🚀 Production Deployment

### Deploy to VPS

```bash
# Copy to server
scp -r ai-devs user@server:/opt/etrid/

# SSH to server
ssh user@server

# Configure environment
cd /opt/etrid/ai-devs
cp .env.example .env
nano .env

# Start services
docker compose up -d

# Enable auto-restart
sudo systemctl enable docker-compose@ai-devs
```

### Systemd Service

Create `/etc/systemd/system/etrid-ai-devs.service`:

```ini
[Unit]
Description=Ëtrid AI Devs MCP Orchestrator
Requires=docker.service
After=docker.service

[Service]
Type=oneshot
RemainAfterExit=yes
WorkingDirectory=/opt/etrid/ai-devs
ExecStart=/usr/bin/docker compose up -d
ExecStop=/usr/bin/docker compose down

[Install]
WantedBy=multi-user.target
```

Enable:
```bash
sudo systemctl daemon-reload
sudo systemctl enable etrid-ai-devs
sudo systemctl start etrid-ai-devs
```

---

## 📞 Support

- **Documentation:** This README
- **Issues:** GitHub Issues
- **Discord:** #ai-devs channel
- **Email:** dev@etrid.io

---

## 📝 License

Apache 2.0 - See LICENSE file

---

**Built with ❤️ by the Ëtrid Foundation**
