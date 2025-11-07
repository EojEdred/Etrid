#!/usr/bin/env python3
"""
Download what's actually on the live server
"""

import ftplib
import os

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

try:
    ftp = ftplib.FTP(FTP_HOST)
    ftp.login(FTP_USER, FTP_PASS)

    print("=" * 70)
    print("📥 Downloading Live Files from Server")
    print("=" * 70)

    # Download main site index.html
    print("\n📥 Downloading /domains/etrid.org/public_html/index.html...")
    with open('live-index.html', 'wb') as f:
        ftp.retrbinary('RETR /domains/etrid.org/public_html/index.html', f.write)

    size = os.path.getsize('live-index.html')
    print(f"   ✅ Downloaded: {size:,} bytes")

    # Count lines
    with open('live-index.html', 'r') as f:
        lines = len(f.readlines())
    print(f"   Lines: {lines}")

    # Check for flame architecture
    with open('live-index.html', 'r') as f:
        content = f.read()
        if 'flame-architecture' in content:
            print("   ✅ Contains 'flame-architecture' section")
        else:
            print("   ❌ Does NOT contain 'flame-architecture' section")

        if 'openModal' in content:
            print("   ✅ Contains 'openModal' JavaScript")
        else:
            print("   ❌ Does NOT contain 'openModal' JavaScript")

        if 'mockTelemetryData' in content:
            print("   ✅ Contains 'mockTelemetryData'")
        else:
            print("   ❌ Does NOT contain 'mockTelemetryData'")

    # Compare with local file
    print("\n📊 Comparing with local file...")
    local_size = os.path.getsize('website/index.html')
    print(f"   Local file: {local_size:,} bytes")
    print(f"   Live file: {size:,} bytes")

    if local_size == size:
        print("   ✅ Sizes match!")
    else:
        print(f"   ❌ SIZE MISMATCH! Difference: {abs(local_size - size):,} bytes")

    print("\n" + "=" * 70)
    ftp.quit()

except Exception as e:
    print(f"❌ Error: {e}")
