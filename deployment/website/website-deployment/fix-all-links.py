#!/usr/bin/env python3
"""
Fix all broken links systematically:
1. wallet.etrid.org/staking → placeholder or remove
2. etrid.io → etrid.org
"""
import os
import re

fixes_made = 0

# Fix whitepaper etrid.io links
whitepaper_files = [
    'website/whitepaper/ivory-paper.md',
    'website/whitepaper/viewer-standalone.html',
    'website/whitepaper/viewer-embedded.html',
]

print("=" * 70)
print("FIXING ALL BROKEN LINKS")
print("=" * 70)

for filepath in whitepaper_files:
    if not os.path.exists(filepath):
        print(f"⚠️  Skipping {filepath} (not found)")
        continue

    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Fix etrid.io → etrid.org
    replacements = [
        ('docs.etrid.io', 'docs.etrid.org'),
        ('wiki.etrid.io', 'docs.etrid.org'),  # wiki doesn't exist, redirect to docs
        ('explorer.etrid.io', 'etrid.org/telemetry'),  # explorer doesn't exist, use telemetry
        ('vote.etrid.io', 'etrid.org'),  # vote doesn't exist, use main site
    ]

    for old, new in replacements:
        if old in content:
            content = content.replace(old, new)
            fixes_made += 1
            print(f"✅ Fixed {old} → {new} in {filepath}")

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)

# Fix validator page staking links
validator_files = [
    'website/validators/index.html',
    'website/validators/participate.html',
]

for filepath in validator_files:
    if not os.path.exists(filepath):
        print(f"⚠️  Skipping {filepath} (not found)")
        continue

    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Remove or replace wallet.etrid.org/staking links
    # Option 1: Remove the link entirely (make it a disabled button)
    content = re.sub(
        r'<a href="https://wallet\.etrid\.org/staking"([^>]*)>([^<]*)</a>',
        r'<span class="opacity-50 cursor-not-allowed" title="Coming Soon">\2 (Coming Soon)</span>',
        content
    )

    if content != original:
        fixes_made += 1
        print(f"✅ Fixed wallet.etrid.org/staking links in {filepath}")
        with open(filepath, 'w') as f:
            f.write(content)

print("=" * 70)
print(f"Total fixes made: {fixes_made}")
print("=" * 70)
