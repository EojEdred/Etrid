# Ëtrid SDK for Python

Python library for interacting with the Ëtrid Protocol blockchain.

## Features

- ✅ Account management (create, import, sign)
- ✅ Async/await support
- ✅ WebSocket RPC client
- ✅ Type hints with Pydantic
- 🔨 Transaction building (in progress)
- 🔨 Event subscriptions (planned)

## Installation

```bash
pip install etrid-sdk
```

## Quick Start

```python
import asyncio
from etrid_sdk import EtridClient, Account

async def main():
    # Connect to node
    async with EtridClient('ws://localhost:9944') as client:
        # Create account
        account = Account.generate()
        print(f'Address: {account.address}')

        # Query balance
        balance = await client.query.balance(account.address)
        print(f'Balance: {balance.free} ETR')

asyncio.run(main())
```

## Documentation

See docstrings for full API documentation.

## Status

**Development Status**: Basic implementation complete, full features in progress.
