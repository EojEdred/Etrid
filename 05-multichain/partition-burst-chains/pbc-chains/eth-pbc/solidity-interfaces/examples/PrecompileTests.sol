// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "../IEtridNativeETH.sol";
import "../IEthereumStateProof.sol";
import "../IEtridTokenRegistry.sol";
import "../IEtridOracle.sol";
import "../IEtridGovernance.sol";
import "../IEtridStaking.sol";

/**
 * @title PrecompileTests
 * @notice Comprehensive test suite for all Etrid precompiles
 * @dev Deploy this contract to test precompile functionality
 */
contract PrecompileTests {
    // Precompile interfaces
    IEtridNativeETH constant nativeETH = IEtridNativeETH(0x0000000000000000000000000000000000000803);
    IEthereumStateProof constant stateProof = IEthereumStateProof(0x0000000000000000000000000000000000000804);
    IEtridTokenRegistry constant tokenRegistry = IEtridTokenRegistry(0x0000000000000000000000000000000000000805);
    IEtridOracle constant oracle = IEtridOracle(0x0000000000000000000000000000000000000800);
    IEtridGovernance constant governance = IEtridGovernance(0x0000000000000000000000000000000000000801);
    IEtridStaking constant staking = IEtridStaking(0x0000000000000000000000000000000000000802);

    // Events for test results
    event TestResult(string testName, bool passed, string message);

    // ====================
    // NATIVE ETH WRAPPING TESTS (0x803)
    // ====================

    function testNativeETHWrap() external payable {
        require(msg.value > 0, "Send ETH to test");

        try nativeETH.wrap{value: msg.value}() returns (uint256 wethAmount) {
            emit TestResult("NativeETH: wrap()", wethAmount == msg.value, "Wrapped ETH successfully");
        } catch Error(string memory reason) {
            emit TestResult("NativeETH: wrap()", false, reason);
        }
    }

    function testNativeETHGetRate() external {
        try nativeETH.getWrapRate() returns (uint256 rate) {
            bool passed = rate > 0;
            emit TestResult("NativeETH: getWrapRate()", passed, passed ? "Rate returned" : "Invalid rate");
        } catch Error(string memory reason) {
            emit TestResult("NativeETH: getWrapRate()", false, reason);
        }
    }

    function testNativeETHUnwrap() external {
        uint256 amount = 1 ether;
        try nativeETH.unwrap(amount) returns (bool success) {
            emit TestResult("NativeETH: unwrap()", success, "Unwrap executed");
        } catch Error(string memory reason) {
            emit TestResult("NativeETH: unwrap()", false, reason);
        }
    }

    // ====================
    // STATE PROOF TESTS (0x804)
    // ====================

    function testStateProofGetLatestBlock() external {
        try stateProof.getLatestEthBlock() returns (
            uint256 blockNumber,
            bytes32 blockHash,
            bytes32 stateRoot,
            uint256 timestamp
        ) {
            bool passed = blockNumber > 0 && blockHash != bytes32(0) && stateRoot != bytes32(0);
            emit TestResult("StateProof: getLatestEthBlock()", passed, "Block info retrieved");
        } catch Error(string memory reason) {
            emit TestResult("StateProof: getLatestEthBlock()", false, reason);
        }
    }

    function testStateProofVerify() external {
        bytes32 stateRoot = keccak256("test_state_root");
        bytes32[] memory proof = new bytes32[](2);
        proof[0] = keccak256("proof1");
        proof[1] = keccak256("proof2");
        bytes32 key = keccak256("test_key");
        bytes memory value = abi.encode(uint256(12345));

        try stateProof.verifyStateProof(stateRoot, proof, key, value) returns (bool valid) {
            emit TestResult("StateProof: verifyStateProof()", valid, "Proof verified");
        } catch Error(string memory reason) {
            emit TestResult("StateProof: verifyStateProof()", false, reason);
        }
    }

    function testStateProofVerifyTransaction() external {
        bytes32 txHash = keccak256("test_tx");
        bytes32 blockHash = keccak256("test_block");
        bytes memory rlpTx = abi.encode(uint256(123));
        bytes32[] memory proof = new bytes32[](1);
        proof[0] = keccak256("tx_proof");

        try stateProof.verifyTransaction(txHash, blockHash, rlpTx, proof) returns (bool valid) {
            emit TestResult("StateProof: verifyTransaction()", valid, "Transaction verified");
        } catch Error(string memory reason) {
            emit TestResult("StateProof: verifyTransaction()", false, reason);
        }
    }

    // ====================
    // TOKEN REGISTRY TESTS (0x805)
    // ====================

    function testTokenRegistryRegister() external {
        address mockToken = address(0x1000);

        try tokenRegistry.registerToken(mockToken) returns (bool success) {
            emit TestResult("TokenRegistry: registerToken()", success, "Token registered");
        } catch Error(string memory reason) {
            emit TestResult("TokenRegistry: registerToken()", false, reason);
        }
    }

    function testTokenRegistryGetInfo() external {
        address mockToken = address(0x1000);

        try tokenRegistry.getTokenInfo(mockToken) returns (
            string memory name,
            string memory symbol,
            uint8 decimals,
            uint256 supply
        ) {
            bool passed = bytes(name).length > 0 && bytes(symbol).length > 0;
            emit TestResult("TokenRegistry: getTokenInfo()", passed, "Token info retrieved");
        } catch Error(string memory reason) {
            emit TestResult("TokenRegistry: getTokenInfo()", false, reason);
        }
    }

    function testTokenRegistryGetBridgedTokens() external {
        try tokenRegistry.getBridgedTokens() returns (address[] memory tokens) {
            bool passed = tokens.length >= 0;
            emit TestResult("TokenRegistry: getBridgedTokens()", passed, "Token list retrieved");
        } catch Error(string memory reason) {
            emit TestResult("TokenRegistry: getBridgedTokens()", false, reason);
        }
    }

    // ====================
    // ORACLE TESTS (0x800)
    // ====================

    function testOracleGetPriceInETH() external {
        try oracle.getPriceInETH("BTC") returns (uint256 price) {
            bool passed = price > 0;
            emit TestResult("Oracle: getPriceInETH()", passed, passed ? "Price retrieved" : "Invalid price");
        } catch Error(string memory reason) {
            emit TestResult("Oracle: getPriceInETH()", false, reason);
        }
    }

    function testOracleGetPrice() external {
        try oracle.getPrice("ETH", "USD") returns (uint256 price) {
            bool passed = price > 0;
            emit TestResult("Oracle: getPrice()", passed, passed ? "Price retrieved" : "Invalid price");
        } catch Error(string memory reason) {
            emit TestResult("Oracle: getPrice()", false, reason);
        }
    }

    function testOracleGetLastUpdate() external {
        try oracle.getLastUpdate("BTC") returns (uint256 timestamp) {
            bool passed = timestamp > 0;
            emit TestResult("Oracle: getLastUpdate()", passed, passed ? "Timestamp retrieved" : "Invalid timestamp");
        } catch Error(string memory reason) {
            emit TestResult("Oracle: getLastUpdate()", false, reason);
        }
    }

    // ====================
    // GOVERNANCE TESTS (0x801)
    // ====================

    function testGovernanceSubmitProposal() external {
        try governance.submitProposal("Test Proposal", "This is a test proposal") returns (uint256 proposalId) {
            bool passed = proposalId > 0;
            emit TestResult("Governance: submitProposal()", passed, "Proposal submitted");
        } catch Error(string memory reason) {
            emit TestResult("Governance: submitProposal()", false, reason);
        }
    }

    function testGovernanceGetProposalStatus() external {
        uint256 proposalId = 1;
        try governance.getProposalStatus(proposalId) returns (uint8 status) {
            bool passed = status <= 3; // Valid status: 0-3
            emit TestResult("Governance: getProposalStatus()", passed, "Status retrieved");
        } catch Error(string memory reason) {
            emit TestResult("Governance: getProposalStatus()", false, reason);
        }
    }

    // ====================
    // STAKING TESTS (0x802)
    // ====================

    function testStakingGetTotalStaked() external {
        try staking.getTotalStaked() returns (uint256 total) {
            bool passed = total >= 0;
            emit TestResult("Staking: getTotalStaked()", passed, "Total stake retrieved");
        } catch Error(string memory reason) {
            emit TestResult("Staking: getTotalStaked()", false, reason);
        }
    }

    function testStakingGetValidatorCount() external {
        try staking.getValidatorCount() returns (uint256 count) {
            bool passed = count >= 0;
            emit TestResult("Staking: getValidatorCount()", passed, "Validator count retrieved");
        } catch Error(string memory reason) {
            emit TestResult("Staking: getValidatorCount()", false, reason);
        }
    }

    // ====================
    // COMPREHENSIVE TEST SUITE
    // ====================

    function runAllTests() external payable {
        emit TestResult("=== STARTING COMPREHENSIVE PRECOMPILE TESTS ===", true, "");

        // Native ETH Tests
        if (msg.value > 0) {
            this.testNativeETHWrap{value: msg.value / 3}();
        }
        this.testNativeETHGetRate();
        this.testNativeETHUnwrap();

        // State Proof Tests
        this.testStateProofGetLatestBlock();
        this.testStateProofVerify();
        this.testStateProofVerifyTransaction();

        // Token Registry Tests
        this.testTokenRegistryRegister();
        this.testTokenRegistryGetInfo();
        this.testTokenRegistryGetBridgedTokens();

        // Oracle Tests
        this.testOracleGetPriceInETH();
        this.testOracleGetPrice();
        this.testOracleGetLastUpdate();

        // Governance Tests
        this.testGovernanceSubmitProposal();
        this.testGovernanceGetProposalStatus();

        // Staking Tests
        this.testStakingGetTotalStaked();
        this.testStakingGetValidatorCount();

        emit TestResult("=== ALL TESTS COMPLETED ===", true, "");
    }

    // Receive function to accept ETH
    receive() external payable {}
}
