#!/usr/bin/env python3
"""Diagnose FTP login issue with detailed error reporting"""

import ftplib
import socket

HOST = "157.173.214.206"
PORT = 21

# Try multiple username formats
CREDENTIALS = [
    ("u724092535", "Fullashit13!", "Original username"),
    ("eojedredbitepubkey1@proton.me", "Fullashit13!", "Email address"),
    ("u724092535@etrid.org", "Fullashit13!", "Username@domain"),
    ("u724092535.etrid.org", "Fullashit13!", "Username.domain"),
]

def test_connection():
    """Test basic FTP connection"""
    print("=" * 70)
    print("🔍 FTP Server Diagnostics")
    print("=" * 70)
    print(f"\nServer: {HOST}:{PORT}")

    try:
        # Test socket connection
        print("\n1. Testing socket connection...")
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(10)
        sock.connect((HOST, PORT))
        print(f"   ✅ Socket connection successful")
        sock.close()

        # Connect with FTP
        print("\n2. Testing FTP protocol...")
        ftp = ftplib.FTP()
        ftp.set_debuglevel(2)  # Verbose output

        print(f"\n   Connecting to {HOST}...")
        response = ftp.connect(HOST, PORT, timeout=15)
        print(f"   ✅ FTP Response: {response}")

        # Get welcome message
        welcome = ftp.getwelcome()
        print(f"   ✅ Welcome: {welcome}")

        # Close
        ftp.close()

        return True

    except Exception as e:
        print(f"   ❌ Error: {e}")
        return False

def test_login(username, password, description):
    """Test login with specific credentials"""
    print(f"\n{'='*70}")
    print(f"Testing: {description}")
    print(f"Username: {username}")
    print(f"Password: {'*' * len(password)}")
    print("="*70)

    try:
        ftp = ftplib.FTP()
        ftp.set_debuglevel(1)  # Show commands

        # Connect
        ftp.connect(HOST, PORT, timeout=15)

        # Try login
        print(f"\n   Attempting login...")
        ftp.login(username, password)

        print(f"\n   ✅ LOGIN SUCCESSFUL!")
        print(f"   Current directory: {ftp.pwd()}")

        # List files
        print(f"\n   Directory listing:")
        files = ftp.nlst()
        for f in files[:10]:
            print(f"      - {f}")

        ftp.quit()
        return True

    except ftplib.error_perm as e:
        error_msg = str(e)
        print(f"\n   ❌ Login failed: {error_msg}")

        if "530" in error_msg:
            print(f"\n   💡 Error 530 means:")
            print(f"      • Invalid username or password")
            print(f"      • FTP account may not exist")
            print(f"      • FTP may be disabled for this user")

        return False

    except Exception as e:
        print(f"\n   ❌ Error: {e}")
        return False

def check_hostinger_ftp_docs():
    """Show Hostinger FTP setup info"""
    print("\n" + "=" * 70)
    print("📚 Hostinger FTP Account Setup")
    print("=" * 70)
    print("""
To get correct FTP credentials:

1. Login to: https://hpanel.hostinger.com
2. Go to: Files → FTP Accounts
3. Check existing accounts OR create new one:

   For Existing Account:
   • Click 'Configure FTP Client'
   • Note the exact username (might be u724092535 or different)
   • Note the hostname (should be 157.173.214.206 or ftp.etrid.org)

   For New Account:
   • Click 'Create FTP Account'
   • Username: anything you want
   • Password: anything you want
   • Directory: public_html
   • Click 'Create'

4. Common Hostinger FTP username formats:
   • u[number]          (e.g., u724092535)
   • u[number]@domain   (e.g., u724092535@etrid.org)
   • custom@domain      (if you created custom FTP account)

5. The password is what you set (or was generated) in hPanel
""")

def main():
    # Test basic connection
    if not test_connection():
        print("\n❌ Cannot connect to FTP server!")
        return

    print("\n" + "=" * 70)
    print("🔐 Testing Login Credentials")
    print("=" * 70)

    # Try each credential combination
    for username, password, description in CREDENTIALS:
        if test_login(username, password, description):
            print("\n" + "=" * 70)
            print("🎉 FOUND WORKING CREDENTIALS!")
            print("=" * 70)
            print(f"Username: {username}")
            print(f"Password: {password}")
            return

    # None worked
    print("\n" + "=" * 70)
    print("❌ ALL LOGIN ATTEMPTS FAILED")
    print("=" * 70)

    check_hostinger_ftp_docs()

if __name__ == "__main__":
    main()
