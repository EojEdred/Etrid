"""
pyE CLI - Main command-line interface

Inspired by Ethereum's Ape framework patterns.
"""

import click
from rich.console import Console
from rich.table import Table
from rich import print as rprint
from pye import __version__
from pye.commands import account, stake, query, send, consensus

console = Console()


@click.group()
@click.version_option(version=__version__, prog_name="pyE")
@click.option(
    "--node",
    "-n",
    default="ws://localhost:9944",
    envvar="ETRID_NODE_URL",
    help="ËTRID node URL (default: ws://localhost:9944)",
)
@click.option(
    "--verbose",
    "-v",
    is_flag=True,
    help="Enable verbose output",
)
@click.pass_context
def cli(ctx, node: str, verbose: bool):
    """
    🚀 pyE - ËTRID's Python Command-Line Interface

    A modern CLI for interacting with the ËTRID blockchain network.

    \b
    Examples:
        pye account create              # Create new account
        pye account list                # List all accounts
        pye query block                 # Query latest block
        pye send 0x123... 100           # Send 100 ETR
        pye stake deposit 1000          # Stake 1000 ETR
        pye consensus status            # Check consensus status

    \b
    Environment Variables:
        ETRID_NODE_URL    Node WebSocket URL
        ETRID_KEYSTORE    Custom keystore directory
    """
    ctx.ensure_object(dict)
    ctx.obj["node_url"] = node
    ctx.obj["verbose"] = verbose


# Register command groups
cli.add_command(account.account)
cli.add_command(stake.stake)
cli.add_command(query.query)
cli.add_command(send.send)
cli.add_command(consensus.consensus)


@cli.command()
def info():
    """Display ËTRID network information"""
    from pye.client import EtridClient, EtridRPCError

    try:
        with EtridClient() as client:
            console.print("\n[bold cyan]ËTRID Network Information[/bold cyan]\n")

            # Chain info
            chain_info = client.get_chain_info()
            health = client.get_node_health()
            sync_state = client.get_sync_state()

            table = Table(show_header=True, header_style="bold magenta")
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Chain", str(chain_info.get("chain", "ËTRID")))
            table.add_row("Node Health", "✓ Healthy" if health.get("is_healthy") else "✗ Unhealthy")
            table.add_row("Syncing", "Yes" if sync_state.get("is_syncing") else "No")
            table.add_row("Peers", str(health.get("peers", "N/A")))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"[red]Error:[/red] {e}")
        raise click.Abort()


@cli.command()
def version():
    """Display pyE version"""
    console.print(f"\n[bold cyan]pyE[/bold cyan] version [green]{__version__}[/green]")
    console.print("[dim]ËTRID's Python Command-Line Interface[/dim]\n")


def main():
    """Main entry point for the CLI"""
    try:
        cli(obj={})
    except KeyboardInterrupt:
        console.print("\n[yellow]Operation cancelled by user[/yellow]")
    except Exception as e:
        console.print(f"\n[red]Fatal error:[/red] {e}")
        if "--verbose" in click.get_current_context().params:
            raise


if __name__ == "__main__":
    main()
