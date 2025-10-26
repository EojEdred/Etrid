#ifndef ETRCPP_RPC_CLIENT_H
#define ETRCPP_RPC_CLIENT_H

#include <string>
#include <memory>
#include <nlohmann/json.hpp>
#include "types.h"

namespace etrid {

/**
 * RPCClient - JSON-RPC client for ËTRID node communication
 * Inspired by Bitcoin Core's RPC client architecture
 */
class RPCClient {
public:
    /**
     * Constructor
     * @param url The ËTRID node RPC endpoint (default: http://localhost:9944)
     * @param timeout Connection timeout in seconds (default: 30)
     */
    explicit RPCClient(const std::string& url = "http://localhost:9944", int timeout = 30);

    /**
     * Destructor
     */
    ~RPCClient();

    /**
     * Send JSON-RPC request to ËTRID node
     * @param method RPC method name
     * @param params JSON parameters
     * @return RPCResponse containing result or error
     */
    RPCResponse sendRequest(const std::string& method, const nlohmann::json& params);

    /**
     * Set custom RPC endpoint
     * @param url New RPC endpoint URL
     */
    void setURL(const std::string& url);

    /**
     * Set request timeout
     * @param timeout Timeout in seconds
     */
    void setTimeout(int timeout);

    /**
     * Set authentication credentials (if required)
     * @param username RPC username
     * @param password RPC password
     */
    void setAuth(const std::string& username, const std::string& password);

    /**
     * Test connection to ËTRID node
     * @return true if connection successful
     */
    bool testConnection();

private:
    class Impl;
    std::unique_ptr<Impl> pImpl;

    // Disable copy
    RPCClient(const RPCClient&) = delete;
    RPCClient& operator=(const RPCClient&) = delete;

    // Helper methods
    std::string makeHTTPRequest(const std::string& body);
    nlohmann::json parseResponse(const std::string& response);
};

} // namespace etrid

#endif // ETRCPP_RPC_CLIENT_H
