# How to Use AI Prompts to Generate Lightning Apps

This guide shows you exactly how to use the AI prompts in `AI_ASSISTANT_PROMPTS.md` to instantly generate production-ready applications.

---

## 🎯 Overview

You have **4 ready-to-use prompts** that generate complete applications:

1. **Landing Page** - Next.js website (✅ Already built!)
2. **MetaMask Extension** - Browser extension for Lightning payments
3. **Mobile App** - React Native iOS/Android app
4. **Solana Wallet Adapter** - Integration with Phantom, Solflare, etc.

---

## 📋 Step-by-Step Instructions

### Prerequisites
- Access to Claude (claude.ai) or ChatGPT (chat.openai.com)
- A new chat window (don't use the same chat as this one)

---

## 🔥 Method 1: Generate MetaMask Extension (Most Popular)

### Step 1: Open the AI Prompts File
```bash
cat /home/user/Etrid/AI_ASSISTANT_PROMPTS.md
```

Or open it in your file browser/editor.

### Step 2: Find Prompt #2 (MetaMask Extension)

Scroll to the section titled:
```
## Prompt #2: MetaMask Browser Extension
```

### Step 3: Copy the ENTIRE Prompt

Copy everything from:
```
Create a production-ready browser extension...
```

All the way to the end of that prompt section (before "Prompt #3").

**Important:** Copy the WHOLE prompt - it's about 400 lines and includes:
- Context
- Requirements
- Technical Implementation
- All component specifications
- Deliverables

### Step 4: Open a New Claude/ChatGPT Chat

Go to:
- **Claude:** https://claude.ai (Recommended - better for code)
- **ChatGPT:** https://chat.openai.com

Click "New Chat" or "+" to start fresh.

### Step 5: Paste and Send

Just paste the entire prompt and hit Enter. That's it!

### Step 6: Wait for Generation (2-5 minutes)

The AI will generate:
- Complete file structure
- All TypeScript/React code
- manifest.json
- package.json
- Configuration files
- README with setup instructions

### Step 7: Copy the Code

The AI will provide files one by one or in a ZIP structure. Copy each file to your local machine:

```bash
# Create project directory
mkdir ~/metamask-lightning-extension
cd ~/metamask-lightning-extension

# Copy files from AI responses
# (The AI will show you the complete file structure)
```

### Step 8: Install and Test

```bash
npm install
npm run build

# Load in Chrome:
# 1. Go to chrome://extensions/
# 2. Enable "Developer mode"
# 3. Click "Load unpacked"
# 4. Select the dist/ folder
```

### Step 9: Start Using!

Your Lightning-enabled MetaMask extension is now installed and ready!

---

## 📱 Method 2: Generate Mobile App

### Step 1: Copy Prompt #3

From `AI_ASSISTANT_PROMPTS.md`, copy the entire **Prompt #3: React Native Mobile App** section.

### Step 2: Paste in New Claude Chat

Same process as above - new chat, paste entire prompt.

### Step 3: Get Complete React Native App

The AI generates:
- iOS project
- Android project
- All React Native components
- Navigation setup
- API integration
- Build scripts

### Step 4: Build and Run

```bash
cd lightning-mobile-app
npm install

# iOS
cd ios && pod install && cd ..
npx react-native run-ios

# Android
npx react-native run-android
```

---

## 🔗 Method 3: Generate Solana Wallet Adapter

### Step 1: Copy Prompt #4

Copy entire **Prompt #4: Solana Wallet Adapter** from `AI_ASSISTANT_PROMPTS.md`.

### Step 2: Generate with AI

Paste in new Claude/ChatGPT chat.

### Step 3: Get NPM Package

The AI creates a complete npm package that integrates with:
- Phantom Wallet
- Solflare
- Backpack
- All Solana wallets

---

## 💡 Pro Tips

### Tip 1: Use Claude for Best Results
Claude (especially Sonnet 3.5+) is better at generating large code projects.

### Tip 2: Ask for Specific Files
If the AI doesn't show a file, ask:
```
"Can you show me the complete code for components/Hero.tsx?"
```

### Tip 3: Request Improvements
After initial generation, ask:
```
"Add dark mode support"
"Add TypeScript strict mode"
"Add more error handling"
```

### Tip 4: Generate in Stages
For large projects, you can say:
```
"First, just show me the project structure and package.json"
```

Then request files as needed.

### Tip 5: Save Conversation
Bookmark the chat or export it so you can reference the code later.

---

## 🎬 Quick Start Examples

### Example 1: MetaMask Extension (5 minutes)

```bash
# 1. Copy Prompt #2 from AI_ASSISTANT_PROMPTS.md
# 2. Paste into new Claude chat
# 3. Wait 2-3 minutes for full generation
# 4. Copy all files to local folder
# 5. Run:

npm install
npm run build
# Load in Chrome → chrome://extensions → Load unpacked → dist/
```

### Example 2: Mobile App (10 minutes)

```bash
# 1. Copy Prompt #3 from AI_ASSISTANT_PROMPTS.md
# 2. Paste into new Claude chat
# 3. Wait 3-5 minutes for full generation
# 4. Copy all files
# 5. Run:

npm install
npx react-native run-ios    # or run-android
```

---

## 🚨 Common Issues & Solutions

### Issue 1: "Prompt Too Long"

**Solution:** The prompts fit within Claude's limits. If you hit an issue:
1. Use Claude.ai (not ChatGPT)
2. Make sure you copied the FULL prompt
3. Try Claude Sonnet 3.5 or Opus

### Issue 2: "Incomplete Generation"

**Solution:** Ask the AI:
```
"Can you continue from where you left off?"
"Please show me the rest of [filename]"
```

### Issue 3: "Doesn't Work When I Run It"

**Solution:**
1. Make sure you copied ALL files
2. Run `npm install` first
3. Check Node.js version (need 18+)
4. Read the AI-generated README.md

### Issue 4: "Want to Modify Generated Code"

**Solution:** In the same chat, ask:
```
"Can you modify the Hero component to add a video background?"
"Add authentication to the mobile app"
```

---

## 📊 What You'll Get

### From Prompt #2 (MetaMask Extension):
```
metamask-lightning-extension/
├── manifest.json
├── background/
│   ├── service-worker.ts
│   ├── channel-manager.ts
│   └── payment-handler.ts
├── popup/
│   ├── App.tsx
│   ├── pages/ (6 pages)
│   └── components/ (5 components)
├── lib/
├── package.json
└── README.md

Total: ~40 files, ~2,000 lines of TypeScript
```

### From Prompt #3 (Mobile App):
```
lightning-mobile-app/
├── src/
│   ├── screens/ (9 screens)
│   ├── components/ (7 components)
│   ├── services/ (6 services)
│   ├── navigation/
│   └── store/
├── android/
├── ios/
├── package.json
└── README.md

Total: ~50 files, ~3,000 lines of TypeScript/React Native
```

### From Prompt #4 (Solana Adapter):
```
@etrid/solana-lightning-adapter/
├── src/
│   ├── adapter.ts
│   ├── provider.tsx
│   ├── hooks/ (5 hooks)
│   └── components/ (4 components)
├── examples/ (3 example apps)
├── package.json
└── README.md

Total: ~20 files, ~1,500 lines of TypeScript
```

---

## ✅ Checklist: Using AI Prompts

Use this checklist when generating apps:

- [ ] Open `AI_ASSISTANT_PROMPTS.md`
- [ ] Identify which prompt you want (2, 3, or 4)
- [ ] Copy the ENTIRE prompt (from start to end)
- [ ] Open NEW Claude.ai chat (https://claude.ai)
- [ ] Paste prompt and send
- [ ] Wait 2-5 minutes for generation
- [ ] Copy all generated files
- [ ] Create local project folder
- [ ] Run `npm install`
- [ ] Follow AI-generated README
- [ ] Test the application
- [ ] Customize as needed

---

## 🎯 Recommended Order

1. **Start with MetaMask Extension** (Prompt #2)
   - Quickest to build and test
   - Works in browser immediately
   - Great for demos

2. **Then build Mobile App** (Prompt #3)
   - Takes longer to set up
   - Requires mobile dev environment
   - Best for end users

3. **Finally add Solana Adapter** (Prompt #4)
   - Expands ecosystem
   - Works with existing wallets
   - Great for integrations

---

## 💬 Example Conversation Flow

**You:** [Paste Prompt #2]

**Claude:** "I'll create a production-ready browser extension... Here's the complete structure:

[Shows file tree]

Let me start with manifest.json:
[Shows code]

Now for background/service-worker.ts:
[Shows code]

..."

**You:** "Can you also add support for dark mode?"

**Claude:** "Sure! I'll update the CSS and add a theme toggle..."

**You:** "Perfect! Can you show me how to publish this to Chrome Web Store?"

**Claude:** "Here's the step-by-step publishing guide..."

---

## 📞 Need Help?

If you get stuck:

1. **Check the AI-generated README** - It has setup instructions
2. **Ask the AI for clarification** - "How do I fix [error]?"
3. **Review the prompt** - Make sure you copied everything
4. **Try again in new chat** - Sometimes helps with context

---

## 🚀 You're Ready!

You now have everything you need to generate complete applications in minutes:

1. ✅ AI prompts ready in `AI_ASSISTANT_PROMPTS.md`
2. ✅ Step-by-step instructions above
3. ✅ Example conversation flows
4. ✅ Troubleshooting guide

**Just copy a prompt, paste in Claude, and watch the magic happen!** ✨

---

## 📚 Additional Resources

- **Landing Page:** `/home/user/Etrid/lightning-landing` (already built!)
- **AI Prompts:** `/home/user/Etrid/AI_ASSISTANT_PROMPTS.md`
- **Feature Docs:** `/home/user/Etrid/PHASE_2_FEATURES_SUMMARY.md`
- **Roadmap:** `/home/user/Etrid/LIGHTNING_FEATURE_ROADMAP.md`

---

**Ready to generate your first app? Pick a prompt and go!** 🚀
