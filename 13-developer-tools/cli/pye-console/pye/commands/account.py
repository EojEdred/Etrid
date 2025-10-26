"""
Account management commands

Create, list, import, and export ËTRID accounts.
"""

import os
import json
import secrets
from pathlib import Path
from typing import Optional
import click
from rich.console import Console
from rich.table import Table
from rich.prompt import Prompt, Confirm
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.backends import default_backend

console = Console()

# Default keystore location
KEYSTORE_DIR = Path.home() / ".etrid" / "keystore"


def get_keystore_dir() -> Path:
    """Get keystore directory from environment or default"""
    custom_dir = os.getenv("ETRID_KEYSTORE")
    if custom_dir:
        return Path(custom_dir)
    return KEYSTORE_DIR


def ensure_keystore_dir() -> Path:
    """Ensure keystore directory exists"""
    keystore = get_keystore_dir()
    keystore.mkdir(parents=True, exist_ok=True)
    return keystore


def generate_keypair():
    """Generate Ed25519 keypair"""
    private_key = ed25519.Ed25519PrivateKey.generate()
    public_key = private_key.public_key()
    return private_key, public_key


def save_account(name: str, private_key, public_key, password: Optional[str] = None) -> Path:
    """Save account to keystore"""
    keystore = ensure_keystore_dir()

    # Serialize keys
    private_bytes = private_key.private_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PrivateFormat.PKCS8,
        encryption_algorithm=serialization.BestAvailableEncryption(password.encode())
        if password
        else serialization.NoEncryption(),
    )

    public_bytes = public_key.public_bytes(
        encoding=serialization.Encoding.PEM,
        format=serialization.PublicFormat.SubjectPublicKeyInfo,
    )

    # Generate address (simplified - first 20 bytes of public key hash)
    address = "0x" + public_bytes.hex()[:40]

    # Account data
    account_data = {
        "name": name,
        "address": address,
        "public_key": public_bytes.hex(),
        "encrypted": password is not None,
        "version": 1,
    }

    # Save account file
    account_file = keystore / f"{name}.json"
    with open(account_file, "w") as f:
        json.dump(account_data, f, indent=2)

    # Save private key separately
    key_file = keystore / f"{name}.key"
    with open(key_file, "wb") as f:
        f.write(private_bytes)

    # Set restrictive permissions
    os.chmod(key_file, 0o600)

    return account_file


def list_accounts() -> list:
    """List all accounts in keystore"""
    keystore = get_keystore_dir()
    if not keystore.exists():
        return []

    accounts = []
    for account_file in keystore.glob("*.json"):
        try:
            with open(account_file) as f:
                account_data = json.load(f)
                accounts.append(account_data)
        except Exception as e:
            console.print(f"[yellow]Warning: Failed to load {account_file.name}: {e}[/yellow]")

    return accounts


@click.group()
def account():
    """Manage ËTRID accounts"""
    pass


@account.command()
@click.argument("name", required=False)
@click.option("--password", "-p", help="Password to encrypt private key")
@click.option("--no-password", is_flag=True, help="Don't encrypt private key")
def create(name: Optional[str], password: Optional[str], no_password: bool):
    """
    Create a new ËTRID account

    \b
    Examples:
        pye account create              # Interactive
        pye account create alice        # Create account named 'alice'
        pye account create alice -p pwd # Create with password
    """
    console.print("\n[bold cyan]Creating new ËTRID account[/bold cyan]\n")

    # Get account name
    if not name:
        name = Prompt.ask("Account name")

    # Check if account already exists
    keystore = get_keystore_dir()
    account_file = keystore / f"{name}.json"
    if account_file.exists():
        console.print(f"[red]Error:[/red] Account '{name}' already exists")
        raise click.Abort()

    # Get password
    if not no_password and not password:
        use_password = Confirm.ask("Encrypt private key with password?", default=True)
        if use_password:
            password = Prompt.ask("Enter password", password=True)
            password_confirm = Prompt.ask("Confirm password", password=True)
            if password != password_confirm:
                console.print("[red]Error:[/red] Passwords do not match")
                raise click.Abort()

    # Generate keypair
    with console.status("[bold green]Generating keypair..."):
        private_key, public_key = generate_keypair()

    # Save account
    try:
        account_path = save_account(name, private_key, public_key, password)

        # Load account data to display
        with open(account_path) as f:
            account_data = json.load(f)

        console.print("\n[green]✓[/green] Account created successfully!\n")

        table = Table(show_header=False)
        table.add_column("Property", style="cyan")
        table.add_column("Value", style="green")

        table.add_row("Name", account_data["name"])
        table.add_row("Address", account_data["address"])
        table.add_row("Encrypted", "Yes" if account_data["encrypted"] else "No")
        table.add_row("Location", str(account_path))

        console.print(table)
        console.print()

        console.print("[yellow]⚠ Keep your private key safe and never share it![/yellow]\n")

    except Exception as e:
        console.print(f"[red]Error creating account:[/red] {e}")
        raise click.Abort()


@account.command()
@click.option("--format", "-f", type=click.Choice(["table", "json"]), default="table")
def list(format: str):
    """
    List all accounts

    \b
    Examples:
        pye account list            # Table format
        pye account list -f json    # JSON format
    """
    accounts = list_accounts()

    if not accounts:
        console.print("\n[yellow]No accounts found.[/yellow]")
        console.print(f"[dim]Keystore: {get_keystore_dir()}[/dim]\n")
        console.print("Create an account with: [cyan]pye account create[/cyan]\n")
        return

    if format == "json":
        console.print_json(data=accounts)
        return

    # Table format
    console.print(f"\n[bold cyan]ËTRID Accounts[/bold cyan] ({len(accounts)})\n")

    table = Table(show_header=True, header_style="bold magenta")
    table.add_column("Name", style="cyan")
    table.add_column("Address", style="green")
    table.add_column("Encrypted", style="yellow")

    for acc in accounts:
        table.add_row(
            acc["name"],
            acc["address"],
            "✓" if acc.get("encrypted") else "✗",
        )

    console.print(table)
    console.print(f"\n[dim]Keystore: {get_keystore_dir()}[/dim]\n")


@account.command()
@click.argument("name")
def show(name: str):
    """
    Show account details

    \b
    Examples:
        pye account show alice
    """
    keystore = get_keystore_dir()
    account_file = keystore / f"{name}.json"

    if not account_file.exists():
        console.print(f"[red]Error:[/red] Account '{name}' not found")
        raise click.Abort()

    try:
        with open(account_file) as f:
            account_data = json.load(f)

        console.print(f"\n[bold cyan]Account: {name}[/bold cyan]\n")

        table = Table(show_header=False)
        table.add_column("Property", style="cyan")
        table.add_column("Value", style="green")

        table.add_row("Name", account_data["name"])
        table.add_row("Address", account_data["address"])
        table.add_row("Public Key", account_data["public_key"][:64] + "...")
        table.add_row("Encrypted", "Yes" if account_data.get("encrypted") else "No")
        table.add_row("Version", str(account_data.get("version", 1)))

        console.print(table)
        console.print()

    except Exception as e:
        console.print(f"[red]Error:[/red] {e}")
        raise click.Abort()


@account.command()
@click.argument("name")
@click.option("--output", "-o", type=click.Path(), help="Output file (default: stdout)")
def export(name: str, output: Optional[str]):
    """
    Export account public key

    \b
    Examples:
        pye account export alice              # Print to stdout
        pye account export alice -o key.json  # Save to file
    """
    keystore = get_keystore_dir()
    account_file = keystore / f"{name}.json"

    if not account_file.exists():
        console.print(f"[red]Error:[/red] Account '{name}' not found")
        raise click.Abort()

    try:
        with open(account_file) as f:
            account_data = json.load(f)

        # Export data (public key only)
        export_data = {
            "name": account_data["name"],
            "address": account_data["address"],
            "public_key": account_data["public_key"],
        }

        if output:
            with open(output, "w") as f:
                json.dump(export_data, f, indent=2)
            console.print(f"\n[green]✓[/green] Account exported to: {output}\n")
        else:
            console.print_json(data=export_data)

    except Exception as e:
        console.print(f"[red]Error:[/red] {e}")
        raise click.Abort()


@account.command()
@click.argument("name")
@click.confirmation_option(prompt="Are you sure you want to delete this account?")
def delete(name: str):
    """
    Delete an account

    \b
    Examples:
        pye account delete alice
    """
    keystore = get_keystore_dir()
    account_file = keystore / f"{name}.json"
    key_file = keystore / f"{name}.key"

    if not account_file.exists():
        console.print(f"[red]Error:[/red] Account '{name}' not found")
        raise click.Abort()

    try:
        # Delete files
        account_file.unlink()
        if key_file.exists():
            key_file.unlink()

        console.print(f"\n[green]✓[/green] Account '{name}' deleted\n")

    except Exception as e:
        console.print(f"[red]Error:[/red] {e}")
        raise click.Abort()
