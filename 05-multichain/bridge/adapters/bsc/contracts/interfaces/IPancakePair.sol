// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title IPancakePair
 * @dev Interface for PancakeSwap V2 pair contract
 */
interface IPancakePair {
    function token0() external view returns (address);
    function token1() external view returns (address);
    function getReserves() external view returns (
        uint112 reserve0,
        uint112 reserve1,
        uint32 blockTimestampLast
    );
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
}
