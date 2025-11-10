// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

/**
 * @title IEtridNativeETH
 * @notice Interface for instant ETH <-> wETH conversion on ETH-PBC
 * @dev Precompile address: 0x0000000000000000000000000000000000000803
 *
 * This precompile provides zero-fee, atomic conversion between native ETH
 * and wrapped ETH (wETH) tokens. Unlike traditional wrapped ETH contracts,
 * this uses a precompile for gas-efficient conversions.
 *
 * Key Benefits:
 * - Zero gas fees for wrapping/unwrapping
 * - Instant atomic execution
 * - Integrated with FlareChain bridge
 * - No smart contract risk (native runtime logic)
 *
 * Example Usage:
 * ```solidity
 * IEtridNativeETH wrapper = IEtridNativeETH(0x0000000000000000000000000000000000000803);
 *
 * // Wrap 1 ETH to wETH
 * uint256 wethAmount = wrapper.wrap{value: 1 ether}();
 *
 * // Unwrap 0.5 wETH back to ETH
 * wrapper.unwrap(0.5 ether);
 * ```
 */
interface IEtridNativeETH {
    /**
     * @notice Emitted when ETH is wrapped to wETH
     * @param user The address that wrapped ETH
     * @param amount The amount of ETH wrapped (and wETH minted)
     */
    event Wrap(address indexed user, uint256 amount);

    /**
     * @notice Emitted when wETH is unwrapped to ETH
     * @param user The address that unwrapped wETH
     * @param amount The amount of wETH unwrapped (and ETH released)
     */
    event Unwrap(address indexed user, uint256 amount);

    /**
     * @notice Wrap native ETH to wETH (ERC-20) instantly
     * @return wethAmount The amount of wETH minted (equal to msg.value at 1:1 rate)
     *
     * This function converts native ETH sent with the transaction into
     * wETH ERC-20 tokens. The conversion is instant and fee-free.
     *
     * Requirements:
     * - msg.value must be > 0
     *
     * Effects:
     * - Mints wETH tokens to msg.sender
     * - Locks native ETH in bridge reserve
     * - Emits Wrap event
     *
     * Example:
     * ```solidity
     * // Wrap 10 ETH
     * uint256 weth = wrapper.wrap{value: 10 ether}();
     * // Returns: 10000000000000000000 (10 * 1e18)
     * ```
     */
    function wrap() external payable returns (uint256 wethAmount);

    /**
     * @notice Unwrap wETH back to native ETH
     * @param amount The amount of wETH to unwrap
     * @return success True if unwrap was successful
     *
     * This function burns wETH ERC-20 tokens and releases native ETH
     * to the caller. The conversion is instant and fee-free.
     *
     * Requirements:
     * - Caller must have sufficient wETH balance
     * - amount must be > 0
     * - Bridge must have sufficient ETH liquidity
     *
     * Effects:
     * - Burns wETH tokens from msg.sender
     * - Transfers native ETH to msg.sender
     * - Emits Unwrap event
     *
     * Example:
     * ```solidity
     * // Unwrap 5 wETH to ETH
     * bool success = wrapper.unwrap(5 ether);
     * require(success, "Unwrap failed");
     * ```
     */
    function unwrap(uint256 amount) external returns (bool success);

    /**
     * @notice Get the current wrap/unwrap exchange rate
     * @return rate The exchange rate scaled by 1e18 (1e18 = 1:1)
     *
     * Returns the current rate for wrapping/unwrapping. Under normal
     * conditions, this is 1:1 (1e18). It may differ if:
     * - Bridge liquidity is constrained
     * - Market conditions create premium/discount
     * - FlareChain governance adjusts rates
     *
     * Example:
     * ```solidity
     * uint256 rate = wrapper.getWrapRate();
     * // Returns: 1000000000000000000 (1e18 = 1:1 rate)
     *
     * // Calculate expected wETH from 10 ETH
     * uint256 expectedWETH = (10 ether * rate) / 1e18;
     * ```
     */
    function getWrapRate() external view returns (uint256 rate);
}

/**
 * @title Advanced Usage Examples
 * @dev These examples show common patterns for using the Native ETH wrapper
 */
contract NativeETHWrapperExamples {
    IEtridNativeETH constant WRAPPER = IEtridNativeETH(0x0000000000000000000000000000000000000803);

    /**
     * @notice Example: Wrap all ETH balance to wETH
     */
    function wrapAll() external payable {
        require(msg.value > 0, "No ETH sent");
        uint256 wethReceived = WRAPPER.wrap{value: msg.value}();
        // wethReceived now available as ERC-20 tokens
    }

    /**
     * @notice Example: Unwrap specific amount with safety check
     */
    function safeUnwrap(uint256 amount) external {
        uint256 rate = WRAPPER.getWrapRate();
        require(rate > 0, "Invalid rate");

        // Calculate expected ETH output
        uint256 expectedETH = (amount * rate) / 1e18;
        require(expectedETH > 0, "Amount too small");

        // Perform unwrap
        bool success = WRAPPER.unwrap(amount);
        require(success, "Unwrap failed");
    }

    /**
     * @notice Example: Batch wrap/unwrap for gas efficiency
     */
    function batchWrap(uint256[] calldata amounts) external payable {
        require(msg.value == sumArray(amounts), "Incorrect ETH sent");

        for (uint256 i = 0; i < amounts.length; i++) {
            WRAPPER.wrap{value: amounts[i]}();
        }
    }

    /**
     * @notice Example: Emergency unwrap with fallback
     */
    function emergencyUnwrap(uint256 amount) external returns (bool) {
        try WRAPPER.unwrap(amount) returns (bool success) {
            return success;
        } catch {
            // Fallback: keep as wETH if unwrap fails
            return false;
        }
    }

    // Helper function
    function sumArray(uint256[] memory arr) internal pure returns (uint256 sum) {
        for (uint256 i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
    }

    // Receive ETH from unwraps
    receive() external payable {}
}
