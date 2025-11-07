#!/usr/bin/env python3
import ftplib

FTP_HOST = "ftp.etrid.org"
FTP_USER = "u724092535"
FTP_PASS = "Fullashit13!"

print("🔍 Checking deployed SEO files on server...\n")

try:
    ftp = ftplib.FTP(FTP_HOST, timeout=30)
    ftp.login(FTP_USER, FTP_PASS)
    
    remote_dir = "/domains/etrid.org/public_html"
    ftp.cwd(remote_dir)
    
    files_to_check = [
        "robots.txt",
        "sitemap.xml",
        ".htaccess",
        "wallet.html",
        "explorer.html",
        "bridge.html",
        "forum.html",
        "blog.html"
    ]
    
    print("📁 Files in public_html:\n")
    all_files = ftp.nlst()
    
    for filename in files_to_check:
        if filename in all_files:
            size = ftp.size(filename) if filename != '.' and filename != '..' else 0
            print(f"✅ {filename} ({size:,} bytes)")
        else:
            print(f"❌ {filename} - NOT FOUND")
    
    ftp.quit()
    
except Exception as e:
    print(f"❌ Error: {str(e)}")
