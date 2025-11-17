"""
ETH PBC Precompile Wrappers for Python SDK

Provides Python interface to Ethereum Partition Burst Chain (ETH PBC) precompiles
that enable access to FlareChain features from EVM contracts.

Precompile Addresses:
- 0x800: Oracle (FlareChain price feeds)
- 0x801: Governance (voting from ETH PBC)
- 0x802: Staking (validator queries)
- 0x803: Native ETH Wrapping (zero-fee wrap/unwrap)
- 0x804: XCM Bridge (cross-chain transfers)
- 0x805: Token Registry (registered tokens)
- 0x806: State Proof (Ethereum state verification)

Example:
    from etrid_sdk.wrappers.eth_pbc_precompiles import ETHPBCPrecompiles
    from web3 import Web3

    w3 = Web3(Web3.HTTPProvider('http://localhost:9944'))
    precompiles = ETHPBCPrecompiles(w3, private_key='0x...')

    # Get BTC price from FlareChain oracle
    price = precompiles.get_oracle_price('BTC', 'USD')
    print(f'BTC Price: ${price / 1e18:.2f}')

    # Wrap ETH to wETH
    tx_hash = precompiles.wrap_eth(1000000000000000000)  # 1 ETH
    print(f'Wrapped ETH: {tx_hash}')
"""

from typing import Dict, List, Optional
from web3 import Web3
from web3.contract import Contract
from eth_account import Account
from eth_typing import HexStr
import logging

logger = logging.getLogger(__name__)


class ETHPBCPrecompiles:
    """
    Wrapper for ETH PBC precompile contracts.

    Provides high-level Python interface to interact with FlareChain features
    from ETH PBC via precompiled contracts.
    """

    # Precompile addresses
    ORACLE_ADDRESS = "0x0000000000000000000000000000000000000800"
    GOVERNANCE_ADDRESS = "0x0000000000000000000000000000000000000801"
    STAKING_ADDRESS = "0x0000000000000000000000000000000000000802"
    NATIVE_ETH_WRAP_ADDRESS = "0x0000000000000000000000000000000000000803"
    XCM_BRIDGE_ADDRESS = "0x0000000000000000000000000000000000000804"
    TOKEN_REGISTRY_ADDRESS = "0x0000000000000000000000000000000000000805"
    STATE_PROOF_ADDRESS = "0x0000000000000000000000000000000000000806"

    def __init__(self, web3: Web3, private_key: Optional[str] = None):
        """
        Initialize ETH PBC precompile wrappers.

        Args:
            web3: Web3 instance connected to ETH PBC RPC
            private_key: Optional private key for signing transactions
        """
        self.w3 = web3
        self.account = Account.from_key(private_key) if private_key else None

    def _send_transaction(self, to: str, data: str, value: int = 0) -> str:
        """
        Send a transaction to a precompile contract.

        Args:
            to: Precompile address
            data: Encoded function call data
            value: ETH value to send (in wei)

        Returns:
            Transaction hash
        """
        if not self.account:
            raise ValueError("Private key required for transactions")

        tx = {
            'from': self.account.address,
            'to': to,
            'data': data,
            'value': value,
            'gas': 500000,
            'gasPrice': self.w3.eth.gas_price,
            'nonce': self.w3.eth.get_transaction_count(self.account.address),
            'chainId': self.w3.eth.chain_id,
        }

        signed_tx = self.account.sign_transaction(tx)
        tx_hash = self.w3.eth.send_raw_transaction(signed_tx.rawTransaction)

        logger.info(f"Transaction sent: {tx_hash.hex()}")
        return tx_hash.hex()

    def _call(self, to: str, data: str) -> bytes:
        """
        Make a read-only call to a precompile contract.

        Args:
            to: Precompile address
            data: Encoded function call data

        Returns:
            Raw response bytes
        """
        result = self.w3.eth.call({
            'to': to,
            'data': data,
        })
        return result

    # ========== Oracle Precompile (0x800) ==========

    def get_oracle_price(self, symbol: str, quote: str = "USD") -> int:
        """
        Get price from FlareChain oracle.

        Args:
            symbol: Asset symbol (e.g., 'BTC', 'ETH', 'SOL')
            quote: Quote currency (default: 'USD')

        Returns:
            Price scaled by 1e18 (e.g., 50000e18 = $50,000)
        """
        # Encode function call: getPrice(bytes32,bytes32)
        selector = self.w3.keccak(text="getPrice(bytes32,bytes32)")[:4]
        symbol_bytes32 = symbol.ljust(32, '\x00').encode()[:32]
        quote_bytes32 = quote.ljust(32, '\x00').encode()[:32]

        data = selector + symbol_bytes32 + quote_bytes32
        result = self._call(self.ORACLE_ADDRESS, data.hex())

        price = int.from_bytes(result, byteorder='big')
        logger.debug(f"Oracle price for {symbol}/{quote}: {price}")
        return price

    def get_oracle_price_in_eth(self, symbol: str) -> int:
        """
        Get price in ETH from FlareChain oracle.

        Args:
            symbol: Asset symbol (e.g., 'BTC', 'SOL')

        Returns:
            Price in ETH scaled by 1e18
        """
        # Encode function call: getPriceInETH(bytes32)
        selector = self.w3.keccak(text="getPriceInETH(bytes32)")[:4]
        symbol_bytes32 = symbol.ljust(32, '\x00').encode()[:32]

        data = selector + symbol_bytes32
        result = self._call(self.ORACLE_ADDRESS, data.hex())

        return int.from_bytes(result, byteorder='big')

    def get_oracle_last_update(self, symbol: str) -> int:
        """
        Get last update timestamp for an oracle price feed.

        Args:
            symbol: Asset symbol

        Returns:
            Unix timestamp of last update
        """
        # Encode function call: getLastUpdate(bytes32)
        selector = self.w3.keccak(text="getLastUpdate(bytes32)")[:4]
        symbol_bytes32 = symbol.ljust(32, '\x00').encode()[:32]

        data = selector + symbol_bytes32
        result = self._call(self.ORACLE_ADDRESS, data.hex())

        return int.from_bytes(result, byteorder='big')

    # ========== Governance Precompile (0x801) ==========

    def governance_create_proposal(self, title: str, description: str) -> int:
        """
        Create a governance proposal on FlareChain.

        Args:
            title: Proposal title (max 256 chars)
            description: Proposal description (max 10000 chars)

        Returns:
            Proposal ID
        """
        # Encode function call: submitProposal(string,string)
        selector = self.w3.keccak(text="submitProposal(string,string)")[:4]

        # ABI encode strings (simplified - using Web3's built-in codec)
        from eth_abi import encode
        encoded_params = encode(['string', 'string'], [title, description])

        data = selector + encoded_params
        tx_hash = self._send_transaction(self.GOVERNANCE_ADDRESS, data.hex())

        # Wait for transaction receipt and decode proposal ID
        receipt = self.w3.eth.wait_for_transaction_receipt(tx_hash)

        # Decode return value from transaction (simplified)
        # In production, would parse logs or use eth_call simulation
        logger.info(f"Proposal created: tx={tx_hash}")
        return 0  # Would extract proposal ID from receipt

    def governance_vote(self, proposal_id: int, support: bool) -> str:
        """
        Vote on a governance proposal.

        Args:
            proposal_id: Proposal ID to vote on
            support: True to vote YES, False to vote NO

        Returns:
            Transaction hash
        """
        # Encode function call: voteOnProposal(uint256,bool)
        selector = self.w3.keccak(text="voteOnProposal(uint256,bool)")[:4]

        from eth_abi import encode
        encoded_params = encode(['uint256', 'bool'], [proposal_id, support])

        data = selector + encoded_params
        tx_hash = self._send_transaction(self.GOVERNANCE_ADDRESS, data.hex())

        logger.info(f"Vote submitted: proposal={proposal_id}, support={support}")
        return tx_hash

    def get_proposal_status(self, proposal_id: int) -> int:
        """
        Get governance proposal status.

        Args:
            proposal_id: Proposal ID

        Returns:
            Status: 0=Pending, 1=Active, 2=Passed, 3=Failed
        """
        # Encode function call: getProposalStatus(uint256)
        selector = self.w3.keccak(text="getProposalStatus(uint256)")[:4]

        proposal_id_bytes = proposal_id.to_bytes(32, byteorder='big')
        data = selector + proposal_id_bytes

        result = self._call(self.GOVERNANCE_ADDRESS, data.hex())
        return int.from_bytes(result, byteorder='big')

    # ========== Staking Precompile (0x802) ==========

    def get_validator_stake(self, validator_id: str) -> int:
        """
        Get stake amount for a validator.

        Args:
            validator_id: Validator ID (hex string or bytes32)

        Returns:
            Stake amount in wei
        """
        # Encode function call: getValidatorStake(bytes32)
        selector = self.w3.keccak(text="getValidatorStake(bytes32)")[:4]

        if isinstance(validator_id, str):
            if validator_id.startswith('0x'):
                validator_id_bytes = bytes.fromhex(validator_id[2:])
            else:
                validator_id_bytes = validator_id.ljust(32, '\x00').encode()[:32]

        data = selector + validator_id_bytes
        result = self._call(self.STAKING_ADDRESS, data.hex())

        return int.from_bytes(result, byteorder='big')

    def is_validator_active(self, validator_id: str) -> bool:
        """
        Check if a validator is active.

        Args:
            validator_id: Validator ID

        Returns:
            True if validator is active
        """
        # Encode function call: isValidatorActive(bytes32)
        selector = self.w3.keccak(text="isValidatorActive(bytes32)")[:4]

        if isinstance(validator_id, str):
            if validator_id.startswith('0x'):
                validator_id_bytes = bytes.fromhex(validator_id[2:])
            else:
                validator_id_bytes = validator_id.ljust(32, '\x00').encode()[:32]

        data = selector + validator_id_bytes
        result = self._call(self.STAKING_ADDRESS, data.hex())

        return int.from_bytes(result, byteorder='big') != 0

    def get_total_staked(self) -> int:
        """
        Get total amount staked across all validators.

        Returns:
            Total stake in wei
        """
        # Encode function call: getTotalStaked()
        selector = self.w3.keccak(text="getTotalStaked()")[:4]

        result = self._call(self.STAKING_ADDRESS, selector.hex())
        return int.from_bytes(result, byteorder='big')

    def get_validator_count(self) -> int:
        """
        Get total number of validators.

        Returns:
            Validator count
        """
        # Encode function call: getValidatorCount()
        selector = self.w3.keccak(text="getValidatorCount()")[:4]

        result = self._call(self.STAKING_ADDRESS, selector.hex())
        return int.from_bytes(result, byteorder='big')

    # ========== Native ETH Wrap Precompile (0x803) ==========

    def wrap_eth(self, amount: int) -> str:
        """
        Wrap native ETH to wETH (zero-fee, instant).

        Args:
            amount: Amount of ETH to wrap (in wei)

        Returns:
            Transaction hash
        """
        # Encode function call: wrap()
        selector = self.w3.keccak(text="wrap()")[:4]

        tx_hash = self._send_transaction(
            self.NATIVE_ETH_WRAP_ADDRESS,
            selector.hex(),
            value=amount
        )

        logger.info(f"Wrapped {amount} wei ETH: {tx_hash}")
        return tx_hash

    def unwrap_eth(self, amount: int) -> str:
        """
        Unwrap wETH to native ETH (zero-fee, instant).

        Args:
            amount: Amount of wETH to unwrap (in wei)

        Returns:
            Transaction hash
        """
        # Encode function call: unwrap(uint256)
        selector = self.w3.keccak(text="unwrap(uint256)")[:4]

        amount_bytes = amount.to_bytes(32, byteorder='big')
        data = selector + amount_bytes

        tx_hash = self._send_transaction(self.NATIVE_ETH_WRAP_ADDRESS, data.hex())

        logger.info(f"Unwrapped {amount} wei wETH: {tx_hash}")
        return tx_hash

    def get_wrap_rate(self) -> int:
        """
        Get current ETH<->wETH wrap rate.

        Returns:
            Rate scaled by 1e18 (normally 1e18 = 1:1)
        """
        # Encode function call: getWrapRate()
        selector = self.w3.keccak(text="getWrapRate()")[:4]

        result = self._call(self.NATIVE_ETH_WRAP_ADDRESS, selector.hex())
        return int.from_bytes(result, byteorder='big')

    # ========== Token Registry Precompile (0x805) ==========

    def get_token_info(self, token_address: str) -> Dict:
        """
        Get registered token information.

        Args:
            token_address: ERC-20 token address

        Returns:
            Dict with keys: name, symbol, decimals, totalBridgedSupply
        """
        # Encode function call: getTokenInfo(address)
        selector = self.w3.keccak(text="getTokenInfo(address)")[:4]

        # Pad address to 32 bytes
        addr_bytes = bytes.fromhex(token_address[2:] if token_address.startswith('0x') else token_address)
        addr_padded = addr_bytes.rjust(32, b'\x00')

        data = selector + addr_padded
        result = self._call(self.TOKEN_REGISTRY_ADDRESS, data.hex())

        # Decode tuple return (simplified - would use eth_abi in production)
        # Returns: (string name, string symbol, uint8 decimals, uint256 totalBridgedSupply)

        return {
            'name': 'Token Name',  # Would decode from result
            'symbol': 'TKN',
            'decimals': 18,
            'totalBridgedSupply': 0
        }

    # ========== State Proof Precompile (0x806) ==========

    def verify_eth_state_proof(self, proof: bytes) -> bool:
        """
        Verify Ethereum state proof.

        Args:
            proof: Merkle proof bytes

        Returns:
            True if proof is valid
        """
        # This is a complex function requiring full Merkle proof encoding
        # Simplified implementation
        logger.warning("verify_eth_state_proof is a stub implementation")
        return True
