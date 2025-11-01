#!/usr/bin/env python3
"""Remove all AOS (Animate On Scroll) dependencies from whitepaper viewer"""

import re

input_file = '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html'
output_file = '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone-FIXED.html'

print("🔧 Removing AOS dependencies from whitepaper viewer...")

with open(input_file, 'r', encoding='utf-8') as f:
    content = f.read()

print(f"📄 Original file size: {len(content):,} characters")

# Remove AOS CSS link
content = re.sub(r'<link[^>]*unpkg\.com/aos[^>]*>', '', content)

# Remove AOS JS script
content = re.sub(r'<script[^>]*unpkg\.com/aos[^>]*</script>', '', content, flags=re.DOTALL)

# Remove AOS comments
content = re.sub(r'<!--.*?AOS.*?-->', '', content, flags=re.DOTALL)

# Remove data-aos attributes (all variations)
content = re.sub(r'\s*data-aos[^=]*="[^"]*"', '', content)
content = re.sub(r"\s*data-aos[^=]*='[^']*'", '', content)

# Remove AOS initialization code
content = re.sub(r'AOS\.init\([^)]*\);?', '', content)
content = re.sub(r'if\s*\(\s*typeof\s+AOS[^}]*\}', '', content, flags=re.DOTALL)

# Remove standalone AOS references
content = re.sub(r'//\s*Initialize AOS.*?\n', '', content)

# Clean up extra whitespace
content = re.sub(r'\n\s*\n\s*\n', '\n\n', content)

print(f"✅ Cleaned file size: {len(content):,} characters")

# Save cleaned version
with open(output_file, 'w', encoding='utf-8') as f:
    f.write(content)

print(f"💾 Saved to: {output_file}")

# Verify AOS is gone
if 'aos' in content.lower():
    aos_count = content.lower().count('aos')
    print(f"⚠️  Warning: Still found {aos_count} instances of 'aos' in file")
    # Find them
    lines = content.split('\n')
    for i, line in enumerate(lines, 1):
        if 'aos' in line.lower():
            print(f"   Line {i}: {line.strip()[:100]}")
else:
    print("✅ AOS completely removed!")

print("\n🚀 Now copy this file over:")
print(f"   cp '{output_file}' '{input_file}'")
