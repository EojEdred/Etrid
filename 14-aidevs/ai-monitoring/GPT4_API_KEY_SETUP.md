# GPT-4 API Key Setup Guide
## How to Configure OpenAI GPT-4 for AI Monitoring

**Purpose:** Enable the GPT-4 tier of Gizzi's distributed consciousness for technical analysis and code diagnostics.

---

## Step 1: Get Your OpenAI API Key

### Option A: Use Existing Key
If you already have an OpenAI API key, skip to Step 2.

### Option B: Create New API Key

1. Go to: https://platform.openai.com/api-keys
2. Sign in with your OpenAI account (or create one)
3. Click **"+ Create new secret key"**
4. Name it: `etrid-ai-monitoring`
5. Copy the key (starts with `sk-proj-...` or `sk-...`)
6. **IMPORTANT:** Save it immediately - you can't view it again!

---

## Step 2: Add Credits to Your OpenAI Account

GPT-4 requires prepaid credits:

1. Go to: https://platform.openai.com/settings/organization/billing/overview
2. Click **"Add to credit balance"**
3. Recommended: Add **$20-50** (lasts 1-2 months for monitoring)
4. Set up auto-recharge for uninterrupted service

**Cost Estimate:**
- GPT-4 Turbo: ~$0.01 per 1K input tokens, ~$0.03 per 1K output tokens
- Expected usage: $10-15/month for 21 validators
- Each AI dev worker uses ~500-1000 tokens per analysis

---

## Step 3: Configure the API Key

### Method 1: Using .api_key File (Recommended)

```bash
# On your local machine
cd ~/Desktop/etrid/ai-monitoring

# Create .api_key file with GPT-4 key
echo "YOUR_OPENAI_API_KEY_HERE" > .openai_key

# Secure the file
chmod 600 .openai_key

# Update ai_dev_workers.py to use it
# (Already configured to read from .openai_key)
```

### Method 2: Environment Variable

```bash
# Add to ~/.zshrc or ~/.bashrc
export OPENAI_API_KEY="sk-proj-YOUR_KEY_HERE"

# Reload shell
source ~/.zshrc
```

### Method 3: Direct in Python Code

```python
# In ai_router.py (NOT RECOMMENDED - use .openai_key instead)
openai.api_key = "sk-proj-YOUR_KEY_HERE"
```

---

## Step 4: Update ai_router.py to Use OpenAI Key

The `ai_router.py` already supports GPT-4, but ensure it reads the key:

```python
# ai_router.py should have:
import os
import openai

# Read from .openai_key file
if os.path.exists('.openai_key'):
    with open('.openai_key', 'r') as f:
        openai.api_key = f.read().strip()
elif os.getenv('OPENAI_API_KEY'):
    openai.api_key = os.getenv('OPENAI_API_KEY')
else:
    print("⚠️  WARNING: No OpenAI API key found. GPT-4 tier will fail.")
```

---

## Step 5: Test GPT-4 Integration

```bash
cd ~/Desktop/etrid/ai-monitoring

# Quick test
python3 << 'EOF'
import os
import openai

# Load API key
if os.path.exists('.openai_key'):
    with open('.openai_key', 'r') as f:
        openai.api_key = f.read().strip()

# Test GPT-4 access
try:
    response = openai.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=[
            {"role": "system", "content": "You are a Substrate validator monitoring assistant."},
            {"role": "user", "content": "Validator has 1 peer. What's wrong?"}
        ],
        max_tokens=100
    )
    print("✅ GPT-4 connection successful!")
    print(f"Response: {response.choices[0].message.content}")
except Exception as e:
    print(f"❌ GPT-4 test failed: {e}")
EOF
```

---

## Step 6: Deploy to Gizzi VM

```bash
# Copy .openai_key to Gizzi VM
scp .openai_key gizzi@64.181.215.19:/opt/ai-monitoring/

# SSH into Gizzi VM
ssh gizzi@64.181.215.19

# Secure the file
cd /opt/ai-monitoring
chmod 600 .openai_key

# Verify
ls -l .openai_key
# Should show: -rw------- 1 gizzi gizzi 56 Oct 31 14:00 .openai_key
```

---

## API Key File Locations

**Local Development:**
```
~/Desktop/etrid/ai-monitoring/.openai_key
```

**Deployed on Gizzi VM:**
```
/opt/ai-monitoring/.openai_key
```

**What Each File Contains:**
- `.openai_key` → OpenAI GPT-4 API key (starts with `sk-proj-` or `sk-`)

---

## Claude API Key (Already Configured)

You mentioned Claude API key was already given. Verify it's in place:

```bash
# Check for Claude key
cat ~/Desktop/etrid/ai-monitoring/.api_key

# Should contain your Anthropic API key (starts with sk-ant-)
```

If missing, create it:
```bash
echo "YOUR_ANTHROPIC_API_KEY" > ~/Desktop/etrid/ai-monitoring/.api_key
chmod 600 ~/Desktop/etrid/ai-monitoring/.api_key
```

---

## Complete API Key Setup Checklist

- [ ] OpenAI API key obtained from platform.openai.com
- [ ] Added $20-50 credits to OpenAI account
- [ ] Created `.openai_key` file with GPT-4 key
- [ ] Secured file with `chmod 600`
- [ ] Tested GPT-4 connection locally
- [ ] Verified Claude API key in `.api_key` file
- [ ] Deployed both keys to Gizzi VM
- [ ] Confirmed ai_router.py reads both keys

---

## Troubleshooting

### Error: "You exceeded your current quota"
**Solution:** Add more credits to your OpenAI account
- Go to: https://platform.openai.com/settings/organization/billing

### Error: "Invalid API key"
**Solution:**
1. Verify key starts with `sk-proj-` or `sk-`
2. Check for extra spaces/newlines in `.openai_key`
3. Regenerate key if needed

### Error: "Rate limit exceeded"
**Solution:**
1. OpenAI has rate limits for new accounts
2. Wait a few minutes or upgrade to Tier 2 ($50+ spent)
3. Fallback to Ollama tier automatically

### GPT-4 Not Being Used
**Solution:**
1. Check `ai_router.py` is loading the key
2. Verify priority: `complexity < 5` uses Ollama, `>= 5` uses GPT-4
3. Test with: `ai_router.route_to_ai(prompt, complexity=8)`

---

## Cost Monitoring

### OpenAI Dashboard
https://platform.openai.com/usage

**What to Watch:**
- Daily token usage
- Cost per day
- Rate limits

**Expected Usage for 21 Validators:**
- ~500 API calls/day (one per 5-min cycle × 12 AI devs)
- ~50K tokens/day
- ~$0.50-1.00/day = $15-30/month

### Set Up Billing Alerts

1. Go to: https://platform.openai.com/settings/organization/billing
2. Set **soft limit:** $50/month
3. Set **hard limit:** $100/month (emergency stop)

---

## Architecture: 3-Tier AI System

```
User Question/Validator Issue
         ↓
    AI Router (ai_router.py)
         ↓
    ┌────┴────┬─────────┬─────────┐
    ↓         ↓         ↓         ↓
  Ollama    GPT-4    Claude    Multi-AI
 (Local)   (Cloud)  (Cloud)   (Consensus)
    ↓         ↓         ↓         ↓
 Reflex   Technical  Strategic  Critical
Response  Analysis   Decisions  Decisions
```

**When Each Tier is Used:**
- **Ollama (Complexity 1-4):** Quick checks, simple restarts
- **GPT-4 (Complexity 5-7):** Code analysis, root cause diagnosis
- **Claude (Complexity 8-10):** Governance, strategic planning
- **Multi-AI (Complexity 10):** Network-wide critical decisions

---

## Next Steps

1. ✅ OpenAI API key configured
2. ✅ Claude API key verified
3. → Deploy AI monitoring to Gizzi VM
4. → Test 3-tier AI routing
5. → Monitor costs and adjust

**Ready to deploy?** Proceed with: `CLAUDE_DEPLOYMENT_PROMPT.md`

---

*Last updated: October 31, 2025*
*Part of: ËTRID AI Monitoring System*
