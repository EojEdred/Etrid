"""
Blockchain Monitor for AI Devs Social Automation

Connects to FlareChain (Ëtrid Layer 0) to fetch:
- Block production metrics
- Validator/staking data
- Governance proposals
- Oracle reserve ratios
- Bridge events
- Network statistics

Uses Polkadot.js-compatible WebSocket connection via substrateinterface
"""

import os
import asyncio
from typing import Dict, List, Optional, Any
from datetime import datetime, timedelta
from substrateinterface import SubstrateInterface
from substrateinterface.exceptions import SubstrateRequestException


class BlockchainMonitor:
    def __init__(self, ws_url: Optional[str] = None):
        """Initialize blockchain connection"""
        self.ws_url = ws_url or os.getenv('BLOCKCHAIN_WS_URL', 'ws://127.0.0.1:9944')
        self.substrate = None
        self._connect()

    def _connect(self):
        """Establish WebSocket connection to blockchain node"""
        try:
            self.substrate = SubstrateInterface(
                url=self.ws_url,
                ss58_format=42,  # Generic Substrate format
                type_registry_preset='substrate-node-template'
            )
            print(f"✅ Connected to blockchain: {self.ws_url}")
        except Exception as e:
            print(f"⚠️  Failed to connect to blockchain: {e}")
            print(f"   Using mock data mode")
            self.substrate = None

    async def get_block_number(self) -> int:
        """Get current block number"""
        if not self.substrate:
            # Mock data for development
            return 1_500_000 + int(datetime.utcnow().timestamp()) % 10000

        try:
            block = self.substrate.get_block()
            return block['header']['number']
        except SubstrateRequestException as e:
            print(f"Error fetching block number: {e}")
            return 0

    async def get_avg_block_time(self, start_block: int, end_block: int) -> float:
        """Calculate average block time over a range"""
        if not self.substrate:
            # Mock: ~6 second blocks with slight variance
            return 6.0 + (hash(str(start_block)) % 100) / 100

        try:
            start_time = await self._get_block_timestamp(start_block)
            end_time = await self._get_block_timestamp(end_block)

            if start_time and end_time:
                blocks = end_block - start_block
                seconds = (end_time - start_time).total_seconds()
                return seconds / blocks if blocks > 0 else 6.0

            return 6.0
        except Exception as e:
            print(f"Error calculating avg block time: {e}")
            return 6.0

    async def _get_block_timestamp(self, block_number: int) -> Optional[datetime]:
        """Get timestamp for a specific block"""
        if not self.substrate:
            return datetime.utcnow() - timedelta(seconds=(await self.get_block_number() - block_number) * 6)

        try:
            block_hash = self.substrate.get_block_hash(block_number)
            block = self.substrate.get_block(block_hash=block_hash)

            # Timestamp is in milliseconds in Substrate
            timestamp_ms = None
            for extrinsic in block['extrinsics']:
                if extrinsic['call']['call_module'] == 'Timestamp' and \
                   extrinsic['call']['call_function'] == 'set':
                    timestamp_ms = extrinsic['call']['call_args'][0]['value']
                    break

            if timestamp_ms:
                return datetime.fromtimestamp(timestamp_ms / 1000)

            return None
        except Exception as e:
            print(f"Error getting block timestamp: {e}")
            return None

    async def get_active_validator_count(self) -> int:
        """Get number of active validators"""
        if not self.substrate:
            # Mock: 21-50 validators
            return 21 + int(datetime.utcnow().timestamp()) % 30

        try:
            # Query session validators
            result = self.substrate.query(
                module='Session',
                storage_function='Validators'
            )
            return len(result.value) if result.value else 0
        except Exception as e:
            print(f"Error fetching validator count: {e}")
            return 0

    async def get_total_staked(self) -> int:
        """Get total staked amount (in base units)"""
        if not self.substrate:
            # Mock: 150-200M ETR staked
            base = 150_000_000_000_000_000_000_000_000  # 150M ETR in base units (18 decimals)
            variance = int(datetime.utcnow().timestamp()) % 50_000_000_000_000_000_000_000_000
            return base + variance

        try:
            # Query staking totals
            result = self.substrate.query(
                module='Staking',
                storage_function='TotalStake'
            )
            return int(result.value) if result.value else 0
        except Exception as e:
            print(f"Error fetching total staked: {e}")
            return 0

    async def get_reserve_ratio(self, asset: str = "EDSC") -> float:
        """
        Get reserve ratio for stablecoin (EDSC)

        Reserve Ratio = Total Reserves / Total Supply
        Healthy: > 1.5 (150% overcollateralized)
        Warning: 1.0 - 1.5
        Critical: < 1.0
        """
        if not self.substrate:
            # Mock: healthy reserve ratio around 1.8
            return 1.7 + (hash(asset + str(datetime.utcnow().hour)) % 30) / 100

        try:
            # Query reserve oracle pallet
            reserves = self.substrate.query(
                module='ReserveOracle',
                storage_function='TotalReserves',
                params=[asset]
            )

            supply = self.substrate.query(
                module='Assets',
                storage_function='TotalSupply',
                params=[asset]
            )

            if reserves.value and supply.value and supply.value > 0:
                return float(reserves.value) / float(supply.value)

            return 1.8  # Default healthy ratio
        except Exception as e:
            print(f"Error fetching reserve ratio: {e}")
            return 1.8

    async def get_uptime_percentage(self, blocks: int) -> float:
        """
        Calculate network uptime percentage over last N blocks

        Checks for missed blocks (gaps in block numbers)
        """
        if not self.substrate:
            # Mock: very high uptime (99.8-99.99%)
            return 99.8 + (hash(str(blocks)) % 20) / 100

        try:
            current_block = await self.get_block_number()
            start_block = current_block - blocks

            # Sample every 100th block to avoid too many queries
            sample_size = min(blocks // 100, 100)
            missed_blocks = 0

            for i in range(sample_size):
                block_num = start_block + (blocks * i // sample_size)
                block_hash = self.substrate.get_block_hash(block_num)

                if not block_hash:
                    missed_blocks += 1

            uptime = ((sample_size - missed_blocks) / sample_size) * 100
            return round(uptime, 2)
        except Exception as e:
            print(f"Error calculating uptime: {e}")
            return 99.9

    async def get_governance_proposals(self, status: str = "active") -> List[Dict[str, Any]]:
        """
        Get governance proposals

        Args:
            status: "active", "pending", "passed", "failed", "all"

        Returns:
            List of proposal objects
        """
        if not self.substrate:
            # Mock: return sample proposals
            return [
                {
                    "id": 42,
                    "title": "Reduce minimum staking amount to 1000 ETR",
                    "proposer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    "status": "active",
                    "created_at": datetime.utcnow() - timedelta(days=2),
                    "voting_ends": datetime.utcnow() + timedelta(days=5),
                    "votes_for": 15_000_000,
                    "votes_against": 2_000_000,
                    "description": "Lower barrier to entry for validators"
                }
            ]

        try:
            # Query democracy pallet
            proposals = []

            # Get all active referendums
            referendum_count = self.substrate.query(
                module='Democracy',
                storage_function='ReferendumCount'
            )

            for i in range(referendum_count.value):
                referendum = self.substrate.query(
                    module='Democracy',
                    storage_function='ReferendumInfoOf',
                    params=[i]
                )

                if referendum.value:
                    proposals.append({
                        "id": i,
                        "status": "active" if referendum.value.get('Ongoing') else "completed",
                        "proposal_hash": referendum.value.get('proposal_hash'),
                        "created_at": datetime.utcnow(),  # Would need to look up in events
                    })

            return proposals
        except Exception as e:
            print(f"Error fetching governance proposals: {e}")
            return []

    async def get_audit_events(self, since_block: Optional[int] = None) -> List[Dict[str, Any]]:
        """
        Get audit events (proposal flags, security alerts, etc.)

        Monitors events from:
        - Governance (suspicious proposals)
        - Bridge (transaction anomalies)
        - Staking (slashing events)
        """
        if not self.substrate:
            # Mock: occasional audit events
            if datetime.utcnow().hour % 6 == 0:  # Every 6 hours
                return [
                    {
                        "event_type": "proposal_flagged",
                        "severity": "medium",
                        "proposal_id": 42,
                        "reason": "Unusually high parameter change (10x increase)",
                        "detected_at": datetime.utcnow(),
                        "block_number": await self.get_block_number()
                    }
                ]
            return []

        try:
            current_block = await self.get_block_number()
            start_block = since_block or (current_block - 1000)

            audit_events = []

            # Scan recent blocks for audit-worthy events
            for block_num in range(start_block, current_block, 100):
                block_hash = self.substrate.get_block_hash(block_num)
                events = self.substrate.get_events(block_hash)

                for event in events:
                    # Slashing events
                    if event.value['module_id'] == 'Staking' and \
                       event.value['event_id'] == 'Slash':
                        audit_events.append({
                            "event_type": "validator_slashed",
                            "severity": "high",
                            "block_number": block_num,
                            "validator": event.value['attributes'][0],
                            "amount": event.value['attributes'][1],
                        })

                    # Large treasury spends
                    if event.value['module_id'] == 'Treasury' and \
                       event.value['event_id'] == 'Spending':
                        amount = event.value['attributes'][0]
                        if amount > 100_000_000_000_000_000_000_000:  # > 100k ETR
                            audit_events.append({
                                "event_type": "large_treasury_spend",
                                "severity": "medium",
                                "block_number": block_num,
                                "amount": amount,
                            })

            return audit_events
        except Exception as e:
            print(f"Error fetching audit events: {e}")
            return []

    async def get_exchange_listings(self) -> List[Dict[str, Any]]:
        """
        Get recent exchange listings

        Note: This would typically come from an external API or web scraper
        For now, returns empty list (to be implemented with CoinGecko/CMC API)
        """
        # TODO: Implement external API integration
        # - CoinGecko API for listing updates
        # - CMC API for new exchanges
        # - Custom scraper for DEX listings

        return []

    async def get_network_stats(self) -> Dict[str, Any]:
        """Get comprehensive network statistics"""
        current_block = await self.get_block_number()
        blocks_per_day = 24 * 60 * 60 // 6  # Assuming 6s blocks

        return {
            "block_number": current_block,
            "avg_block_time": await self.get_avg_block_time(
                current_block - blocks_per_day,
                current_block
            ),
            "active_validators": await self.get_active_validator_count(),
            "total_staked": await self.get_total_staked(),
            "reserve_ratio_edsc": await self.get_reserve_ratio("EDSC"),
            "network_uptime_24h": await self.get_uptime_percentage(blocks_per_day),
            "timestamp": datetime.utcnow().isoformat(),
        }

    def close(self):
        """Close blockchain connection"""
        if self.substrate:
            self.substrate.close()
            print("✅ Blockchain connection closed")


# Singleton instance for reuse across workflows
_monitor_instance = None

def get_monitor() -> BlockchainMonitor:
    """Get or create blockchain monitor singleton"""
    global _monitor_instance
    if _monitor_instance is None:
        _monitor_instance = BlockchainMonitor()
    return _monitor_instance
