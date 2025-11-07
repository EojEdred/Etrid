#!/usr/bin/env python3
"""
Systematically simplify and fix the ËTRID website
"""

# Read original file
with open('website/index.html', 'r') as f:
    lines = f.readlines()

# Find key line numbers
flame_start = None
flame_end = None
for i, line in enumerate(lines):
    if '<!-- Flame Architecture Section -->' in line:
        flame_start = i
    if flame_start and '<!-- Governance Section -->' in line:
        flame_end = i
        break

print(f"Flame Architecture Section: lines {flame_start}-{flame_end}")
print(f"Total lines to remove: {flame_end - flame_start}")

# Remove flame architecture section
new_lines = lines[:flame_start] + lines[flame_end:]

# Fix navigation - remove Architecture link
output = []
for line in new_lines:
    # Skip architecture nav links
    if 'href="#flame-architecture"' in line and 'Architecture' in line:
        continue  # Skip this line entirely
    output.append(line)

# Write output
with open('website/index-simplified.html', 'w') as f:
    f.writelines(output)

print(f"\nOriginal file: {len(lines)} lines")
print(f"New file: {len(output)} lines")
print(f"Removed: {len(lines) - len(output)} lines")
print(f"\nSaved to: website/index-simplified.html")
