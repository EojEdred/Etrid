# Ollama Local AI Deployment for Ëtrid Validators
## Free, Fast, Local AI Inference + Gizzi API Integration

**Date:** 2025-10-31
**Purpose:** Deploy Ollama for local AI inference on validators to complement Claude API monitoring

---

## 🎯 Architecture Options

### **Option A: Centralized (Recommended)**

Deploy Ollama **only on Gizzi VM (64.181.215.19)**

```
┌─────────────────────────────────────────┐
│   Gizzi VM (64.181.215.19)             │
│                                         │
│   ┌──────────────────────────────────┐ │
│   │  Ollama (llama3.1:8b)            │ │
│   │  - Port: 11434                   │ │
│   │  - RAM: ~8GB                     │ │
│   │  - API: http://64.181.215.19:... │ │
│   └──────────────────────────────────┘ │
│                                         │
│   ┌──────────────────────────────────┐ │
│   │  Gizzi Network APIs              │ │
│   │  - Validator metrics             │ │
│   │  - Network status                │ │
│   │  - Prometheus queries            │ │
│   └──────────────────────────────────┘ │
└─────────────────────────────────────────┘
                  ↑
                  │ HTTP API calls
                  │
     ┌────────────┴────────────┐
     │                         │
┌────┴────┐  ┌────────┐  ┌────┴────┐
│Validator│  │Validator│  │Validator│
│   6     │  │   7     │  │   8     │
└─────────┘  └─────────┘  └─────────┘
     (All 21 validators can query Ollama on Gizzi)
```

**Pros:**
- ✅ Single deployment (easy maintenance)
- ✅ Lower resource usage (1 instance vs 21)
- ✅ Centralized with monitoring infrastructure
- ✅ Network latency minimal (all in same datacenter)
- ✅ Cost: Free (Ollama is open source)

**Cons:**
- ⚠️ Single point of failure (mitigated by Claude API backup)
- ⚠️ Slight network latency (~10-50ms)

---

### **Option B: Distributed**

Deploy Ollama on **all 21 validators**

```
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ Validator 1  │  │ Validator 6  │  │ Validator 12 │
│              │  │              │  │              │
│  ┌────────┐  │  │  ┌────────┐  │  │  ┌────────┐  │
│  │ Ollama │  │  │  │ Ollama │  │  │  │ Ollama │  │
│  │  8GB   │  │  │  │  8GB   │  │  │  │  8GB   │  │
│  └────────┘  │  │  └────────┘  │  │  └────────┘  │
└──────────────┘  └──────────────┘  └──────────────┘
    ... (21 total Ollama instances)
```

**Pros:**
- ✅ Zero latency (local inference)
- ✅ Highly resilient (no single point of failure)
- ✅ Each validator fully autonomous

**Cons:**
- ⚠️ 21x resource usage (~168GB RAM total)
- ⚠️ More complex deployment and updates
- ⚠️ Need to verify all VMs have sufficient resources

---

## 💰 Cost Comparison

| Component | Claude API | Ollama (Centralized) | Ollama (Distributed) |
|-----------|-----------|---------------------|---------------------|
| **Setup Cost** | $0 (API only) | $0 | $0 |
| **Monthly Cost** | ~$56 (optimized) | $0 | $0 |
| **Latency** | 200-500ms | 10-50ms | <5ms |
| **Resource Usage** | None | 8GB RAM (Gizzi) | 168GB RAM (all VMs) |
| **Quality** | Excellent (Claude 3.5) | Good (Llama 3.1 8B) | Good (Llama 3.1 8B) |

**Recommendation:** Use **both** systems:
- **Ollama (Centralized)** for quick, frequent queries (network status, simple diagnostics)
- **Claude API** for complex analysis, critical decisions, audit trail

---

## 🚀 Deployment: Option A (Centralized - Recommended)

### Step 1: Install Ollama on Gizzi VM

```bash
# SSH to Gizzi
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Start Ollama service
sudo systemctl start ollama
sudo systemctl enable ollama

# Pull Llama 3.1 8B model (recommended for validators)
ollama pull llama3.1:8b

# Test it
ollama run llama3.1:8b "What is a blockchain validator?"
```

### Step 2: Configure Ollama for Network Access

```bash
# Allow external connections
sudo mkdir -p /etc/systemd/system/ollama.service.d/
sudo tee /etc/systemd/system/ollama.service.d/override.conf > /dev/null <<'EOF'
[Service]
Environment="OLLAMA_HOST=0.0.0.0:11434"
EOF

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart ollama

# Test from another VM
curl http://64.181.215.19:11434/api/version
```

### Step 3: Configure Firewall (if needed)

```bash
# Oracle Cloud: Open port 11434
sudo iptables -I INPUT -p tcp --dport 11434 -j ACCEPT
sudo netfilter-persistent save

# Or use ufw
sudo ufw allow 11434/tcp
```

---

## 🚀 Deployment: Option B (Distributed)

### Step 1: Create Installation Script

```bash
#!/bin/bash
# install-ollama.sh - Deploy Ollama to all validators

VALIDATORS=(
  "ubuntu@64.181.215.19"
  "ubuntu@129.80.122.34"
  "runtime-dev01@20.224.104.239"
  "runtime-dev01@108.142.205.177"
  "compiler-dev01@4.180.238.67"
  # ... (all 21 validators)
)

for validator in "${VALIDATORS[@]}"; do
  echo "Installing Ollama on $validator..."
  ssh -i ~/.ssh/gizzi-validator "$validator" "curl -fsSL https://ollama.com/install.sh | sh && ollama pull llama3.1:8b"
done
```

### Step 2: Deploy to All VMs

```bash
chmod +x install-ollama.sh
./install-ollama.sh
```

---

## 🔌 Python Client Library for Validators

Create this on each validator to query Ollama:

```python
# /opt/validator/ollama_client.py
import requests
import json

class OllamaClient:
    def __init__(self, host="http://64.181.215.19:11434"):
        """
        Client for querying Ollama on Gizzi VM

        Args:
            host: Ollama API endpoint (default: Gizzi VM)
        """
        self.host = host
        self.model = "llama3.1:8b"

    def query(self, prompt: str, system_prompt: str = None) -> str:
        """
        Send a query to Ollama and get response

        Args:
            prompt: User prompt
            system_prompt: Optional system instructions

        Returns:
            Model response text
        """
        url = f"{self.host}/api/generate"

        payload = {
            "model": self.model,
            "prompt": prompt,
            "stream": False
        }

        if system_prompt:
            payload["system"] = system_prompt

        try:
            response = requests.post(url, json=payload, timeout=30)
            response.raise_for_status()
            return response.json()["response"]
        except Exception as e:
            return f"Error querying Ollama: {str(e)}"

    def query_gizzi_api(self, api_endpoint: str, analysis_prompt: str) -> str:
        """
        Query a Gizzi API and ask Ollama to analyze the result

        Args:
            api_endpoint: Gizzi API endpoint (e.g., "/api/network/status")
            analysis_prompt: What to ask Ollama about the data

        Returns:
            Ollama's analysis
        """
        # Get data from Gizzi API
        try:
            gizzi_url = f"http://64.181.215.19{api_endpoint}"
            api_response = requests.get(gizzi_url, timeout=10)
            api_data = api_response.json()
        except Exception as e:
            return f"Error querying Gizzi API: {str(e)}"

        # Ask Ollama to analyze it
        full_prompt = f"""
Here is data from the Gizzi network API:

{json.dumps(api_data, indent=2)}

{analysis_prompt}
"""

        return self.query(full_prompt)

# Example usage
if __name__ == "__main__":
    client = OllamaClient()

    # Simple query
    response = client.query("What are signs of a healthy blockchain validator?")
    print(response)

    # Query Gizzi API and analyze
    analysis = client.query_gizzi_api(
        "/api/prometheus/query?query=substrate_block_height",
        "Are all validators at similar block heights? Any concerning lags?"
    )
    print(analysis)
```

---

## 🔗 Integration with Gizzi APIs

### Create API Endpoints on Gizzi

```python
# /opt/gizzi/api_server.py
from flask import Flask, jsonify
import requests

app = Flask(__name__)

@app.route('/api/network/status')
def network_status():
    """Network-wide status"""
    # Query Prometheus
    prom_url = "http://localhost:9090/api/v1/query"

    validators = []
    for i in range(1, 22):
        # Get metrics for each validator
        block_height = requests.get(f"{prom_url}?query=substrate_block_height{{validator=\"{i}\"}}")
        peers = requests.get(f"{prom_url}?query=substrate_peers_count{{validator=\"{i}\"}}")

        validators.append({
            "number": i,
            "block_height": block_height.json()["data"]["result"][0]["value"][1],
            "peers": peers.json()["data"]["result"][0]["value"][1]
        })

    return jsonify({"validators": validators})

@app.route('/api/validator/<int:validator_id>')
def validator_details(validator_id):
    """Single validator details"""
    prom_url = "http://localhost:9090/api/v1/query"

    # Get all metrics for this validator
    metrics = {
        "block_height": requests.get(f"{prom_url}?query=substrate_block_height{{validator=\"{validator_id}\"}}").json(),
        "peers": requests.get(f"{prom_url}?query=substrate_peers_count{{validator=\"{validator_id}\"}}").json(),
        "finalized_height": requests.get(f"{prom_url}?query=substrate_finalized_height{{validator=\"{validator_id}\"}}").json()
    }

    return jsonify(metrics)

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080)
```

Deploy as systemd service on Gizzi:

```bash
sudo tee /etc/systemd/system/gizzi-api.service > /dev/null <<'EOF'
[Unit]
Description=Gizzi Network API Server
After=network.target

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/opt/gizzi
ExecStart=/usr/bin/python3 /opt/gizzi/api_server.py
Restart=always

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl start gizzi-api
sudo systemctl enable gizzi-api
```

---

## 🎯 Use Cases

### 1. Validators Query Network Status

```python
# On any validator VM
from ollama_client import OllamaClient

client = OllamaClient()

# Ask about current network health
status = client.query_gizzi_api(
    "/api/network/status",
    "Summarize the health of the network. Any validators falling behind?"
)
print(status)
```

### 2. Self-Diagnosis

```python
# Validator checks its own health
import subprocess

# Get local logs
logs = subprocess.check_output(['journalctl', '-u', 'flare-node', '-n', '100']).decode()

# Ask Ollama to analyze
client = OllamaClient()
analysis = client.query(f"""
Here are my recent validator logs:

{logs}

Am I experiencing any issues? Should I restart?
""")

print(analysis)
```

### 3. Peer Discovery Help

```python
# Validator having peer connection issues
my_peers = subprocess.check_output(['curl', 'http://localhost:9615/metrics']).decode()

analysis = client.query(f"""
My validator metrics:

{my_peers}

I only have 2 peers. How can I improve peer discovery? What bootnodes should I add?
""")

print(analysis)
```

---

## 🔄 Integration with Existing AI Dev Monitoring

Update the AI dev monitoring system to use Ollama for quick checks:

```python
# In ai_dev_workers.py

class AIDevWorker:
    def __init__(self, aidev_id, anthropic_api_key, monitor, ollama_url=None, optimized=True):
        self.aidev_id = aidev_id
        self.anthropic_client = anthropic.Anthropic(api_key=anthropic_api_key)
        self.monitor = monitor
        self.optimized = optimized

        # Add Ollama client for quick queries
        if ollama_url:
            self.ollama = OllamaClient(ollama_url)
        else:
            self.ollama = None

    def quick_health_check(self, validator_statuses):
        """
        Use Ollama for quick health check (free, fast)
        Only call Claude API if Ollama detects issues
        """
        if not self.ollama:
            return self.analyze_with_claude(validator_statuses)

        # Quick check with Ollama (free)
        ollama_analysis = self.ollama.query(f"""
You are {self.aidev_id}. Analyze these validator statuses:

{json.dumps(validator_statuses, indent=2)}

Respond with ONLY "HEALTHY" or "ISSUE_DETECTED" and a brief reason.
""")

        # If Ollama detects issue, escalate to Claude for detailed analysis
        if "ISSUE_DETECTED" in ollama_analysis:
            print(f"[{self.aidev_id}] Ollama detected issue, escalating to Claude API...")
            return self.analyze_with_claude(validator_statuses)

        # No issues, save Claude API cost
        return {
            "summary": "All validators healthy (Ollama check)",
            "health": "healthy",
            "actions": []
        }
```

---

## 📊 Resource Requirements

### Gizzi VM (Option A)

| Component | CPU | RAM | Disk | Network |
|-----------|-----|-----|------|---------|
| **Ollama (llama3.1:8b)** | 2-4 cores | 8GB | 5GB | ~10Mbps |
| **Existing AI monitoring** | 1 core | 2GB | 1GB | ~5Mbps |
| **Validator node** | 2 cores | 4GB | 100GB | ~20Mbps |
| **Prometheus** | 1 core | 2GB | 20GB | ~5Mbps |
| **TOTAL** | **6-8 cores** | **16GB** | **126GB** | **~40Mbps** |

Verify Gizzi VM specs:

```bash
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19 "free -h && nproc && df -h"
```

### All VMs (Option B)

Each VM needs:
- CPU: +2 cores
- RAM: +8GB
- Disk: +5GB

**Total across 21 validators:**
- CPU: +42 cores
- RAM: +168GB
- Disk: +105GB

---

## 🎯 Recommended Models

| Model | Size | RAM | Speed | Quality | Use Case |
|-------|------|-----|-------|---------|----------|
| **llama3.1:8b** | 4.7GB | 8GB | Fast | Good | **Recommended** - Best balance |
| **phi3:mini** | 2.2GB | 4GB | Very fast | Okay | Low-resource VMs |
| **mistral** | 4.1GB | 8GB | Fast | Good | Alternative to Llama |
| **llama3.1:70b** | 40GB | 64GB | Slow | Excellent | Gizzi only (if resources) |

**For most validators: `llama3.1:8b`**

---

## ✅ Deployment Checklist

### Option A (Centralized)

- [ ] Verify Gizzi VM has 16GB+ RAM
- [ ] Install Ollama on Gizzi
- [ ] Pull llama3.1:8b model
- [ ] Configure network access (port 11434)
- [ ] Test from another validator VM
- [ ] Deploy Gizzi API server (port 8080)
- [ ] Deploy ollama_client.py to all validators
- [ ] Update AI dev monitoring to use Ollama
- [ ] Test end-to-end query flow

### Option B (Distributed)

- [ ] Verify all VMs have 12GB+ RAM
- [ ] Create deployment script
- [ ] Deploy Ollama to all 21 validators
- [ ] Pull model on each VM
- [ ] Test local inference on each
- [ ] Deploy ollama_client.py (local mode)
- [ ] Update AI dev monitoring

---

## 🚨 Troubleshooting

### Ollama won't start

```bash
# Check logs
sudo journalctl -u ollama -f

# Common fix: insufficient RAM
free -h

# Restart
sudo systemctl restart ollama
```

### Can't connect from other VMs

```bash
# Check firewall
sudo iptables -L | grep 11434

# Check Ollama is listening on 0.0.0.0
sudo netstat -tulpn | grep 11434

# Test connection
curl http://64.181.215.19:11434/api/version
```

### Model download fails

```bash
# Manual download
ollama pull llama3.1:8b

# Or use smaller model
ollama pull phi3:mini
```

---

## 📈 Performance Benchmarks

**Llama 3.1 8B on Standard VM (4 cores, 8GB RAM):**
- Simple query: ~2-5 seconds
- Complex analysis: ~10-20 seconds
- Memory usage: ~6GB during inference
- Concurrent requests: 2-3 max

**Comparison to Claude API:**
- Claude API: 200-500ms (network latency)
- Ollama local: 2-5 seconds (processing time)
- Ollama remote: 2-5 seconds + 10-50ms network

**Winner:** Claude API for speed, Ollama for cost

---

## 💡 Best Practices

1. **Use Tiered Approach:**
   - Ollama for frequent, simple queries (network status, basic health)
   - Claude API for complex decisions (restarts, escalations, audit)

2. **Cache Ollama Responses:**
   - Network status changes slowly, cache for 30-60 seconds
   - Reduces load on Ollama

3. **Monitor Resource Usage:**
   - Watch Ollama memory usage: `ps aux | grep ollama`
   - If RAM is tight, use smaller model (phi3:mini)

4. **Keep Models Updated:**
   - Update monthly: `ollama pull llama3.1:8b`

---

## 🎯 Next Steps

**Immediate:**
1. Test Gizzi VM resources: `ssh ... "free -h && nproc"`
2. Choose architecture (A or B)
3. Deploy Ollama
4. Test basic queries

**Short Term:**
5. Create Gizzi API server
6. Deploy ollama_client.py to validators
7. Integrate with AI dev monitoring

**Long Term:**
8. Monitor cost savings vs Claude API
9. Consider upgrading to llama3.1:70b if resources allow
10. Build custom fine-tuned model for Ëtrid-specific tasks

---

**Status:** Ready to deploy
**Recommended:** Option A (Centralized on Gizzi)
**Cost:** $0/month (open source)
**Time to Deploy:** 15-20 minutes
