"""Oracle Wrapper - Price Feeds

Decentralized oracle network providing real-time price data.
"""

from typing import Dict, Any, List, Optional, Callable
from substrateinterface import SubstrateInterface, Keypair
from ..errors import (
    NotConnectedError,
    OracleError,
    PriceNotFoundError,
    StalePriceError,
)


class OracleWrapper:
    """
    Wrapper for Oracle pallet - Decentralized price feeds.

    Provides real-time price data for trading pairs with TWAP support
    and automated staleness detection.
    """

    # Maximum age for price data (in seconds)
    MAX_PRICE_AGE = 300  # 5 minutes

    def __init__(self, api: SubstrateInterface):
        """
        Initialize Oracle wrapper.

        Args:
            api: Connected Substrate API instance
        """
        self.api = api
        self._price_subscribers: Dict[str, List[Callable]] = {}

    def _ensure_connected(self):
        """Ensure API is connected."""
        if not self.api.websocket or not self.api.websocket.connected:
            raise NotConnectedError()

    async def get_price(self, pair: str) -> int:
        """
        Get current price for trading pair.

        Args:
            pair: Trading pair (e.g., "ETR/USD", "BTC/USD")

        Returns:
            Price in planck (1 USD = 10^18 planck)

        Raises:
            PriceNotFoundError: If price feed not found
            StalePriceError: If price data is stale

        Example:
            >>> price = await wrapper.get_price("ETR/USD")
            >>> print(f'ETR price: ${price / 10**18}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Oracle",
                storage_function="Prices",
                params=[pair]
            )

            if result.value is None:
                raise PriceNotFoundError(f"Price feed not found: {pair}")

            price_data = result.value
            price = int(price_data.get('price', 0))
            timestamp = int(price_data.get('timestamp', 0))

            # Check if price is stale
            current_time = self.api.query(
                module="Timestamp",
                storage_function="Now"
            )
            current_timestamp = int(current_time.value) if current_time else 0

            if current_timestamp - timestamp > self.MAX_PRICE_AGE * 1000:  # Convert to ms
                raise StalePriceError(f"Price data is stale for {pair}")

            return price

        except (PriceNotFoundError, StalePriceError):
            raise
        except Exception as e:
            raise OracleError(f"Failed to get price: {str(e)}")

    async def get_price_with_metadata(self, pair: str) -> Dict[str, Any]:
        """
        Get price with metadata.

        Args:
            pair: Trading pair

        Returns:
            Dictionary with price, timestamp, source, and confidence

        Raises:
            PriceNotFoundError: If price feed not found

        Example:
            >>> data = await wrapper.get_price_with_metadata("ETR/USD")
            >>> print(f'Price: ${data["price"] / 10**18}')
            >>> print(f'Confidence: {data["confidence"]}%')
            >>> print(f'Source: {data["source"]}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Oracle",
                storage_function="Prices",
                params=[pair]
            )

            if result.value is None:
                raise PriceNotFoundError(f"Price feed not found: {pair}")

            price_data = result.value

            return {
                "pair": pair,
                "price": int(price_data.get('price', 0)),
                "timestamp": int(price_data.get('timestamp', 0)),
                "source": price_data.get('source', ''),
                "confidence": int(price_data.get('confidence', 100)),
                "volume_24h": int(price_data.get('volume_24h', 0)),
                "change_24h": float(price_data.get('change_24h', 0.0)),
            }

        except PriceNotFoundError:
            raise
        except Exception as e:
            raise OracleError(f"Failed to get price metadata: {str(e)}")

    async def get_twap(
        self,
        pair: str,
        window_seconds: int = 3600,
    ) -> int:
        """
        Get Time-Weighted Average Price (TWAP).

        Args:
            pair: Trading pair
            window_seconds: Time window in seconds (default: 1 hour)

        Returns:
            TWAP in planck

        Raises:
            PriceNotFoundError: If price feed not found

        Example:
            >>> twap_1h = await wrapper.get_twap("ETR/USD", 3600)
            >>> twap_24h = await wrapper.get_twap("ETR/USD", 86400)
            >>> print(f'1h TWAP: ${twap_1h / 10**18}')
            >>> print(f'24h TWAP: ${twap_24h / 10**18}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Oracle",
                storage_function="calculateTWAP",
                params=[pair, window_seconds]
            )

            if result.value is None:
                raise PriceNotFoundError(f"TWAP not available for {pair}")

            return int(result.value)

        except PriceNotFoundError:
            raise
        except Exception as e:
            raise OracleError(f"Failed to get TWAP: {str(e)}")

    async def submit_price(
        self,
        keypair: Keypair,
        pair: str,
        price: int,
        source: str = "external",
    ) -> str:
        """
        Submit price data (oracle providers only).

        Args:
            keypair: Oracle provider keypair
            pair: Trading pair
            price: Price in planck
            source: Data source identifier

        Returns:
            Transaction hash

        Raises:
            OracleError: If submission fails

        Example:
            >>> tx_hash = await wrapper.submit_price(
            ...     oracle_keypair,
            ...     "ETR/USD",
            ...     5000000000000000000,  # $5.00
            ...     "chainlink"
            ... )
        """
        self._ensure_connected()

        try:
            call = self.api.compose_call(
                call_module="Oracle",
                call_function="submitPrice",
                call_params={
                    "pair": pair,
                    "price": price,
                    "source": source,
                }
            )

            extrinsic = self.api.create_signed_extrinsic(call=call, keypair=keypair)
            receipt = self.api.submit_extrinsic(extrinsic, wait_for_inclusion=True)

            if not receipt.is_success:
                raise OracleError(f"Failed to submit price: {receipt.error_message}")

            return receipt.extrinsic_hash

        except Exception as e:
            raise OracleError(f"Failed to submit price: {str(e)}")

    async def subscribe_to_price_updates(
        self,
        pair: str,
        callback: Callable[[Dict[str, Any]], None],
    ) -> str:
        """
        Subscribe to price updates for a trading pair.

        Args:
            pair: Trading pair
            callback: Callback function to receive price updates

        Returns:
            Subscription ID

        Example:
            >>> def on_price_update(data):
            ...     print(f'New price for {data["pair"]}: ${data["price"] / 10**18}')
            >>>
            >>> sub_id = await wrapper.subscribe_to_price_updates(
            ...     "ETR/USD",
            ...     on_price_update
            ... )
        """
        self._ensure_connected()

        try:
            # Add callback to subscribers
            if pair not in self._price_subscribers:
                self._price_subscribers[pair] = []

            self._price_subscribers[pair].append(callback)

            # Generate subscription ID
            sub_id = f"{pair}_{len(self._price_subscribers[pair])}"

            return sub_id

        except Exception as e:
            raise OracleError(f"Failed to subscribe to price updates: {str(e)}")

    async def unsubscribe_from_price_updates(self, subscription_id: str):
        """
        Unsubscribe from price updates.

        Args:
            subscription_id: Subscription ID from subscribe_to_price_updates

        Example:
            >>> await wrapper.unsubscribe_from_price_updates(sub_id)
        """
        # Implementation would remove callback from subscribers
        pass

    async def get_available_pairs(self) -> List[str]:
        """
        Get list of available trading pairs.

        Returns:
            List of trading pair names

        Example:
            >>> pairs = await wrapper.get_available_pairs()
            >>> print('Available pairs:', pairs)
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Oracle",
                storage_function="SupportedPairs"
            )

            if result.value is None:
                return []

            return list(result.value) if isinstance(result.value, list) else []

        except Exception as e:
            raise OracleError(f"Failed to get available pairs: {str(e)}")

    async def get_price_history(
        self,
        pair: str,
        limit: int = 100,
    ) -> List[Dict[str, Any]]:
        """
        Get price history for trading pair.

        Args:
            pair: Trading pair
            limit: Maximum number of price points

        Returns:
            List of price data points

        Example:
            >>> history = await wrapper.get_price_history("ETR/USD", limit=24)
            >>> for point in history:
            ...     print(f'{point["timestamp"]}: ${point["price"] / 10**18}')
        """
        self._ensure_connected()

        try:
            result = self.api.query(
                module="Oracle",
                storage_function="PriceHistory",
                params=[pair]
            )

            if result.value is None:
                return []

            history = result.value if isinstance(result.value, list) else []
            return history[:limit]

        except Exception as e:
            raise OracleError(f"Failed to get price history: {str(e)}")
