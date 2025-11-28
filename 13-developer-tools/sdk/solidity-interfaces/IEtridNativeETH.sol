// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title IEtridNativeETH
 * @dev Interface for Native ETH Wrapping Precompile (0x803)
 *
 * Provides instant, ZERO-FEE conversion between native ETH and wETH.
 * This is a novel feature - no gas costs for wrapping/unwrapping.
 *
 * Address: 0x0000000000000000000000000000000000000803
 *
 * @notice Novel Feature: Traditional WETH contracts charge gas.
 *         Etrid's precompile provides FREE wrapping via native execution.
 */
interface IEtridNativeETH {
    /**
     * @notice Wrap native ETH to wETH (ERC-20)
     * @return wethAmount Amount of wETH minted (equals msg.value)
     *
     * @dev Example:
     *   // Wrap 1 ETH to wETH (zero gas cost for wrapping)
     *   uint256 wethReceived = wrapper.wrap{value: 1 ether}();
     *   assert(wethReceived == 1 ether);
     *
     * @dev This is ZERO-FEE - no gas charged for the wrap operation itself
     *      Only standard transaction gas applies
     */
    function wrap() external payable returns (uint256 wethAmount);

    /**
     * @notice Unwrap wETH to native ETH
     * @param amount Amount of wETH to unwrap
     * @return success true if unwrap succeeded
     *
     * @dev Example:
     *   // Unwrap 0.5 wETH back to native ETH
     *   bool success = wrapper.unwrap(0.5 ether);
     *   require(success, "Unwrap failed");
     *
     * @dev This is ZERO-FEE - no gas charged for the unwrap operation
     *      Requires prior approval if wETH is ERC-20
     */
    function unwrap(uint256 amount) external returns (bool success);

    /**
     * @notice Get current wrap/unwrap exchange rate
     * @return rate Exchange rate scaled by 1e18 (normally 1e18 = 1:1 ratio)
     *
     * @dev In normal operation, rate is always 1e18 (1:1 ratio)
     *      Rate could vary if:
     *      - Bridge liquidity is unbalanced
     *      - Dynamic rate adjustment is enabled
     *      - Special market conditions
     *
     * @dev Example:
     *   uint256 rate = wrapper.getWrapRate();
     *   require(rate >= 0.99e18, "Unfavorable rate");
     *   wrapper.wrap{value: 10 ether}();
     */
    function getWrapRate() external view returns (uint256 rate);
}
