#!/usr/bin/env python3
"""Upload whitepaper with marked.js embedded"""

import ftplib
import os

HOST = "157.173.214.206"
USER = "u724092535"
PASS = "Fullashit13!"

LOCAL_FILE = '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html'
REMOTE_PATH = '/domains/etrid.org/public_html/whitepaper/viewer-standalone.html'

print("🚀 Uploading FIXED whitepaper with marked.js embedded")
print("=" * 70)

# Check file
if not os.path.exists(LOCAL_FILE):
    print(f"❌ File not found: {LOCAL_FILE}")
    exit(1)

size = os.path.getsize(LOCAL_FILE)
print(f"📄 File: viewer-standalone.html")
print(f"📦 Size: {size:,} bytes ({size/1024:.1f} KB)")

# Connect
print(f"\n🔌 Connecting to {HOST}...")
ftp = ftplib.FTP()
ftp.connect(HOST, 21, timeout=30)
ftp.login(USER, PASS)
ftp.set_pasv(True)
print(f"✅ Connected!")

# Navigate
remote_dir = os.path.dirname(REMOTE_PATH)
remote_file = os.path.basename(REMOTE_PATH)

ftp.cwd(remote_dir)
print(f"📁 Directory: {remote_dir}")

# Upload
print(f"\n⏳ Uploading...")
with open(LOCAL_FILE, 'rb') as f:
    ftp.storbinary(f'STOR {remote_file}', f, blocksize=8192)

print(f"✅ UPLOAD COMPLETE!")

ftp.quit()

print(f"\n{'='*70}")
print(f"🎉 Whitepaper viewer updated successfully!")
print(f"{'='*70}")
print(f"\n✅ Fixed:")
print(f"   • Marked.js v4.3.0 embedded (49 KB)")
print(f"   • NO AOS library (CSP compliant)")
print(f"   • All 5 documents embedded")
print(f"\n🔍 Test now:")
print(f"   https://etrid.org/whitepaper/viewer-standalone.html")
print(f"\n💡 Hard refresh in Firefox:")
print(f"   • Press Ctrl+Shift+R (Windows/Linux)")
print(f"   • Press Cmd+Shift+R (Mac)")
print(f"   • Or Ctrl+F5")
