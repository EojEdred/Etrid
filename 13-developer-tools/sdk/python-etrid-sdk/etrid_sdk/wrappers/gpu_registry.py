"""GPU Registry Wrapper - GPU Provider Registration and Management"""

from typing import Dict, Any, List, Optional, Literal
from substrateinterface import SubstrateInterface, Keypair
from ..errors import NotConnectedError, GPURegistryError, GPUNotFoundError, InsufficientStakeError


# Type aliases for clarity
GpuId = int
Balance = int
Timestamp = int
BasisPoints = int  # 0-10000 representing 0%-100%


class GpuSpecs:
    """GPU hardware specifications"""

    def __init__(
        self,
        model: str,
        vram_gb: int,
        compute_units: int,
        clock_speed_mhz: int,
        tdp_watts: int
    ):
        """
        Initialize GPU specifications

        Args:
            model: GPU model name (e.g., "RTX 4090", "A100")
            vram_gb: VRAM in gigabytes
            compute_units: Number of compute units (CUDA cores, stream processors, etc.)
            clock_speed_mhz: Clock speed in MHz
            tdp_watts: Power consumption in watts
        """
        self.model = model
        self.vram_gb = vram_gb
        self.compute_units = compute_units
        self.clock_speed_mhz = clock_speed_mhz
        self.tdp_watts = tdp_watts

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for substrate call"""
        return {
            'model': self.model.encode('utf-8'),
            'vram_gb': self.vram_gb,
            'compute_units': self.compute_units,
            'clock_speed_mhz': self.clock_speed_mhz,
            'tdp_watts': self.tdp_watts,
        }


class HardwareAttestation:
    """Hardware attestation proof (TPM, benchmarks)"""

    def __init__(
        self,
        tpm_quote: bytes,
        benchmark_score: int,
        timestamp: int
    ):
        """
        Initialize hardware attestation

        Args:
            tpm_quote: TPM quote proving hardware authenticity
            benchmark_score: Benchmark score proving performance
            timestamp: Attestation timestamp
        """
        self.tpm_quote = tpm_quote
        self.benchmark_score = benchmark_score
        self.timestamp = timestamp

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for substrate call"""
        return {
            'tpm_quote': self.tpm_quote,
            'benchmark_score': self.benchmark_score,
            'timestamp': self.timestamp,
        }


class Reputation:
    """GPU provider reputation metrics"""

    def __init__(
        self,
        jobs_completed: int = 0,
        jobs_failed: int = 0,
        uptime_bps: int = 0,
        rating: int = 0,
        rating_count: int = 0
    ):
        """
        Initialize reputation

        Args:
            jobs_completed: Total jobs completed
            jobs_failed: Total jobs failed
            uptime_bps: Uptime percentage (0-10000 = 0.00%-100.00%)
            rating: Average rating (0-50000 = 0.0-5.0 stars, scaled by 10000)
            rating_count: Total ratings received
        """
        self.jobs_completed = jobs_completed
        self.jobs_failed = jobs_failed
        self.uptime_bps = uptime_bps
        self.rating = rating
        self.rating_count = rating_count

    @property
    def uptime_percent(self) -> float:
        """Get uptime as percentage (0-100)"""
        return self.uptime_bps / 100.0

    @property
    def rating_stars(self) -> float:
        """Get rating as stars (0.0-5.0)"""
        return self.rating / 10000.0

    @property
    def success_rate(self) -> float:
        """Get job success rate as percentage"""
        total = self.jobs_completed + self.jobs_failed
        if total == 0:
            return 0.0
        return (self.jobs_completed / total) * 100.0


AvailabilitySchedule = Literal["AlwaysOn", "BusinessHours"] | Dict[str, bytes]


class GpuStatus:
    """GPU node status enumeration"""
    ACTIVE = "Active"
    PAUSED = "Paused"
    OFFLINE = "Offline"
    SLASHED = "Slashed"


class GpuNode:
    """GPU provider node information"""

    def __init__(
        self,
        provider: str,
        specs: GpuSpecs,
        attestation: HardwareAttestation,
        stake: Balance,
        status: str,
        reputation: Reputation,
        schedule: Any,
        registered_at: Timestamp,
        last_heartbeat: Timestamp
    ):
        self.provider = provider
        self.specs = specs
        self.attestation = attestation
        self.stake = stake
        self.status = status
        self.reputation = reputation
        self.schedule = schedule
        self.registered_at = registered_at
        self.last_heartbeat = last_heartbeat


class ProviderEarnings:
    """Provider earnings history"""

    def __init__(
        self,
        total_earned: Balance,
        pending_payout: Balance,
        last_payout: Timestamp,
        earnings_history: List[Dict[str, Any]]
    ):
        self.total_earned = total_earned
        self.pending_payout = pending_payout
        self.last_payout = last_payout
        self.earnings_history = earnings_history


class GPURegistryWrapper:
    """
    Wrapper for pallet-gpu-registry - GPU Provider Registration & Management

    Manages GPU provider registration, staking, reputation tracking, and marketplace
    integration for the AI Compute PBC.

    Features:
    - GPU node registration with hardware attestation
    - Staking mechanism (providers stake ËDSC to participate)
    - Reputation tracking (uptime, job success rate, user ratings)
    - Hardware verification (prevent fake/virtualized GPUs)
    - Scheduled availability (24/7, business hours, custom schedules)
    - Provider earnings tracking

    Example:
        ```python
        from etrid_sdk import EtridClient
        from etrid_sdk.wrappers import GPURegistryWrapper, GpuSpecs, HardwareAttestation

        client = EtridClient("wss://ai-compute-pbc.etrid.io")
        gpu_registry = GPURegistryWrapper(client.api)

        # Register GPU
        specs = GpuSpecs(
            model="RTX 4090",
            vram_gb=24,
            compute_units=16384,
            clock_speed_mhz=2520,
            tdp_watts=450
        )

        attestation = HardwareAttestation(
            tpm_quote=b"...",
            benchmark_score=98500,
            timestamp=1700000000
        )

        gpu_id = await gpu_registry.register_gpu(
            keypair,
            specs,
            attestation,
            stake=100_000_000_000_000_000_000,  # 100 ËDSC
            schedule="AlwaysOn"
        )

        # Query GPU details
        gpu = await gpu_registry.get_gpu_specs(gpu_id)
        print(f"GPU: {gpu.specs.model}, {gpu.specs.vram_gb}GB VRAM")

        # Check reputation
        rep = await gpu_registry.get_reputation(gpu_id)
        print(f"Rating: {rep.rating_stars}/5.0, Uptime: {rep.uptime_percent}%")

        # Search for GPUs
        results = await gpu_registry.search_gpus(
            min_vram_gb=16,
            min_compute_units=10000,
            status="Active"
        )
        ```
    """

    def __init__(self, api: SubstrateInterface):
        """
        Initialize GPU Registry wrapper

        Args:
            api: Connected Substrate API instance
        """
        self.api = api
        self.pallet = "gpuRegistry"

    def _ensure_connected(self):
        """Ensure API is connected"""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def register_gpu(
        self,
        keypair: Keypair,
        specs: GpuSpecs,
        attestation: HardwareAttestation,
        stake: Balance,
        schedule: AvailabilitySchedule = "AlwaysOn"
    ) -> GpuId:
        """
        Register a new GPU node

        Registers GPU with hardware specifications, attestation proof, and stake.
        Provider must stake minimum amount of ËDSC tokens.

        Args:
            keypair: Provider's keypair for signing
            specs: GPU hardware specifications
            attestation: Hardware attestation proof (TPM quote, benchmark)
            stake: Amount to stake in ËDSC (must be >= MinimumStake)
            schedule: Availability schedule ("AlwaysOn", "BusinessHours", or custom)

        Returns:
            GPU ID assigned to the registered GPU

        Raises:
            InsufficientStakeError: If stake is below minimum
            GPURegistryError: If registration fails

        Example:
            ```python
            gpu_id = await registry.register_gpu(
                keypair,
                GpuSpecs("RTX 4090", 24, 16384, 2520, 450),
                attestation,
                stake=100_000_000_000_000_000_000,  # 100 ËDSC
                schedule="AlwaysOn"
            )
            print(f"Registered GPU with ID: {gpu_id}")
            ```
        """
        self._ensure_connected()

        try:
            # Build schedule enum
            if schedule == "AlwaysOn":
                schedule_param = {"AlwaysOn": None}
            elif schedule == "BusinessHours":
                schedule_param = {"BusinessHours": None}
            else:
                schedule_param = schedule

            # Create extrinsic
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="register_gpu",
                call_params={
                    'specs': specs.to_dict(),
                    'attestation': attestation.to_dict(),
                    'stake': stake,
                    'schedule': schedule_param,
                }
            )

            # Submit and wait for finalization
            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            # Extract GPU ID from events
            for event in receipt.triggered_events:
                if event.value['module_id'] == self.pallet and event.value['event_id'] == 'GpuRegistered':
                    gpu_id = event.value['attributes']['gpu_id']
                    return gpu_id

            raise GPURegistryError("GPU registration failed: no GpuRegistered event found")

        except Exception as e:
            if "InsufficientStake" in str(e):
                raise InsufficientStakeError(f"Stake {stake} is below minimum required")
            raise GPURegistryError(f"Failed to register GPU: {str(e)}")

    async def unregister_gpu(self, keypair: Keypair, gpu_id: GpuId) -> bool:
        """
        Unregister GPU and withdraw stake

        Removes GPU from registry and returns staked ËDSC to provider.
        Only the GPU owner can unregister.

        Args:
            keypair: Provider's keypair
            gpu_id: GPU ID to unregister

        Returns:
            True if successful

        Raises:
            GPUNotFoundError: If GPU doesn't exist
            GPURegistryError: If not owner or unregistration fails

        Example:
            ```python
            await registry.unregister_gpu(keypair, gpu_id)
            print("GPU unregistered, stake withdrawn")
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="unregister_gpu",
                call_params={'gpu_id': gpu_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            if "GpuNotFound" in str(e):
                raise GPUNotFoundError(f"GPU {gpu_id} not found")
            raise GPURegistryError(f"Failed to unregister GPU: {str(e)}")

    async def update_availability(
        self,
        keypair: Keypair,
        gpu_id: GpuId,
        schedule: AvailabilitySchedule
    ) -> bool:
        """
        Update GPU availability schedule

        Sets when GPU is available for compute jobs.

        Args:
            keypair: Provider's keypair
            gpu_id: GPU ID
            schedule: Availability schedule
                - "AlwaysOn": 24/7 availability
                - "BusinessHours": 9am-5pm UTC
                - Custom: {"Custom": bytes} - 168-bit weekly schedule

        Returns:
            True if successful

        Example:
            ```python
            # Set to business hours
            await registry.update_availability(keypair, gpu_id, "BusinessHours")

            # Set custom schedule (168 bits = 21 bytes for weekly schedule)
            custom_schedule = {"Custom": b"\\x00" * 21}
            await registry.update_availability(keypair, gpu_id, custom_schedule)
            ```
        """
        self._ensure_connected()

        try:
            if schedule == "AlwaysOn":
                schedule_param = {"AlwaysOn": None}
            elif schedule == "BusinessHours":
                schedule_param = {"BusinessHours": None}
            else:
                schedule_param = schedule

            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="update_availability",
                call_params={
                    'gpu_id': gpu_id,
                    'schedule': schedule_param,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            raise GPURegistryError(f"Failed to update availability: {str(e)}")

    async def get_gpu_specs(self, gpu_id: GpuId) -> GpuNode:
        """
        Query GPU hardware details

        Args:
            gpu_id: GPU ID

        Returns:
            Complete GPU node information

        Raises:
            GPUNotFoundError: If GPU doesn't exist
        """
        self._ensure_connected()

        try:
            result = self.api.query(self.pallet, "GpuNodes", [gpu_id])

            if result.value is None:
                raise GPUNotFoundError(f"GPU {gpu_id} not found")

            data = result.value

            specs = GpuSpecs(
                model=data['specs']['model'].decode('utf-8'),
                vram_gb=data['specs']['vram_gb'],
                compute_units=data['specs']['compute_units'],
                clock_speed_mhz=data['specs']['clock_speed_mhz'],
                tdp_watts=data['specs']['tdp_watts'],
            )

            attestation = HardwareAttestation(
                tpm_quote=data['attestation']['tpm_quote'],
                benchmark_score=data['attestation']['benchmark_score'],
                timestamp=data['attestation']['timestamp'],
            )

            reputation = Reputation(
                jobs_completed=data['reputation']['jobs_completed'],
                jobs_failed=data['reputation']['jobs_failed'],
                uptime_bps=data['reputation']['uptime_bps'],
                rating=data['reputation']['rating'],
                rating_count=data['reputation']['rating_count'],
            )

            return GpuNode(
                provider=data['provider'],
                specs=specs,
                attestation=attestation,
                stake=data['stake'],
                status=data['status'],
                reputation=reputation,
                schedule=data['schedule'],
                registered_at=data['registered_at'],
                last_heartbeat=data['last_heartbeat'],
            )

        except GPUNotFoundError:
            raise
        except Exception as e:
            raise GPURegistryError(f"Failed to query GPU specs: {str(e)}")

    async def get_reputation(self, gpu_id: GpuId) -> Reputation:
        """
        Get provider reputation metrics

        Args:
            gpu_id: GPU ID

        Returns:
            Reputation metrics (jobs, uptime, rating)

        Example:
            ```python
            rep = await registry.get_reputation(gpu_id)
            print(f"Success rate: {rep.success_rate}%")
            print(f"Uptime: {rep.uptime_percent}%")
            print(f"Rating: {rep.rating_stars}/5.0 ({rep.rating_count} reviews)")
            ```
        """
        gpu = await self.get_gpu_specs(gpu_id)
        return gpu.reputation

    async def search_gpus(
        self,
        min_vram_gb: Optional[int] = None,
        min_compute_units: Optional[int] = None,
        status: Optional[str] = None,
        min_rating: Optional[float] = None,
        min_uptime: Optional[float] = None,
        limit: int = 100
    ) -> List[GpuNode]:
        """
        Search for GPUs matching criteria

        Args:
            min_vram_gb: Minimum VRAM in GB
            min_compute_units: Minimum compute units
            status: GPU status filter ("Active", "Paused", etc.)
            min_rating: Minimum rating (0.0-5.0 stars)
            min_uptime: Minimum uptime percentage (0-100)
            limit: Maximum results to return

        Returns:
            List of matching GPU nodes

        Example:
            ```python
            # Find high-end GPUs with good reputation
            gpus = await registry.search_gpus(
                min_vram_gb=24,
                min_compute_units=15000,
                status="Active",
                min_rating=4.0,
                min_uptime=95.0
            )

            for gpu in gpus:
                print(f"{gpu.specs.model}: {gpu.reputation.rating_stars}/5.0")
            ```
        """
        self._ensure_connected()

        try:
            # Get total GPU count
            next_id = self.api.query(self.pallet, "NextGpuId", []).value

            results = []
            for gpu_id in range(next_id):
                if len(results) >= limit:
                    break

                try:
                    gpu = await self.get_gpu_specs(gpu_id)

                    # Apply filters
                    if min_vram_gb and gpu.specs.vram_gb < min_vram_gb:
                        continue
                    if min_compute_units and gpu.specs.compute_units < min_compute_units:
                        continue
                    if status and gpu.status != status:
                        continue
                    if min_rating and gpu.reputation.rating_stars < min_rating:
                        continue
                    if min_uptime and gpu.reputation.uptime_percent < min_uptime:
                        continue

                    results.append(gpu)

                except GPUNotFoundError:
                    continue

            return results

        except Exception as e:
            raise GPURegistryError(f"Failed to search GPUs: {str(e)}")

    async def report_uptime(self, keypair: Keypair, gpu_id: GpuId) -> bool:
        """
        Report GPU online status (heartbeat)

        Provider should call this periodically to prove GPU is online.

        Args:
            keypair: Provider's keypair
            gpu_id: GPU ID

        Returns:
            True if successful

        Example:
            ```python
            # Call this every 5 minutes
            await registry.report_uptime(keypair, gpu_id)
            ```
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="report_heartbeat",
                call_params={'gpu_id': gpu_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            return receipt.is_success

        except Exception as e:
            raise GPURegistryError(f"Failed to report uptime: {str(e)}")

    async def get_provider_earnings(self, provider: str) -> ProviderEarnings:
        """
        Query provider earnings history

        Args:
            provider: Provider account address

        Returns:
            Earnings data (total, pending, history)

        Example:
            ```python
            earnings = await registry.get_provider_earnings(provider_address)
            print(f"Total earned: {earnings.total_earned / 1e18} ËDSC")
            print(f"Pending payout: {earnings.pending_payout / 1e18} ËDSC")
            ```
        """
        self._ensure_connected()

        # This would query a separate earnings tracking storage
        # For now, returning placeholder structure
        return ProviderEarnings(
            total_earned=0,
            pending_payout=0,
            last_payout=0,
            earnings_history=[]
        )

    async def slash_provider(self, keypair: Keypair, gpu_id: GpuId) -> Balance:
        """
        Penalize provider for poor performance (validator/governance only)

        Slashes a percentage of provider's stake for misbehavior.
        Requires root/governance permission.

        Args:
            keypair: Validator/governance keypair
            gpu_id: GPU ID to slash

        Returns:
            Amount slashed

        Raises:
            GPURegistryError: If not authorized or slash fails
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module=self.pallet,
                call_function="slash_provider",
                call_params={'gpu_id': gpu_id}
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_finalization=True)

            # Extract slash amount from events
            for event in receipt.triggered_events:
                if event.value['module_id'] == self.pallet and event.value['event_id'] == 'ProviderSlashed':
                    return event.value['attributes']['slash_amount']

            return 0

        except Exception as e:
            raise GPURegistryError(f"Failed to slash provider: {str(e)}")
