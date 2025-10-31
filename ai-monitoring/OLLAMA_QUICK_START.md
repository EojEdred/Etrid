# Ollama Quick Start for Ëtrid Validators

## 🚀 Deploy in 3 Commands (10 minutes)

### **Option 1: Centralized (Recommended)**

Deploy Ollama on Gizzi VM only - all validators query it via API.

```bash
# Step 1: Deploy Ollama to Gizzi
cd /Users/macbook/Desktop/etrid/ai-monitoring
./deploy-ollama.sh
# Choose option 1

# Step 2: Deploy client to all validators
./deploy-ollama-client.sh

# Step 3: Test from any validator
ssh -i ~/.ssh/gizzi-validator runtime-dev01@20.224.104.239
python3 /opt/validator/ollama_client.py health 6
```

**Done!** All validators can now query Ollama for free, instant AI analysis.

---

### **Option 2: Distributed**

Deploy Ollama on all 21 validators - each has local AI.

```bash
# Step 1: Deploy Ollama to all validators
cd /Users/macbook/Desktop/etrid/ai-monitoring
./deploy-ollama.sh
# Choose option 2

# Step 2: Deploy client to all validators
./deploy-ollama-client.sh

# Step 3: Test from any validator (local)
ssh -i ~/.ssh/gizzi-validator runtime-dev01@20.224.104.239
python3 /opt/validator/ollama_client.py health 6
```

---

## 📊 What You Get

### **Free, Instant AI Queries**

Every validator can now:

```bash
# Quick health check
python3 /opt/validator/ollama_client.py health 6
# Output: "Validator 6 is healthy with 8 peers and in sync at block 12345"

# Analyze logs
python3 /opt/validator/ollama_client.py logs
# Output: "WARNING: Peer count dropped from 12 to 3. Recommend checking firewall rules."

# Compare to network
python3 /opt/validator/ollama_client.py compare 12345 8 12340
# Output: "You are in sync with the network. Peer count is above average."

# Ask questions
python3 /opt/validator/ollama_client.py ask "How many validators are online?"
# Output: "20 out of 21 validators are currently online and healthy."
```

### **Python API**

```python
from ollama_client import OllamaClient

client = OllamaClient()

# Analyze validator health
health = client.analyze_validator_health({
    "block_height": 12345,
    "peers": 8,
    "finalized_height": 12340
})

print(health)
# {
#   "health": "healthy",
#   "issues": [],
#   "recommendations": []
# }

# Ask about network
status = client.ask_network_question("Are all bootnodes online?")
print(status)
```

---

## 🎯 Integration with AI Dev Monitoring

Ollama complements Claude API:

| Task | Use | Cost | Speed |
|------|-----|------|-------|
| **Quick health checks** | Ollama | $0 | 2-5s |
| **Log analysis** | Ollama | $0 | 2-5s |
| **Network status** | Ollama | $0 | 2-5s |
| **Complex diagnostics** | Claude API | ~$0.10 | 200-500ms |
| **Critical decisions** | Claude API | ~$0.10 | 200-500ms |
| **Audit trail** | Claude API | ~$0.10 | 200-500ms |

**Total monthly cost:** ~$20-30 (mostly Claude API for critical tasks)

---

## 📋 Architecture

### Centralized (Option 1)

```
Gizzi VM (64.181.215.19)
├── Ollama (llama3.1:8b) - Port 11434
│   └── Free, instant AI inference
├── Gizzi Network API - Port 8080
│   ├── /api/network/status
│   ├── /api/validator/<id>
│   └── /api/network/summary
└── AI Dev Monitoring (Claude API)
    └── Complex analysis & decisions

All 21 validators query Gizzi via HTTP
```

### Distributed (Option 2)

```
Each of 21 validators:
├── Local Ollama (llama3.1:8b)
│   └── Zero latency, autonomous
├── Local client library
└── Can still query Gizzi API for network-wide data
```

---

## 🔧 Commands

### On Gizzi VM

```bash
# Check Ollama status
sudo systemctl status ollama

# Watch Ollama logs
sudo journalctl -u ollama -f

# Check Gizzi API status
sudo systemctl status gizzi-api

# Test Ollama API
curl http://localhost:11434/api/version

# Test Gizzi Network API
curl http://localhost:8080/health
```

### On Any Validator

```bash
# Quick health check
python3 /opt/validator/ollama_client.py health <validator_id>

# Analyze recent logs
python3 /opt/validator/ollama_client.py logs

# Compare metrics to network
python3 /opt/validator/ollama_client.py compare <block> <peers> <finalized>

# Ask a question
python3 /opt/validator/ollama_client.py ask "your question"
```

---

## 💡 Use Cases

### 1. Validator Self-Diagnosis

Each validator can check itself:

```python
# Get my metrics
my_block = 12345
my_peers = 3
my_finalized = 12340

# Compare to network
analysis = compare_to_network(my_block, my_peers, my_finalized)
print(analysis)
# "WARNING: Your peer count (3) is below network average (7.5)"
```

### 2. Network Status Queries

```python
# Ask about network
status = ask_gizzi("How many validators are behind on blocks?")
print(status)
# "2 validators are behind: #7 is 50 blocks behind, #12 is 120 blocks behind"
```

### 3. Log Analysis

```bash
# Analyze last 100 lines
journalctl -u flare-node -n 100 --no-pager | python3 /opt/validator/ollama_client.py logs
```

---

## 📊 Resource Usage

### Gizzi VM (Centralized)

Before:
- CPU: 4 cores
- RAM: 8GB
- Disk: 120GB

After:
- CPU: 6 cores
- RAM: 16GB (**+8GB for Ollama**)
- Disk: 125GB (+5GB for model)

### Each Validator (Distributed)

+8GB RAM, +5GB disk

---

## 🚨 Troubleshooting

### Ollama won't start

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19
sudo journalctl -u ollama -n 50
# Check for RAM issues: free -h
```

### Can't connect from validators

```bash
# Test from validator
curl http://64.181.215.19:11434/api/version

# If fails, check firewall on Gizzi
sudo iptables -L | grep 11434
```

### Slow responses

- Use smaller model: `ollama pull phi3:mini`
- Or deploy distributed (zero latency)

---

## 📈 Next Steps

1. **Deploy Ollama** (10 minutes)
   ```bash
   ./deploy-ollama.sh
   ```

2. **Test it** (2 minutes)
   ```bash
   python3 /opt/validator/ollama_client.py health 1
   ```

3. **Integrate with monitoring** (optional)
   - Update ai_dev_workers.py to use Ollama for quick checks
   - Only call Claude API for complex issues

4. **Monitor usage**
   - Watch logs: `sudo journalctl -u ollama -f`
   - Check resource usage: `htop`

---

## ✅ Benefits

- ✅ **Free**: No API costs after initial setup
- ✅ **Fast**: 2-5 second responses (vs 200-500ms Claude but free)
- ✅ **Private**: All inference stays within your network
- ✅ **Autonomous**: Each validator can self-diagnose
- ✅ **Complementary**: Use with Claude API for best results

---

**Status:** Ready to deploy
**Time:** 10 minutes
**Cost:** $0/month
**Recommendation:** Deploy centralized (Option 1) first, test, then consider distributed
