#!/usr/bin/env python3
"""Embed marked.js library into whitepaper viewer"""

# Read marked.js
with open('/tmp/marked.min.js', 'r') as f:
    marked_js = f.read()

# Read current HTML
html_file = '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html'
with open(html_file, 'r', encoding='utf-8') as f:
    html_content = f.read()

# Find where to insert marked.js (before embeddedPapers)
insert_marker = "    <script>        // Embedded paper content"

if insert_marker not in html_content:
    print("ERROR: Could not find insertion point!")
    exit(1)

# Create new script tag with marked.js
marked_script = f"    <script>\n        // Marked.js v4.3.0 - Markdown Parser (embedded)\n        {marked_js}\n    </script>\n\n"

# Insert marked.js before the embeddedPapers script
html_content = html_content.replace(insert_marker, marked_script + insert_marker)

# Save
output_file = '/Users/macbook/Desktop/etrid/etrid-hostinger-deployment /website/whitepaper/viewer-standalone.html'
with open(output_file, 'w', encoding='utf-8') as f:
    f.write(html_content)

print(f"✅ Embedded marked.js into viewer")
print(f"   marked.js size: {len(marked_js):,} bytes")
print(f"   New HTML size: {len(html_content):,} bytes")
print(f"   Saved to: {output_file}")
