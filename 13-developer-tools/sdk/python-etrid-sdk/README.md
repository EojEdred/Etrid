# Ëtrid Python SDK

Python SDK for interacting with the Ëtrid Protocol blockchain.

## Features

- **Lightning-Bloc Layer 3** - 500K+ TPS payment channels
- **Distribution Pay** - 27,397 ÉTR daily rewards across 5 categories
- **ETWASM VM** - WebAssembly smart contracts
- **AI DID** - World's first AI identity standard
- **Cross-Chain Bridge** - Support for 13 chains
- **Price Oracles** - Decentralized price feeds with TWAP
- **Reserve Vaults** - Collateralized lending and borrowing
- **Staking** - Nominate validators and earn rewards
- **Governance** - On-chain proposal voting

## Installation

```bash
pip install etrid-sdk
```

Or from source:

```bash
git clone https://github.com/etrid/etrid-protocol
cd etrid-protocol/13-developer-tools/sdk/python-etrid-sdk
pip install -e .
```

## Quick Start

```python
from etrid_sdk import EtridClient
from etrid_sdk.wrappers import LightningBlocWrapper

# Connect to Ëtrid node
client = EtridClient("wss://rpc.etrid.io")

# Create keypair
alice = client.create_keypair()

# Use Lightning-Bloc wrapper
lightning = LightningBlocWrapper(client.api)

# Open payment channel
channel = await lightning.open_channel(
    alice,
    recipient_address="5GrwvaEF...",
    amount=1000 * 10**18  # 1000 ÉTR
)

print(f"Channel ID: {channel['channel_id']}")
```

## Usage Examples

### Distribution Pay - Claim Daily Rewards

```python
from etrid_sdk import EtridClient
from etrid_sdk.wrappers import DistributionPayWrapper

client = EtridClient("wss://rpc.etrid.io")
distribution = DistributionPayWrapper(client.api)

# Check pending rewards
pending = await distribution.get_pending_rewards(alice.ss58_address)
print(f"Total pending: {pending['total'] / 10**18} ÉTR")

# Claim staker rewards
tx_hash = await distribution.claim_reward(
    alice,
    category="stakers"
)
print(f"Claimed! TX: {tx_hash}")
```

### AI DID - Register AI Agent

```python
from etrid_sdk.wrappers import AIDidWrapper

ai_did = AIDidWrapper(client.api)

# Register AI
registration = await ai_did.register_ai(
    alice,
    name="MyAI",
    ai_type="llm",
    api_endpoint="https://api.myai.com",
    metadata={"version": "1.0", "model": "gpt-4"}
)

print(f"AI DID: {registration['did']}")
print(f"Reputation: {registration['reputation']}")
```

### Cross-Chain Bridge

```python
from etrid_sdk.wrappers import BridgeWrapper

bridge = BridgeWrapper(client.api)

# Bridge ETH to BNB
transfer = await bridge.bridge(
    alice,
    from_chain="ethereum",
    to_chain="bnb",
    amount=100 * 10**18,
    recipient="0x742d35Cc..."
)

print(f"Transfer ID: {transfer['transfer_id']}")

# Check status
status = await bridge.get_transfer_status(transfer['transfer_id'])
print(f"Status: {status['status']}")
```

### ETWASM Smart Contracts

```python
from etrid_sdk.wrappers import EtwasmVMWrapper

etwasm = EtwasmVMWrapper(client.api)

# Deploy contract
with open("token.wasm", "rb") as f:
    wasm_code = f.read()

deployment = await etwasm.deploy_contract(
    alice,
    wasm_code=wasm_code,
    constructor_args=["MyToken", "MTK", 18],
    value=0,
    gas_limit=1_000_000
)

print(f"Contract: {deployment['address']}")

# Call contract method
result = await etwasm.call_contract(
    alice,
    contract_address=deployment['address'],
    method="transfer",
    args=[bob.ss58_address, 100 * 10**18],
    value=0,
    gas_limit=500_000
)
```

### Staking

```python
from etrid_sdk.wrappers import StakingWrapper

staking = StakingWrapper(client.api)

# Bond tokens for staking
await staking.bond(
    alice,
    validator=validator_address,
    amount=1000 * 10**18
)

# Check staking info
info = await staking.get_staking_info(alice.ss58_address)
print(f"Staked: {info['staked'] / 10**18} ÉTR")
print(f"Status: {info['status']}")

# Estimate rewards
estimate = await staking.estimate_rewards(1000 * 10**18)
print(f"APY: {estimate['apy']}%")
print(f"Yearly: {estimate['yearly'] / 10**18} ÉTR")
```

### Governance

```python
from etrid_sdk.wrappers import GovernanceWrapper

governance = GovernanceWrapper(client.api)

# Create proposal
proposal = await governance.create_proposal(
    alice,
    title="Increase staking rewards",
    description="Proposal to increase staking APY from 15% to 20%",
    call=runtime_call
)

# Vote on proposal
await governance.vote(
    alice,
    proposal_id=1,
    approve=True,
    stake=1000 * 10**18
)

# Check results
results = await governance.get_proposal_results(1)
print(f"For: {results['votes_for']}")
print(f"Against: {results['votes_against']}")
print(f"Approved: {results['approved']}")
```

## API Documentation

Full API documentation is available at [docs.etrid.io/python-sdk](https://docs.etrid.io/python-sdk)

## Development

```bash
# Install development dependencies
pip install -e ".[dev]"

# Run tests
pytest

# Run with coverage
pytest --cov=etrid_sdk --cov-report=html

# Format code
black etrid_sdk tests

# Lint
pylint etrid_sdk

# Type checking
mypy etrid_sdk
```

## Testing

```bash
# Run all tests
pytest

# Run specific test file
pytest tests/test_lightning_bloc.py

# Run with verbose output
pytest -v

# Run with coverage
pytest --cov=etrid_sdk --cov-report=term-missing
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Apache 2.0 - see LICENSE file for details

## Support

- Documentation: https://docs.etrid.io
- Discord: https://discord.gg/etrid
- Twitter: [@EtridProtocol](https://twitter.com/EtridProtocol)
- Email: dev@etrid.io

## Comparison with Other SDKs

| Feature | Ëtrid Python SDK | Web3.py | Substrate Python |
|---------|------------------|---------|------------------|
| Layer 3 Payments | ✅ Yes | ❌ No | ❌ No |
| AI Identity | ✅ Yes | ❌ No | ❌ No |
| Cross-Chain | ✅ 13 chains | ⚠️ Limited | ⚠️ Limited |
| Smart Contracts | ✅ WASM | ✅ EVM | ✅ WASM |
| Type Safety | ✅ Full | ⚠️ Partial | ⚠️ Partial |
| Examples | ✅ Comprehensive | ⚠️ Basic | ⚠️ Basic |

## Acknowledgments

Built on top of:
- [substrate-interface](https://github.com/polkascan/py-substrate-interface)
- [scalecodec](https://github.com/polkascan/py-scale-codec)

---

**Built with ❤️ by the Ëtrid Foundation**
