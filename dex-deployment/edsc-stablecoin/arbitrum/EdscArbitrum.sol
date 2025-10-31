// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title EDSC - Ëtrid Dollar Stablecoin on Arbitrum
 * @notice USD-pegged stablecoin backed by 150% collateral on FlareChain
 * @dev Mintable/burnable, bridgeable to FlareChain
 *
 * Token Properties:
 * - Name: Etrid Dollar Stablecoin
 * - Symbol: EDSC
 * - Decimals: 18
 * - Peg: $1.00 USD
 * - Total Supply: Backed 1:1 with FlareChain reserve
 * - Collateralization: 150% minimum on FlareChain
 *
 * Use Cases:
 * - Stable payments and transactions
 * - Trading pairs with USDC, USDT, DAI
 * - Cross-chain transfers via Ëtrid bridge
 * - Low-slippage swaps on stablecoin DEXes
 */
contract EdscArbitrum is ERC20, ERC20Burnable, Ownable {
    /// Maximum supply: 1 billion EDSC
    uint256 public constant MAX_SUPPLY = 1_000_000_000 * 10**18;

    /// Base bridge contract address (for bridging to other L2s)
    address public baseBridge;

    /// Cross-chain bridge to FlareChain
    address public flarechainBridge;

    /// Pause state for emergency situations
    bool public paused;

    /// Emitted when bridge addresses are updated
    event BaseBridgeUpdated(address indexed oldBridge, address indexed newBridge);
    event FlarechainBridgeUpdated(address indexed oldBridge, address indexed newBridge);

    /// Emitted when pause state changes
    event PauseStateChanged(bool paused);

    /// Emitted when tokens are minted from bridge
    event BridgeMint(address indexed recipient, uint256 amount, bytes32 indexed txHash);

    /// Emitted when tokens are burned for bridge
    event BridgeBurn(address indexed sender, uint256 amount, bytes32 indexed txHash);

    error Paused();
    error ExceedsMaxSupply();
    error ZeroAddress();
    error OnlyBridge();

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    /**
     * @notice Constructor
     * @param initialOwner Address that will own the contract (governance multisig)
     */
    constructor(address initialOwner)
        ERC20("Etrid Dollar Stablecoin", "EDSC")
        Ownable(initialOwner)
    {
        if (initialOwner == address(0)) revert ZeroAddress();

        // Mint initial supply for stablecoin pool bootstrapping
        // 100,000 EDSC for initial EDSC/USDC pool on Arbitrum
        _mint(initialOwner, 100_000 * 10**18);
    }

    /**
     * @notice Set Base bridge contract address
     * @dev Only owner can update. Bridge handles L2-to-L2 transfers.
     * @param _baseBridge Address of Base bridge contract
     */
    function setBaseBridge(address _baseBridge) external onlyOwner {
        if (_baseBridge == address(0)) revert ZeroAddress();

        address oldBridge = baseBridge;
        baseBridge = _baseBridge;

        emit BaseBridgeUpdated(oldBridge, _baseBridge);
    }

    /**
     * @notice Set FlareChain bridge contract address
     * @dev Only owner can update. Bridge handles Base <-> FlareChain transfers.
     * @param _flarechainBridge Address of FlareChain bridge contract
     */
    function setFlarechainBridge(address _flarechainBridge) external onlyOwner {
        if (_flarechainBridge == address(0)) revert ZeroAddress();

        address oldBridge = flarechainBridge;
        flarechainBridge = _flarechainBridge;

        emit FlarechainBridgeUpdated(oldBridge, _flarechainBridge);
    }

    /**
     * @notice Pause the contract
     * @dev Only owner can pause. Prevents transfers, mints, and burns.
     */
    function pause() external onlyOwner {
        paused = true;
        emit PauseStateChanged(true);
    }

    /**
     * @notice Unpause the contract
     * @dev Only owner can unpause.
     */
    function unpause() external onlyOwner {
        paused = false;
        emit PauseStateChanged(false);
    }

    /**
     * @notice Mint EDSC from bridge
     * @dev Only bridge contracts can mint. Called when receiving from FlareChain.
     * @param to Recipient address
     * @param amount Amount to mint
     * @param txHash Transaction hash from source chain
     */
    function bridgeMint(address to, uint256 amount, bytes32 txHash)
        external
        whenNotPaused
    {
        if (msg.sender != baseBridge && msg.sender != flarechainBridge) {
            revert OnlyBridge();
        }

        if (totalSupply() + amount > MAX_SUPPLY) {
            revert ExceedsMaxSupply();
        }

        _mint(to, amount);
        emit BridgeMint(to, amount, txHash);
    }

    /**
     * @notice Burn EDSC for bridge transfer
     * @dev Only bridge contracts can burn. Called when sending to FlareChain.
     * @param from Address to burn from
     * @param amount Amount to burn
     * @param txHash Transaction hash for tracking
     */
    function bridgeBurn(address from, uint256 amount, bytes32 txHash)
        external
        whenNotPaused
    {
        if (msg.sender != baseBridge && msg.sender != flarechainBridge) {
            revert OnlyBridge();
        }

        _burn(from, amount);
        emit BridgeBurn(from, amount, txHash);
    }

    /**
     * @notice Override transfer to add pause functionality
     */
    function transfer(address to, uint256 amount)
        public
        virtual
        override
        whenNotPaused
        returns (bool)
    {
        return super.transfer(to, amount);
    }

    /**
     * @notice Override transferFrom to add pause functionality
     */
    function transferFrom(address from, address to, uint256 amount)
        public
        virtual
        override
        whenNotPaused
        returns (bool)
    {
        return super.transferFrom(from, to, amount);
    }

    /**
     * @notice Get current circulating supply
     * @return Current total supply of EDSC on Arbitrum
     */
    function circulatingSupply() external view returns (uint256) {
        return totalSupply();
    }

    /**
     * @notice Check if token is a stablecoin
     * @return Always returns true
     */
    function isStablecoin() external pure returns (bool) {
        return true;
    }

    /**
     * @notice Get target peg price in USD (18 decimals)
     * @return Target price: 1.000000000000000000 (1 USD)
     */
    function targetPeg() external pure returns (uint256) {
        return 1 * 10**18; // $1.00 USD
    }
}
