# GitHub Secrets Setup for CI/CD

This guide explains how to set up GitHub repository secrets for automatic deployment to Vercel via GitHub Actions.

## Why GitHub Secrets?

- Securely store sensitive credentials
- Environment variables for CI/CD workflows
- Enable automatic deployments on push/PR
- Keep secrets out of code repository

## Required Secrets

You'll need to add these secrets to your GitHub repository:

### Vercel Secrets

1. **VERCEL_TOKEN** - Vercel authentication token
2. **VERCEL_ORG_ID** - Your Vercel organization ID
3. **VERCEL_PROJECT_ID** - PWA project ID
4. **VERCEL_TOKEN_LANDING** - Same as VERCEL_TOKEN (or separate if needed)
5. **VERCEL_PROJECT_ID_LANDING** - Landing page project ID

### Firebase Secrets

6. **FIREBASE_API_KEY** - Firebase API key
7. **FIREBASE_AUTH_DOMAIN** - Firebase auth domain
8. **FIREBASE_PROJECT_ID** - Firebase project ID
9. **FIREBASE_STORAGE_BUCKET** - Firebase storage bucket
10. **FIREBASE_MESSAGING_SENDER_ID** - Firebase sender ID
11. **FIREBASE_APP_ID** - Firebase app ID
12. **FIREBASE_VAPID_KEY** - Firebase VAPID key for push notifications

## Step-by-Step Setup

### 1. Get Vercel Token

```bash
# Option A: Via Vercel CLI
vercel login
# Then create token at: https://vercel.com/account/tokens

# Option B: Via Vercel Dashboard
# 1. Go to https://vercel.com/account/tokens
# 2. Click "Create Token"
# 3. Name it: "GitHub Actions"
# 4. Set expiration (or no expiration)
# 5. Click "Create"
# 6. Copy the token (you won't see it again!)
```

Save this token as `VERCEL_TOKEN`.

### 2. Get Vercel Organization ID

```bash
# Method 1: From project settings
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
vercel link
cat .vercel/project.json
```

You'll see:
```json
{
  "orgId": "team_xxxxxxxxxxxxx",
  "projectId": "prj_xxxxxxxxxxxxx"
}
```

Save `orgId` as `VERCEL_ORG_ID`.

### 3. Get Vercel Project IDs

```bash
# PWA Project ID
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
vercel link
cat .vercel/project.json
# Copy "projectId" -> Save as VERCEL_PROJECT_ID

# Landing Page Project ID
cd /home/user/Etrid/apps/wallet-mobile/landing-page
vercel link
cat .vercel/project.json
# Copy "projectId" -> Save as VERCEL_PROJECT_ID_LANDING
```

### 4. Get Firebase Credentials

```bash
# From Firebase Console
# 1. Go to https://console.firebase.google.com
# 2. Select your project
# 3. Click Settings (gear icon) → Project settings
# 4. Scroll to "Your apps" section
# 5. Click on your web app (or add one if you haven't)
# 6. Copy all the config values:

const firebaseConfig = {
  apiKey: "AIzaSy...",              // -> FIREBASE_API_KEY
  authDomain: "project.firebaseapp.com",  // -> FIREBASE_AUTH_DOMAIN
  projectId: "project-id",          // -> FIREBASE_PROJECT_ID
  storageBucket: "project.appspot.com",   // -> FIREBASE_STORAGE_BUCKET
  messagingSenderId: "123456789",   // -> FIREBASE_MESSAGING_SENDER_ID
  appId: "1:123:web:abc",           // -> FIREBASE_APP_ID
  measurementId: "G-XXXXXXXXXX"     // (optional, not needed as secret)
};

# For VAPID Key:
# 7. Go to Project settings → Cloud Messaging
# 8. Under "Web configuration" → "Web Push certificates"
# 9. Click "Generate key pair" (if not already generated)
# 10. Copy the key -> FIREBASE_VAPID_KEY
```

### 5. Add Secrets to GitHub

#### Via GitHub Web Interface:

1. Go to your repository on GitHub
2. Click **Settings** (top right)
3. In left sidebar, click **Secrets and variables** → **Actions**
4. Click **New repository secret** (green button)
5. For each secret:
   - Name: `VERCEL_TOKEN`
   - Secret: `paste your token`
   - Click **Add secret**
6. Repeat for all 12 secrets

#### Via GitHub CLI (faster):

```bash
# Install GitHub CLI if not already installed
# https://cli.github.com/

# Authenticate
gh auth login

# Navigate to your repository
cd /home/user/Etrid

# Add secrets
gh secret set VERCEL_TOKEN
# Paste token and press Enter, then Ctrl+D

gh secret set VERCEL_ORG_ID
# Paste org ID and press Enter, then Ctrl+D

gh secret set VERCEL_PROJECT_ID
# Paste project ID and press Enter, then Ctrl+D

gh secret set VERCEL_TOKEN_LANDING
# Paste token (same as VERCEL_TOKEN) and press Enter, then Ctrl+D

gh secret set VERCEL_PROJECT_ID_LANDING
# Paste landing project ID and press Enter, then Ctrl+D

gh secret set FIREBASE_API_KEY
# Paste API key and press Enter, then Ctrl+D

gh secret set FIREBASE_AUTH_DOMAIN
# Paste auth domain and press Enter, then Ctrl+D

gh secret set FIREBASE_PROJECT_ID
# Paste project ID and press Enter, then Ctrl+D

gh secret set FIREBASE_STORAGE_BUCKET
# Paste storage bucket and press Enter, then Ctrl+D

gh secret set FIREBASE_MESSAGING_SENDER_ID
# Paste sender ID and press Enter, then Ctrl+D

gh secret set FIREBASE_APP_ID
# Paste app ID and press Enter, then Ctrl+D

gh secret set FIREBASE_VAPID_KEY
# Paste VAPID key and press Enter, then Ctrl+D
```

### 6. Verify Secrets

```bash
# List all secrets
gh secret list

# You should see all 12 secrets:
# VERCEL_TOKEN
# VERCEL_ORG_ID
# VERCEL_PROJECT_ID
# VERCEL_TOKEN_LANDING
# VERCEL_PROJECT_ID_LANDING
# FIREBASE_API_KEY
# FIREBASE_AUTH_DOMAIN
# FIREBASE_PROJECT_ID
# FIREBASE_STORAGE_BUCKET
# FIREBASE_MESSAGING_SENDER_ID
# FIREBASE_APP_ID
# FIREBASE_VAPID_KEY
```

### 7. Test GitHub Actions

```bash
# Make a small change to trigger workflow
cd /home/user/Etrid/apps/wallet-mobile/etrid-wallet
echo "// Test change" >> next.config.js

# Commit and push
git add .
git commit -m "test: Trigger GitHub Actions deployment"
git push

# Check workflow status
gh run list
# Or go to GitHub → Actions tab in your repository
```

## Secret Details Reference

### VERCEL_TOKEN
- **What**: Vercel API authentication token
- **Where**: https://vercel.com/account/tokens
- **Format**: `v1_xxxxxxxxxxxxxxxxxxxx`
- **Used in**: Both workflows (PWA and Landing)

### VERCEL_ORG_ID
- **What**: Your Vercel team/organization ID
- **Where**: `.vercel/project.json` after `vercel link`
- **Format**: `team_xxxxxxxxxxxx` or `user_xxxxxxxxxxxx`
- **Used in**: Both workflows

### VERCEL_PROJECT_ID
- **What**: PWA project ID on Vercel
- **Where**: `.vercel/project.json` in etrid-wallet directory
- **Format**: `prj_xxxxxxxxxxxx`
- **Used in**: PWA workflow only

### VERCEL_PROJECT_ID_LANDING
- **What**: Landing page project ID on Vercel
- **Where**: `.vercel/project.json` in landing-page directory
- **Format**: `prj_xxxxxxxxxxxx`
- **Used in**: Landing workflow only

### FIREBASE_API_KEY
- **What**: Firebase API key for web app
- **Where**: Firebase Console → Project settings → Web app config
- **Format**: `AIzaSyXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`
- **Security**: Can be public, but should be restricted in Google Cloud Console
- **Used in**: PWA workflow only

### FIREBASE_AUTH_DOMAIN
- **What**: Firebase authentication domain
- **Where**: Firebase Console → Project settings
- **Format**: `project-id.firebaseapp.com`
- **Used in**: PWA workflow only

### FIREBASE_PROJECT_ID
- **What**: Firebase project identifier
- **Where**: Firebase Console → Project settings
- **Format**: `project-id` (alphanumeric with hyphens)
- **Used in**: PWA workflow only

### FIREBASE_STORAGE_BUCKET
- **What**: Firebase Cloud Storage bucket
- **Where**: Firebase Console → Project settings
- **Format**: `project-id.appspot.com`
- **Used in**: PWA workflow only

### FIREBASE_MESSAGING_SENDER_ID
- **What**: Firebase Cloud Messaging sender ID
- **Where**: Firebase Console → Project settings
- **Format**: `123456789012` (12 digits)
- **Used in**: PWA workflow only

### FIREBASE_APP_ID
- **What**: Firebase web app identifier
- **Where**: Firebase Console → Project settings → Web app
- **Format**: `1:123456789012:web:abcdef123456`
- **Used in**: PWA workflow only

### FIREBASE_VAPID_KEY
- **What**: VAPID key for push notifications
- **Where**: Firebase Console → Cloud Messaging → Web Push certificates
- **Format**: Base64 string (long)
- **Used in**: PWA workflow only
- **Note**: Generate if not exists

## Security Best Practices

1. **Never commit secrets to code**
   - Always use GitHub Secrets or Vercel Environment Variables
   - Check `.gitignore` includes `.env*` files

2. **Rotate secrets regularly**
   - Update tokens every 3-6 months
   - Update immediately if compromised

3. **Use least privilege**
   - Vercel token should only have necessary permissions
   - Firebase keys should be restricted to your domains

4. **Separate environments**
   - Use different Firebase projects for dev/staging/prod
   - Use different Vercel projects if needed

5. **Monitor secret usage**
   - Check GitHub Actions logs for failures
   - Review Vercel deployment logs
   - Monitor Firebase usage

## Troubleshooting

### Secret Not Found Error

```
Error: Secret VERCEL_TOKEN not found
```

**Solution**:
- Verify secret name is exactly correct (case-sensitive)
- Check secret is added to repository, not your user account
- Ensure workflow has permission to access secrets

### Invalid Token Error

```
Error: Invalid token
```

**Solution**:
- Regenerate Vercel token
- Update GitHub secret with new token
- Verify token hasn't expired

### Project Not Found Error

```
Error: Project not found
```

**Solution**:
- Verify VERCEL_PROJECT_ID is correct
- Run `vercel link` and check `.vercel/project.json`
- Ensure project exists in Vercel dashboard

### Deployment Fails with Environment Variables

```
Error: Missing environment variable
```

**Solution**:
- Add missing variable to GitHub Secrets
- Update workflow YAML to include the variable
- Verify variable name matches in workflow

## Updating Secrets

### To Update a Secret:

```bash
# Via GitHub CLI
gh secret set SECRET_NAME
# Paste new value

# Via Web Interface
# Go to Settings → Secrets → Click on secret → Update secret
```

### To Delete a Secret:

```bash
# Via GitHub CLI
gh secret delete SECRET_NAME

# Via Web Interface
# Go to Settings → Secrets → Click on secret → Delete secret
```

## Backup Your Secrets

Create a secure backup of your secrets (encrypted):

```bash
# Create a secure file (DO NOT commit to git)
cat > ~/.etrid-secrets.txt << EOF
VERCEL_TOKEN=v1_xxx
VERCEL_ORG_ID=team_xxx
VERCEL_PROJECT_ID=prj_xxx
VERCEL_PROJECT_ID_LANDING=prj_xxx
FIREBASE_API_KEY=AIza_xxx
FIREBASE_AUTH_DOMAIN=project.firebaseapp.com
FIREBASE_PROJECT_ID=project-id
FIREBASE_STORAGE_BUCKET=project.appspot.com
FIREBASE_MESSAGING_SENDER_ID=123456789
FIREBASE_APP_ID=1:123:web:abc
FIREBASE_VAPID_KEY=BPxxxxxx
EOF

# Encrypt it
gpg -c ~/.etrid-secrets.txt

# Store encrypted file safely (cloud storage, password manager)
# Delete unencrypted file
rm ~/.etrid-secrets.txt
```

## Quick Reference Commands

```bash
# List all secrets
gh secret list

# Add a secret (interactive)
gh secret set SECRET_NAME

# Delete a secret
gh secret delete SECRET_NAME

# View workflow runs
gh run list

# View specific workflow run
gh run view RUN_ID

# Re-run failed workflow
gh run rerun RUN_ID

# View workflow logs
gh run view RUN_ID --log
```

## What Happens After Setup?

Once all secrets are configured:

1. **On Push to main or claude/* branches**:
   - GitHub Actions automatically triggers
   - Builds your project
   - Deploys to Vercel (production for main, preview for others)

2. **On Pull Request**:
   - GitHub Actions automatically triggers
   - Builds your project
   - Deploys preview to Vercel
   - Adds preview URL as PR comment

3. **Manual Deployment**:
   - Still possible via `vercel` CLI
   - GitHub Actions is just an additional automation

## Need Help?

- GitHub Secrets Docs: https://docs.github.com/en/actions/security-guides/encrypted-secrets
- Vercel + GitHub Actions: https://vercel.com/guides/how-can-i-use-github-actions-with-vercel
- Firebase Setup: https://firebase.google.com/docs/web/setup
