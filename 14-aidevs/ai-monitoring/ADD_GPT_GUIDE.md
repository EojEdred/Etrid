# Adding GPT-4 to Your AI Monitoring System
## 5-Minute Setup Guide

---

## 🎯 What You'll Get

**3-Tier AI System:**
- **Tier 1: Ollama** (Free) - Simple queries, health checks
- **Tier 2: GPT-4 Turbo** ($0.01/1K tokens) - Code analysis, general queries ✨ NEW
- **Tier 3: Claude Sonnet 4** ($0.003/1K tokens) - Critical decisions, governance

---

## 💰 Cost Comparison

| Model | Cost per Query | Best For |
|-------|---------------|----------|
| **Ollama** | $0 | Health checks, simple queries |
| **GPT-4 Turbo** | ~$0.002 | Code analysis, debugging |
| **Claude Sonnet 4** | ~$0.001 | Critical decisions (cheaper!) |

**Expected Monthly Cost:** ~$35-45 (vs $30-40 without GPT)

**Why Add GPT?** Better at code analysis and has broader knowledge base for developer queries.

---

## 🚀 Setup Steps (5 minutes)

### **Step 1: Get OpenAI API Key** (2 min)

1. Go to: https://platform.openai.com/api-keys
2. Sign in (or create account)
3. Click "Create new secret key"
4. Name it: "Etrid Validator Monitoring"
5. Copy the key (starts with `sk-proj-...`)

**Add Payment Method:**
- Go to: https://platform.openai.com/account/billing
- Add credit card
- Set billing alert at $50

---

### **Step 2: Deploy AI Router** (3 min)

```bash
# Copy ai_router.py to Gizzi VM
scp -i ~/.ssh/gizzi-validator \
  /Users/macbook/Desktop/etrid/ai-monitoring/ai_router.py \
  ubuntu@64.181.215.19:/tmp/

# SSH to Gizzi
ssh -i ~/.ssh/gizzi-validator ubuntu@64.181.215.19

# Move to ai-monitoring directory
sudo mv /tmp/ai_router.py /opt/ai-monitoring/
sudo chown ubuntu:ubuntu /opt/ai-monitoring/ai_router.py
sudo chmod +x /opt/ai-monitoring/ai_router.py

# Install OpenAI SDK
pip3 install openai

# Test it
cd /opt/ai-monitoring
export OPENAI_API_KEY='your-key-here'
python3 ai_router.py
```

---

### **Step 3: Add API Key to Service**

```bash
# Edit systemd service
sudo nano /etc/systemd/system/ai-dev-monitoring.service

# Add this line after ANTHROPIC_API_KEY:
Environment="OPENAI_API_KEY=sk-proj-YOUR_KEY_HERE"

# Save and restart
sudo systemctl daemon-reload
sudo systemctl restart ai-dev-monitoring
```

---

### **Step 4: Update AI Dev Workers** (Optional)

If you want AI devs to use the router:

```bash
cd /opt/ai-monitoring

# Backup current file
sudo cp ai_dev_workers.py ai_dev_workers.py.backup

# Edit ai_dev_workers.py
sudo nano ai_dev_workers.py
```

Add at the top:
```python
from ai_router import AIRouter
```

In `AIDevWorker.__init__`:
```python
def __init__(self, aidev_id, anthropic_api_key, openai_api_key, monitor, optimized=True):
    self.aidev_id = aidev_id
    self.monitor = monitor
    self.router = AIRouter(
        openai_api_key=openai_api_key,
        anthropic_api_key=anthropic_api_key
    )
```

In `analyze_with_claude` (rename to `analyze_with_ai`):
```python
def analyze_with_ai(self, validator_statuses):
    """Use AI router to select best model"""
    context = {
        'is_critical': any(v['health'] == 'critical' for v in validator_statuses)
    }

    result = self.router.query(
        f"Analyze these validators: {validator_statuses}",
        context=context
    )

    print(f"[{self.aidev_id}] Used {result['model']}, cost ${result['cost']:.4f}")
    return result
```

---

## ✅ Verify It's Working

### **Test 1: Simple Query (Should Use Ollama)**

```bash
cd /opt/ai-monitoring
python3 ai_router.py

# Or test directly:
python3 << 'EOF'
from ai_router import AIRouter
router = AIRouter()
result = router.query("Is validator 6 healthy?")
print(f"Model: {result['model']}")
print(f"Cost: ${result['cost']}")
print(f"Response: {result['response'][:100]}")
EOF
```

Expected: `Model: ollama-llama3.1:8b`, `Cost: $0.0`

---

### **Test 2: Code Query (Should Use GPT-4 Turbo)**

```python
python3 << 'EOF'
from ai_router import AIRouter
router = AIRouter()
result = router.query("Analyze this Rust code for bugs: fn add(a: i32, b: i32) -> i32 { a + b }")
print(f"Model: {result['model']}")
print(f"Cost: ${result['cost']}")
print(f"Response: {result['response'][:100]}")
EOF
```

Expected: `Model: gpt-4-turbo-preview`, `Cost: ~$0.002`

---

### **Test 3: Critical Decision (Should Use Claude)**

```python
python3 << 'EOF'
from ai_router import AIRouter
router = AIRouter()
result = router.query(
    "Should we restart validator 6 with 2 peers and 150 block lag?",
    context={"is_critical": True}
)
print(f"Model: {result['model']}")
print(f"Cost: ${result['cost']}")
print(f"Response: {result['response'][:100]}")
EOF
```

Expected: `Model: claude-sonnet-4`, `Cost: ~$0.001`

---

## 📊 Routing Logic

The router automatically selects the best model:

```
Query Type               → Selected Model
─────────────────────────────────────────
"health", "status"      → Ollama (free)
"code", "bug", "rust"   → GPT-4 Turbo ($)
"should", "recommend"   → Claude ($$)
is_critical=True        → Claude ($$)
Default                 → Ollama (free)
```

---

## 💡 Usage Examples

### **Validator Self-Diagnosis with Multi-AI**

```python
from ai_router import AIRouter

router = AIRouter()

# Quick check (Ollama - free)
health = router.query("Quick status check: 8 peers, block 12345, finalized 12340")

# Code issue? (GPT-4 Turbo - better at code)
code_issue = router.query("This Rust error: cannot borrow as mutable. How to fix?")

# Critical decision? (Claude - most careful)
decision = router.query(
    "Should we restart? Risk of losing 100 blocks if we restart now.",
    context={"is_critical": True}
)
```

### **Compare All Models**

```python
from ai_router import AIRouter

router = AIRouter()

# Query all models and compare
comparison = router.compare_models("What are signs of validator issues?")

print(f"Ollama says: {comparison['results']['ollama']['response'][:100]}")
print(f"GPT-4 says: {comparison['results']['gpt4_turbo']['response'][:100]}")
print(f"Claude says: {comparison['results']['claude']['response'][:100]}")
print(f"Total cost: ${comparison['total_cost']:.4f}")
```

---

## 🎯 When to Use Each Model

### **Use Ollama When:**
- ✅ Simple health checks
- ✅ "Is validator X online?"
- ✅ Quick status queries
- ✅ Frequent monitoring (every 5 min)

### **Use GPT-4 Turbo When:**
- ✅ Code analysis: "Why does this compile fail?"
- ✅ Debugging: "Analyze this error message"
- ✅ Developer questions: "How does consensus work?"
- ✅ Math/calculations

### **Use Claude When:**
- ✅ Critical decisions: "Should we restart?"
- ✅ Governance decisions
- ✅ Strategic planning
- ✅ Ethical questions

---

## 📈 Expected Costs

**Without GPT (Current):**
- Ollama: 90% of queries → $0
- Claude: 10% of queries → $30-40/month
- **Total: $30-40/month**

**With GPT (New):**
- Ollama: 70% of queries → $0
- GPT-4 Turbo: 20% of queries → $10-15/month
- Claude: 10% of queries → $25-30/month
- **Total: $35-45/month**

**Benefit:** Better code analysis, broader knowledge, still affordable!

---

## 🚨 Troubleshooting

### **Error: "openai not installed"**
```bash
pip3 install openai
```

### **Error: "No OpenAI API key"**
```bash
export OPENAI_API_KEY='sk-proj-...'
# Or add to systemd service file
```

### **GPT queries always failing?**
Check API key is valid:
```bash
curl https://api.openai.com/v1/models \
  -H "Authorization: Bearer $OPENAI_API_KEY"
```

### **Want to force a specific model?**
```python
result = router.query("your query", context={"force_model": AIModel.GPT4_TURBO})
```

---

## 🎉 What You Now Have

✅ **3-Tier AI System**
- Ollama for free simple queries
- GPT-4 Turbo for code/dev work
- Claude for critical decisions

✅ **Intelligent Routing**
- Automatically selects best model
- Minimizes costs
- Maximizes capabilities

✅ **Flexible & Extensible**
- Add more models anytime
- Customize routing logic
- Force specific models when needed

---

## 📞 Quick Reference

| Task | Command | Expected Cost |
|------|---------|--------------|
| **Get API key** | https://platform.openai.com/api-keys | Free |
| **Install SDK** | `pip3 install openai` | Free |
| **Test router** | `python3 ai_router.py` | ~$0.01 |
| **Check costs** | https://platform.openai.com/usage | - |
| **Force model** | `context={"force_model": AIModel.GPT4_TURBO}` | Varies |

---

## 🔒 Security Note

**IMPORTANT:** Keep your API keys secure!

```bash
# Good: Environment variables
export OPENAI_API_KEY='sk-proj-...'

# Good: Systemd service (not in code)
Environment="OPENAI_API_KEY=sk-proj-..."

# Bad: Hardcoded in code ❌
api_key = "sk-proj-..."  # Don't do this!

# Bad: Committed to git ❌
# .env file in repository # Don't do this!
```

---

**Setup time:** 5 minutes
**Additional cost:** ~$5-10/month
**Benefit:** Better code analysis, broader knowledge
**Ready to deploy!** 🚀
