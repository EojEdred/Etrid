#ifndef ETRCPP_COMMANDS_H
#define ETRCPP_COMMANDS_H

#include <string>
#include <vector>
#include <memory>
#include "rpc_client.h"
#include "types.h"

namespace etrid {

/**
 * Commands - Command handlers for etrcpp CLI
 * Inspired by Bitcoin Core's command structure
 */
class Commands {
public:
    /**
     * Constructor
     * @param client Shared pointer to RPC client
     */
    explicit Commands(std::shared_ptr<RPCClient> client);

    // Account commands
    /**
     * Create a new ËTRID account
     * @param name Optional account name
     * @return Account information
     */
    RPCResponse accountCreate(const std::string& name = "");

    /**
     * List all accounts
     * @return List of accounts
     */
    RPCResponse accountList();

    /**
     * Get account information
     * @param address Account address
     * @return Account details
     */
    RPCResponse accountInfo(const std::string& address);

    /**
     * Import account from private key
     * @param private_key Private key to import
     * @param name Optional account name
     * @return Imported account information
     */
    RPCResponse accountImport(const std::string& private_key, const std::string& name = "");

    // Stake commands
    /**
     * Stake tokens
     * @param address Account address
     * @param amount Amount to stake
     * @return Stake transaction result
     */
    RPCResponse stakeTokens(const std::string& address, uint64_t amount);

    /**
     * Unstake tokens
     * @param address Account address
     * @param amount Amount to unstake (0 = all)
     * @return Unstake transaction result
     */
    RPCResponse unstakeTokens(const std::string& address, uint64_t amount = 0);

    /**
     * Get stake information
     * @param address Account address
     * @return Stake details
     */
    RPCResponse stakeInfo(const std::string& address);

    /**
     * List all validators
     * @return List of active validators
     */
    RPCResponse listValidators();

    // Query commands
    /**
     * Query account balance
     * @param address Account address
     * @return Balance information
     */
    RPCResponse queryBalance(const std::string& address);

    /**
     * Query block by height or hash
     * @param identifier Block height (number) or hash (string)
     * @return Block information
     */
    RPCResponse queryBlock(const std::string& identifier);

    /**
     * Query transaction by hash
     * @param tx_hash Transaction hash
     * @return Transaction details
     */
    RPCResponse queryTransaction(const std::string& tx_hash);

    /**
     * Get blockchain info
     * @return Blockchain status and information
     */
    RPCResponse getBlockchainInfo();

    /**
     * Get network info
     * @return Network status and peer information
     */
    RPCResponse getNetworkInfo();

    // Transaction commands
    /**
     * Send transaction
     * @param from Sender address
     * @param to Recipient address
     * @param amount Amount to send
     * @param fee Transaction fee
     * @return Transaction result
     */
    RPCResponse sendTransaction(const std::string& from, const std::string& to,
                                uint64_t amount, uint64_t fee = 1000);

    /**
     * Send raw transaction (signed)
     * @param raw_tx Raw transaction hex
     * @return Transaction result
     */
    RPCResponse sendRawTransaction(const std::string& raw_tx);

    // Consensus commands
    /**
     * Get current consensus day information
     * @return Current consensus day details
     */
    RPCResponse consensusDay();

    /**
     * Get consensus day by number
     * @param day_number Consensus day number
     * @return Consensus day details
     */
    RPCResponse consensusDayInfo(uint64_t day_number);

    /**
     * Submit consensus vote
     * @param validator_address Validator address
     * @param proposal_id Proposal ID
     * @param vote Vote (true = yes, false = no)
     * @return Vote submission result
     */
    RPCResponse submitVote(const std::string& validator_address,
                          const std::string& proposal_id, bool vote);

private:
    std::shared_ptr<RPCClient> rpc_client_;

    // Helper methods
    void validateAddress(const std::string& address);
    uint64_t parseAmount(const std::string& amount_str);
};

} // namespace etrid

#endif // ETRCPP_COMMANDS_H
