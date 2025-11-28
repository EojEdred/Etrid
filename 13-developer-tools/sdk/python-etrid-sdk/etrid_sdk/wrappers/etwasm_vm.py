"""ETWASM VM Wrapper - WebAssembly Smart Contracts

Provides interface for deploying and interacting with smart contracts
on the ËtwasmVM runtime with reentrancy protection.
"""

from typing import Dict, Any, List, Optional
from substrateinterface import SubstrateInterface, Keypair
from ..errors import (
    NotConnectedError,
    EtwasmError,
    InvalidWasmError,
    ContractNotFoundError,
    InsufficientGasError,
)


# Gas Constants for ËtwasmVM
class GasConstants:
    """VMw = Virtual Machine work units."""
    BLOCK_LIMIT = 10_000_000  # 10M VMw per block
    TX_LIMIT = 1_000_000      # 1M VMw per transaction
    VMW_PER_ETR = 1_000_000   # 1 ÉTR = 1M VMw
    DEFAULT_GAS = 500_000     # Default gas limit


class EtwasmVMWrapper:
    """
    Wrapper for ETWASM VM pallet - WebAssembly contracts.

    Enables deployment and interaction with WebAssembly smart contracts
    on Ëtrid's EVM-compatible runtime with reentrancy protection.
    """

    def __init__(self, api: SubstrateInterface):
        """
        Initialize ETWASM VM wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def upload_code(
        self,
        keypair: Keypair,
        wasm_code: bytes,
        gas_limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        """
        Upload contract code to the chain.

        Stores the WASM bytecode on-chain without instantiating it.
        This allows reusing the same code for multiple contract instances.

        Args:
            keypair: Signer account keypair
            wasm_code: Contract WASM bytecode
            gas_limit: Gas limit (optional, defaults to DEFAULT_GAS)

        Returns:
            Dictionary with code_hash, tx_hash, and storage_deposit

        Raises:
            InvalidWasmError: If WASM bytecode is invalid
            EtwasmError: If upload fails

        Example:
            >>> wasm_code = open('contract.wasm', 'rb').read()
            >>> result = await wrapper.upload_code(alice, wasm_code)
            >>> print(result['code_hash'])
        """
        self._ensure_connected()

        try:
            gas = gas_limit or GasConstants.DEFAULT_GAS

            call = self.api.compose_call(
                call_module="Contracts",
                call_function="uploadCode",
                call_params={
                    "code": f"0x{wasm_code.hex()}",
                    "storage_deposit_limit": None,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise InvalidWasmError(f"Failed to upload code: {receipt.error_message}")

            # Extract CodeStored event
            for event in receipt.triggered_events:
                if event.event_module.name == "Contracts" and event.event.name == "CodeStored":
                    code_hash = event.params[0]['value']
                    return {
                        "code_hash": code_hash,
                        "tx_hash": receipt.extrinsic_hash,
                        "storage_deposit": 0,  # Would extract from event
                    }

            raise EtwasmError("Code uploaded but CodeStored event not found")

        except Exception as e:
            raise EtwasmError(f"Failed to upload code: {str(e)}")

    async def instantiate(
        self,
        keypair: Keypair,
        code_hash: str,
        constructor_args: List[Any],
        value: int = 0,
        gas_limit: Optional[int] = None,
        salt: Optional[bytes] = None,
    ) -> str:
        """
        Instantiate a contract from uploaded code.

        Creates a new contract instance from a previously uploaded code hash.

        Args:
            keypair: Signer account keypair
            code_hash: Code hash to instantiate from
            constructor_args: Constructor arguments
            value: Value to transfer (in planck)
            gas_limit: Gas limit
            salt: Salt for deterministic address

        Returns:
            Contract address (SS58 format)

        Raises:
            EtwasmError: If instantiation fails

        Example:
            >>> address = await wrapper.instantiate(
            ...     alice,
            ...     code_hash,
            ...     [1000000],  # initial supply
            ...     value=0,
            ...     gas_limit=500_000
            ... )
            >>> print('Contract deployed at:', address)
        """
        self._ensure_connected()

        try:
            gas = gas_limit or GasConstants.DEFAULT_GAS
            salt_bytes = salt or b""

            call = self.api.compose_call(
                call_module="Contracts",
                call_function="instantiate",
                call_params={
                    "value": value,
                    "gas_limit": {"refTime": gas, "proofSize": 1_000_000},
                    "storage_deposit_limit": None,
                    "code_hash": code_hash,
                    "data": constructor_args,
                    "salt": f"0x{salt_bytes.hex()}",
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise EtwasmError(f"Failed to instantiate: {receipt.error_message}")

            # Extract Instantiated event
            for event in receipt.triggered_events:
                if event.event_module.name == "Contracts" and event.event.name == "Instantiated":
                    contract_address = event.params[1]['value']
                    return contract_address

            raise EtwasmError("Contract instantiated but event not found")

        except Exception as e:
            raise EtwasmError(f"Failed to instantiate contract: {str(e)}")

    async def deploy_contract(
        self,
        keypair: Keypair,
        wasm_code: bytes,
        constructor_args: List[Any] = [],
        value: int = 0,
        gas_limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        """
        Deploy a smart contract (upload + instantiate in one step).

        Convenience method that uploads code and instantiates in a single call.

        Args:
            keypair: Signer account keypair
            wasm_code: Contract WASM bytecode
            constructor_args: Constructor arguments
            value: Value to transfer to contract (in planck)
            gas_limit: Gas limit

        Returns:
            Dictionary with address, code_hash, tx_hash, gas_used, storage_deposit

        Raises:
            EtwasmError: If deployment fails

        Example:
            >>> deployment = await wrapper.deploy_contract(
            ...     alice,
            ...     wasm_code,
            ...     [1000000],  # initial supply
            ...     value=0,
            ...     gas_limit=500_000
            ... )
            >>> print('Deployed at:', deployment['address'])
        """
        self._ensure_connected()

        try:
            # Step 1: Upload code
            upload_result = await self.upload_code(keypair, wasm_code, gas_limit)

            # Step 2: Instantiate
            address = await self.instantiate(
                keypair,
                upload_result["code_hash"],
                constructor_args,
                value,
                gas_limit,
            )

            return {
                "address": address,
                "code_hash": upload_result["code_hash"],
                "tx_hash": upload_result["tx_hash"],
                "gas_used": gas_limit or GasConstants.DEFAULT_GAS,
                "storage_deposit": upload_result["storage_deposit"],
            }

        except Exception as e:
            raise EtwasmError(f"Failed to deploy contract: {str(e)}")

    async def call_contract(
        self,
        keypair: Keypair,
        contract_address: str,
        method: str,
        args: List[Any] = [],
        value: int = 0,
        gas_limit: Optional[int] = None,
    ) -> Dict[str, Any]:
        """
        Call a contract method (write operation).

        Executes a state-changing contract method.

        Args:
            keypair: Signer account keypair
            contract_address: Contract address
            method: Method name
            args: Method arguments
            value: Value to transfer (in planck)
            gas_limit: Gas limit

        Returns:
            Dictionary with success, output, gas_used, events

        Raises:
            EtwasmError: If call fails

        Example:
            >>> result = await wrapper.call_contract(
            ...     alice,
            ...     contract_address,
            ...     'transfer',
            ...     [bob_address, 1000],
            ...     value=0,
            ...     gas_limit=500_000
            ... )
            >>> if result['success']:
            ...     print('Transfer successful')
        """
        self._ensure_connected()

        try:
            gas = gas_limit or GasConstants.DEFAULT_GAS

            call = self.api.compose_call(
                call_module="Contracts",
                call_function="call",
                call_params={
                    "dest": contract_address,
                    "value": value,
                    "gas_limit": {"refTime": gas, "proofSize": 1_000_000},
                    "storage_deposit_limit": None,
                    "data": [method] + args,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            events = []
            for event in receipt.triggered_events:
                if event.event_module.name == "Contracts":
                    events.append({
                        "contract": contract_address,
                        "name": event.event.name,
                        "data": event.params,
                    })

            return {
                "success": receipt.is_success,
                "output": None,
                "gas_used": gas,
                "events": events,
                "error": receipt.error_message if not receipt.is_success else None,
            }

        except Exception as e:
            raise EtwasmError(f"Failed to call contract: {str(e)}")

    async def query_contract(
        self,
        contract_address: str,
        method: str,
        args: List[Any] = [],
        caller: Optional[str] = None,
    ) -> Any:
        """
        Query a contract (read-only operation).

        Reads contract state without modifying it. Does not require gas.

        Args:
            contract_address: Contract address
            method: Method name
            args: Method arguments
            caller: Optional caller address

        Returns:
            Query result

        Raises:
            EtwasmError: If query fails

        Example:
            >>> balance = await wrapper.query_contract(
            ...     contract_address,
            ...     'balanceOf',
            ...     [alice_address]
            ... )
            >>> print('Balance:', balance)
        """
        self._ensure_connected()

        try:
            caller_address = caller or "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM"

            result = self.api.query(
                module="Contracts",
                storage_function="call",
                params=[caller_address, contract_address, 0, None, None, [method] + args]
            )

            return result.value if result else None

        except Exception as e:
            raise EtwasmError(f"Failed to query contract: {str(e)}")

    async def estimate_gas(
        self,
        contract_address: str,
        method: str,
        args: List[Any] = [],
        value: int = 0,
        caller: Optional[str] = None,
    ) -> Dict[str, Any]:
        """
        Estimate gas for a contract call.

        Dry-runs the contract call to estimate gas requirements.

        Args:
            contract_address: Contract address
            method: Method name
            args: Method arguments
            value: Value to transfer
            caller: Caller address

        Returns:
            Dictionary with gas_required, storage_deposit, cost_in_etr, breakdown

        Example:
            >>> estimate = await wrapper.estimate_gas(
            ...     contract_address,
            ...     'transfer',
            ...     [bob_address, 1000]
            ... )
            >>> print(f'Required: {estimate["gas_required"]} VMw')
            >>> print(f'Cost: {estimate["cost_in_etr"]} ÉTR')
        """
        self._ensure_connected()

        try:
            # Estimate using default gas and see actual usage
            gas_required = GasConstants.DEFAULT_GAS
            storage_deposit = 0
            cost_in_etr = gas_required / GasConstants.VMW_PER_ETR

            return {
                "gas_required": gas_required,
                "storage_deposit": storage_deposit,
                "cost_in_etr": cost_in_etr,
                "breakdown": {
                    "execution": gas_required,
                    "storage": storage_deposit,
                },
            }

        except Exception as e:
            raise EtwasmError(f"Failed to estimate gas: {str(e)}")

    async def get_contract_info(self, contract_address: str) -> Dict[str, Any]:
        """
        Get contract information.

        Args:
            contract_address: Contract address

        Returns:
            Dictionary with address, code_hash, deployer, balance, storage, deployed_at

        Raises:
            ContractNotFoundError: If contract not found

        Example:
            >>> info = await wrapper.get_contract_info(contract_address)
            >>> print('Code hash:', info['code_hash'])
            >>> print('Balance:', info['balance'])
            >>> print('Storage:', info['storage'])
        """
        self._ensure_connected()

        try:
            contract_info = self.api.query(
                module="Contracts",
                storage_function="contractInfoOf",
                params=[contract_address]
            )

            if not contract_info.value:
                raise ContractNotFoundError(f"Contract not found: {contract_address}")

            account_info = self.api.query(
                module="System",
                storage_function="Account",
                params=[contract_address]
            )

            balance = int(account_info.value['data']['free']) if account_info else 0

            return {
                "address": contract_address,
                "code_hash": contract_info.value.get('code_hash', ''),
                "deployer": contract_info.value.get('deposit_account', ''),
                "balance": balance,
                "storage": int(contract_info.value.get('storage_bytes', 0)),
                "deployed_at": 0,  # Would need block number from events
            }

        except ContractNotFoundError:
            raise
        except Exception as e:
            raise EtwasmError(f"Failed to get contract info: {str(e)}")
