# Fix 403 Forbidden Error - ËTRID Hostinger Upload

## 🔍 Common Causes & Solutions

### **Issue 1: File Permissions** (Most Common)

Hostinger requires specific permissions:
- **Files**: 644 (read/write for owner, read for others)
- **Folders**: 755 (read/write/execute for owner, read/execute for others)

#### **How to Fix in Hostinger File Manager:**

1. Log into Hostinger → File Manager
2. Navigate to the subdomain folder (e.g., `/public_html/validator/`)
3. Select ALL files and folders (checkbox at top)
4. Right-click → **"Change Permissions"** or **"Permissions"**
5. Set permissions:
   - **For folders**: `755` (or check: read/write/execute for owner, read/execute for group/public)
   - **For files**: `644` (or check: read/write for owner, read for group/public)
6. Check **"Apply to subdirectories recursively"**
7. Click **"Change"** or **"Apply"**

**Quick Permission Settings:**
```
Folders: 755
Files: 644
```

---

### **Issue 2: Missing index.html**

Hostinger looks for `index.html` in the subdomain root folder.

#### **Verify File Structure:**

Each subdomain folder should have `index.html` at the root:

```
/public_html/validator/
├── index.html          ← MUST be here (not in subfolder)
├── _next/
├── 404.html
└── ...other files
```

**NOT like this:**
```
/public_html/validator/
└── validator/          ← Wrong! Extra folder
    ├── index.html
    └── _next/
```

#### **How to Fix:**

If you have a nested folder structure:
1. Move ALL contents from the nested folder up one level
2. Delete the empty nested folder

**In Hostinger File Manager:**
1. Navigate to `/public_html/validator/`
2. If you see a `validator/` folder inside, open it
3. Select all files inside
4. Click "Move"
5. Move to parent directory (`/public_html/validator/`)
6. Delete the now-empty `validator/` folder

---

### **Issue 3: .htaccess Blocking**

Some `.htaccess` files may block access.

#### **Temporary Test: Rename .htaccess**

1. Navigate to subdomain folder
2. Find `.htaccess` file (you may need to show hidden files)
3. Rename it to `.htaccess.bak`
4. Test the URL again
5. If it works, the .htaccess was the problem

#### **Create a Simple .htaccess:**

If you need .htaccess, create this simple one:

```apache
# Enable directory browsing (only for testing)
Options +Indexes

# Set default index files
DirectoryIndex index.html index.htm

# Enable following symlinks
Options +FollowSymLinks

# Allow access
<Files *>
    Require all granted
</Files>
```

**After testing, remove the `Options +Indexes` line for security.**

---

### **Issue 4: DirectoryIndex Not Set**

Hostinger needs to know which file to serve.

#### **Create/Update .htaccess in Each Subdomain:**

Add this to `/public_html/validator/.htaccess`:

```apache
DirectoryIndex index.html
Options -Indexes
RewriteEngine On
```

Do this for:
- `/public_html/validator/.htaccess`
- `/public_html/watchtower/.htaccess`
- `/public_html/masterchef/.htaccess`
- `/public_html/wallet/.htaccess`

---

## 🧪 **Quick Test Method**

### **Step 1: Create a Simple Test File**

1. In Hostinger File Manager
2. Navigate to `/public_html/validator/`
3. Create new file: `test.html`
4. Content:
```html
<!DOCTYPE html>
<html>
<head><title>Test</title></head>
<body><h1>It Works!</h1></body>
</html>
```
5. Save
6. Visit: `https://validator.etrid.org/test.html`

**If test.html works but index.html doesn't:**
- Problem is with the Next.js files or index.html itself

**If test.html also shows 403:**
- Problem is with permissions or server configuration

---

## 🔧 **Step-by-Step Fix Procedure**

### **For validator.etrid.org (repeat for all subdomains):**

1. **Check File Structure**
   ```
   /public_html/validator/
   ├── index.html     ← Must exist here
   ├── _next/         ← Assets folder
   ├── 404.html
   └── ...
   ```

2. **Fix Permissions**
   - Select all files/folders
   - Right-click → Permissions
   - Set: Folders 755, Files 644
   - Apply recursively

3. **Create/Update .htaccess**
   - File: `/public_html/validator/.htaccess`
   - Content:
   ```apache
   DirectoryIndex index.html
   Options -Indexes

   # Enable rewrite engine
   RewriteEngine On

   # Don't rewrite files or directories
   RewriteCond %{REQUEST_FILENAME} -f [OR]
   RewriteCond %{REQUEST_FILENAME} -d
   RewriteRule ^ - [L]

   # Allow access
   <FilesMatch "^.*$">
       Require all granted
   </FilesMatch>
   ```

4. **Test**
   - Visit: `https://validator.etrid.org`
   - Should show validator dashboard

5. **Check Browser Console**
   - Press F12 → Console tab
   - Look for 404 errors on `_next/` files
   - If you see 404s, files aren't uploaded correctly

---

## 🚨 **Emergency Fix: Create Simple .htaccess for Each Subdomain**

Copy this `.htaccess` to each subdomain folder:

```apache
# ËTRID Subdomain Configuration
DirectoryIndex index.html index.htm
Options -Indexes
RewriteEngine On

# Set proper MIME types
<IfModule mod_mime.c>
    AddType text/html .html .htm
    AddType text/css .css
    AddType application/javascript .js
    AddType application/json .json
</IfModule>

# Allow access to all files
<FilesMatch "^.*$">
    Require all granted
</FilesMatch>

# Enable compression
<IfModule mod_deflate.c>
    AddOutputFilterByType DEFLATE text/html text/css application/javascript
</IfModule>

# Cache control
<IfModule mod_expires.c>
    ExpiresActive On
    ExpiresByType text/html "access plus 0 seconds"
    ExpiresByType text/css "access plus 1 year"
    ExpiresByType application/javascript "access plus 1 year"
</IfModule>
```

---

## 📋 **Checklist Before Testing**

For **each subdomain**, verify:

- [ ] Subdomain exists in Hostinger
- [ ] Subdomain points to correct folder
- [ ] `index.html` exists at root of subdomain folder
- [ ] `_next/` folder exists alongside `index.html`
- [ ] File permissions: folders 755, files 644
- [ ] `.htaccess` configured properly (or removed for testing)
- [ ] No extra nested folders
- [ ] SSL enabled for subdomain

---

## 🆘 **Still Getting 403?**

### **Try This:**

1. **Delete everything in subdomain folder**
2. **Re-upload the zip file**
3. **Extract directly in the subdomain folder**
4. **Check that files appear at root level** (not in nested folder)
5. **Set permissions** (755 for folders, 644 for files)
6. **Create simple .htaccess** (see above)
7. **Test again**

### **Contact Hostinger Support:**

If nothing works, ask Hostinger support:
- "My subdomain validator.etrid.org shows 403 Forbidden"
- "I've set permissions to 755/644 and have index.html at root"
- "Please check server configuration for this subdomain"

---

## 🎯 **Common Solutions Summary**

| Problem | Solution |
|---------|----------|
| **403 on all files** | Fix permissions (755/644) |
| **403 only on index** | Check .htaccess, set DirectoryIndex |
| **Nested folders** | Move files up, delete nested folder |
| **Missing files** | Re-extract zip properly |
| **Server config** | Contact Hostinger support |

---

## ✅ **After Fixing**

Once you get one subdomain working, the same fix applies to all others:
1. Fix validator first
2. Note what fixed it
3. Apply same solution to watchtower, masterchef, wallet

**All zip files have the correct structure - if you're getting 403, it's a Hostinger server configuration issue, not the files.**
