// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title EtridETH
 * @dev ÉTR Token on Ethereum (ERC-20)
 *
 * Official ÉTR token deployment for Ethereum mainnet.
 * Designed for Uniswap V3 integration and cross-chain bridge support.
 *
 * Token Specifications:
 * - Name: Ëtrid Coin
 * - Symbol: ÉTR
 * - Decimals: 18 (Ethereum standard)
 * - Initial Supply: 25M ÉTR (for Uniswap V3 liquidity)
 * - Max Supply: 1B ÉTR (enforced by bridge protocol)
 *
 * Reference Documents:
 * - FOUNDATION_CHARTER.md (governance)
 * - protocol-charter.md (token economics)
 * - COMPLETE_DEX_DEPLOYMENT_GUIDE.md (deployment strategy)
 *
 * @notice This contract is controlled by the Foundation multisig
 * @custom:security-contact security@etrid.org
 */
contract EtridETH is ERC20, ERC20Burnable, Ownable {

    /// @dev Maximum supply cap (1 billion ÉTR)
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18;

    /// @dev Bridge contract address (can mint/burn for cross-chain transfers)
    address public bridgeContract;

    /// @notice Emitted when bridge contract is updated
    event BridgeContractUpdated(address indexed oldBridge, address indexed newBridge);

    /// @notice Emitted when tokens are minted via bridge
    event BridgeMint(address indexed to, uint256 amount);

    /// @notice Emitted when tokens are burned via bridge
    event BridgeBurn(address indexed from, uint256 amount);

    /**
     * @dev Constructor mints initial supply for Uniswap V3 liquidity
     * @param initialOwner Foundation multisig address
     */
    constructor(address initialOwner) ERC20("Etrid Coin", "ETR") Ownable(initialOwner) {
        // Mint 100K ÉTR for bootstrap liquidity (expand later)
        _mint(initialOwner, 100_000 * 10**18);
    }

    /**
     * @dev Set the bridge contract address
     * @param _bridgeContract Address of the cross-chain bridge
     *
     * Requirements:
     * - Caller must be owner (Foundation multisig)
     * - Bridge address cannot be zero
     */
    function setBridgeContract(address _bridgeContract) external onlyOwner {
        require(_bridgeContract != address(0), "Bridge cannot be zero address");
        address oldBridge = bridgeContract;
        bridgeContract = _bridgeContract;
        emit BridgeContractUpdated(oldBridge, _bridgeContract);
    }

    /**
     * @dev Mint tokens when bridging FROM FlareChain TO Ethereum
     * @param to Recipient address on Ethereum
     * @param amount Amount to mint (in wei, 18 decimals)
     *
     * Requirements:
     * - Caller must be bridge contract
     * - Total supply cannot exceed MAX_SUPPLY
     *
     * @notice This is called when users lock ÉTR on FlareChain and mint on Ethereum
     */
    function bridgeMint(address to, uint256 amount) external {
        require(msg.sender == bridgeContract, "Only bridge can mint");
        require(totalSupply() + amount <= MAX_SUPPLY, "Exceeds max supply");
        require(to != address(0), "Cannot mint to zero address");

        _mint(to, amount);
        emit BridgeMint(to, amount);
    }

    /**
     * @dev Burn tokens when bridging FROM Ethereum TO FlareChain
     * @param from Address to burn from
     * @param amount Amount to burn (in wei, 18 decimals)
     *
     * Requirements:
     * - Caller must be bridge contract
     * - Account must have sufficient balance
     *
     * @notice This is called when users burn ÉTR on Ethereum to unlock on FlareChain
     */
    function bridgeBurn(address from, uint256 amount) external {
        require(msg.sender == bridgeContract, "Only bridge can burn");
        require(balanceOf(from) >= amount, "Insufficient balance");

        _burn(from, amount);
        emit BridgeBurn(from, amount);
    }

    /**
     * @dev Emergency pause function (circuit breaker)
     * @notice Can only be called by owner in case of critical security issue
     *
     * This function is intentionally left empty but can be extended
     * to implement ERC20Pausable if needed in future upgrades.
     */
    function emergencyPause() external onlyOwner {
        // Reserved for future emergency functionality
        // Requires 7-of-9 Foundation multisig per charter
    }
}
