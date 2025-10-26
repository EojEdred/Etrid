"""
Ëtrid SDK for Python

Provides a comprehensive interface to interact with Ëtrid Protocol blockchain.

Example:
    >>> import asyncio
    >>> from etrid_sdk import EtridClient, Account
    >>>
    >>> async def main():
    ...     async with EtridClient('ws://localhost:9944') as client:
    ...         account = Account.generate()
    ...         balance = await client.query.balance(account.address)
    ...         print(f'Balance: {balance.free} ETR')
    >>>
    >>> asyncio.run(main())
"""

from .client import EtridClient
from .account import Account
from .types import Balance, Block, TxHash, Address

__version__ = "0.1.0"
__all__ = ["EtridClient", "Account", "Balance", "Block", "TxHash", "Address"]
