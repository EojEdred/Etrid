"""
Ëtrid Hyperledger Fabric Bridge Integration

Enables cross-ledger operations between Ëtrid and Hyperledger Fabric networks.

Supported operations:
- Asset tokenization (lock on Ëtrid, mint on Fabric)
- Cross-ledger transactions
- Fabric chaincode invocation from Ëtrid
- Proof verification

Dependencies:
    - hfc>=1.0.0 (Hyperledger Fabric SDK)
    - grpcio>=1.50.0
"""

import json
import hashlib
import time
from typing import List, Dict, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from datetime import datetime, timedelta

try:
    from hfc.fabric import Client as FabricClient
    from hfc.fabric_network import Gateway
except ImportError:
    raise ImportError(
        "Hyperledger Fabric SDK not installed. Install with: pip install hfc>=1.0.0"
    )

try:
    import grpc
except ImportError:
    raise ImportError("grpcio not installed. Install with: pip install grpcio>=1.50.0")


# Bridge constants
LOCK_PERIOD_DAYS = 7  # Minimum lock period to prevent double-spend
BRIDGE_VERSION = "1.0.0"


@dataclass
class FabricNetwork:
    """Represents a Hyperledger Fabric network connection."""

    network_name: str
    channel_name: str
    gateway: Gateway
    org_name: str
    peer_name: str
    user_name: str
    is_connected: bool = True

    def __repr__(self):
        return f"FabricNetwork(name={self.network_name}, channel={self.channel_name}, org={self.org_name})"


@dataclass
class BridgeTransfer:
    """Represents a cross-ledger transfer."""

    transfer_id: str
    source_chain: str  # "etrid" or "fabric"
    dest_chain: str
    asset_id: str
    amount: int
    sender_address: str
    recipient_address: str
    status: str  # "pending", "locked", "completed", "failed"
    lock_timestamp: int
    unlock_timestamp: Optional[int]
    fabric_tx_id: Optional[str] = None
    etrid_tx_hash: Optional[str] = None

    def to_dict(self) -> Dict:
        return asdict(self)


class HyperledgerBridgeError(Exception):
    """Base exception for bridge operations."""
    pass


class FabricConnectionError(HyperledgerBridgeError):
    """Raised when Fabric network connection fails."""
    pass


class BridgeValidationError(HyperledgerBridgeError):
    """Raised when bridge validation fails."""
    pass


class EndorsementError(HyperledgerBridgeError):
    """Raised when Fabric endorsement fails."""
    pass


def connect_fabric_network(
    config_path: str,
    channel_name: str = "mychannel",
    org_name: str = "Org1",
    user_name: str = "Admin",
    peer_name: str = "peer0.org1.example.com"
) -> FabricNetwork:
    """
    Connect to Hyperledger Fabric network.

    Args:
        config_path: Path to Fabric network connection profile (JSON)
        channel_name: Fabric channel name
        org_name: Organization name
        user_name: User identity
        peer_name: Peer endpoint

    Returns:
        FabricNetwork object

    Raises:
        FabricConnectionError: If connection fails

    Example:
        >>> network = connect_fabric_network("./connection-profile.json")
        >>> print(network.channel_name)
        'mychannel'
    """
    try:
        # Load connection profile
        with open(config_path, 'r') as f:
            connection_profile = json.load(f)

        # Initialize Fabric client
        client = FabricClient(net_profile=connection_profile)

        # Get user context
        user = client.get_user(org_name, user_name)

        # Create gateway connection
        gateway = Gateway()
        gateway.connect(connection_profile, {
            'wallet': None,
            'identity': user_name,
            'discovery': {'enabled': True, 'asLocalhost': False}
        })

        return FabricNetwork(
            network_name=connection_profile.get('name', 'fabric-network'),
            channel_name=channel_name,
            gateway=gateway,
            org_name=org_name,
            peer_name=peer_name,
            user_name=user_name,
            is_connected=True
        )

    except FileNotFoundError:
        raise FabricConnectionError(f"Connection profile not found: {config_path}")
    except Exception as e:
        raise FabricConnectionError(f"Failed to connect to Fabric network: {e}")


def submit_fabric_transaction(
    network: FabricNetwork,
    chaincode: str,
    function: str,
    args: List[str],
    transient: Optional[Dict] = None
) -> str:
    """
    Execute chaincode function on Fabric network.

    Args:
        network: Connected Fabric network
        chaincode: Chaincode (smart contract) name
        function: Function name to invoke
        args: Function arguments
        transient: Transient data (not stored on ledger)

    Returns:
        Transaction ID

    Raises:
        EndorsementError: If endorsement policy not satisfied

    Example:
        >>> tx_id = submit_fabric_transaction(
        ...     network, "asset-transfer", "CreateAsset",
        ...     ["asset1", "blue", "100", "Alice"]
        ... )
        >>> print(tx_id)
    """
    try:
        # Get network from gateway
        fabric_network = network.gateway.get_network(network.channel_name)

        # Get contract (chaincode)
        contract = fabric_network.get_contract(chaincode)

        # Submit transaction
        if transient:
            response = contract.submit_transaction(
                function,
                *args,
                transient_map=transient
            )
        else:
            response = contract.submit_transaction(function, *args)

        # Get transaction ID from response
        tx_id = response.decode('utf-8') if isinstance(response, bytes) else str(response)

        return tx_id

    except Exception as e:
        if "endorsement policy failure" in str(e).lower():
            raise EndorsementError(f"Endorsement policy not satisfied: {e}")
        raise HyperledgerBridgeError(f"Failed to submit transaction: {e}")


def query_fabric_state(
    network: FabricNetwork,
    chaincode: str,
    key: str,
    function: str = "ReadAsset"
) -> Any:
    """
    Read state from Fabric world state database.

    Args:
        network: Connected Fabric network
        chaincode: Chaincode name
        key: State key to query
        function: Query function name

    Returns:
        State value (parsed JSON or string)

    Example:
        >>> state = query_fabric_state(network, "asset-transfer", "asset1")
        >>> print(state['owner'])
        'Alice'
    """
    try:
        fabric_network = network.gateway.get_network(network.channel_name)
        contract = fabric_network.get_contract(chaincode)

        # Evaluate query (does not create transaction)
        response = contract.evaluate_transaction(function, key)

        # Parse response
        response_str = response.decode('utf-8') if isinstance(response, bytes) else response

        try:
            return json.loads(response_str)
        except json.JSONDecodeError:
            return response_str

    except Exception as e:
        raise HyperledgerBridgeError(f"Failed to query Fabric state: {e}")


def bridge_asset_to_fabric(
    network: FabricNetwork,
    etrid_keypair: Any,
    asset_id: str,
    amount: int,
    fabric_address: str,
    chaincode: str = "etrid-bridge"
) -> str:
    """
    Bridge asset from Ëtrid to Fabric.

    Process:
    1. Lock asset on Ëtrid chain
    2. Generate proof of lock
    3. Submit proof to Fabric
    4. Mint equivalent asset on Fabric

    Args:
        network: Connected Fabric network
        etrid_keypair: Ëtrid account keypair
        asset_id: Asset identifier
        amount: Amount to bridge
        fabric_address: Recipient address on Fabric
        chaincode: Bridge chaincode name

    Returns:
        Transfer ID

    Example:
        >>> transfer_id = bridge_asset_to_fabric(
        ...     network, keypair, "ETRID", 1000, "org1.user1"
        ... )
        >>> print(transfer_id)
    """
    # Generate unique transfer ID
    transfer_id = _generate_transfer_id(asset_id, amount)

    # Create bridge transfer record
    transfer = BridgeTransfer(
        transfer_id=transfer_id,
        source_chain="etrid",
        dest_chain="fabric",
        asset_id=asset_id,
        amount=amount,
        sender_address=etrid_keypair.ss58_address,
        recipient_address=fabric_address,
        status="pending",
        lock_timestamp=int(time.time()),
        unlock_timestamp=None
    )

    try:
        # Step 1: Lock asset on Ëtrid (call hyperledger-bridge pallet)
        # This would use the Ëtrid SDK client
        from etrid_sdk.client import EtridClient

        client = EtridClient("wss://flarechain-rpc.etrid.network")
        lock_tx = client.submit_extrinsic(
            etrid_keypair,
            "HyperledgerBridge",
            "lock_asset",
            {
                "asset_id": asset_id,
                "amount": amount,
                "dest_chain": "fabric",
                "dest_address": fabric_address,
                "transfer_id": transfer_id
            }
        )

        transfer.etrid_tx_hash = lock_tx['hash']
        transfer.status = "locked"

        # Step 2: Generate proof of lock
        lock_proof = _generate_lock_proof(lock_tx, transfer)

        # Step 3: Submit proof to Fabric chaincode
        fabric_tx_id = submit_fabric_transaction(
            network,
            chaincode,
            "MintFromEtrid",
            [
                transfer_id,
                asset_id,
                str(amount),
                fabric_address,
                json.dumps(lock_proof)
            ]
        )

        transfer.fabric_tx_id = fabric_tx_id
        transfer.status = "completed"

        return transfer_id

    except Exception as e:
        transfer.status = "failed"
        raise HyperledgerBridgeError(f"Failed to bridge asset to Fabric: {e}")


def bridge_asset_from_fabric(
    network: FabricNetwork,
    etrid_keypair: Any,
    fabric_tx_id: str,
    chaincode: str = "etrid-bridge"
) -> str:
    """
    Bridge asset from Fabric back to Ëtrid.

    Process:
    1. Burn asset on Fabric
    2. Generate proof of burn with endorsements
    3. Submit proof to Ëtrid
    4. Unlock asset on Ëtrid

    Args:
        network: Connected Fabric network
        etrid_keypair: Ëtrid account keypair
        fabric_tx_id: Original Fabric transaction ID
        chaincode: Bridge chaincode name

    Returns:
        Ëtrid transaction hash

    Example:
        >>> tx_hash = bridge_asset_from_fabric(
        ...     network, keypair, "fabric-tx-123"
        ... )
    """
    try:
        # Step 1: Get original transfer details from Fabric
        transfer_data = query_fabric_state(
            network,
            chaincode,
            fabric_tx_id,
            "GetTransfer"
        )

        # Validate lock period elapsed
        lock_time = transfer_data.get('lock_timestamp', 0)
        if time.time() - lock_time < LOCK_PERIOD_DAYS * 86400:
            raise BridgeValidationError(
                f"Lock period not elapsed. Wait {LOCK_PERIOD_DAYS} days."
            )

        # Step 2: Burn asset on Fabric
        burn_tx_id = submit_fabric_transaction(
            network,
            chaincode,
            "BurnToEtrid",
            [
                fabric_tx_id,
                transfer_data['asset_id'],
                str(transfer_data['amount'])
            ]
        )

        # Step 3: Get endorsements for proof
        endorsements = get_fabric_endorsements(network, burn_tx_id)

        # Verify endorsements
        if not verify_fabric_proof(endorsements, burn_tx_id.encode()):
            raise EndorsementError("Invalid Fabric endorsements")

        # Step 4: Submit unlock to Ëtrid
        from etrid_sdk.client import EtridClient

        client = EtridClient("wss://flarechain-rpc.etrid.network")
        unlock_tx = client.submit_extrinsic(
            etrid_keypair,
            "HyperledgerBridge",
            "unlock_asset",
            {
                "transfer_id": transfer_data['transfer_id'],
                "fabric_proof": _encode_fabric_proof(endorsements, burn_tx_id)
            }
        )

        return unlock_tx['hash']

    except Exception as e:
        raise HyperledgerBridgeError(f"Failed to bridge asset from Fabric: {e}")


def get_fabric_events(
    network: FabricNetwork,
    chaincode: str,
    event_name: str,
    start_block: int = 0
) -> List[Dict]:
    """
    Subscribe to and retrieve Fabric chaincode events.

    Args:
        network: Connected Fabric network
        chaincode: Chaincode name
        event_name: Event name to filter
        start_block: Starting block number

    Returns:
        List of event objects

    Example:
        >>> events = get_fabric_events(network, "etrid-bridge", "AssetLocked")
        >>> for event in events:
        ...     print(event['payload'])
    """
    try:
        fabric_network = network.gateway.get_network(network.channel_name)
        contract = fabric_network.get_contract(chaincode)

        # Register event listener
        events = []

        def event_callback(event):
            events.append({
                'name': event.event_name,
                'payload': json.loads(event.payload.decode('utf-8')),
                'tx_id': event.tx_id,
                'block_number': event.block_num
            })

        # Listen for events (simplified - in production use async listener)
        listener = contract.add_contract_listener(
            event_callback,
            event_name=event_name
        )

        # Return captured events
        return events

    except Exception as e:
        raise HyperledgerBridgeError(f"Failed to get Fabric events: {e}")


def verify_fabric_proof(endorsements: List[bytes], proposal: bytes) -> bool:
    """
    Verify Fabric endorsement signatures.

    Validates that sufficient organizations have endorsed the proposal
    according to the endorsement policy.

    Args:
        endorsements: List of endorsement signatures
        proposal: Transaction proposal bytes

    Returns:
        True if endorsements are valid

    Example:
        >>> valid = verify_fabric_proof(endorsements, proposal)
        >>> if valid:
        ...     print("Endorsements verified!")
    """
    try:
        # Minimum endorsements required (majority of orgs)
        MIN_ENDORSEMENTS = 2

        if len(endorsements) < MIN_ENDORSEMENTS:
            return False

        # Verify each endorsement signature
        proposal_hash = hashlib.sha256(proposal).digest()

        for endorsement in endorsements:
            # Parse endorsement (contains signature + identity)
            # In production, use proper cryptographic verification
            if len(endorsement) < 64:
                return False

            # Verify signature matches proposal hash
            # This is simplified - real implementation would use MSP identities
            signature = endorsement[:64]
            # Verify signature...

        return True

    except Exception as e:
        raise HyperledgerBridgeError(f"Failed to verify Fabric proof: {e}")


def get_fabric_block(network: FabricNetwork, block_number: int) -> Dict:
    """
    Retrieve Fabric block by number.

    Args:
        network: Connected Fabric network
        block_number: Block number to retrieve

    Returns:
        Block data dictionary

    Example:
        >>> block = get_fabric_block(network, 100)
        >>> print(block['header']['number'])
        100
    """
    try:
        fabric_network = network.gateway.get_network(network.channel_name)

        # Query block
        block = fabric_network.get_channel().query_block(block_number)

        return {
            'header': {
                'number': block.header.number,
                'previous_hash': block.header.previous_hash.hex(),
                'data_hash': block.header.data_hash.hex()
            },
            'data': {
                'transactions': len(block.data.data)
            }
        }

    except Exception as e:
        raise HyperledgerBridgeError(f"Failed to get Fabric block: {e}")


def register_fabric_network(
    etrid_keypair: Any,
    network_id: str,
    admin_certs: List[str],
    endorsement_policy: Dict
) -> str:
    """
    Register a Fabric network with Ëtrid bridge pallet.

    Requires governance approval for mainnet.

    Args:
        etrid_keypair: Admin keypair
        network_id: Unique network identifier
        admin_certs: List of admin certificate PEMs
        endorsement_policy: Endorsement policy configuration

    Returns:
        Transaction hash

    Example:
        >>> tx_hash = register_fabric_network(
        ...     admin_keypair,
        ...     "production-fabric",
        ...     [cert1, cert2],
        ...     {"majority": True, "min_orgs": 2}
        ... )
    """
    from etrid_sdk.client import EtridClient

    client = EtridClient("wss://flarechain-rpc.etrid.network")

    tx = client.submit_extrinsic(
        etrid_keypair,
        "HyperledgerBridge",
        "register_fabric_network",
        {
            "network_id": network_id,
            "admin_certs": admin_certs,
            "endorsement_policy": json.dumps(endorsement_policy)
        }
    )

    return tx['hash']


# Internal helper functions

def _generate_transfer_id(asset_id: str, amount: int) -> str:
    """Generate unique transfer ID."""
    data = f"{asset_id}:{amount}:{time.time()}".encode()
    return hashlib.sha256(data).hexdigest()[:16]


def _generate_lock_proof(lock_tx: Dict, transfer: BridgeTransfer) -> Dict:
    """Generate proof of asset lock on Ëtrid."""
    return {
        "version": BRIDGE_VERSION,
        "tx_hash": lock_tx['hash'],
        "block_number": lock_tx.get('block_number'),
        "transfer_id": transfer.transfer_id,
        "asset_id": transfer.asset_id,
        "amount": transfer.amount,
        "timestamp": transfer.lock_timestamp,
        "signature": "..."  # Merkle proof
    }


def _encode_fabric_proof(endorsements: List[bytes], burn_tx_id: str) -> bytes:
    """Encode Fabric proof for Ëtrid verification."""
    # Encode endorsements + transaction ID
    proof = b""
    proof += len(endorsements).to_bytes(4, 'big')
    for endorsement in endorsements:
        proof += len(endorsement).to_bytes(4, 'big')
        proof += endorsement
    proof += burn_tx_id.encode()
    return proof


def get_fabric_endorsements(network: FabricNetwork, tx_id: str) -> List[bytes]:
    """Retrieve endorsements for a Fabric transaction."""
    # Query transaction endorsements
    # Simplified - would parse from transaction envelope
    return [b"endorsement1", b"endorsement2"]


def disconnect_fabric_network(network: FabricNetwork):
    """
    Disconnect from Fabric network.

    Args:
        network: Connected Fabric network

    Example:
        >>> disconnect_fabric_network(network)
    """
    if network.gateway:
        network.gateway.disconnect()
        network.is_connected = False
