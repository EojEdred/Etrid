#!/usr/bin/env python3
"""
Download and verify what's actually live on the server
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
    print("📥 Downloading Current Live Files")
    print("=" * 70)

    # Download main index
    print("\n1. Main site index.html:")
    with open('LIVE-index.html', 'wb') as f:
        ftp.retrbinary('RETR /domains/etrid.org/public_html/index.html', f.write)

    size = os.path.getsize('LIVE-index.html')
    print(f"   Downloaded: {size:,} bytes")

    # Check what's in it
    with open('LIVE-index.html', 'r') as f:
        content = f.read()

    print(f"\n2. Checking live content:")
    print(f"   - Has flame architecture section: {'flame-architecture' in content}")
    print(f"   - Has telemetry.etrid.org link: {'telemetry.etrid.org' in content}")
    print(f"   - Has etrid.org/telemetry link: {'etrid.org/telemetry' in content}")
    print(f"   - Has wallet.etrid.org link: {'wallet.etrid.org' in content}")
    print(f"   - Has Coming Soon badges: {'Coming Soon' in content}")

    # Compare with local
    local_size = os.path.getsize('website/index.html')
    print(f"\n3. Size comparison:")
    print(f"   Local file: {local_size:,} bytes")
    print(f"   Live file:  {size:,} bytes")
    print(f"   Match: {local_size == size}")

    if local_size != size:
        print(f"\n❌ MISMATCH! Files are different!")
        print(f"   Difference: {abs(local_size - size):,} bytes")

    ftp.quit()

except Exception as e:
    print(f"❌ Error: {e}")
