# ⚡ QUICK FIX CHECKLIST - Ivory Papers

Follow these steps IN ORDER. Check off each one as you complete it.

---

## ☐ STEP 1: Test Basic JavaScript

**Upload:** `test-ultra-simple.html` to Hostinger

**Visit:** `http://yourdomain.com/whitepaper/test-ultra-simple.html`

**Expected:** Green text saying "✓ SUCCESS - JavaScript IS working"

**If FAILED:** Contact Hostinger support - JavaScript is broken on your server

---

## ☐ STEP 2: Verify File Size

**Check:** File size of `viewer-standalone.html` on Hostinger

**Expected:** Exactly **224,407 bytes** (or "219 KB")

**If WRONG SIZE:**
- Delete the old file from Hostinger
- Re-upload from: `/whitepaper 2/viewer-standalone.html`
- If using FTP: Set transfer mode to **BINARY** (not TEXT)
- Check size again after upload

---

## ☐ STEP 3: Clear Browser Cache

**Do ALL of these:**
- [ ] Hard refresh: `Cmd+Shift+R` (Mac) or `Ctrl+F5` (Windows)
- [ ] Open incognito/private window
- [ ] Try a different browser (if available)

---

## ☐ STEP 4: Visit Viewer

**URL:** `http://yourdomain.com/whitepaper/viewer-standalone.html`

**Expected:** See "ËTRID IVORY PAPERS" header and 4 volume buttons

**If still shows "Loading content...":** Continue to Step 5

---

## ☐ STEP 5: Check Browser Console

**Open Console:** Press `F12` or `Cmd+Option+J`

**Look for RED errors:**

- **"marked is not defined"** → File corrupted. Re-upload in BINARY mode (Step 2)
- **"404 Not Found"** → File in wrong directory. Check upload location.
- **"CSP" or "Content Security Policy"** → Contact Hostinger support
- **No errors shown** → Continue to Step 6

---

## ☐ STEP 6: Run Diagnostic

**Upload:** `diagnostic.html` to Hostinger

**Visit:** `http://yourdomain.com/whitepaper/diagnostic.html`

**Screenshot** the results and check:
- ✓ JavaScript is running
- ✓ marked library is loaded
- ✓ DOM is accessible

**If marked library shows ✗ NOT loaded:**
- File is corrupted
- Delete from server
- Re-upload using FTP in BINARY mode

---

## 🎯 MOST COMMON FIX (Try This First!)

**The Problem:** File uploaded in TEXT mode instead of BINARY mode

**The Fix:**
1. Install FileZilla (free FTP client)
2. Connect to your Hostinger account
3. Go to: Transfer menu → Transfer type → **Binary**
4. Delete old `viewer-standalone.html` from server
5. Upload new one from `/whitepaper 2/viewer-standalone.html`
6. Verify file size is 224,407 bytes
7. Clear browser cache and test

---

## 📁 FILE LOCATIONS REFERENCE

**On your Mac (CORRECT files to upload):**
```
Main viewer (219 KB):
/whitepaper 2/viewer-standalone.html

Test files:
/website/whitepaper/test-ultra-simple.html
/website/whitepaper/test-simple.html
/website/whitepaper/diagnostic.html
```

**On Hostinger (where to upload TO):**
```
Usually: public_html/whitepaper/
(or wherever your existing whitepaper folder is)
```

---

## ✅ SUCCESS CRITERIA

You'll know it's working when you see:

1. ✓ No "Loading content..." message
2. ✓ "ËTRID IVORY PAPERS" header visible
3. ✓ 4 volume buttons show
4. ✓ Complete Edition content displays by default
5. ✓ Can click volume buttons to switch papers
6. ✓ Download buttons work

---

## 🆘 STILL NOT WORKING?

Gather these 3 screenshots:
1. `test-ultra-simple.html` results
2. `diagnostic.html` results
3. Browser console (F12) showing any red errors

These will show exactly what's wrong.

---

**File to upload:** 224,407 bytes
**Transfer mode:** BINARY
**Clear cache:** Yes, always
