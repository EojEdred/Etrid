"""
Query commands

Query blockchain state, blocks, transactions, and balances.
"""

import click
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich import print as rprint
from pye.client import EtridClient, EtridRPCError

console = Console()


@click.group()
def query():
    """Query blockchain state"""
    pass


@query.command()
@click.argument("block_hash", required=False)
@click.option("--number", "-n", type=int, help="Block number")
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def block(block_hash: str, number: int, node: str):
    """
    Query block information

    \b
    Examples:
        pye query block                    # Latest block
        pye query block -n 1000            # Block by number
        pye query block 0x123...           # Block by hash
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching block..."):
                if number is not None:
                    # Get block hash first
                    block_hash = client.get_block_hash(number)

                block_data = client.get_block(block_hash)

            # Display block info
            console.print(f"\n[bold cyan]Block Information[/bold cyan]\n")

            # Block header
            header = block_data.get("block", {}).get("header", {})

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Number", str(header.get("number", "N/A")))
            table.add_row("Hash", str(header.get("hash", block_hash))[:64] + "...")
            table.add_row("Parent Hash", str(header.get("parentHash", "N/A"))[:64] + "...")
            table.add_row("State Root", str(header.get("stateRoot", "N/A"))[:64] + "...")
            table.add_row("Extrinsics Root", str(header.get("extrinsicsRoot", "N/A"))[:64] + "...")
            table.add_row("Timestamp", str(header.get("timestamp", "N/A")))

            # Transaction count
            extrinsics = block_data.get("block", {}).get("extrinsics", [])
            table.add_row("Transactions", str(len(extrinsics)))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@query.command()
@click.argument("address")
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def balance(address: str, node: str):
    """
    Query account balance

    \b
    Examples:
        pye query balance 0x123...
        pye query balance alice
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching balance..."):
                balance_data = client.get_balance(address)

            console.print(f"\n[bold cyan]Balance: {address}[/bold cyan]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Free Balance", f"{balance_data.get('free', 0)} ETR")
            table.add_row("Reserved", f"{balance_data.get('reserved', 0)} ETR")
            table.add_row("Total", f"{balance_data.get('total', 0)} ETR")

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@query.command()
@click.argument("tx_hash")
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def transaction(tx_hash: str, node: str):
    """
    Query transaction information

    \b
    Examples:
        pye query transaction 0x123...
        pye query tx 0x123...
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching transaction..."):
                tx_data = client.get_transaction(tx_hash)

            console.print(f"\n[bold cyan]Transaction Information[/bold cyan]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Hash", tx_hash[:64] + "...")
            table.add_row("From", str(tx_data.get("from", "N/A")))
            table.add_row("To", str(tx_data.get("to", "N/A")))
            table.add_row("Value", f"{tx_data.get('value', 0)} ETR")
            table.add_row("Block Number", str(tx_data.get("blockNumber", "Pending")))
            table.add_row("Block Hash", str(tx_data.get("blockHash", "N/A"))[:64] + "...")
            table.add_row("Status", tx_data.get("status", "Unknown"))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


# Alias for transaction command
@query.command()
@click.argument("tx_hash")
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def tx(tx_hash: str, node: str):
    """Alias for 'query transaction'"""
    ctx = click.get_current_context()
    ctx.invoke(transaction, tx_hash=tx_hash, node=node)


@query.command()
@click.argument("address")
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def account(address: str, node: str):
    """
    Query account information

    \b
    Examples:
        pye query account 0x123...
        pye query account alice
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching account info..."):
                account_data = client.get_account_info(address)
                balance_data = client.get_balance(address)

            console.print(f"\n[bold cyan]Account: {address}[/bold cyan]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Address", address)
            table.add_row("Nonce", str(account_data.get("nonce", 0)))
            table.add_row("Free Balance", f"{balance_data.get('free', 0)} ETR")
            table.add_row("Reserved", f"{balance_data.get('reserved', 0)} ETR")
            table.add_row("Total Balance", f"{balance_data.get('total', 0)} ETR")

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@query.command()
@click.argument("storage_key")
@click.option("--block", "-b", help="Block hash to query at")
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def state(storage_key: str, block: str, node: str):
    """
    Query chain state

    \b
    Examples:
        pye query state System.Account
        pye query state Balances.TotalIssuance -b 0x123...
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Querying state..."):
                state_value = client.query_state(storage_key, block)

            console.print(f"\n[bold cyan]State Query[/bold cyan]\n")
            console.print(f"[cyan]Key:[/cyan] {storage_key}")
            if block:
                console.print(f"[cyan]Block:[/cyan] {block}")
            console.print(f"\n[green]Value:[/green]")
            rprint(state_value)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@query.command()
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def chain(node: str):
    """
    Query chain information

    \b
    Examples:
        pye query chain
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching chain info..."):
                chain_info = client.get_chain_info()
                health = client.get_node_health()
                sync_state = client.get_sync_state()
                latest_block = client.get_block()

            console.print(f"\n[bold cyan]ËTRID Chain Information[/bold cyan]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Chain", str(chain_info.get("chain", "ËTRID")))
            table.add_row("Node Health", "✓ Healthy" if health.get("is_healthy") else "✗ Unhealthy")
            table.add_row("Peers", str(health.get("peers", 0)))
            table.add_row("Syncing", "Yes" if sync_state.get("is_syncing") else "No")

            if latest_block:
                header = latest_block.get("block", {}).get("header", {})
                table.add_row("Latest Block", str(header.get("number", "N/A")))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@query.command()
@click.option("--node", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def health(node: str):
    """
    Query node health

    \b
    Examples:
        pye query health
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Checking node health..."):
                health_data = client.get_node_health()
                sync_state = client.get_sync_state()

            console.print(f"\n[bold cyan]Node Health Status[/bold cyan]\n")

            is_healthy = health_data.get("is_healthy", False)
            status_icon = "✓" if is_healthy else "✗"
            status_color = "green" if is_healthy else "red"

            console.print(f"[{status_color}]{status_icon} Node is {'healthy' if is_healthy else 'unhealthy'}[/{status_color}]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Peers", str(health_data.get("peers", 0)))
            table.add_row("Syncing", "Yes" if sync_state.get("is_syncing") else "No")
            table.add_row("Should Have Peers", str(health_data.get("should_have_peers", True)))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()
