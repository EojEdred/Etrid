#!/usr/bin/env python3
"""
Test script to verify pyE structure without requiring full installation.
This checks that all modules can be imported and basic structure is correct.
"""

import sys
import os
from pathlib import Path

# Add pye to path
pye_dir = Path(__file__).parent
sys.path.insert(0, str(pye_dir))

def test_structure():
    """Test that all modules are structured correctly"""

    print("Testing pyE structure...\n")

    # Test package imports (without dependencies)
    try:
        # Check package exists
        assert (pye_dir / "pye" / "__init__.py").exists()
        print("✓ Package __init__.py exists")

        # Check main modules
        assert (pye_dir / "pye" / "cli.py").exists()
        print("✓ CLI module exists")

        assert (pye_dir / "pye" / "client.py").exists()
        print("✓ Client module exists")

        # Check commands package
        assert (pye_dir / "pye" / "commands" / "__init__.py").exists()
        print("✓ Commands package exists")

        # Check command modules
        commands = ["account", "stake", "query", "send", "consensus"]
        for cmd in commands:
            cmd_file = pye_dir / "pye" / "commands" / f"{cmd}.py"
            assert cmd_file.exists(), f"Missing {cmd}.py"
            print(f"✓ Command module '{cmd}' exists")

        # Check config files
        assert (pye_dir / "pyproject.toml").exists()
        print("✓ pyproject.toml exists")

        assert (pye_dir / "README.md").exists()
        print("✓ README.md exists")

        assert (pye_dir / "setup.py").exists()
        print("✓ setup.py exists")

        print("\n✓ All structure tests passed!")
        return True

    except AssertionError as e:
        print(f"\n✗ Structure test failed: {e}")
        return False
    except Exception as e:
        print(f"\n✗ Unexpected error: {e}")
        return False

def test_syntax():
    """Test that Python files have valid syntax"""

    print("\nTesting Python syntax...\n")

    import py_compile

    python_files = []
    for root, dirs, files in os.walk(pye_dir / "pye"):
        for file in files:
            if file.endswith(".py"):
                python_files.append(os.path.join(root, file))

    errors = []
    for py_file in python_files:
        try:
            py_compile.compile(py_file, doraise=True)
            rel_path = os.path.relpath(py_file, pye_dir)
            print(f"✓ {rel_path} - valid syntax")
        except py_compile.PyCompileError as e:
            errors.append((py_file, str(e)))
            print(f"✗ {py_file} - syntax error")

    if errors:
        print("\n✗ Syntax errors found:")
        for file, error in errors:
            print(f"  {file}: {error}")
        return False
    else:
        print("\n✓ All syntax tests passed!")
        return True

def main():
    """Run all tests"""

    print("=" * 60)
    print("pyE Structure and Syntax Verification")
    print("=" * 60)
    print()

    structure_ok = test_structure()
    syntax_ok = test_syntax()

    print("\n" + "=" * 60)
    if structure_ok and syntax_ok:
        print("✓ ALL TESTS PASSED")
        print("=" * 60)
        print("\nNext steps:")
        print("1. Install dependencies: pip3 install -r requirements.txt")
        print("2. Install pyE: pip3 install -e .")
        print("3. Test CLI: pye --version")
        return 0
    else:
        print("✗ SOME TESTS FAILED")
        print("=" * 60)
        return 1

if __name__ == "__main__":
    sys.exit(main())
