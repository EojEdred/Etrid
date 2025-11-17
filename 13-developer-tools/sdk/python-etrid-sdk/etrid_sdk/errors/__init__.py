"""
Ëtrid SDK Errors
"""


class EtridError(Exception):
    """Base exception for all Ëtrid SDK errors."""
    pass


class NotConnectedError(EtridError):
    """Raised when API is not connected."""
    
    def __init__(self, message: str = "Not connected to Ëtrid node"):
        self.message = message
        super().__init__(self.message)


class InvalidAddressError(EtridError):
    """Raised when an invalid address is provided."""
    
    def __init__(self, address: str, message: str = "Invalid address"):
        self.address = address
        self.message = f"{message}: {address}"
        super().__init__(self.message)


class TransactionError(EtridError):
    """Base exception for transaction errors."""
    pass


# Lightning-Bloc Errors
class ChannelError(TransactionError):
    """Raised when channel operation fails."""
    pass


class RouteNotFoundError(ChannelError):
    """Raised when no route found for payment."""
    pass


class InsufficientChannelBalanceError(ChannelError):
    """Raised when channel has insufficient balance."""
    pass


# Distribution Pay Errors
class DistributionError(TransactionError):
    """Raised when distribution operation fails."""
    pass


class NotEligibleError(DistributionError):
    """Raised when account is not eligible for distribution."""
    pass


class ClaimTooEarlyError(DistributionError):
    """Raised when claim is attempted too early."""
    pass


# ETWASM VM Errors
class EtwasmError(TransactionError):
    """Raised when ETWASM operation fails."""
    pass


class InvalidWasmError(EtwasmError):
    """Raised when WASM bytecode is invalid."""
    pass


class ContractNotFoundError(EtwasmError):
    """Raised when contract is not found."""
    pass


class InsufficientGasError(EtwasmError):
    """Raised when gas limit is insufficient."""
    pass


# AI DID Errors
class AIDidError(TransactionError):
    """Raised when AI DID operation fails."""
    pass


class AIAlreadyRegisteredError(AIDidError):
    """Raised when AI is already registered."""
    pass


class InvalidProfileError(AIDidError):
    """Raised when AI profile is invalid."""
    pass


# Bridge Errors
class BridgeError(TransactionError):
    """Raised when bridge operation fails."""
    pass


class UnsupportedChainError(BridgeError):
    """Raised when chain is not supported."""
    pass


class AmountBelowMinimumError(BridgeError):
    """Raised when amount is below minimum."""
    pass


# Oracle Errors
class OracleError(TransactionError):
    """Raised when oracle operation fails."""
    pass


class PriceNotFoundError(OracleError):
    """Raised when price feed is not found."""
    pass


class StalePriceError(OracleError):
    """Raised when price is stale."""
    pass


# Vault Errors
class VaultError(TransactionError):
    """Raised when vault operation fails."""
    pass


class InsufficientCollateralError(VaultError):
    """Raised when collateral is insufficient."""
    pass


class UndercollateralizedVaultError(VaultError):
    """Raised when vault would become undercollateralized."""
    pass


# Staking Errors
class StakingError(TransactionError):
    """Raised when staking operation fails."""
    pass


# Governance Errors
class GovernanceError(TransactionError):
    """Raised when governance operation fails."""
    pass


class ProposalNotFoundError(GovernanceError):
    """Raised when proposal is not found."""
    pass


__all__ = [
    "EtridError",
    "NotConnectedError",
    "InvalidAddressError",
    "TransactionError",
    "ChannelError",
    "RouteNotFoundError",
    "InsufficientChannelBalanceError",
    "DistributionError",
    "NotEligibleError",
    "ClaimTooEarlyError",
    "EtwasmError",
    "InvalidWasmError",
    "ContractNotFoundError",
    "InsufficientGasError",
    "AIDidError",
    "AIAlreadyRegisteredError",
    "InvalidProfileError",
    "BridgeError",
    "UnsupportedChainError",
    "AmountBelowMinimumError",
    "OracleError",
    "PriceNotFoundError",
    "StalePriceError",
    "VaultError",
    "InsufficientCollateralError",
    "UndercollateralizedVaultError",
    "StakingError",
    "GovernanceError",
    "ProposalNotFoundError",
]
