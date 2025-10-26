"""
Staking commands

Manage ETR token staking and validator operations.
"""

import click
from rich.console import Console
from rich.table import Table
from rich.prompt import Confirm
from pye.client import EtridClient, EtridRPCError

console = Console()


@click.group()
def stake():
    """Manage ETR token staking"""
    pass


@stake.command()
@click.argument("amount", type=float)
@click.option("--account", "-a", required=True, help="Account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.pass_context
def deposit(ctx, amount: float, account: str, node: str):
    """
    Stake ETR tokens

    \b
    Examples:
        pye stake deposit 1000 -a alice
        pye stake deposit 5000 -a 0x123...
    """
    console.print(f"\n[bold cyan]Staking {amount} ETR[/bold cyan]\n")

    # Confirm action
    if not Confirm.ask(f"Stake {amount} ETR from account '{account}'?"):
        console.print("[yellow]Operation cancelled[/yellow]")
        return

    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Submitting stake transaction..."):
                tx_hash = client.stake(account, amount)

            console.print(f"\n[green]✓[/green] Stake transaction submitted!\n")
            console.print(f"[cyan]Transaction Hash:[/cyan] {tx_hash}\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@stake.command()
@click.argument("amount", type=float)
@click.option("--account", "-a", required=True, help="Account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.pass_context
def withdraw(ctx, amount: float, account: str, node: str):
    """
    Unstake ETR tokens

    \b
    Examples:
        pye stake withdraw 500 -a alice
        pye stake withdraw 1000 -a 0x123...
    """
    console.print(f"\n[bold cyan]Unstaking {amount} ETR[/bold cyan]\n")

    # Confirm action
    if not Confirm.ask(f"Unstake {amount} ETR from account '{account}'?"):
        console.print("[yellow]Operation cancelled[/yellow]")
        return

    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Submitting unstake transaction..."):
                tx_hash = client.unstake(account, amount)

            console.print(f"\n[green]✓[/green] Unstake transaction submitted!\n")
            console.print(f"[cyan]Transaction Hash:[/cyan] {tx_hash}\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@stake.command()
@click.option("--account", "-a", required=True, help="Account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.pass_context
def info(ctx, account: str, node: str):
    """
    Query staking information

    \b
    Examples:
        pye stake info -a alice
        pye stake info -a 0x123...
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching stake info..."):
                stake_info = client.get_stake_info(account)

            console.print(f"\n[bold cyan]Staking Info: {account}[/bold cyan]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Total Staked", f"{stake_info.get('total_staked', 0)} ETR")
            table.add_row("Active Stake", f"{stake_info.get('active_stake', 0)} ETR")
            table.add_row("Pending Unstake", f"{stake_info.get('pending_unstake', 0)} ETR")
            table.add_row("Rewards Earned", f"{stake_info.get('rewards', 0)} ETR")
            table.add_row("Validator Status", stake_info.get("is_validator", False))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@stake.command()
@click.option("--account", "-a", required=True, help="Account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.pass_context
def rewards(ctx, account: str, node: str):
    """
    Query staking rewards

    \b
    Examples:
        pye stake rewards -a alice
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching rewards..."):
                stake_info = client.get_stake_info(account)

            rewards_amount = stake_info.get("rewards", 0)

            console.print(f"\n[bold cyan]Staking Rewards: {account}[/bold cyan]\n")
            console.print(f"[green]Total Rewards:[/green] {rewards_amount} ETR\n")

            if rewards_amount > 0:
                console.print("[dim]Use 'pye stake claim' to claim rewards[/dim]\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@stake.command()
@click.option("--account", "-a", required=True, help="Account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.pass_context
def claim(ctx, account: str, node: str):
    """
    Claim staking rewards

    \b
    Examples:
        pye stake claim -a alice
    """
    console.print(f"\n[bold cyan]Claiming Staking Rewards[/bold cyan]\n")

    try:
        with EtridClient(ws_url=node) as client:
            # Get current rewards
            stake_info = client.get_stake_info(account)
            rewards_amount = stake_info.get("rewards", 0)

            if rewards_amount <= 0:
                console.print("[yellow]No rewards to claim[/yellow]\n")
                return

            console.print(f"Available rewards: [green]{rewards_amount} ETR[/green]\n")

            if not Confirm.ask("Claim rewards?"):
                console.print("[yellow]Operation cancelled[/yellow]")
                return

            with console.status("[bold green]Submitting claim transaction..."):
                tx_hash = client._send_request("staking_claimRewards", [account])

            console.print(f"\n[green]✓[/green] Rewards claim submitted!\n")
            console.print(f"[cyan]Transaction Hash:[/cyan] {tx_hash}\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@stake.command()
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
@click.pass_context
def validators(ctx, node: str):
    """
    List active validators

    \b
    Examples:
        pye stake validators
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching validators..."):
                validators_data = client._send_request("staking_getValidators", [])

            console.print(f"\n[bold cyan]Active Validators[/bold cyan] ({len(validators_data)})\n")

            table = Table(show_header=True, header_style="bold magenta")
            table.add_column("Address", style="cyan")
            table.add_column("Stake", style="green", justify="right")
            table.add_column("Commission", style="yellow", justify="right")
            table.add_column("Status", style="blue")

            for validator in validators_data:
                table.add_row(
                    validator.get("address", "")[:16] + "...",
                    f"{validator.get('stake', 0)} ETR",
                    f"{validator.get('commission', 0)}%",
                    "Active" if validator.get("active") else "Inactive",
                )

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()
