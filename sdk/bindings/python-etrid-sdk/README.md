# Python Ëtrid SDK

**Status:** 📋 Planned for v1.2 (Post-Mainnet)

## Overview

Python bindings for the Ëtrid blockchain SDK using PyO3.

## Planned Features

- **Native Performance**: Rust backend with Python interface
- **Type Hints**: Full mypy compatibility
- **Async Support**: asyncio-based API
- **Pythonic API**: Follows Python conventions
- **Jupyter Integration**: Works in notebooks

## Installation (When Available)

```bash
pip install etrid-sdk
```

## Usage Example (Planned API)

```python
import asyncio
from etrid_sdk import EtridClient, Wallet

async def main():
    # Connect to FlareChain
    client = await EtridClient.connect('wss://flarechain.etrid.io')

    # Create or import wallet
    wallet = Wallet.from_mnemonic('your twelve word mnemonic...')

    # Get balance
    balance = await client.get_balance(wallet.address)
    print(f"Balance: {balance} ETR")

    # Send transaction
    tx = await client.transfer(
        from_wallet=wallet,
        to_address='5Gx...',
        amount='1000000000000',  # 1 ETR
        chain='flarechain'
    )

    # Multichain operations
    btc_pbc = await client.connect_chain('btc-pbc')
    btc_balance = await btc_pbc.get_bridged_balance(wallet.address)

asyncio.run(main())
```

## Use Cases

- **Trading Bots**: Automated trading on Ëtrid DEX
- **Data Analysis**: Pandas integration for on-chain analytics
- **Backend Services**: Python microservices
- **Research**: Jupyter notebooks for blockchain research

## Roadmap

- **v1.2.0**: Initial release with core functionality
- **v1.2.1**: Pandas DataFrame export for analytics
- **v1.3.0**: Machine learning integration
- **v1.4.0**: Web3.py compatibility layer

## Development

This SDK will be implemented after mainnet deployment.

**Target Timeline:** Q2 2026 (post-mainnet)

## Temporary Alternative

Until this SDK is ready, use Substrate Interface:

```python
from substrateinterface import SubstrateInterface

substrate = SubstrateInterface(
    url="ws://127.0.0.1:9944",
    ss58_format=42,
    type_registry_preset='substrate-node-template'
)
```
