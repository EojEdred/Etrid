#!/usr/bin/env python3
"""
Proper FTP Check using LIST command
"""

import ftplib

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

try:
    ftp = ftplib.FTP(FTP_HOST)
    ftp.login(FTP_USER, FTP_PASS)

    print("=" * 70)
    print("🔍 Proper FTP Structure Check")
    print("=" * 70)

    # Check root
    print("\n📂 Root directory (/):")
    ftp.cwd('/')
    items = []
    ftp.retrlines('LIST', items.append)
    for item in items:
        print(f"  {item}")

    # Check if there's a public_html symlink or directory
    print("\n📂 Looking for web roots:")

    # Common Hostinger patterns
    test_paths = [
        'public_html',
        'htdocs',
        'www',
        'domains/etrid.org/public_html',
    ]

    for path in test_paths:
        try:
            ftp.cwd('/')
            ftp.cwd(path)
            print(f"\n✅ Found: /{path}")
            items = []
            ftp.retrlines('LIST', items.append)
            print(f"   Contents ({len(items)} items):")
            for item in items[:10]:
                print(f"     {item}")
            break
        except:
            pass

    # Check the symlink target
    print("\n📂 Checking for symlinks in root:")
    ftp.cwd('/')
    try:
        # Try to get a file listing that shows symlink targets
        lines = []
        ftp.retrlines('LIST -la', lines.append)
        for line in lines:
            if 'public_html' in line or '->' in line:
                print(f"  {line}")
    except:
        pass

    ftp.quit()

except Exception as e:
    print(f"Error: {e}")
