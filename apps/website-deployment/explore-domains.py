#!/usr/bin/env python3
"""
Explore Domains Directory Structure
"""

import ftplib

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

try:
    ftp = ftplib.FTP(FTP_HOST)
    ftp.login(FTP_USER, FTP_PASS)

    print("=" * 70)
    print("📂 Exploring domains/ directory")
    print("=" * 70)

    ftp.cwd('domains')
    domains = []
    ftp.retrlines('NLST', domains.append)

    for domain in domains:
        if domain not in ['.', '..']:
            print(f"\n📁 {domain}/")
            try:
                ftp.cwd(f'domains/{domain}')
                subdirs = []
                ftp.retrlines('NLST', subdirs.append)
                for subdir in subdirs[:20]:
                    if subdir not in ['.', '..']:
                        print(f"   └── {subdir}")
                        # Check public_html
                        if 'public_html' in subdirs:
                            try:
                                ftp.cwd(f'domains/{domain}/public_html')
                                files = []
                                ftp.retrlines('NLST', files.append)
                                print(f"       public_html/ ({len(files)} items):")
                                for f in files[:10]:
                                    if f not in ['.', '..']:
                                        try:
                                            size = ftp.size(f'domains/{domain}/public_html/{f}')
                                            print(f"         • {f} ({size:,} bytes)")
                                        except:
                                            print(f"         • {f}/")
                            except:
                                pass
            except Exception as e:
                print(f"   Error: {e}")

    print("\n" + "=" * 70)
    ftp.quit()

except Exception as e:
    print(f"Error: {e}")
