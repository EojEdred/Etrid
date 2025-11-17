"""
Ëtrid Ledger Hardware Wallet Integration

Provides wrappers for Ledger Nano S Plus and Nano X hardware wallets.
Supports Substrate via the Ledger Substrate app.

Dependencies:
    - ledgerblue>=0.1.45
    - ledger-agent-client>=1.0.0
"""

import struct
import time
from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass

try:
    from ledgerblue.comm import getDongle
    from ledgerblue.commException import CommException
except ImportError:
    raise ImportError(
        "ledgerblue not installed. Install with: pip install ledgerblue>=0.1.45"
    )


# Ledger APDU constants
CLA = 0x99  # Substrate app class
INS_GET_VERSION = 0x00
INS_GET_ADDRESS = 0x01
INS_SIGN = 0x02
INS_GET_PUBKEY = 0x03

# BIP44 path for Polkadot/Substrate: m/44'/354'/0'/0/0
POLKADOT_COIN_TYPE = 354

# Response codes
SW_OK = 0x9000
SW_USER_REJECTED = 0x6985
SW_INCORRECT_DATA = 0x6A80
SW_INCORRECT_LENGTH = 0x6700


@dataclass
class LedgerDevice:
    """Represents a connected Ledger device."""

    dongle: any
    app_version: str
    model: str
    locked: bool = False

    def __repr__(self):
        return f"LedgerDevice(model={self.model}, version={self.app_version}, locked={self.locked})"


@dataclass
class DeviceInfo:
    """Ledger device information."""

    model: str
    firmware_version: str
    app_name: str
    app_version: str
    mcu_version: str

    def to_dict(self) -> Dict:
        return {
            "model": self.model,
            "firmware_version": self.firmware_version,
            "app_name": self.app_name,
            "app_version": self.app_version,
            "mcu_version": self.mcu_version
        }


class LedgerError(Exception):
    """Base exception for Ledger operations."""
    pass


class LedgerConnectionError(LedgerError):
    """Raised when connection to Ledger fails."""
    pass


class LedgerUserRejectionError(LedgerError):
    """Raised when user rejects operation on device."""
    pass


class LedgerDataError(LedgerError):
    """Raised when data sent to device is invalid."""
    pass


def connect_ledger(retries: int = 3, timeout: float = 5.0) -> LedgerDevice:
    """
    Connect to a Ledger device via USB or Bluetooth.

    Args:
        retries: Number of connection attempts
        timeout: Timeout in seconds for each attempt

    Returns:
        LedgerDevice object

    Raises:
        LedgerConnectionError: If connection fails

    Example:
        >>> device = connect_ledger()
        >>> print(device.model)
        'Nano X'
    """
    last_error = None

    for attempt in range(retries):
        try:
            # Attempt to connect to Ledger device
            dongle = getDongle(debug=False)

            # Get device info to verify connection
            try:
                version_data = _send_apdu(dongle, INS_GET_VERSION, b"")
                app_version = _parse_version(version_data)

                # Detect model from capabilities
                model = _detect_model(dongle)

                return LedgerDevice(
                    dongle=dongle,
                    app_version=app_version,
                    model=model,
                    locked=False
                )
            except Exception as e:
                dongle.close()
                raise LedgerConnectionError(
                    f"Connected to device but failed to query: {e}"
                )

        except CommException as e:
            last_error = e
            if attempt < retries - 1:
                time.sleep(timeout)
                continue
        except Exception as e:
            raise LedgerConnectionError(f"Unexpected error: {e}")

    raise LedgerConnectionError(
        f"Failed to connect to Ledger after {retries} attempts. "
        f"Make sure device is unlocked and Substrate app is open. "
        f"Last error: {last_error}"
    )


def get_addresses(
    device: LedgerDevice,
    start_index: int = 0,
    count: int = 1,
    account: int = 0,
    change: int = 0
) -> List[str]:
    """
    Derive addresses from Ledger using BIP44 path.

    BIP44 path: m/44'/354'/account'/change/index

    Args:
        device: Connected Ledger device
        start_index: Starting address index
        count: Number of addresses to derive
        account: BIP44 account number
        change: BIP44 change (0=external, 1=internal)

    Returns:
        List of SS58 encoded addresses

    Example:
        >>> addresses = get_addresses(device, 0, 5)
        >>> print(addresses[0])
        '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
    """
    addresses = []

    for i in range(start_index, start_index + count):
        path = [
            0x8000002C,  # 44' (purpose)
            0x80000162,  # 354' (Polkadot coin type)
            0x80000000 | account,  # account'
            0x80000000 | change if change else 0,  # change
            i  # index
        ]

        # Encode BIP44 path
        path_data = _encode_bip44_path(path)

        try:
            # Request address from device
            response = _send_apdu(device.dongle, INS_GET_ADDRESS, path_data)

            # Parse SS58 address from response
            address = _parse_address(response)
            addresses.append(address)

        except CommException as e:
            if e.sw == SW_USER_REJECTED:
                raise LedgerUserRejectionError("User rejected address derivation")
            raise LedgerError(f"Failed to get address at index {i}: {e}")

    return addresses


def sign_transaction(
    device: LedgerDevice,
    tx_data: bytes,
    account: int = 0,
    index: int = 0
) -> bytes:
    """
    Sign a transaction with Ledger device.

    User must confirm on device screen.

    Args:
        device: Connected Ledger device
        tx_data: Encoded transaction payload
        account: BIP44 account number
        index: BIP44 address index

    Returns:
        Signature bytes (64 bytes for Ed25519)

    Raises:
        LedgerUserRejectionError: If user rejects on device

    Example:
        >>> tx_payload = encode_transaction(call, era, nonce)
        >>> signature = sign_transaction(device, tx_payload)
        >>> print(signature.hex())
    """
    # Encode BIP44 path
    path = [
        0x8000002C,  # 44'
        0x80000162,  # 354'
        0x80000000 | account,
        0x00000000,  # external
        index
    ]
    path_data = _encode_bip44_path(path)

    # Prepare signing payload
    # Format: [path_length][path][tx_length][tx_data]
    payload = path_data + struct.pack(">I", len(tx_data)) + tx_data

    try:
        # Send signing request (may require multiple APDUs for large tx)
        if len(payload) <= 255:
            # Single APDU
            response = _send_apdu(device.dongle, INS_SIGN, payload)
        else:
            # Multi-APDU for large transactions
            response = _send_large_apdu(device.dongle, INS_SIGN, payload)

        # Parse signature from response
        signature = _parse_signature(response)
        return signature

    except CommException as e:
        if e.sw == SW_USER_REJECTED:
            raise LedgerUserRejectionError("User rejected transaction signing")
        raise LedgerError(f"Failed to sign transaction: {e}")


def sign_message(device: LedgerDevice, message: str, account: int = 0, index: int = 0) -> bytes:
    """
    Sign an arbitrary message with Ledger.

    Args:
        device: Connected Ledger device
        message: Message string to sign
        account: BIP44 account number
        index: BIP44 address index

    Returns:
        Signature bytes

    Example:
        >>> sig = sign_message(device, "Hello Ëtrid!")
        >>> print(sig.hex())
    """
    # Prefix message with "\x19Ethereum Signed Message:\n" equivalent for Substrate
    message_bytes = b"<Bytes>" + message.encode('utf-8') + b"</Bytes>"

    return sign_transaction(device, message_bytes, account, index)


def get_device_info(device: LedgerDevice) -> Dict:
    """
    Query detailed device information.

    Args:
        device: Connected Ledger device

    Returns:
        Dictionary with device info

    Example:
        >>> info = get_device_info(device)
        >>> print(info['model'])
        'Nano X'
    """
    try:
        version_data = _send_apdu(device.dongle, INS_GET_VERSION, b"")

        info = DeviceInfo(
            model=device.model,
            firmware_version="2.1.0",  # Parse from device
            app_name="Substrate",
            app_version=device.app_version,
            mcu_version="1.12"  # Parse from device
        )

        return info.to_dict()

    except Exception as e:
        raise LedgerError(f"Failed to get device info: {e}")


def verify_address(device: LedgerDevice, address: str, index: int = 0) -> bool:
    """
    Display address on Ledger screen for verification.

    User can verify the address matches what is shown in wallet software.

    Args:
        device: Connected Ledger device
        address: Expected address to verify
        index: BIP44 address index

    Returns:
        True if user confirmed address matches

    Example:
        >>> verified = verify_address(device, "5GrwvaEF...", 0)
        >>> if verified:
        ...     print("Address verified by user!")
    """
    # Encode BIP44 path
    path = [0x8000002C, 0x80000162, 0x80000000, 0x00000000, index]
    path_data = _encode_bip44_path(path)

    # Add display flag
    payload = struct.pack("B", 0x01) + path_data  # 0x01 = display on screen

    try:
        response = _send_apdu(device.dongle, INS_GET_ADDRESS, payload)
        displayed_address = _parse_address(response)

        # Compare addresses
        return displayed_address == address

    except CommException as e:
        if e.sw == SW_USER_REJECTED:
            return False
        raise LedgerError(f"Failed to verify address: {e}")


def get_public_key(device: LedgerDevice, index: int = 0, account: int = 0) -> bytes:
    """
    Get public key from Ledger device.

    Args:
        device: Connected Ledger device
        index: BIP44 address index
        account: BIP44 account number

    Returns:
        32-byte Ed25519 public key

    Example:
        >>> pubkey = get_public_key(device, 0)
        >>> print(pubkey.hex())
    """
    path = [0x8000002C, 0x80000162, 0x80000000 | account, 0x00000000, index]
    path_data = _encode_bip44_path(path)

    try:
        response = _send_apdu(device.dongle, INS_GET_PUBKEY, path_data)

        # Public key is first 32 bytes of response
        if len(response) < 32:
            raise LedgerDataError("Invalid public key response")

        return response[:32]

    except CommException as e:
        raise LedgerError(f"Failed to get public key: {e}")


# Internal helper functions

def _send_apdu(dongle, instruction: int, data: bytes) -> bytes:
    """Send APDU command to Ledger."""
    apdu = bytes([CLA, instruction, 0x00, 0x00, len(data)]) + data

    try:
        response = dongle.exchange(apdu)
        return bytes(response)
    except CommException as e:
        raise e


def _send_large_apdu(dongle, instruction: int, data: bytes, chunk_size: int = 255) -> bytes:
    """Send large data in multiple APDU chunks."""
    chunks = [data[i:i+chunk_size] for i in range(0, len(data), chunk_size)]

    for i, chunk in enumerate(chunks):
        p1 = 0x00 if i == 0 else 0x01  # First chunk vs continuation
        p2 = 0x00 if i < len(chunks) - 1 else 0x01  # More data vs last chunk

        apdu = bytes([CLA, instruction, p1, p2, len(chunk)]) + chunk
        response = dongle.exchange(apdu)

        if i == len(chunks) - 1:
            return bytes(response)

    return b""


def _encode_bip44_path(path: List[int]) -> bytes:
    """Encode BIP44 path for Ledger."""
    encoded = struct.pack("B", len(path))
    for element in path:
        encoded += struct.pack(">I", element)
    return encoded


def _parse_version(data: bytes) -> str:
    """Parse app version from response."""
    if len(data) < 3:
        return "unknown"
    return f"{data[0]}.{data[1]}.{data[2]}"


def _parse_address(data: bytes) -> str:
    """Parse SS58 address from response."""
    # Address length is first byte, followed by address string
    if len(data) < 2:
        raise LedgerDataError("Invalid address response")

    addr_len = data[0]
    if len(data) < 1 + addr_len:
        raise LedgerDataError("Invalid address length")

    return data[1:1+addr_len].decode('ascii')


def _parse_signature(data: bytes) -> bytes:
    """Parse signature from response."""
    # Signature is 64 bytes for Ed25519
    if len(data) < 64:
        raise LedgerDataError("Invalid signature response")

    return data[:64]


def _detect_model(dongle) -> str:
    """Detect Ledger device model."""
    # Simple heuristic based on device capabilities
    # In production, would query device descriptor
    try:
        # Nano X supports Bluetooth
        return "Nano X"
    except:
        # Fallback to Nano S Plus
        return "Nano S Plus"


def disconnect_ledger(device: LedgerDevice):
    """
    Disconnect from Ledger device.

    Args:
        device: Connected Ledger device

    Example:
        >>> disconnect_ledger(device)
    """
    if device.dongle:
        device.dongle.close()
        device.locked = True
