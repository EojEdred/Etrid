#!/usr/bin/env python3
"""
Find Correct File Paths
"""

import ftplib

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

def test_path(ftp, path):
    """Test if a path exists"""
    try:
        size = ftp.size(path)
        return True, size
    except:
        pass

    try:
        ftp.cwd(path)
        return True, "directory"
    except:
        return False, None

try:
    ftp = ftplib.FTP(FTP_HOST)
    ftp.login(FTP_USER, FTP_PASS)

    print("=" * 70)
    print("🔍 Testing Common Paths")
    print("=" * 70)

    paths_to_test = [
        "public_html/index.html",
        "htdocs/index.html",
        "www/index.html",
        "domains/etrid.org/public_html/index.html",
        "/domains/etrid.org/public_html/index.html",
        "domains/etrid.org/htdocs/index.html",
        "domains/telemetry.etrid.org/public_html/index.html",
        "telemetry/index.html",
    ]

    print("\n📁 Testing paths:")
    for path in paths_to_test:
        exists, info = test_path(ftp, path)
        if exists:
            if isinstance(info, int):
                print(f"  ✅ {path} ({info:,} bytes)")
            else:
                print(f"  ✅ {path} ({info})")
        else:
            print(f"  ❌ {path}")

    # Try to find where telemetry/index.html actually is
    print("\n📂 Checking telemetry directory in root:")
    try:
        ftp.cwd('/telemetry')
        files = []
        ftp.retrlines('NLST', files.append)
        print(f"  ✅ Found /telemetry with {len(files)} items:")
        for f in files[:15]:
            if f not in ['.', '..']:
                try:
                    size = ftp.size(f'/telemetry/{f}')
                    print(f"     • {f} ({size:,} bytes)")
                except:
                    print(f"     • {f}/ (directory)")
    except Exception as e:
        print(f"  ❌ Error: {e}")

    ftp.quit()

except Exception as e:
    print(f"Error: {e}")
