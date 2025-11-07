#!/usr/bin/env python3
"""
Create redirect HTML pages for broken subdomains
These will redirect to main site and tell Google to deindex them
"""

import os

# Broken subdomains that need redirect pages
SUBDOMAINS = [
    'wallet',
    'explorer',
    'bridge',
    'masterchef',
    'faucet',
    'governance-portal',
    'forum',
    'blog',
    'validator-dashboard'
]

REDIRECT_HTML = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="robots" content="noindex, nofollow">
    <meta http-equiv="refresh" content="0;url=https://etrid.org/">
    <link rel="canonical" href="https://etrid.org/">
    <title>Redirecting to ËTRID</title>
    <script>
        window.location.href = 'https://etrid.org/';
    </script>
</head>
<body>
    <p>Redirecting to <a href="https://etrid.org/">etrid.org</a>...</p>
</body>
</html>
"""

# Create redirect pages
os.makedirs('website/redirects', exist_ok=True)

for subdomain in SUBDOMAINS:
    filepath = f'website/redirects/{subdomain}.html'
    with open(filepath, 'w') as f:
        f.write(REDIRECT_HTML)
    print(f"✅ Created redirect: {filepath}")

print("\n📋 These need to be placed on the server at:")
print("   /domains/etrid.org/public_html/wallet.html")
print("   /domains/etrid.org/public_html/explorer.html")
print("   etc...")
