"""GPU NFT Wrapper - Tradeable GPU Certificates"""

from typing import Dict, Any, List, Optional
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, GPUNFTError


# Type aliases
NftId = int
GpuId = int
Balance = int
Timestamp = int


class GpuNFT:
    """GPU NFT certificate representing GPU ownership"""

    def __init__(
        self,
        nft_id: NftId,
        owner: str,
        gpu_id: GpuId,
        reputation_snapshot: int,
        total_earnings: Balance,
        is_listed: bool,
        list_price: Balance,
        minted_at: Timestamp
    ):
        """
        Initialize GPU NFT

        Args:
            nft_id: NFT ID
            owner: Current owner address
            gpu_id: Associated GPU ID
            reputation_snapshot: Reputation score at mint time
            total_earnings: Total earnings accumulated
            is_listed: Whether listed for sale
            list_price: Sale price (if listed)
            minted_at: Mint timestamp
        """
        self.nft_id = nft_id
        self.owner = owner
        self.gpu_id = gpu_id
        self.reputation_snapshot = reputation_snapshot
        self.total_earnings = total_earnings
        self.is_listed = is_listed
        self.list_price = list_price
        self.minted_at = minted_at

    @property
    def reputation_score(self) -> float:
        """Get reputation as score out of 100"""
        return self.reputation_snapshot / 100.0


class OwnershipRecord:
    """NFT ownership history record"""

    def __init__(
        self,
        previous_owner: str,
        new_owner: str,
        price: Balance,
        timestamp: Timestamp
    ):
        self.previous_owner = previous_owner
        self.new_owner = new_owner
        self.price = price
        self.timestamp = timestamp


class RentalTerms:
    """GPU rental pricing and terms"""

    def __init__(
        self,
        hourly_rate: Balance,
        minimum_hours: int,
        maximum_hours: int,
        deposit_required: Balance,
        auto_renew: bool
    ):
        """
        Initialize rental terms

        Args:
            hourly_rate: Price per hour in ËDSC
            minimum_hours: Minimum rental duration
            maximum_hours: Maximum rental duration
            deposit_required: Security deposit
            auto_renew: Auto-renew after expiration
        """
        self.hourly_rate = hourly_rate
        self.minimum_hours = minimum_hours
        self.maximum_hours = maximum_hours
        self.deposit_required = deposit_required
        self.auto_renew = auto_renew


class GPUNFTWrapper:
    """
    Wrapper for pallet-gpu-nft - Tradeable GPU Certificates

    Provides NFT-based GPU ownership, marketplace trading, and rental capabilities.
    GPU owners can mint NFTs representing their hardware, trade them, or rent them
    out for compute jobs.

    Features:
    - Mint GPUs as tradeable NFTs
    - Transfer GPU ownership
    - List/buy GPUs on marketplace
    - Track ownership history and provenance
    - Configure rental terms and pricing
    - Rent GPUs for compute jobs

    Example:
        ```python
        from etrid_sdk import EtridClient
        from etrid_sdk.wrappers import GPUNFTWrapper, RentalTerms

        client = EtridClient("wss://ai-compute-pbc.etrid.io")
        gpu_nft = GPUNFTWrapper(client.api)

        # Mint GPU as NFT
        nft_id = await gpu_nft.mint_gpu_nft(keypair, gpu_id=1)
        print(f"Minted NFT #{nft_id}")

        # List for sale
        sale_price = 1000_000_000_000_000_000_000  # 1000 ËDSC
        await gpu_nft.list_for_sale(keypair, nft_id, sale_price)

        # Configure rental terms
        terms = RentalTerms(
            hourly_rate=10_000_000_000_000_000_000,  # 10 ËDSC/hour
            minimum_hours=1,
            maximum_hours=720,  # 30 days
            deposit_required=100_000_000_000_000_000_000,  # 100 ËDSC
            auto_renew=False
        )
        await gpu_nft.set_rental_terms(keypair, nft_id, terms)

        # Rent GPU
        rental_id = await gpu_nft.rent_gpu(
            keypair,
            nft_id,
            duration_hours=24
        )

        # Check ownership history
        history = await gpu_nft.get_ownership_history(nft_id)
        for record in history:
            print(f"{record.previous_owner} -> {record.new_owner}: {record.price}")
        ```
    """

    def __init__(self, api: SubstrateInterface):
        """
        Initialize GPU NFT wrapper

        Args:
            api: Connected Substrate API instance
        """
        self.api = api
        self.pallet = "gpuNft"

    def _ensure_connected(self):
        """Ensure API is connected"""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def mint_gpu_nft(self, keypair: Keypair, gpu_id: GpuId) -> NftId:
        """
        Mint GPU as NFT

        Creates an NFT representing ownership of a GPU. The minter must be
        the GPU owner/provider.

        Args:
            keypair: Owner's keypair
            gpu_id: GPU ID to mint as NFT

        Returns:
            NFT ID of newly minted certificate

        Raises:
            GPUNFTError: If minting fails or not authorized

        Example:
            ```python
            nft_id = await gpu_nft.mint_gpu_nft(keypair, gpu_id=1)
            print(f"Minted GPU NFT #{nft_id}")
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="mint_nft",
                call_params={'gpu_id': gpu_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            # Extract NFT ID from events
            for event in receipt.triggered_events:
                if event.value['module_id'] == self.pallet and event.value['event_id'] == 'NFTMinted':
                    return event.value['attributes']['nft_id']

            raise GPUNFTError("NFT minting failed: no NFTMinted event found")

        except Exception as e:
            raise GPUNFTError(f"Failed to mint GPU NFT: {str(e)}")

    async def transfer_gpu_nft(
        self,
        keypair: Keypair,
        nft_id: NftId,
        to: str
    ) -> bool:
        """
        Transfer GPU NFT to another account

        Args:
            keypair: Current owner's keypair
            nft_id: NFT ID to transfer
            to: Recipient address

        Returns:
            True if successful

        Raises:
            GPUNFTError: If not owner or transfer fails

        Example:
            ```python
            await gpu_nft.transfer_gpu_nft(
                keypair,
                nft_id=5,
                to="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
            )
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="transfer_nft",
                call_params={
                    'nft_id': nft_id,
                    'to': to,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            raise GPUNFTError(f"Failed to transfer NFT: {str(e)}")

    async def list_for_sale(
        self,
        keypair: Keypair,
        nft_id: NftId,
        price: Balance
    ) -> bool:
        """
        List GPU NFT for sale on marketplace

        Args:
            keypair: Owner's keypair
            nft_id: NFT ID to list
            price: Sale price in ËDSC

        Returns:
            True if successful

        Example:
            ```python
            # List for 1000 ËDSC
            price = 1000_000_000_000_000_000_000
            await gpu_nft.list_for_sale(keypair, nft_id, price)
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="list_nft",
                call_params={
                    'nft_id': nft_id,
                    'price': price,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            raise GPUNFTError(f"Failed to list NFT for sale: {str(e)}")

    async def buy_gpu_nft(
        self,
        keypair: Keypair,
        nft_id: NftId
    ) -> bool:
        """
        Purchase GPU NFT from marketplace

        Buys a listed NFT, transferring ownership and paying the list price.

        Args:
            keypair: Buyer's keypair
            nft_id: NFT ID to purchase

        Returns:
            True if successful

        Raises:
            GPUNFTError: If not listed, insufficient funds, or purchase fails

        Example:
            ```python
            # Check if listed
            nft = await gpu_nft.get_nft_metadata(nft_id)
            if nft.is_listed:
                print(f"Price: {nft.list_price / 1e18} ËDSC")
                await gpu_nft.buy_gpu_nft(keypair, nft_id)
                print("Purchase complete!")
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="buy_nft",
                call_params={'nft_id': nft_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            if "NotListed" in str(e):
                raise GPUNFTError(f"NFT {nft_id} is not listed for sale")
            raise GPUNFTError(f"Failed to buy NFT: {str(e)}")

    async def get_nft_metadata(self, nft_id: NftId) -> GpuNFT:
        """
        Query NFT metadata and details

        Args:
            nft_id: NFT ID

        Returns:
            Complete NFT information

        Raises:
            GPUNFTError: If NFT doesn't exist

        Example:
            ```python
            nft = await gpu_nft.get_nft_metadata(5)
            print(f"Owner: {nft.owner}")
            print(f"GPU ID: {nft.gpu_id}")
            print(f"Reputation: {nft.reputation_score}/100")
            print(f"Earnings: {nft.total_earnings / 1e18} ËDSC")
            ```
        """
        self._ensure_connected()

        try:
            result = self.api.query(self.pallet, "GpuNFTs", [nft_id])

            if result.value is None:
                raise GPUNFTError(f"NFT {nft_id} not found")

            data = result.value

            return GpuNFT(
                nft_id=nft_id,
                owner=data['owner'],
                gpu_id=data['gpu_id'],
                reputation_snapshot=data['reputation_snapshot'],
                total_earnings=data['total_earnings'],
                is_listed=data['is_listed'],
                list_price=data['list_price'],
                minted_at=data['minted_at'],
            )

        except Exception as e:
            raise GPUNFTError(f"Failed to query NFT metadata: {str(e)}")

    async def get_ownership_history(self, nft_id: NftId) -> List[OwnershipRecord]:
        """
        Track NFT ownership provenance

        Returns complete ownership history showing all transfers.

        Args:
            nft_id: NFT ID

        Returns:
            List of ownership records (chronological)

        Example:
            ```python
            history = await gpu_nft.get_ownership_history(nft_id)
            print(f"NFT has changed hands {len(history)} times")

            for i, record in enumerate(history):
                print(f"Transfer {i+1}:")
                print(f"  From: {record.previous_owner}")
                print(f"  To: {record.new_owner}")
                print(f"  Price: {record.price / 1e18} ËDSC")
                print(f"  Date: {record.timestamp}")
            ```
        """
        self._ensure_connected()

        try:
            result = self.api.query(self.pallet, "OwnershipHistory", [nft_id])

            if result.value is None:
                return []

            history = []
            for record in result.value:
                history.append(OwnershipRecord(
                    previous_owner=record['previous_owner'],
                    new_owner=record['new_owner'],
                    price=record['price'],
                    timestamp=record['timestamp'],
                ))

            return history

        except Exception as e:
            raise GPUNFTError(f"Failed to query ownership history: {str(e)}")

    async def set_rental_terms(
        self,
        keypair: Keypair,
        nft_id: NftId,
        terms: RentalTerms
    ) -> bool:
        """
        Configure GPU rental pricing and terms

        Sets hourly rate, duration limits, deposit requirements for renting GPU.

        Args:
            keypair: Owner's keypair
            nft_id: NFT ID
            terms: Rental terms configuration

        Returns:
            True if successful

        Example:
            ```python
            terms = RentalTerms(
                hourly_rate=10_000_000_000_000_000_000,  # 10 ËDSC/hour
                minimum_hours=1,
                maximum_hours=720,  # 30 days max
                deposit_required=100_000_000_000_000_000_000,  # 100 ËDSC
                auto_renew=False
            )

            await gpu_nft.set_rental_terms(keypair, nft_id, terms)
            print("Rental terms configured")
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="set_rental_terms",
                call_params={
                    'nft_id': nft_id,
                    'hourly_rate': terms.hourly_rate,
                    'minimum_hours': terms.minimum_hours,
                    'maximum_hours': terms.maximum_hours,
                    'deposit_required': terms.deposit_required,
                    'auto_renew': terms.auto_renew,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            raise GPUNFTError(f"Failed to set rental terms: {str(e)}")

    async def rent_gpu(
        self,
        keypair: Keypair,
        nft_id: NftId,
        duration_hours: int
    ) -> int:
        """
        Rent GPU for compute jobs

        Rents GPU for specified duration, paying hourly rate + deposit.

        Args:
            keypair: Renter's keypair
            nft_id: NFT ID to rent
            duration_hours: Rental duration in hours

        Returns:
            Rental ID

        Raises:
            GPUNFTError: If terms not set, duration invalid, or insufficient funds

        Example:
            ```python
            # Rent for 24 hours
            rental_id = await gpu_nft.rent_gpu(
                keypair,
                nft_id=5,
                duration_hours=24
            )

            print(f"GPU rented! Rental ID: {rental_id}")
            print("You can now submit compute jobs to this GPU")
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="rent_gpu",
                call_params={
                    'nft_id': nft_id,
                    'duration_hours': duration_hours,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            # Extract rental ID from events
            for event in receipt.triggered_events:
                if event.value['module_id'] == self.pallet and event.value['event_id'] == 'GPURented':
                    return event.value['attributes']['rental_id']

            raise GPUNFTError("GPU rental failed: no GPURented event found")

        except Exception as e:
            raise GPUNFTError(f"Failed to rent GPU: {str(e)}")

    async def get_listed_nfts(self, limit: int = 100) -> List[GpuNFT]:
        """
        Get all NFTs currently listed for sale

        Args:
            limit: Maximum results to return

        Returns:
            List of listed NFTs

        Example:
            ```python
            listed = await gpu_nft.get_listed_nfts(limit=50)

            print(f"Found {len(listed)} GPUs for sale:")
            for nft in listed:
                print(f"NFT #{nft.nft_id}: {nft.list_price / 1e18} ËDSC")
            ```
        """
        self._ensure_connected()

        try:
            next_id = self.api.query(self.pallet, "NextNftId", []).value

            listed = []
            for nft_id in range(next_id):
                if len(listed) >= limit:
                    break

                try:
                    nft = await self.get_nft_metadata(nft_id)
                    if nft.is_listed:
                        listed.append(nft)
                except GPUNFTError:
                    continue

            return listed

        except Exception as e:
            raise GPUNFTError(f"Failed to get listed NFTs: {str(e)}")
