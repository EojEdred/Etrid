#!/usr/bin/env python3
"""Create separate static HTML files for each whitepaper document"""

import os
import markdown
from pathlib import Path

# Paths
SCRIPT_DIR = Path(__file__).parent
DOCS_DIR = SCRIPT_DIR.parent / "docs" / "specifications"
OUTPUT_DIR = SCRIPT_DIR / "website" / "whitepaper"

# Documents to create
DOCUMENTS = [
    {
        'key': 'complete',
        'title': 'ËTRID Ivory Paper v2.0 - Complete Edition',
        'source': 'ivory-paper.md',
        'output': 'complete-edition.html'
    },
    {
        'key': 'vol1',
        'title': 'Volume I: Conceptual Architecture',
        'source': 'ivory-paper-vol1-conceptual.md',
        'output': 'volume-1.html'
    },
    {
        'key': 'vol2',
        'title': 'Volume II: Technical Specification',
        'source': 'ivory-paper-vol2-technical.md',
        'output': 'volume-2.html'
    },
    {
        'key': 'vol3',
        'title': 'Volume III: Governance & Fiscal Mechanics',
        'source': 'ivory-paper-vol3-governance.md',
        'output': 'volume-3.html'
    },
    {
        'key': 'charter',
        'title': 'ËTRID Protocol Charter',
        'source': 'protocol-charter.md',
        'output': 'protocol-charter.html'
    }
]

HTML_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - ËTRID</title>

    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        :root {{
            --primary-color: #00d4ff;
            --secondary-color: #0077ff;
            --dark-bg: #0a0e1a;
            --card-bg: #151b2e;
            --text-primary: #ffffff;
            --text-secondary: #a0aec0;
            --accent: #00ff88;
        }}

        body {{
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, var(--dark-bg) 0%, #0f1729 100%);
            color: var(--text-primary);
            line-height: 1.8;
            min-height: 100vh;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}

        header {{
            text-align: center;
            padding: 3rem 2rem 2rem;
            background: linear-gradient(180deg, rgba(0,212,255,0.05) 0%, transparent 100%);
            border-bottom: 1px solid rgba(0,212,255,0.1);
        }}

        h1 {{
            font-size: 2.5rem;
            font-weight: 800;
            background: linear-gradient(135deg, var(--primary-color) 0%, var(--accent) 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 1rem;
        }}

        .back-link {{
            display: inline-block;
            padding: 0.8rem 1.5rem;
            background: var(--card-bg);
            border: 2px solid var(--primary-color);
            border-radius: 8px;
            color: var(--primary-color);
            text-decoration: none;
            margin: 1rem 0 2rem;
            transition: all 0.3s ease;
        }}

        .back-link:hover {{
            background: rgba(0,212,255,0.1);
            transform: translateY(-2px);
        }}

        .content {{
            background: var(--card-bg);
            border-radius: 16px;
            padding: 3rem;
            margin: 2rem 0;
            box-shadow: 0 10px 40px rgba(0,0,0,0.3);
        }}

        .content h1 {{
            color: var(--primary-color);
            font-size: 2.5rem;
            margin-top: 2rem;
            margin-bottom: 1rem;
            border-bottom: 2px solid var(--primary-color);
            padding-bottom: 0.5rem;
        }}

        .content h2 {{
            color: var(--primary-color);
            font-size: 2rem;
            margin-top: 2rem;
            margin-bottom: 1rem;
        }}

        .content h3 {{
            color: var(--accent);
            font-size: 1.5rem;
            margin-top: 1.5rem;
            margin-bottom: 0.8rem;
        }}

        .content h4 {{
            color: var(--text-primary);
            font-size: 1.2rem;
            margin-top: 1.2rem;
            margin-bottom: 0.6rem;
        }}

        .content p {{
            margin: 1rem 0;
            color: var(--text-secondary);
            line-height: 1.8;
        }}

        .content ul, .content ol {{
            margin: 1rem 0 1rem 2rem;
            color: var(--text-secondary);
        }}

        .content li {{
            margin: 0.5rem 0;
        }}

        .content code {{
            background: rgba(0,212,255,0.1);
            padding: 0.2rem 0.5rem;
            border-radius: 4px;
            font-family: 'Courier New', monospace;
            color: var(--accent);
            font-size: 0.9em;
        }}

        .content pre {{
            background: rgba(0,0,0,0.5);
            padding: 1.5rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1.5rem 0;
            border-left: 4px solid var(--primary-color);
        }}

        .content pre code {{
            background: none;
            padding: 0;
            color: var(--text-primary);
        }}

        .content blockquote {{
            border-left: 4px solid var(--accent);
            padding-left: 1.5rem;
            margin: 1.5rem 0;
            color: var(--text-secondary);
            font-style: italic;
        }}

        .content table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1.5rem 0;
        }}

        .content table th,
        .content table td {{
            padding: 1rem;
            border: 1px solid rgba(0,212,255,0.2);
            text-align: left;
        }}

        .content table th {{
            background: rgba(0,212,255,0.1);
            color: var(--primary-color);
            font-weight: 600;
        }}

        .content a {{
            color: var(--primary-color);
            text-decoration: none;
            border-bottom: 1px solid transparent;
            transition: border-color 0.3s ease;
        }}

        .content a:hover {{
            border-bottom-color: var(--primary-color);
        }}

        .back-to-top {{
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            width: 50px;
            height: 50px;
            background: var(--primary-color);
            color: var(--dark-bg);
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
            cursor: pointer;
            opacity: 0;
            transition: opacity 0.3s ease;
            text-decoration: none;
        }}

        .back-to-top.visible {{
            opacity: 1;
        }}

        footer {{
            text-align: center;
            padding: 2rem;
            color: var(--text-secondary);
            margin-top: 3rem;
        }}
    </style>
</head>
<body>
    <header>
        <h1>{title}</h1>
        <a href="index.html" class="back-link">← Back to All Documents</a>
    </header>

    <div class="container">
        <div class="content">
{content}
        </div>
    </div>

    <a href="#" class="back-to-top" id="back-to-top">↑</a>

    <footer>
        <p>&copy; 2025 ËTRID Foundation. All rights reserved.</p>
    </footer>

    <script>
        // Back to top button
        const backToTop = document.getElementById('back-to-top');
        window.addEventListener('scroll', () => {{
            if (window.scrollY > 300) {{
                backToTop.classList.add('visible');
            }} else {{
                backToTop.classList.remove('visible');
            }}
        }});

        backToTop.addEventListener('click', (e) => {{
            e.preventDefault();
            window.scrollTo({{ top: 0, behavior: 'smooth' }});
        }});
    </script>
</body>
</html>
"""

INDEX_TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ËTRID Ivory Papers - Documentation</title>

    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        :root {{
            --primary-color: #00d4ff;
            --secondary-color: #0077ff;
            --dark-bg: #0a0e1a;
            --card-bg: #151b2e;
            --text-primary: #ffffff;
            --text-secondary: #a0aec0;
            --accent: #00ff88;
        }}

        body {{
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, var(--dark-bg) 0%, #0f1729 100%);
            color: var(--text-primary);
            line-height: 1.6;
            min-height: 100vh;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }}

        header {{
            text-align: center;
            padding: 4rem 2rem 2rem;
            background: linear-gradient(180deg, rgba(0,212,255,0.05) 0%, transparent 100%);
        }}

        h1 {{
            font-size: 3rem;
            font-weight: 800;
            background: linear-gradient(135deg, var(--primary-color) 0%, var(--accent) 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 1rem;
        }}

        .subtitle {{
            font-size: 1.2rem;
            color: var(--text-secondary);
            margin-bottom: 2rem;
        }}

        .documents-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 2rem;
            margin: 3rem 0;
        }}

        .doc-card {{
            background: var(--card-bg);
            border: 2px solid transparent;
            border-radius: 16px;
            padding: 2rem;
            transition: all 0.3s ease;
            text-decoration: none;
            color: inherit;
            display: block;
        }}

        .doc-card:hover {{
            border-color: var(--primary-color);
            transform: translateY(-5px);
            box-shadow: 0 10px 30px rgba(0,212,255,0.2);
        }}

        .doc-card h2 {{
            color: var(--primary-color);
            font-size: 1.5rem;
            margin-bottom: 1rem;
        }}

        .doc-card p {{
            color: var(--text-secondary);
            margin-bottom: 1rem;
        }}

        .doc-card .arrow {{
            color: var(--accent);
            font-size: 1.5rem;
        }}

        footer {{
            text-align: center;
            padding: 2rem;
            color: var(--text-secondary);
            margin-top: 3rem;
        }}
    </style>
</head>
<body>
    <header>
        <h1>ËTRID Ivory Papers</h1>
        <p class="subtitle">Technical Documentation & Protocol Specifications</p>
    </header>

    <div class="container">
        <div class="documents-grid">
            <a href="complete-edition.html" class="doc-card">
                <h2>Complete Edition v2.0</h2>
                <p>Full ËTRID Ivory Paper with all specifications</p>
                <span class="arrow">→</span>
            </a>

            <a href="volume-1.html" class="doc-card">
                <h2>Volume I</h2>
                <p>Conceptual Architecture</p>
                <span class="arrow">→</span>
            </a>

            <a href="volume-2.html" class="doc-card">
                <h2>Volume II</h2>
                <p>Technical Specification</p>
                <span class="arrow">→</span>
            </a>

            <a href="volume-3.html" class="doc-card">
                <h2>Volume III</h2>
                <p>Governance & Fiscal Mechanics</p>
                <span class="arrow">→</span>
            </a>

            <a href="protocol-charter.html" class="doc-card">
                <h2>Protocol Charter</h2>
                <p>ËTRID Foundation Charter & Governance</p>
                <span class="arrow">→</span>
            </a>
        </div>
    </div>

    <footer>
        <p>&copy; 2025 ËTRID Foundation. All rights reserved.</p>
    </footer>
</body>
</html>
"""

def create_static_pages():
    """Create static HTML pages for each document"""

    print("=" * 70)
    print("Creating Static Whitepaper Pages")
    print("=" * 70)

    # Create each document page
    for doc in DOCUMENTS:
        source_path = DOCS_DIR / doc['source']
        output_path = OUTPUT_DIR / doc['output']

        print(f"\n📄 Creating: {doc['output']}")
        print(f"   Source: {doc['source']}")

        if not source_path.exists():
            print(f"   ❌ Source not found: {source_path}")
            continue

        # Read markdown
        with open(source_path, 'r', encoding='utf-8') as f:
            md_content = f.read()

        # Convert to HTML
        html_content = markdown.markdown(
            md_content,
            extensions=['extra', 'codehilite', 'toc']
        )

        # Create full HTML page
        full_html = HTML_TEMPLATE.format(
            title=doc['title'],
            content=html_content
        )

        # Save
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(full_html)

        size = len(full_html)
        print(f"   ✅ Created: {size:,} bytes ({size/1024:.1f} KB)")

    # Create index page
    print(f"\n📄 Creating: index.html")
    index_path = OUTPUT_DIR / "index.html"
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write(INDEX_TEMPLATE)
    print(f"   ✅ Created index page")

    print(f"\n{'='*70}")
    print(f"✅ All static pages created!")
    print(f"{'='*70}")
    print(f"\nFiles created in: {OUTPUT_DIR}")
    print(f"   • index.html (landing page)")
    for doc in DOCUMENTS:
        print(f"   • {doc['output']}")

if __name__ == "__main__":
    create_static_pages()
