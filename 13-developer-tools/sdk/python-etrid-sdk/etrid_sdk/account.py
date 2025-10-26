"""Account management and cryptographic operations"""

from substrateinterface import Keypair

class Account:
    """Ëtrid account"""

    def __init__(self, keypair: Keypair):
        self._keypair = keypair

    @classmethod
    def from_mnemonic(cls, mnemonic: str) -> "Account":
        """
        Create account from mnemonic phrase

        Args:
            mnemonic: 12 or 24 word mnemonic phrase

        Returns:
            New Account instance
        """
        keypair = Keypair.create_from_mnemonic(mnemonic)
        return cls(keypair)

    @classmethod
    def generate(cls) -> "Account":
        """
        Generate a new random account

        Returns:
            New Account instance
        """
        keypair = Keypair.create_from_mnemonic(Keypair.generate_mnemonic())
        return cls(keypair)

    @property
    def address(self) -> str:
        """Get the account address (SS58 format)"""
        return self._keypair.ss58_address

    @property
    def public_key(self) -> str:
        """Get the public key (hex format)"""
        return self._keypair.public_key.hex()

    def sign(self, message: bytes) -> bytes:
        """
        Sign a message

        Args:
            message: Message to sign

        Returns:
            Signature bytes
        """
        return self._keypair.sign(message)
