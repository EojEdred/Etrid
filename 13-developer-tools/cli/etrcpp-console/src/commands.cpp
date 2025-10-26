#include "commands.h"
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <regex>

namespace etrid {

Commands::Commands(std::shared_ptr<RPCClient> client)
    : rpc_client_(client) {
}

// Account commands
RPCResponse Commands::accountCreate(const std::string& name) {
    nlohmann::json params = {
        {"name", name}
    };
    return rpc_client_->sendRequest("account_create", params);
}

RPCResponse Commands::accountList() {
    return rpc_client_->sendRequest("account_list", nlohmann::json::object());
}

RPCResponse Commands::accountInfo(const std::string& address) {
    validateAddress(address);
    nlohmann::json params = {
        {"address", address}
    };
    return rpc_client_->sendRequest("account_info", params);
}

RPCResponse Commands::accountImport(const std::string& private_key, const std::string& name) {
    nlohmann::json params = {
        {"private_key", private_key},
        {"name", name}
    };
    return rpc_client_->sendRequest("account_import", params);
}

// Stake commands
RPCResponse Commands::stakeTokens(const std::string& address, uint64_t amount) {
    validateAddress(address);
    if (amount == 0) {
        return RPCResponse(false, "Invalid stake amount: must be greater than 0", -1);
    }

    nlohmann::json params = {
        {"address", address},
        {"amount", amount}
    };
    return rpc_client_->sendRequest("stake_tokens", params);
}

RPCResponse Commands::unstakeTokens(const std::string& address, uint64_t amount) {
    validateAddress(address);
    nlohmann::json params = {
        {"address", address},
        {"amount", amount}
    };
    return rpc_client_->sendRequest("unstake_tokens", params);
}

RPCResponse Commands::stakeInfo(const std::string& address) {
    validateAddress(address);
    nlohmann::json params = {
        {"address", address}
    };
    return rpc_client_->sendRequest("stake_info", params);
}

RPCResponse Commands::listValidators() {
    return rpc_client_->sendRequest("list_validators", nlohmann::json::object());
}

// Query commands
RPCResponse Commands::queryBalance(const std::string& address) {
    validateAddress(address);
    nlohmann::json params = {
        {"address", address}
    };
    return rpc_client_->sendRequest("eth_getBalance", params);
}

RPCResponse Commands::queryBlock(const std::string& identifier) {
    nlohmann::json params;

    // Check if identifier is a number (block height) or hash
    bool is_number = true;
    for (char c : identifier) {
        if (!std::isdigit(c)) {
            is_number = false;
            break;
        }
    }

    if (is_number) {
        params = {
            {"block_height", std::stoull(identifier)}
        };
    } else {
        params = {
            {"block_hash", identifier}
        };
    }

    return rpc_client_->sendRequest("eth_getBlockByNumber", params);
}

RPCResponse Commands::queryTransaction(const std::string& tx_hash) {
    nlohmann::json params = {
        {"tx_hash", tx_hash}
    };
    return rpc_client_->sendRequest("eth_getTransactionByHash", params);
}

RPCResponse Commands::getBlockchainInfo() {
    return rpc_client_->sendRequest("blockchain_info", nlohmann::json::object());
}

RPCResponse Commands::getNetworkInfo() {
    return rpc_client_->sendRequest("network_info", nlohmann::json::object());
}

// Transaction commands
RPCResponse Commands::sendTransaction(const std::string& from, const std::string& to,
                                     uint64_t amount, uint64_t fee) {
    validateAddress(from);
    validateAddress(to);

    if (amount == 0) {
        return RPCResponse(false, "Invalid amount: must be greater than 0", -1);
    }

    nlohmann::json params = {
        {"from", from},
        {"to", to},
        {"amount", amount},
        {"fee", fee}
    };
    return rpc_client_->sendRequest("eth_sendTransaction", params);
}

RPCResponse Commands::sendRawTransaction(const std::string& raw_tx) {
    if (raw_tx.empty()) {
        return RPCResponse(false, "Invalid raw transaction: empty", -1);
    }

    nlohmann::json params = {
        {"raw_tx", raw_tx}
    };
    return rpc_client_->sendRequest("eth_sendRawTransaction", params);
}

// Consensus commands
RPCResponse Commands::consensusDay() {
    return rpc_client_->sendRequest("consensus_current_day", nlohmann::json::object());
}

RPCResponse Commands::consensusDayInfo(uint64_t day_number) {
    nlohmann::json params = {
        {"day_number", day_number}
    };
    return rpc_client_->sendRequest("consensus_day_info", params);
}

RPCResponse Commands::submitVote(const std::string& validator_address,
                                const std::string& proposal_id, bool vote) {
    validateAddress(validator_address);

    nlohmann::json params = {
        {"validator", validator_address},
        {"proposal_id", proposal_id},
        {"vote", vote}
    };
    return rpc_client_->sendRequest("consensus_submit_vote", params);
}

// Helper methods
void Commands::validateAddress(const std::string& address) {
    // Basic validation: should start with "0x" and be 42 characters (0x + 40 hex chars)
    // or follow ËTRID's address format
    if (address.empty()) {
        throw std::invalid_argument("Address cannot be empty");
    }

    // Allow both Ethereum-style and custom ËTRID addresses
    if (address.substr(0, 2) == "0x") {
        if (address.length() != 42) {
            throw std::invalid_argument("Invalid Ethereum-style address length");
        }
        // Check if remaining characters are hex
        std::regex hex_regex("^0x[0-9a-fA-F]{40}$");
        if (!std::regex_match(address, hex_regex)) {
            throw std::invalid_argument("Invalid Ethereum-style address format");
        }
    } else if (address.substr(0, 3) == "etr") {
        // ËTRID-specific address format
        if (address.length() < 10) {
            throw std::invalid_argument("Invalid ËTRID address length");
        }
    } else {
        throw std::invalid_argument("Invalid address format: must start with '0x' or 'etr'");
    }
}

uint64_t Commands::parseAmount(const std::string& amount_str) {
    try {
        return std::stoull(amount_str);
    } catch (const std::exception& e) {
        throw std::invalid_argument("Invalid amount format: " + amount_str);
    }
}

} // namespace etrid
