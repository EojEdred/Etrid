# pyE Installation Guide

This guide provides multiple installation methods for pyE - ËTRID's Python CLI.

## Prerequisites

- Python 3.8 or higher
- pip (Python package manager)

## Installation Methods

### Method 1: Using Virtual Environment (Recommended)

This is the recommended method to avoid permission issues:

```bash
# Navigate to the pye directory
cd /Users/macbook/Desktop/etrid/13-clients/pye

# Create virtual environment
python3 -m venv venv

# Activate virtual environment
source venv/bin/activate

# Upgrade pip
pip install --upgrade pip

# Install pyE
pip install -e .

# Test installation
pye --version
```

When you want to use pyE later, always activate the virtual environment first:

```bash
cd /Users/macbook/Desktop/etrid/13-clients/pye
source venv/bin/activate
pye --help
```

### Method 2: Using pipx (Isolated CLI Tools)

pipx installs CLI tools in isolated environments:

```bash
# Install pipx if not already installed
python3 -m pip install --user pipx
python3 -m pipx ensurepath

# Install pyE
pipx install /Users/macbook/Desktop/etrid/13-clients/pye

# Test installation
pye --version
```

### Method 3: System-wide with sudo (Not Recommended)

Only use this if you have admin access and understand the implications:

```bash
cd /Users/macbook/Desktop/etrid/13-clients/pye
sudo pip3 install -e .
pye --version
```

### Method 4: Direct Execution (Development/Testing)

For development or testing without full installation:

```bash
cd /Users/macbook/Desktop/etrid/13-clients/pye

# Install dependencies only
pip3 install --user click requests websocket-client rich pydantic cryptography

# Run directly with Python
python3 -m pye.cli --help

# Or create an alias
alias pye="python3 -m pye.cli"
pye --help
```

## Verifying Installation

After installation, verify pyE is working:

```bash
# Check version
pye --version

# Get help
pye --help

# Test network connection (requires running ËTRID node)
pye info

# Create test account
pye account create test
```

## Troubleshooting

### Permission Denied Errors

If you get permission errors, use the virtual environment method (Method 1) or pipx (Method 2).

### Module Not Found Errors

Make sure all dependencies are installed:

```bash
pip3 install click requests websocket-client rich pydantic cryptography
```

### Command Not Found

If `pye` command is not found after installation:

1. Check if the Python scripts directory is in your PATH:
   ```bash
   echo $PATH
   ```

2. Find where pip installed the script:
   ```bash
   python3 -m pip show pye
   ```

3. Add the scripts directory to your PATH in `~/.zshrc` or `~/.bash_profile`:
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

### Node Connection Issues

If pyE can't connect to an ËTRID node:

1. Make sure a node is running on `ws://localhost:9944`
2. Use a custom node URL:
   ```bash
   pye --node ws://your-node:9944 info
   ```
3. Set the environment variable:
   ```bash
   export ETRID_NODE_URL=ws://your-node:9944
   pye info
   ```

## Uninstallation

To uninstall pyE:

```bash
# If installed with pip
pip3 uninstall pye

# If installed with pipx
pipx uninstall pye

# If using virtual environment, just delete it
rm -rf /Users/macbook/Desktop/etrid/13-clients/pye/venv
```

## Development Setup

For development work on pyE:

```bash
cd /Users/macbook/Desktop/etrid/13-clients/pye

# Create virtual environment
python3 -m venv venv
source venv/bin/activate

# Install with dev dependencies
pip install -e ".[dev]"

# Run tests
pytest tests/

# Format code
black pye/

# Lint code
ruff check pye/
```

## Getting Help

If you encounter issues:

1. Check the [README.md](README.md) for usage examples
2. Check ËTRID documentation: https://docs.etrid.io
3. Open an issue: https://github.com/etrid/etrid/issues
4. Join Discord: https://discord.gg/etrid
