#!/usr/bin/env python3
"""
Update Ivory Papers Viewer with Latest Content
Regenerates viewer-standalone.html with updated Ivory Papers from docs/specifications
"""

import os
import json
import re
from pathlib import Path

# Paths
SCRIPT_DIR = Path(__file__).parent
DOCS_SPECS_DIR = SCRIPT_DIR.parent / "docs" / "specifications"
WHITEPAPER_DIR = SCRIPT_DIR / "website" / "whitepaper"
WHITEPAPER2_DIR = SCRIPT_DIR / "whitepaper 2"

# Paper definitions
PAPERS = [
    {
        'key': 'ivory-paper',
        'title': 'ËTRID Ivory Paper v2.0 - Complete Edition',
        'filename': 'ivory-paper.md'
    },
    {
        'key': 'vol1',
        'title': 'Volume I: Conceptual Architecture',
        'filename': 'ivory-paper-vol1-conceptual.md'
    },
    {
        'key': 'vol2',
        'title': 'Volume II: Technical Specification',
        'filename': 'ivory-paper-vol2-technical.md'
    },
    {
        'key': 'vol3',
        'title': 'Volume III: Governance & Fiscal Mechanics',
        'filename': 'ivory-paper-vol3-governance.md'
    },
    {
        'key': 'charter',
        'title': 'ËTRID Protocol Charter',
        'filename': 'protocol-charter.md'
    }
]

def read_markdown_file(filepath):
    """Read and return markdown content from file"""
    with open(filepath, 'r', encoding='utf-8') as f:
        return f.read()

def escape_js_string(content):
    """Escape content for JavaScript template literal"""
    # Escape backticks and ${} for template literal
    content = content.replace('\\', '\\\\')  # Escape backslashes first
    content = content.replace('`', '\\`')     # Escape backticks
    content = content.replace('${', '\\${')   # Escape template literal syntax
    return content

def generate_embedded_papers_js():
    """Generate JavaScript object with embedded papers"""
    js_code = "        // Embedded paper content\n"
    js_code += "        const embeddedPapers = {\n"

    for i, paper in enumerate(PAPERS):
        filepath = DOCS_SPECS_DIR / paper['filename']

        if not filepath.exists():
            print(f"Warning: {filepath} not found, skipping...")
            continue

        content = read_markdown_file(filepath)
        escaped_content = escape_js_string(content)

        js_code += f"    '{paper['key']}': {{\n"
        js_code += f"        title: '{paper['title']}',\n"
        js_code += f"        filename: '{paper['filename']}',\n"
        js_code += f"        content: `{escaped_content}`\n"
        js_code += "    }"

        if i < len(PAPERS) - 1:
            js_code += ","
        js_code += "\n"

    js_code += "        };\n"
    return js_code

def update_viewer_html(viewer_path):
    """Update viewer HTML with new embedded papers"""
    if not viewer_path.exists():
        print(f"Error: {viewer_path} not found!")
        return False

    # Read current HTML
    with open(viewer_path, 'r', encoding='utf-8') as f:
        html_content = f.read()

    # Generate new embedded papers JavaScript
    new_papers_js = generate_embedded_papers_js()

    # Find and replace the embeddedPapers object
    # Pattern matches from "// Embedded paper content" to the closing "};"
    # More flexible pattern that handles any indentation
    pattern = r'(\s*// Embedded paper content\s*\n\s*const embeddedPapers = \{).*?(\n\s*\};)'

    replacement = new_papers_js.rstrip('\n')

    updated_html = re.sub(pattern, replacement, html_content, flags=re.DOTALL)

    if updated_html == html_content:
        print(f"Warning: Pattern not found or no changes made to {viewer_path}")
        return False

    # Backup original
    backup_path = viewer_path.with_suffix('.html.backup-' +
                                           __import__('datetime').datetime.now().strftime('%Y%m%d-%H%M%S'))
    with open(backup_path, 'w', encoding='utf-8') as f:
        f.write(html_content)
    print(f"✓ Backed up original to: {backup_path.name}")

    # Write updated HTML
    with open(viewer_path, 'w', encoding='utf-8') as f:
        f.write(updated_html)

    return True

def main():
    print("=" * 70)
    print("IVORY PAPERS VIEWER UPDATER")
    print("=" * 70)
    print()

    # Check if source markdown files exist
    print("Checking source Ivory Papers...")
    for paper in PAPERS:
        filepath = DOCS_SPECS_DIR / paper['filename']
        if filepath.exists():
            size = filepath.stat().st_size
            print(f"  ✓ {paper['filename']}: {size:,} bytes")
        else:
            print(f"  ✗ {paper['filename']}: NOT FOUND")
    print()

    # Update viewer in /website/whitepaper/
    viewer1 = WHITEPAPER_DIR / "viewer-standalone.html"
    print(f"Updating: {viewer1.relative_to(SCRIPT_DIR)}")
    if update_viewer_html(viewer1):
        new_size = viewer1.stat().st_size
        print(f"  ✓ Updated successfully: {new_size:,} bytes")
    else:
        print(f"  ✗ Failed to update")
    print()

    # Update viewer in /whitepaper 2/
    viewer2 = WHITEPAPER2_DIR / "viewer-standalone.html"
    print(f"Updating: {viewer2.relative_to(SCRIPT_DIR)}")
    if update_viewer_html(viewer2):
        new_size = viewer2.stat().st_size
        print(f"  ✓ Updated successfully: {new_size:,} bytes")
    else:
        print(f"  ✗ Failed to update")
    print()

    print("=" * 70)
    print("DONE!")
    print("=" * 70)
    print()
    print("Next steps:")
    print("1. Test the updated viewer locally:")
    print(f"   open '{viewer1}'")
    print("2. Upload to Hostinger via FTP (see FIX_NOW.md)")
    print("3. Verify with check-file-integrity.html")
    print()

if __name__ == "__main__":
    main()
