#!/usr/bin/env python3
"""Explore FTP server to find correct paths"""

import ftplib

HOST = "157.173.214.206"
USER = "u724092535"
PASS = "Fullashit13!"

print("🔍 Exploring FTP Server Structure...")

ftp = ftplib.FTP()
ftp.connect(HOST, 21)
ftp.login(USER, PASS)

print(f"\n📂 Current directory: {ftp.pwd()}")

print(f"\n📋 Directory listing:")
files = ftp.nlst()
for f in files:
    print(f"   {f}")

# Try detailed listing
print(f"\n📋 Detailed listing:")
ftp.dir()

# Try to find public_html
print(f"\n🔍 Looking for public_html...")
for item in files:
    if 'public' in item.lower() or 'html' in item.lower():
        print(f"   Found: {item}")
        try:
            ftp.cwd(item)
            print(f"   Inside {item}:")
            subfiles = ftp.nlst()
            for sf in subfiles[:20]:
                print(f"      - {sf}")
            ftp.cwd('/')
        except:
            pass

ftp.quit()
