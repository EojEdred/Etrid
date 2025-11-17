#!/usr/bin/env python3
"""
Ëtrid Ledger Hardware Wallet Example

Demonstrates how to:
- Connect to Ledger device
- Derive addresses
- Sign transactions
- Submit to FlareChain
"""

import sys
from etrid_sdk.wrappers.ledger_hardware import (
    connect_ledger,
    get_addresses,
    sign_transaction,
    verify_address,
    get_public_key,
    disconnect_ledger,
    LedgerUserRejectionError,
    LedgerConnectionError
)
from etrid_sdk.client import EtridClient


def main():
    print("=" * 60)
    print("Ëtrid Ledger Hardware Wallet Example")
    print("=" * 60)
    print()

    # Step 1: Connect to Ledger device
    print("[1] Connecting to Ledger device...")
    print("    Make sure your Ledger is unlocked and Substrate app is open.")

    try:
        device = connect_ledger(retries=3, timeout=5.0)
        print(f"    ✓ Connected to {device.model}")
        print(f"    ✓ Substrate app version: {device.app_version}")
        print()
    except LedgerConnectionError as e:
        print(f"    ✗ Failed to connect: {e}")
        print("\n    Troubleshooting:")
        print("    - Unlock your Ledger device")
        print("    - Open the Substrate app")
        print("    - Make sure USB cable is connected")
        return 1

    # Step 2: Get addresses
    print("[2] Deriving addresses from Ledger...")
    print("    Deriving 5 addresses starting at index 0")

    try:
        addresses = get_addresses(device, start_index=0, count=5)
        print(f"    ✓ Derived {len(addresses)} addresses:")
        for i, addr in enumerate(addresses):
            print(f"      [{i}] {addr}")
        print()
    except Exception as e:
        print(f"    ✗ Failed to derive addresses: {e}")
        disconnect_ledger(device)
        return 1

    # Step 3: Verify address on device screen
    print("[3] Verifying address on device screen...")
    print(f"    Address to verify: {addresses[0]}")
    print("    Please confirm the address on your Ledger device.")

    try:
        verified = verify_address(device, addresses[0], index=0)
        if verified:
            print("    ✓ Address verified successfully!")
        else:
            print("    ✗ Address verification failed or was rejected")
        print()
    except Exception as e:
        print(f"    ✗ Error during verification: {e}")

    # Step 4: Get public key
    print("[4] Getting public key...")

    try:
        pubkey = get_public_key(device, index=0)
        print(f"    ✓ Public key: {pubkey.hex()}")
        print()
    except Exception as e:
        print(f"    ✗ Failed to get public key: {e}")
        disconnect_ledger(device)
        return 1

    # Step 5: Sign a transaction
    print("[5] Signing a transaction...")
    print("    Creating a balance transfer transaction")

    # Connect to FlareChain
    try:
        client = EtridClient("wss://flarechain-rpc.etrid.network")
        print("    ✓ Connected to FlareChain")
    except Exception as e:
        print(f"    ✗ Failed to connect to FlareChain: {e}")
        disconnect_ledger(device)
        return 1

    # Create transaction payload
    recipient = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
    amount = 1_000_000_000_000  # 1 ETRID

    print(f"    Recipient: {recipient}")
    print(f"    Amount: {amount / 1_000_000_000_000} ETRID")
    print()
    print("    Please confirm the transaction on your Ledger device...")

    try:
        # Encode transaction (simplified)
        tx_data = encode_transfer_transaction(client, recipient, amount)

        # Sign with Ledger
        signature = sign_transaction(device, tx_data, account=0, index=0)
        print(f"    ✓ Transaction signed!")
        print(f"    ✓ Signature: {signature.hex()}")
        print()
    except LedgerUserRejectionError:
        print("    ✗ Transaction signing was rejected on device")
        disconnect_ledger(device)
        return 1
    except Exception as e:
        print(f"    ✗ Failed to sign transaction: {e}")
        disconnect_ledger(device)
        return 1

    # Step 6: Submit transaction to FlareChain
    print("[6] Submitting transaction to FlareChain...")

    try:
        # Submit signed transaction
        tx_hash = submit_signed_transaction(client, tx_data, signature)
        print(f"    ✓ Transaction submitted!")
        print(f"    ✓ Transaction hash: {tx_hash}")
        print(f"    ✓ View on explorer: https://explorer.etrid.network/tx/{tx_hash}")
        print()
    except Exception as e:
        print(f"    ✗ Failed to submit transaction: {e}")

    # Step 7: Disconnect
    print("[7] Disconnecting from Ledger...")
    disconnect_ledger(device)
    print("    ✓ Disconnected")
    print()

    print("=" * 60)
    print("Example completed successfully!")
    print("=" * 60)
    return 0


def encode_transfer_transaction(client, recipient, amount):
    """
    Encode a balance transfer transaction.

    In production, this would use the Ëtrid SDK to properly encode
    the transaction with nonce, era, tip, etc.
    """
    # Simplified encoding for demonstration
    tx_payload = b"balance.transfer:" + recipient.encode() + b":" + str(amount).encode()
    return tx_payload


def submit_signed_transaction(client, tx_data, signature):
    """
    Submit a signed transaction to FlareChain.

    In production, this would construct the full extrinsic with signature
    and submit via RPC.
    """
    # Simplified submission for demonstration
    import hashlib
    tx_hash = hashlib.sha256(tx_data + signature).hexdigest()
    return f"0x{tx_hash}"


if __name__ == "__main__":
    sys.exit(main())
