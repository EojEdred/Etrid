#ifndef ETRCPP_TYPES_H
#define ETRCPP_TYPES_H

#include <string>
#include <vector>
#include <cstdint>
#include <nlohmann/json.hpp>

namespace etrid {

// Account structure
struct Account {
    std::string address;
    std::string public_key;
    uint64_t balance;
    uint64_t nonce;
    bool is_validator;

    nlohmann::json to_json() const {
        return {
            {"address", address},
            {"public_key", public_key},
            {"balance", balance},
            {"nonce", nonce},
            {"is_validator", is_validator}
        };
    }
};

// Transaction structure
struct Transaction {
    std::string from;
    std::string to;
    uint64_t amount;
    uint64_t fee;
    uint64_t nonce;
    std::string signature;
    std::string hash;

    nlohmann::json to_json() const {
        return {
            {"from", from},
            {"to", to},
            {"amount", amount},
            {"fee", fee},
            {"nonce", nonce},
            {"signature", signature},
            {"hash", hash}
        };
    }
};

// Block structure
struct Block {
    uint64_t height;
    std::string hash;
    std::string previous_hash;
    uint64_t timestamp;
    std::vector<Transaction> transactions;
    std::string validator;

    nlohmann::json to_json() const {
        nlohmann::json txs = nlohmann::json::array();
        for (const auto& tx : transactions) {
            txs.push_back(tx.to_json());
        }

        return {
            {"height", height},
            {"hash", hash},
            {"previous_hash", previous_hash},
            {"timestamp", timestamp},
            {"transactions", txs},
            {"validator", validator}
        };
    }
};

// Stake information
struct StakeInfo {
    std::string address;
    uint64_t staked_amount;
    uint64_t rewards;
    bool is_active;
    uint64_t stake_time;

    nlohmann::json to_json() const {
        return {
            {"address", address},
            {"staked_amount", staked_amount},
            {"rewards", rewards},
            {"is_active", is_active},
            {"stake_time", stake_time}
        };
    }
};

// Consensus day information
struct ConsensusDay {
    uint64_t day_number;
    uint64_t start_time;
    uint64_t end_time;
    std::vector<std::string> validators;
    uint64_t total_stake;
    bool is_active;

    nlohmann::json to_json() const {
        return {
            {"day_number", day_number},
            {"start_time", start_time},
            {"end_time", end_time},
            {"validators", validators},
            {"total_stake", total_stake},
            {"is_active", is_active}
        };
    }
};

// RPC Response wrapper
struct RPCResponse {
    bool success;
    nlohmann::json result;
    std::string error_message;
    int error_code;

    RPCResponse() : success(false), error_code(0) {}

    RPCResponse(bool s, const nlohmann::json& r)
        : success(s), result(r), error_code(0) {}

    RPCResponse(bool s, const std::string& err, int code = -1)
        : success(s), error_message(err), error_code(code) {}
};

} // namespace etrid

#endif // ETRCPP_TYPES_H
