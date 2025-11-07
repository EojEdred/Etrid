#!/usr/bin/env python3
"""
Check FTP Structure to Find Correct Paths
"""

import ftplib
import sys

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

def list_directory(ftp, path, indent=0):
    """Recursively list directory structure"""
    try:
        ftp.cwd(path)
        items = []
        ftp.retrlines('NLST', items.append)

        for item in items[:20]:  # Limit to first 20 items
            print("  " * indent + f"📁 {item}")
            if item not in ['.', '..'] and not item.startswith('.'):
                try:
                    list_directory(ftp, path + "/" + item, indent + 1)
                    ftp.cwd(path)
                except:
                    pass
    except:
        pass

def main():
    print("=" * 70)
    print("🔍 Checking FTP Structure")
    print("=" * 70)
    print()

    try:
        ftp = ftplib.FTP(FTP_HOST)
        ftp.login(FTP_USER, FTP_PASS)
        print(f"✅ Connected\n")

        print("📂 Root Directory:")
        items = []
        ftp.retrlines('NLST', items.append)
        for item in items:
            print(f"  {item}")

        print("\n📂 Checking for public_html:")
        try:
            ftp.cwd('public_html')
            files = []
            ftp.retrlines('NLST', files.append)
            print(f"  ✅ Found public_html with {len(files)} items")
            for f in files[:10]:
                print(f"     {f}")
        except:
            print("  ❌ No public_html in root")

        ftp.quit()

    except Exception as e:
        print(f"❌ Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
