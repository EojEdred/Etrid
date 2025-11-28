"""
Consensus commands

Manage consensus day operations, voting, and validity node registration.
"""

import click
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.prompt import Confirm
from pye.client import EtridClient, EtridRPCError

console = Console()


@click.group()
def consensus():
    """Manage consensus operations"""
    pass


@consensus.command()
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def status(node: str):
    """
    Get current consensus status

    \b
    Examples:
        pye consensus status
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching consensus status..."):
                status_data = client.get_consensus_status()

            console.print(f"\n[bold cyan]Consensus Status[/bold cyan]\n")

            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("Current Phase", status_data.get("phase", "Unknown"))
            table.add_row("Active Proposals", str(status_data.get("active_proposals", 0)))
            table.add_row("Total Validators", str(status_data.get("total_validators", 0)))
            table.add_row("Active Validators", str(status_data.get("active_validators", 0)))
            table.add_row("Current Round", str(status_data.get("round", 0)))
            table.add_row("Next Consensus Day", status_data.get("next_consensus_day", "N/A"))

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@consensus.command()
@click.option("--account", "-a", required=True, help="Account name or address")
@click.option("--stake", "-s", type=float, required=True, help="Stake amount (ETR)")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def register(account: str, stake: float, node: str):
    """
    Register as a validity node

    \b
    Examples:
        pye consensus register -a alice -s 10000
        pye consensus register -a 0x123... -s 50000
    """
    console.print(f"\n[bold cyan]Validity Node Registration[/bold cyan]\n")
    console.print(f"[cyan]Account:[/cyan] {account}")
    console.print(f"[cyan]Stake:[/cyan]   {stake} ETR\n")

    console.print("[yellow]Note:[/yellow] Registering as a validity node requires:")
    console.print("  • Minimum stake amount")
    console.print("  • Running a full ËTRID node")
    console.print("  • Meeting hardware requirements\n")

    if not Confirm.ask("Continue with registration?"):
        console.print("[yellow]Registration cancelled[/yellow]")
        return

    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Submitting registration..."):
                tx_hash = client.register_validity_node(account, stake)

            console.print(f"\n[green]✓[/green] Validity node registration submitted!\n")
            console.print(f"[cyan]Transaction Hash:[/cyan] {tx_hash}\n")
            console.print("[dim]Registration will be active after confirmation[/dim]\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@consensus.command()
@click.argument("proposal_id")
@click.argument("vote", type=click.Choice(["yes", "no", "abstain"]))
@click.option("--account", "-a", required=True, help="Voter account name or address")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def vote(proposal_id: str, vote: str, account: str, node: str):
    """
    Submit a vote on a proposal

    \b
    Examples:
        pye consensus vote PROP-001 yes -a alice
        pye consensus vote PROP-002 no -a validator1
    """
    vote_map = {"yes": True, "no": False, "abstain": None}
    vote_value = vote_map[vote]

    console.print(f"\n[bold cyan]Submitting Vote[/bold cyan]\n")
    console.print(f"[cyan]Proposal:[/cyan] {proposal_id}")
    console.print(f"[cyan]Vote:[/cyan]     {vote.upper()}")
    console.print(f"[cyan]Voter:[/cyan]    {account}\n")

    if not Confirm.ask("Submit vote?"):
        console.print("[yellow]Vote cancelled[/yellow]")
        return

    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Submitting vote..."):
                tx_hash = client.submit_vote(proposal_id, vote_value)

            console.print(f"\n[green]✓[/green] Vote submitted successfully!\n")
            console.print(f"[cyan]Transaction Hash:[/cyan] {tx_hash}\n")

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@consensus.command()
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def proposals(node: str):
    """
    List active proposals

    \b
    Examples:
        pye consensus proposals
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching proposals..."):
                proposals_data = client._send_request("consensus_getProposals", [])

            if not proposals_data:
                console.print("\n[yellow]No active proposals[/yellow]\n")
                return

            console.print(f"\n[bold cyan]Active Proposals[/bold cyan] ({len(proposals_data)})\n")

            table = Table(show_header=True, header_style="bold magenta")
            table.add_column("ID", style="cyan")
            table.add_column("Title", style="green")
            table.add_column("Status", style="yellow")
            table.add_column("Yes", justify="right")
            table.add_column("No", justify="right")
            table.add_column("Ends", style="dim")

            for proposal in proposals_data:
                table.add_row(
                    proposal.get("id", ""),
                    proposal.get("title", "")[:40],
                    proposal.get("status", ""),
                    str(proposal.get("votes_yes", 0)),
                    str(proposal.get("votes_no", 0)),
                    proposal.get("end_time", "N/A"),
                )

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@consensus.command()
@click.argument("proposal_id")
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def proposal(proposal_id: str, node: str):
    """
    Get proposal details

    \b
    Examples:
        pye consensus proposal PROP-001
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching proposal..."):
                proposal_data = client._send_request("consensus_getProposal", [proposal_id])

            console.print(f"\n[bold cyan]Proposal: {proposal_id}[/bold cyan]\n")

            # Basic info
            table = Table(show_header=False)
            table.add_column("Property", style="cyan")
            table.add_column("Value", style="green")

            table.add_row("ID", proposal_data.get("id", ""))
            table.add_row("Title", proposal_data.get("title", ""))
            table.add_row("Status", proposal_data.get("status", ""))
            table.add_row("Proposer", proposal_data.get("proposer", ""))
            table.add_row("Created", proposal_data.get("created_at", ""))
            table.add_row("Ends", proposal_data.get("end_time", ""))

            console.print(table)

            # Voting results
            console.print(f"\n[bold cyan]Voting Results[/bold cyan]\n")

            votes_table = Table(show_header=True, header_style="bold magenta")
            votes_table.add_column("Vote", style="cyan")
            votes_table.add_column("Count", justify="right", style="green")
            votes_table.add_column("Percentage", justify="right", style="yellow")

            total_votes = (
                proposal_data.get("votes_yes", 0)
                + proposal_data.get("votes_no", 0)
                + proposal_data.get("votes_abstain", 0)
            )

            if total_votes > 0:
                yes_pct = (proposal_data.get("votes_yes", 0) / total_votes) * 100
                no_pct = (proposal_data.get("votes_no", 0) / total_votes) * 100
                abstain_pct = (proposal_data.get("votes_abstain", 0) / total_votes) * 100

                votes_table.add_row("Yes", str(proposal_data.get("votes_yes", 0)), f"{yes_pct:.1f}%")
                votes_table.add_row("No", str(proposal_data.get("votes_no", 0)), f"{no_pct:.1f}%")
                votes_table.add_row(
                    "Abstain", str(proposal_data.get("votes_abstain", 0)), f"{abstain_pct:.1f}%"
                )
            else:
                votes_table.add_row("Yes", "0", "0.0%")
                votes_table.add_row("No", "0", "0.0%")
                votes_table.add_row("Abstain", "0", "0.0%")

            console.print(votes_table)

            # Description
            if proposal_data.get("description"):
                console.print(f"\n[bold cyan]Description[/bold cyan]\n")
                panel = Panel(
                    proposal_data.get("description", ""),
                    border_style="blue",
                    padding=(1, 2),
                )
                console.print(panel)

            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()


@consensus.command()
@click.option("--node", "-n", envvar="ETRID_NODE_URL", default="ws://localhost:9944")
def validators(node: str):
    """
    List validity nodes

    \b
    Examples:
        pye consensus validators
    """
    try:
        with EtridClient(ws_url=node) as client:
            with console.status("[bold green]Fetching validators..."):
                validators_data = client._send_request("consensus_getValidators", [])

            console.print(f"\n[bold cyan]Validity Nodes[/bold cyan] ({len(validators_data)})\n")

            table = Table(show_header=True, header_style="bold magenta")
            table.add_column("Address", style="cyan")
            table.add_column("Stake", style="green", justify="right")
            table.add_column("Status", style="yellow")
            table.add_column("Votes Cast", justify="right")

            for validator in validators_data:
                table.add_row(
                    validator.get("address", "")[:16] + "...",
                    f"{validator.get('stake', 0)} ETR",
                    "Active" if validator.get("active") else "Inactive",
                    str(validator.get("votes_cast", 0)),
                )

            console.print(table)
            console.print()

    except EtridRPCError as e:
        console.print(f"\n[red]Error:[/red] {e}\n")
        raise click.Abort()
