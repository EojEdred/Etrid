#!/usr/bin/env python3
"""Find the actual website path"""

import ftplib

HOST = "157.173.214.206"
USER = "u724092535"
PASS = "Fullashit13!"

ftp = ftplib.FTP()
ftp.connect(HOST, 21)
ftp.login(USER, PASS)

print("🔍 Finding website root...")

# Check domains folder
print(f"\n📂 Checking /domains/")
ftp.cwd('/domains')
domains = ftp.nlst()
print(f"Found domains:")
for d in domains:
    print(f"   - {d}")

# Check each domain
for domain in domains:
    try:
        print(f"\n📂 Checking /domains/{domain}/")
        ftp.cwd(f'/domains/{domain}')
        subdirs = ftp.nlst()
        for sd in subdirs:
            print(f"   - {sd}")
            if 'public' in sd.lower():
                print(f"   ✅ FOUND PUBLIC_HTML: /domains/{domain}/{sd}")

                # Check inside
                ftp.cwd(f'/domains/{domain}/{sd}')
                files = ftp.nlst()
                print(f"\n   Contents (first 20):")
                for f in files[:20]:
                    print(f"      • {f}")
    except:
        pass

ftp.quit()
