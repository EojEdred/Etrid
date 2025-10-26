"""
Transaction sending commands

Send ETR tokens and submit transactions.
"""

import click
from rich.console import Console
from rich.prompt import Confirm
from pye.client import EtridClient, EtridRPCError

console = Console()


@click.command()
@click.argument("to_address")
@click.argument("amount", type=float)
@click.option("--from", "-f", "from_address", required=True, help="Sender account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.option("--yes", "-y", is_flag=True, help="Skip confirmation prompt")
def send(to_address: str, amount: float, from_address: str, node: str, yes: bool):
    """
    Send ETR tokens to an address

    \b
    Examples:
        pye send 0x123... 100 -f alice      # Send 100 ETR from alice
        pye send bob 50.5 -f alice          # Send 50.5 ETR to bob
        pye send 0x456... 1000 -f 0x789... -y  # Skip confirmation
    """
    console.print(f"\n[bold cyan]Sending {amount} ETR[/bold cyan]\n")
    console.print(f"[cyan]From:[/cyan] {from_address}")
    console.print(f"[cyan]To:[/cyan]   {to_address}")
    console.print(f"[cyan]Amount:[/cyan] {amount} ETR\n")

    # Confirm transaction
    if not yes:
        if not Confirm.ask("Send transaction?"):
            console.print("[yellow]Transaction cancelled[/yellow]")
            return

    try:
        with EtridClient(ws_url=node) as client:
            # Check sender balance first
            with console.status("[bold green]Checking balance..."):
                try:
                    balance_data = client.get_balance(from_address)
                    free_balance = balance_data.get("free", 0)

                    if free_balance < amount:
                        console.print(f"\n[red]Error:[/red] Insufficient balance")
                        console.print(f"Available: {free_balance} ETR, Required: {amount} ETR\n")
                        raise click.Abort()

                except EtridRPCError:
                    console.print("[yellow]Warning: Could not verify balance[/yellow]")

            # Send transaction
            with console.status("[bold green]Sending transaction..."):
                tx_hash = client.send_transaction(
                    from_addr=from_address,
                    to_addr=to_address,
                    amount=amount,
                )

            console.print(f"\n[green]✓[/green] Transaction sent successfully!\n")
            console.print(f"[cyan]Transaction Hash:[/cyan] {tx_hash}\n")
            console.print("[dim]Use 'pye query transaction' to check status[/dim]\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()
