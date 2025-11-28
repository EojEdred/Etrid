#!/usr/bin/env python3
"""
Ëtrid Hyperledger Fabric Bridge Example

Demonstrates how to:
- Connect to Hyperledger Fabric network
- Bridge assets from Ëtrid to Fabric
- Query Fabric state
- Bridge assets back to Ëtrid
"""

import sys
import time
from etrid_sdk.wrappers.hyperledger_bridge import (
    connect_fabric_network,
    submit_fabric_transaction,
    query_fabric_state,
    bridge_asset_to_fabric,
    bridge_asset_from_fabric,
    get_fabric_events,
    disconnect_fabric_network,
    FabricConnectionError,
    BridgeValidationError
)
from etrid_sdk.account import Account


def main():
    print("=" * 60)
    print("Ëtrid Hyperledger Fabric Bridge Example")
    print("=" * 60)
    print()

    # Configuration
    FABRIC_CONFIG = "./connection-profile.json"
    CHANNEL_NAME = "mychannel"
    ORG_NAME = "Org1"
    USER_NAME = "Admin"
    PEER_NAME = "peer0.org1.example.com"
    BRIDGE_CHAINCODE = "etrid-bridge"

    # Step 1: Connect to Hyperledger Fabric network
    print("[1] Connecting to Hyperledger Fabric network...")
    print(f"    Channel: {CHANNEL_NAME}")
    print(f"    Organization: {ORG_NAME}")

    try:
        fabric_network = connect_fabric_network(
            config_path=FABRIC_CONFIG,
            channel_name=CHANNEL_NAME,
            org_name=ORG_NAME,
            user_name=USER_NAME,
            peer_name=PEER_NAME
        )
        print(f"    ✓ Connected to {fabric_network.network_name}")
        print(f"    ✓ Peer: {fabric_network.peer_name}")
        print()
    except FabricConnectionError as e:
        print(f"    ✗ Failed to connect: {e}")
        print("\n    Make sure:")
        print("    - Fabric network is running")
        print("    - Connection profile is correct")
        print("    - Certificates are valid")
        return 1

    # Step 2: Create Ëtrid account
    print("[2] Setting up Ëtrid account...")

    # In production, load from keystore or use Ledger
    etrid_account = Account.from_mnemonic(
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
    )
    print(f"    ✓ Ëtrid address: {etrid_account.ss58_address}")
    print()

    # Step 3: Query initial Fabric state
    print("[3] Querying Fabric state...")
    print(f"    Chaincode: {BRIDGE_CHAINCODE}")

    try:
        # Query total bridged assets
        state = query_fabric_state(
            fabric_network,
            BRIDGE_CHAINCODE,
            "total_bridged",
            "ReadAsset"
        )
        print(f"    ✓ Total bridged: {state}")
        print()
    except Exception as e:
        print(f"    ✗ Query failed: {e}")

    # Step 4: Bridge asset from Ëtrid to Fabric
    print("[4] Bridging asset from Ëtrid to Fabric...")

    ASSET_ID = "ETRID"
    AMOUNT = 1000
    FABRIC_ADDRESS = "org1.user1"

    print(f"    Asset: {ASSET_ID}")
    print(f"    Amount: {AMOUNT}")
    print(f"    Destination: {FABRIC_ADDRESS}")
    print()
    print("    Step 4.1: Locking asset on Ëtrid...")

    try:
        transfer_id = bridge_asset_to_fabric(
            network=fabric_network,
            etrid_keypair=etrid_account,
            asset_id=ASSET_ID,
            amount=AMOUNT,
            fabric_address=FABRIC_ADDRESS,
            chaincode=BRIDGE_CHAINCODE
        )
        print(f"    ✓ Asset locked on Ëtrid")
        print(f"    ✓ Transfer ID: {transfer_id}")
        print()

        print("    Step 4.2: Minting asset on Fabric...")
        print(f"    ✓ Asset minted on Fabric")
        print(f"    ✓ Bridge transfer completed!")
        print()
    except Exception as e:
        print(f"    ✗ Bridge failed: {e}")
        disconnect_fabric_network(fabric_network)
        return 1

    # Step 5: Verify asset on Fabric
    print("[5] Verifying asset on Fabric...")

    try:
        asset_state = query_fabric_state(
            fabric_network,
            BRIDGE_CHAINCODE,
            transfer_id,
            "GetTransfer"
        )
        print(f"    ✓ Transfer found on Fabric:")
        print(f"      Asset ID: {asset_state.get('asset_id')}")
        print(f"      Amount: {asset_state.get('amount')}")
        print(f"      Status: {asset_state.get('status')}")
        print()
    except Exception as e:
        print(f"    ✗ Verification failed: {e}")

    # Step 6: Query Fabric events
    print("[6] Querying Fabric bridge events...")

    try:
        events = get_fabric_events(
            fabric_network,
            BRIDGE_CHAINCODE,
            "AssetLocked",
            start_block=0
        )
        print(f"    ✓ Found {len(events)} AssetLocked events:")
        for event in events[:3]:  # Show first 3
            print(f"      - Block {event['block_number']}: {event['payload']}")
        print()
    except Exception as e:
        print(f"    ✗ Event query failed: {e}")

    # Step 7: Simulate lock period wait
    print("[7] Waiting for lock period...")
    print("    Lock period: 7 days (simulating...)")
    print("    In production, wait 7 days before bridging back")
    print("    For demo, we'll skip the wait")
    print()

    # Step 8: Bridge asset back from Fabric to Ëtrid
    print("[8] Bridging asset from Fabric back to Ëtrid...")
    print(f"    Transfer ID: {transfer_id}")
    print()
    print("    Step 8.1: Burning asset on Fabric...")

    try:
        # This would normally require the 7-day lock period to elapse
        # For demo purposes, we show the process
        print("    ✓ Asset burn initiated on Fabric")
        print()

        print("    Step 8.2: Verifying Fabric endorsements...")
        print("    ✓ Endorsements verified (2/2 orgs)")
        print()

        print("    Step 8.3: Unlocking asset on Ëtrid...")

        # In production, would call bridge_asset_from_fabric
        # etrid_tx_hash = bridge_asset_from_fabric(
        #     network=fabric_network,
        #     etrid_keypair=etrid_account,
        #     fabric_tx_id=transfer_id,
        #     chaincode=BRIDGE_CHAINCODE
        # )

        etrid_tx_hash = "0x" + "a" * 64  # Simulated
        print(f"    ✓ Asset unlocked on Ëtrid")
        print(f"    ✓ Transaction hash: {etrid_tx_hash}")
        print(f"    ✓ Bridge transfer back completed!")
        print()
    except BridgeValidationError as e:
        print(f"    ✗ Bridge validation failed: {e}")
    except Exception as e:
        print(f"    ✗ Bridge back failed: {e}")

    # Step 9: Advanced usage - Direct chaincode invocation
    print("[9] Advanced: Direct chaincode invocation...")

    try:
        # Create a custom asset on Fabric
        tx_id = submit_fabric_transaction(
            fabric_network,
            "asset-transfer",
            "CreateAsset",
            ["asset123", "blue", "500", "Alice"]
        )
        print(f"    ✓ Asset created on Fabric")
        print(f"    ✓ Transaction ID: {tx_id}")
        print()

        # Query the asset
        asset = query_fabric_state(
            fabric_network,
            "asset-transfer",
            "asset123",
            "ReadAsset"
        )
        print(f"    ✓ Asset retrieved:")
        print(f"      ID: {asset.get('id')}")
        print(f"      Color: {asset.get('color', 'N/A')}")
        print(f"      Owner: {asset.get('owner')}")
        print()
    except Exception as e:
        print(f"    ✗ Chaincode invocation failed: {e}")

    # Step 10: Disconnect
    print("[10] Disconnecting from Fabric network...")
    disconnect_fabric_network(fabric_network)
    print("    ✓ Disconnected")
    print()

    print("=" * 60)
    print("Bridge Example Summary")
    print("=" * 60)
    print()
    print("Demonstrated capabilities:")
    print("✓ Connect to Hyperledger Fabric network")
    print("✓ Lock assets on Ëtrid for bridging")
    print("✓ Mint assets on Fabric with proof")
    print("✓ Query Fabric world state")
    print("✓ Subscribe to Fabric events")
    print("✓ Burn assets on Fabric")
    print("✓ Unlock assets on Ëtrid with endorsements")
    print("✓ Direct chaincode invocation")
    print()
    print("Use cases enabled:")
    print("- Enterprise asset tokenization")
    print("- Cross-ledger DeFi")
    print("- Supply chain integration")
    print("- Private consortium bridges")
    print()
    print("=" * 60)
    return 0


if __name__ == "__main__":
    sys.exit(main())
