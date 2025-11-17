"""
Ëtrid Python SDK
================

Python SDK for interacting with the Ëtrid Protocol blockchain.

Features:
- Lightning-Bloc Layer 3 payment channels (500K+ TPS)
- Distribution Pay system (27,397 ÉTR daily rewards)
- ETWASM VM smart contracts
- AI DID (world's first AI identity standard)
- Cross-chain bridge (13 supported chains)
- Price oracles and TWAP
- Reserve vaults and lending

Usage:
    >>> from etrid_sdk import EtridClient
    >>> from etrid_sdk.wrappers import LightningBlocWrapper
    >>>
    >>> # Connect to Ëtrid node
    >>> client = EtridClient("wss://rpc.etrid.io")
    >>> lightning = LightningBlocWrapper(client.api)
    >>>
    >>> # Create payment channel
    >>> channel = await lightning.open_channel(keypair, recipient, amount)
"""

from .client import EtridClient
from .wrappers import (
    LightningBlocWrapper,
    DistributionPayWrapper,
    EtwasmVMWrapper,
    AIDidWrapper,
    BridgeWrapper,
    OracleWrapper,
    ReserveVaultWrapper,
    StakingWrapper,
    GovernanceWrapper,
)

__version__ = "0.1.0"
__author__ = "Ëtrid Foundation"
__all__ = [
    "EtridClient",
    "LightningBlocWrapper",
    "DistributionPayWrapper",
    "EtwasmVMWrapper",
    "AIDidWrapper",
    "BridgeWrapper",
    "OracleWrapper",
    "ReserveVaultWrapper",
    "StakingWrapper",
    "GovernanceWrapper",
]
