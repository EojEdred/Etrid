# 🆓 Run AI Devs Social Automation for FREE (or < $10/month)

**Goal:** Deploy fully autonomous AI Devs with minimal or zero monthly cost.

---

## 💰 Cost Breakdown & Free Alternatives

### Current Plan Costs
| Service | Original Cost | **Free Alternative** | Savings |
|---------|---------------|---------------------|---------|
| Twitter API | $0 | ✅ FREE (included) | $0 |
| Claude API | $100-150 | ❌ → Use local LLM | **-$100-150** |
| Blockchain RPC | $0 | ✅ FREE (public RPC) | $0 |
| Hosting/Server | $0-50 | ✅ FREE (multiple options) | **-$50** |
| Database | $0-20 | ✅ FREE (SQLite/Redis) | **-$20** |
| **TOTAL** | **$100-220** | **$0-10** | **-$190-210** |

---

## 🎯 The Free/Cheap Setup

### **Total Cost: $0-10/month** 🎉

**Free Tier Strategy:**
1. ✅ Local LLM instead of Claude API → **$0** (was $100-150)
2. ✅ Oracle Cloud free tier hosting → **$0** (was $50)
3. ✅ Public blockchain RPC → **$0** (already free)
4. ✅ Twitter API free tier → **$0** (already free)

**Alternative: Raspberry Pi (one-time cost)**
- Buy Raspberry Pi 4 (4GB): **$35-55 one-time**
- Run everything locally at home
- Monthly cost: **$0** (just electricity ~$2/month)

---

## 🔧 Implementation: Free Setup

### Option 1: 100% FREE - Oracle Cloud Free Tier ⭐ **RECOMMENDED**

**What you get (FOREVER FREE):**
- 2 AMD VMs (1GB RAM each) OR 4 ARM VMs (24GB RAM total)
- 200GB storage
- 10TB egress/month
- **No credit card required after free tier**

**Setup Time:** 30 minutes

#### Step 1: Create Oracle Cloud Account
```bash
# 1. Go to: https://www.oracle.com/cloud/free/
# 2. Sign up (free, no credit card required)
# 3. Create VM instance (Ubuntu 22.04)
#    - Shape: VM.Standard.A1.Flex (ARM, 4 OCPUs, 24GB RAM - FREE)
#    - Or: VM.Standard.E2.1.Micro (x86, 1GB RAM - FREE)
```

#### Step 2: Install Dependencies on Oracle VM
```bash
# SSH into your VM
ssh ubuntu@<your-vm-ip>

# Update system
sudo apt update && sudo apt upgrade -y

# Install Python 3.11
sudo apt install -y python3.11 python3.11-venv python3-pip git

# Install Ollama (for local LLM)
curl -fsSL https://ollama.com/install.sh | sh

# Pull a good model (Llama 3.1 8B or Mistral)
ollama pull llama3.1:8b
# Or smaller model if limited RAM: ollama pull phi3:mini
```

#### Step 3: Clone and Setup AI Devs
```bash
# Clone your repo (or upload files)
cd ~
git clone https://github.com/yourusername/etrid.git
cd etrid/ai-devs/social

# Create virtual environment
python3.11 -m venv venv
source venv/bin/activate

# Install dependencies
pip install -r requirements.txt
```

#### Step 4: Configure for Local LLM
```bash
# Copy and edit .env
cp .env.example .env
nano .env
```

**Edit `.env` for FREE setup:**
```bash
# Blockchain (use public RPC)
BLOCKCHAIN_WS_URL=wss://rpc.etrid.network

# Twitter (get free API keys from developer.twitter.com)
TWITTER_API_KEY=your_free_twitter_key
TWITTER_API_SECRET=your_free_twitter_secret
TWITTER_ACCESS_TOKEN=your_free_twitter_token
TWITTER_ACCESS_SECRET=your_free_twitter_secret
TWITTER_USERNAME=EtridAI_Devs

# Claude API - LEAVE EMPTY (we'll use local LLM)
ANTHROPIC_API_KEY=

# Use Ollama instead
USE_LOCAL_LLM=true
LOCAL_LLM_URL=http://localhost:11434
LOCAL_LLM_MODEL=llama3.1:8b

# Other settings
DRY_RUN=false
DEBUG=true
ENVIRONMENT=production
```

#### Step 5: Modify ContentGenerator to Use Local LLM

Create a wrapper for Ollama in `content/generator.py`:

```python
# Add this to content/generator.py after the imports:

import requests  # Add to imports at top

class ContentGenerator:
    def __init__(self, api_key: Optional[str] = None):
        """Initialize content generator (Claude or local LLM)"""
        self.use_local_llm = os.getenv('USE_LOCAL_LLM', 'false').lower() == 'true'

        if self.use_local_llm:
            # Use Ollama (local LLM)
            self.local_llm_url = os.getenv('LOCAL_LLM_URL', 'http://localhost:11434')
            self.local_llm_model = os.getenv('LOCAL_LLM_MODEL', 'llama3.1:8b')
            self.client = None
            print(f"✅ Using local LLM: {self.local_llm_model}")
        else:
            # Use Claude API
            self.api_key = api_key or os.getenv('ANTHROPIC_API_KEY')
            if not self.api_key:
                print("⚠️  No ANTHROPIC_API_KEY found - using mock content")
                self.client = None
            else:
                self.client = AsyncAnthropic(api_key=self.api_key)

        self.personas = self._load_personas()

    async def _generate_with_ollama(self, prompt: str, system_prompt: str, max_tokens: int = 300) -> str:
        """Generate content using local Ollama LLM"""
        try:
            response = requests.post(
                f"{self.local_llm_url}/api/generate",
                json={
                    "model": self.local_llm_model,
                    "prompt": f"{system_prompt}\n\nUser: {prompt}\n\nAssistant:",
                    "stream": False,
                    "options": {
                        "num_predict": max_tokens,
                        "temperature": 0.7,
                    }
                },
                timeout=30
            )

            if response.status_code == 200:
                result = response.json()
                return result.get('response', '').strip()
            else:
                print(f"❌ Ollama error: {response.status_code}")
                return self._generate_mock_content(prompt, "oracle-dev")

        except Exception as e:
            print(f"❌ Local LLM error: {e}")
            return self._generate_mock_content(prompt, "oracle-dev")

    async def generate(
        self,
        prompt: str,
        dev: str = "oracle-dev",
        max_tokens: int = 300,
        temperature: float = 0.7,
    ) -> str:
        """Generate content using Claude API or local LLM"""

        # Build system prompt
        persona = self.personas.get(dev, self.personas["oracle-dev"])
        system_prompt = f"""You are {persona['name']}, one of Ëtrid's AI developers.
Voice: {persona['voice']}
Tone: {persona['tone']}
Style: {persona['style']}
Keep tweets under 250 characters. Be factual and accurate."""

        # Use local LLM if configured
        if self.use_local_llm:
            return await self._generate_with_ollama(prompt, system_prompt, max_tokens)

        # Otherwise use Claude or mock
        if not self.client:
            return self._generate_mock_content(prompt, dev)

        # ... rest of existing Claude code ...
```

#### Step 6: Start Services with Systemd
```bash
# Create systemd service
sudo nano /etc/systemd/system/etrid-social.service
```

```ini
[Unit]
Description=Etrid AI Devs Social Automation
After=network.target ollama.service

[Service]
Type=simple
User=ubuntu
WorkingDirectory=/home/ubuntu/etrid/ai-devs/social
Environment="PATH=/home/ubuntu/etrid/ai-devs/social/venv/bin"
ExecStart=/home/ubuntu/etrid/ai-devs/social/venv/bin/python workflows/twitter_mentions.py --mode stream
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
# Create cron jobs for scheduled posts
crontab -e
```

```cron
# Daily stats at 12:00 UTC
0 12 * * * cd /home/ubuntu/etrid/ai-devs/social && /home/ubuntu/etrid/ai-devs/social/venv/bin/python workflows/daily_stats.py

# Weekly summary Sunday 18:00 UTC
0 18 * * 0 cd /home/ubuntu/etrid/ai-devs/social && /home/ubuntu/etrid/ai-devs/social/venv/bin/python workflows/weekly_summary.py
```

```bash
# Enable and start
sudo systemctl enable etrid-social
sudo systemctl start etrid-social

# Check status
sudo systemctl status etrid-social
```

**🎉 DONE! Total cost: $0/month forever**

---

## Option 2: Raspberry Pi at Home ($35 one-time + $2/month electricity)

**Perfect if you want to:**
- Run everything at home
- Own the hardware
- Learn about self-hosting

### Hardware Needed (One-Time Cost: ~$35-80)
- **Raspberry Pi 4 (4GB)**: $35-55
- **microSD card (32GB)**: $8-12
- **Power supply**: $8 (or use USB-C phone charger)
- **Case** (optional): $5-10
- **Total**: $35-80 one-time

### Setup (30 minutes)

#### Step 1: Install Raspberry Pi OS
```bash
# 1. Download Raspberry Pi Imager
#    https://www.raspberrypi.com/software/

# 2. Flash Raspberry Pi OS Lite (64-bit) to SD card

# 3. Enable SSH before first boot:
#    Create empty file named 'ssh' on boot partition

# 4. Boot Pi, SSH in:
ssh pi@raspberrypi.local  # Password: raspberry

# 5. Update system
sudo apt update && sudo apt upgrade -y
```

#### Step 2: Install Dependencies
```bash
# Install Python
sudo apt install -y python3.11 python3.11-venv python3-pip git

# Install Ollama (ARM version)
curl -fsSL https://ollama.com/install.sh | sh

# Pull model (use smaller model for 4GB RAM)
ollama pull phi3:mini  # 3.8GB, fast, good quality
# Or: ollama pull mistral:7b-instruct-v0.2-q4_0  # 4.1GB
```

#### Step 3: Setup AI Devs
```bash
# Upload your code or clone repo
cd ~
# ... upload files via scp or git clone ...

cd etrid/ai-devs/social
python3.11 -m venv venv
source venv/bin/activate
pip install -r requirements.txt

# Configure .env (same as Oracle Cloud setup above)
cp .env.example .env
nano .env
```

#### Step 4: Auto-Start on Boot
```bash
# Add to crontab
crontab -e
```

```cron
# Start mention monitoring on boot
@reboot sleep 60 && cd /home/pi/etrid/ai-devs/social && /home/pi/etrid/ai-devs/social/venv/bin/python workflows/twitter_mentions.py --mode stream >> /home/pi/logs/mentions.log 2>&1

# Daily stats at 12:00 UTC
0 12 * * * cd /home/pi/etrid/ai-devs/social && /home/pi/etrid/ai-devs/social/venv/bin/python workflows/daily_stats.py >> /home/pi/logs/daily.log 2>&1

# Weekly summary Sunday 18:00 UTC
0 18 * * 0 cd /home/pi/etrid/ai-devs/social && /home/pi/etrid/ai-devs/social/venv/bin/python workflows/weekly_summary.py >> /home/pi/logs/weekly.log 2>&1
```

**🎉 DONE! Cost: $0/month (just ~$2 electricity)**

---

## Option 3: Your Existing Mac/PC (100% Free)

**If your computer is on 24/7 or most of the time:**

### Setup (15 minutes)

```bash
# Already done from previous setup!
cd /Users/macbook/Desktop/etrid/ai-devs/social

# Install Ollama for Mac
brew install ollama
# Or download from: https://ollama.com/download

# Start Ollama
ollama serve

# In another terminal, pull model
ollama pull llama3.1:8b  # Mac can handle larger models

# Configure .env
cp .env.example .env
nano .env
# Set USE_LOCAL_LLM=true, LOCAL_LLM_URL=http://localhost:11434

# Test
python workflows/daily_stats.py --dry-run

# Run continuously
python workflows/twitter_mentions.py --mode stream &
```

**Auto-start on Mac boot:**
```bash
# Create LaunchAgent
nano ~/Library/LaunchAgents/com.etrid.social.plist
```

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.etrid.social</string>
    <key>ProgramArguments</key>
    <array>
        <string>/Users/macbook/Desktop/etrid/ai-devs/social/venv/bin/python</string>
        <string>/Users/macbook/Desktop/etrid/ai-devs/social/workflows/twitter_mentions.py</string>
        <string>--mode</string>
        <string>stream</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/etrid-social.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/etrid-social-error.log</string>
</dict>
</plist>
```

```bash
# Load it
launchctl load ~/Library/LaunchAgents/com.etrid.social.plist
```

**🎉 DONE! Cost: $0/month**

---

## Option 4: Almost-Free Cloud Hosting ($4-6/month)

**If you want dedicated VPS but don't want Oracle Cloud:**

### Best Budget Options:

| Provider | Cost | Specs | Notes |
|----------|------|-------|-------|
| **Hetzner Cloud** | €3.79/month (~$4) | 2GB RAM, 1 vCPU | Germany/Finland |
| **Vultr** | $6/month | 1GB RAM, 1 vCPU | USA |
| **DigitalOcean** | $6/month | 1GB RAM, 1 vCPU | USA |
| **Linode** | $5/month | 1GB RAM, 1 vCPU | USA |
| **Contabo** | €4/month (~$4.50) | 4GB RAM, 2 vCPU | Germany |

**Setup:** Same as Oracle Cloud above

---

## 🤖 Local LLM Performance Comparison

### Model Options (Ranked by Quality vs. Speed)

| Model | Size | RAM | Quality | Speed | Best For |
|-------|------|-----|---------|-------|----------|
| **llama3.1:8b** | 4.7GB | 8GB+ | ⭐⭐⭐⭐⭐ | Medium | Best overall |
| **mistral:7b** | 4.1GB | 6GB+ | ⭐⭐⭐⭐ | Fast | Good balance |
| **phi3:mini** | 2.3GB | 4GB+ | ⭐⭐⭐⭐ | Very Fast | Raspberry Pi |
| **gemma2:2b** | 1.6GB | 3GB+ | ⭐⭐⭐ | Very Fast | Minimal RAM |
| **tinyllama** | 0.6GB | 2GB+ | ⭐⭐ | Ultra Fast | Testing only |

**Recommendation:**
- **Mac/Desktop**: llama3.1:8b (best quality)
- **Oracle Cloud (24GB)**: llama3.1:8b or mixtral
- **Raspberry Pi 4 (4GB)**: phi3:mini
- **Low RAM (<4GB)**: gemma2:2b

### Content Quality: Local LLM vs. Claude

**Claude API:**
- Quality: ⭐⭐⭐⭐⭐ (10/10)
- Cost: $100-150/month
- Speed: Fast (~1-2 sec)

**Llama 3.1 8B (local):**
- Quality: ⭐⭐⭐⭐ (8/10)
- Cost: $0/month
- Speed: Medium (~3-5 sec on decent hardware)

**Phi3 Mini (local, Raspberry Pi):**
- Quality: ⭐⭐⭐½ (7/10)
- Cost: $0/month
- Speed: Fast (~2-3 sec on Pi 4)

**Verdict:** Local LLMs are 80-90% as good as Claude for social media posts. Totally acceptable for automated tweets!

---

## 💡 Hybrid Approach: Best of Both Worlds

**Use free local LLM PLUS occasional Claude calls:**

```bash
# In .env:
USE_LOCAL_LLM=true
LOCAL_LLM_URL=http://localhost:11434

# But also set Claude key for special cases:
ANTHROPIC_API_KEY=your_key_here
CLAUDE_FALLBACK=true  # Use Claude if local LLM fails
CLAUDE_FOR_CRITICAL=true  # Use Claude for critical alerts only
```

**Modify `generator.py`:**
```python
async def generate(self, prompt: str, dev: str, critical: bool = False):
    # Use Claude for critical content only
    if critical and self.api_key:
        return await self._generate_with_claude(prompt, dev)

    # Otherwise use free local LLM
    return await self._generate_with_ollama(prompt, dev)
```

**Result:**
- 95% of posts use free local LLM
- 5% of critical posts use Claude
- **Cost: ~$5-10/month** instead of $100-150

---

## 📊 Cost Comparison: All Options

| Setup | Hardware | Monthly Cost | One-Time Cost | Quality |
|-------|----------|--------------|---------------|---------|
| **Oracle Cloud Free** | None | **$0** | $0 | ⭐⭐⭐⭐ |
| **Raspberry Pi** | RPi 4 | **$2** (electricity) | $35-80 | ⭐⭐⭐⭐ |
| **Your Mac/PC** | Existing | **$0** | $0 | ⭐⭐⭐⭐⭐ |
| **Budget VPS** | None | **$4-6** | $0 | ⭐⭐⭐⭐ |
| **Hybrid (local + occasional Claude)** | None | **$5-10** | $0 | ⭐⭐⭐⭐½ |
| **Original (all Claude)** | None | **$100-150** | $0 | ⭐⭐⭐⭐⭐ |

---

## ✅ Quick Decision Guide

**Choose Oracle Cloud Free if:**
- ✅ You want $0/month forever
- ✅ You want it "just works" in the cloud
- ✅ You don't mind 80-90% Claude quality

**Choose Raspberry Pi if:**
- ✅ You want to own the hardware
- ✅ You like tinkering
- ✅ You have reliable home internet
- ✅ $35-80 one-time is okay

**Choose Your Existing Mac/PC if:**
- ✅ Your computer is on 24/7 (or most of the time)
- ✅ You want to test before committing
- ✅ You want the highest local LLM quality (8B or larger models)

**Choose Budget VPS ($4-6) if:**
- ✅ You want guaranteed uptime
- ✅ You don't trust Oracle Cloud free tier
- ✅ $4-6/month is acceptable

**Choose Hybrid (local + occasional Claude) if:**
- ✅ You want near-Claude quality for critical posts
- ✅ $5-10/month is okay
- ✅ You want the best cost/quality balance

---

## 🚀 Recommended FREE Setup (Step-by-Step)

### **BEST FOR MOST PEOPLE: Oracle Cloud Free Tier**

1. **Create Oracle Cloud account** (15 min)
   - Go to oracle.com/cloud/free
   - Sign up (free, no credit card after trial)

2. **Create VM** (10 min)
   - Ubuntu 22.04
   - VM.Standard.A1.Flex (ARM, 4 cores, 24GB RAM)
   - FREE FOREVER

3. **Install everything** (20 min)
   ```bash
   # SSH in
   ssh ubuntu@<your-vm-ip>

   # Install Ollama + Python
   curl -fsSL https://ollama.com/install.sh | sh
   sudo apt install -y python3.11 python3.11-venv python3-pip git

   # Pull model
   ollama pull llama3.1:8b

   # Setup AI Devs (upload your code)
   cd ~/etrid/ai-devs/social
   python3.11 -m venv venv
   source venv/bin/activate
   pip install -r requirements.txt

   # Configure
   cp .env.example .env
   nano .env
   # Set: USE_LOCAL_LLM=true, add Twitter keys
   ```

4. **Add local LLM support** (15 min)
   - Modify `content/generator.py` with Ollama integration (code above)

5. **Start services** (5 min)
   ```bash
   # Test
   python workflows/daily_stats.py --dry-run

   # Setup systemd + cron (code above)
   sudo systemctl enable etrid-social
   sudo systemctl start etrid-social
   ```

**Total Time:** 65 minutes
**Total Cost:** **$0/month FOREVER** ✨

---

## 🎯 Bottom Line

**Can you run this for free?**
✅ **YES - Multiple ways:**

1. **$0/month**: Oracle Cloud + local LLM (llama3.1)
2. **$2/month**: Raspberry Pi + local LLM (phi3)
3. **$0/month**: Your Mac/PC + local LLM (llama3.1)
4. **$4-6/month**: Budget VPS + local LLM
5. **$5-10/month**: Hybrid (mostly local, critical posts use Claude)

**Recommended for most:** Oracle Cloud Free Tier
- $0/month forever
- Good specs (24GB RAM, 4 ARM cores)
- 80-90% of Claude quality
- Always-on, cloud-hosted
- No hardware needed

**Your Mac is Already Ready:**
You can literally start RIGHT NOW for $0:
```bash
cd /Users/macbook/Desktop/etrid/ai-devs/social
brew install ollama
ollama pull llama3.1:8b
# Modify generator.py to use Ollama
python workflows/daily_stats.py --dry-run
```

**The AI Devs can be autonomous for FREE.** 🎉

---

Questions? Check the model size that fits your setup and deploy!
