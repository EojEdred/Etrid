#!/usr/bin/env python3
"""
GPU Marketplace Example

Demonstrates GPU marketplace integration on AI Compute PBC:
- Registering GPU nodes
- Searching for available GPUs
- Minting GPU NFTs
- Trading GPU ownership
- Renting GPUs for compute jobs

This example showcases the GPU registry and NFT marketplace features.
"""

import asyncio
from substrateinterface import SubstrateInterface, Keypair
from etrid_sdk.wrappers import GPURegistryWrapper, GPUNFTWrapper
from etrid_sdk.wrappers.gpu_registry import GpuSpecs, HardwareAttestation
from etrid_sdk.wrappers.gpu_nft import RentalTerms


async def main():
    """Main execution flow"""

    # Connect to AI Compute PBC
    print("Connecting to AI Compute PBC...")
    api = SubstrateInterface(
        url="wss://ai-compute-pbc.etrid.io",
        ss58_format=42,  # Substrate default
        type_registry_preset="substrate-node-template"
    )

    # Initialize wrappers
    gpu_registry = GPURegistryWrapper(api)
    gpu_nft = GPUNFTWrapper(api)

    # Create test accounts
    print("\nGenerating test accounts...")
    provider_keypair = Keypair.create_from_mnemonic(Keypair.generate_mnemonic())
    renter_keypair = Keypair.create_from_mnemonic(Keypair.generate_mnemonic())

    print(f"Provider address: {provider_keypair.ss58_address}")
    print(f"Renter address: {renter_keypair.ss58_address}")


    # ========================================================================
    # PART 1: Register GPU
    # ========================================================================
    print("\n" + "="*70)
    print("PART 1: REGISTERING GPU")
    print("="*70)

    # Define GPU specifications
    specs = GpuSpecs(
        model="NVIDIA RTX 4090",
        vram_gb=24,
        compute_units=16384,  # CUDA cores
        clock_speed_mhz=2520,
        tdp_watts=450
    )

    print(f"\nGPU Specs:")
    print(f"  Model: {specs.model}")
    print(f"  VRAM: {specs.vram_gb} GB")
    print(f"  Compute Units: {specs.compute_units}")
    print(f"  Clock Speed: {specs.clock_speed_mhz} MHz")
    print(f"  TDP: {specs.tdp_watts} W")

    # Create hardware attestation
    # In production, this would come from TPM and actual benchmarks
    attestation = HardwareAttestation(
        tpm_quote=b"mock_tpm_quote_data_here_would_be_256_bytes" + b"\x00" * 215,
        benchmark_score=98500,  # Out of 100,000
        timestamp=1700000000
    )

    print(f"\nHardware Attestation:")
    print(f"  Benchmark Score: {attestation.benchmark_score / 1000}%")
    print(f"  Timestamp: {attestation.timestamp}")

    # Register GPU with stake
    stake_amount = 100_000_000_000_000_000_000  # 100 ËDSC
    print(f"\nRegistering GPU with stake of {stake_amount / 1e18} ËDSC...")

    try:
        gpu_id = await gpu_registry.register_gpu(
            provider_keypair,
            specs,
            attestation,
            stake=stake_amount,
            schedule="AlwaysOn"
        )
        print(f"✓ GPU registered successfully! GPU ID: {gpu_id}")
    except Exception as e:
        print(f"✗ Registration failed: {e}")
        return


    # ========================================================================
    # PART 2: Query GPU Details
    # ========================================================================
    print("\n" + "="*70)
    print("PART 2: QUERYING GPU DETAILS")
    print("="*70)

    print(f"\nFetching GPU #{gpu_id} details...")
    gpu_node = await gpu_registry.get_gpu_specs(gpu_id)

    print(f"\nGPU Node Information:")
    print(f"  Provider: {gpu_node.provider}")
    print(f"  Model: {gpu_node.specs.model}")
    print(f"  VRAM: {gpu_node.specs.vram_gb} GB")
    print(f"  Status: {gpu_node.status}")
    print(f"  Stake: {gpu_node.stake / 1e18} ËDSC")
    print(f"  Registered: {gpu_node.registered_at}")

    # Check reputation
    reputation = await gpu_registry.get_reputation(gpu_id)
    print(f"\nReputation:")
    print(f"  Jobs Completed: {reputation.jobs_completed}")
    print(f"  Jobs Failed: {reputation.jobs_failed}")
    print(f"  Success Rate: {reputation.success_rate:.2f}%")
    print(f"  Uptime: {reputation.uptime_percent:.2f}%")
    print(f"  Rating: {reputation.rating_stars:.1f}/5.0 ({reputation.rating_count} reviews)")


    # ========================================================================
    # PART 3: Search for GPUs
    # ========================================================================
    print("\n" + "="*70)
    print("PART 3: SEARCHING GPU MARKETPLACE")
    print("="*70)

    print("\nSearching for high-end GPUs (24GB+ VRAM, 15000+ cores)...")
    search_results = await gpu_registry.search_gpus(
        min_vram_gb=24,
        min_compute_units=15000,
        status="Active",
        limit=10
    )

    print(f"\nFound {len(search_results)} matching GPUs:")
    for gpu in search_results:
        print(f"  GPU #{gpu.specs.model}: {gpu.specs.vram_gb}GB, "
              f"{gpu.specs.compute_units} cores, "
              f"{gpu.reputation.rating_stars:.1f}/5.0")


    # ========================================================================
    # PART 4: Mint GPU as NFT
    # ========================================================================
    print("\n" + "="*70)
    print("PART 4: MINTING GPU NFT")
    print("="*70)

    print(f"\nMinting GPU #{gpu_id} as NFT...")
    try:
        nft_id = await gpu_nft.mint_gpu_nft(provider_keypair, gpu_id)
        print(f"✓ NFT minted successfully! NFT ID: {nft_id}")
    except Exception as e:
        print(f"✗ Minting failed: {e}")
        return

    # Query NFT metadata
    nft_metadata = await gpu_nft.get_nft_metadata(nft_id)
    print(f"\nNFT Metadata:")
    print(f"  Owner: {nft_metadata.owner}")
    print(f"  GPU ID: {nft_metadata.gpu_id}")
    print(f"  Reputation Snapshot: {nft_metadata.reputation_score}/100")
    print(f"  Total Earnings: {nft_metadata.total_earnings / 1e18} ËDSC")
    print(f"  Minted At: {nft_metadata.minted_at}")


    # ========================================================================
    # PART 5: Configure Rental Terms
    # ========================================================================
    print("\n" + "="*70)
    print("PART 5: CONFIGURING RENTAL TERMS")
    print("="*70)

    rental_terms = RentalTerms(
        hourly_rate=10_000_000_000_000_000_000,  # 10 ËDSC/hour
        minimum_hours=1,
        maximum_hours=720,  # 30 days max
        deposit_required=100_000_000_000_000_000_000,  # 100 ËDSC deposit
        auto_renew=False
    )

    print(f"\nSetting rental terms:")
    print(f"  Hourly Rate: {rental_terms.hourly_rate / 1e18} ËDSC")
    print(f"  Min Duration: {rental_terms.minimum_hours} hours")
    print(f"  Max Duration: {rental_terms.maximum_hours} hours")
    print(f"  Deposit: {rental_terms.deposit_required / 1e18} ËDSC")

    try:
        await gpu_nft.set_rental_terms(provider_keypair, nft_id, rental_terms)
        print("✓ Rental terms configured successfully!")
    except Exception as e:
        print(f"✗ Failed to set rental terms: {e}")


    # ========================================================================
    # PART 6: List NFT for Sale
    # ========================================================================
    print("\n" + "="*70)
    print("PART 6: LISTING NFT FOR SALE")
    print("="*70)

    sale_price = 1000_000_000_000_000_000_000  # 1000 ËDSC
    print(f"\nListing NFT #{nft_id} for {sale_price / 1e18} ËDSC...")

    try:
        await gpu_nft.list_for_sale(provider_keypair, nft_id, sale_price)
        print("✓ NFT listed for sale!")
    except Exception as e:
        print(f"✗ Failed to list NFT: {e}")

    # Query listed NFTs
    print("\nFetching all listed NFTs...")
    listed_nfts = await gpu_nft.get_listed_nfts(limit=10)

    print(f"\nCurrently {len(listed_nfts)} NFTs for sale:")
    for nft in listed_nfts:
        print(f"  NFT #{nft.nft_id}: GPU #{nft.gpu_id}, "
              f"Price: {nft.list_price / 1e18} ËDSC, "
              f"Reputation: {nft.reputation_score}/100")


    # ========================================================================
    # PART 7: Rent GPU (Alternative to Buying)
    # ========================================================================
    print("\n" + "="*70)
    print("PART 7: RENTING GPU FOR COMPUTE")
    print("="*70)

    rental_duration = 24  # 24 hours
    total_cost = (rental_terms.hourly_rate * rental_duration) + rental_terms.deposit_required

    print(f"\nRenting GPU for {rental_duration} hours...")
    print(f"  Hourly Rate: {rental_terms.hourly_rate / 1e18} ËDSC")
    print(f"  Duration: {rental_duration} hours")
    print(f"  Compute Cost: {(rental_terms.hourly_rate * rental_duration) / 1e18} ËDSC")
    print(f"  Deposit: {rental_terms.deposit_required / 1e18} ËDSC")
    print(f"  Total: {total_cost / 1e18} ËDSC")

    try:
        rental_id = await gpu_nft.rent_gpu(
            renter_keypair,
            nft_id,
            duration_hours=rental_duration
        )
        print(f"✓ GPU rented successfully! Rental ID: {rental_id}")
        print(f"\nYou can now submit compute jobs to GPU #{gpu_id}")
        print(f"Rental expires in {rental_duration} hours")
    except Exception as e:
        print(f"✗ Rental failed: {e}")


    # ========================================================================
    # PART 8: Provider Earnings & Reputation
    # ========================================================================
    print("\n" + "="*70)
    print("PART 8: PROVIDER EARNINGS & REPUTATION")
    print("="*70)

    # Report uptime (heartbeat)
    print("\nReporting GPU uptime...")
    try:
        await gpu_registry.report_uptime(provider_keypair, gpu_id)
        print("✓ Heartbeat reported")
    except Exception as e:
        print(f"✗ Failed to report uptime: {e}")

    # Query earnings
    print(f"\nQuerying provider earnings...")
    earnings = await gpu_registry.get_provider_earnings(provider_keypair.ss58_address)

    print(f"\nProvider Earnings:")
    print(f"  Total Earned: {earnings.total_earned / 1e18} ËDSC")
    print(f"  Pending Payout: {earnings.pending_payout / 1e18} ËDSC")
    print(f"  Last Payout: {earnings.last_payout}")


    # ========================================================================
    # Summary
    # ========================================================================
    print("\n" + "="*70)
    print("EXAMPLE COMPLETE")
    print("="*70)
    print("\nGPU Marketplace Features Demonstrated:")
    print("  ✓ GPU registration with hardware attestation")
    print("  ✓ Staking mechanism (100 ËDSC stake)")
    print("  ✓ GPU specifications and reputation tracking")
    print("  ✓ Marketplace search (filter by specs/status)")
    print("  ✓ NFT minting (GPU ownership certificates)")
    print("  ✓ Rental configuration (hourly rates, terms)")
    print("  ✓ NFT marketplace (list for sale)")
    print("  ✓ GPU rental (compute job access)")
    print("  ✓ Provider earnings and uptime tracking")

    print("\nNext Steps:")
    print("  - Integrate with compute job scheduler")
    print("  - Set up automated heartbeat reporting")
    print("  - Configure dynamic pricing based on demand")
    print("  - Monitor reputation and optimize performance")

    # Cleanup
    await api.close()


if __name__ == "__main__":
    asyncio.run(main())
