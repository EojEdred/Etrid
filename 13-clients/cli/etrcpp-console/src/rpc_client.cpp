#include "rpc_client.h"
#include <curl/curl.h>
#include <sstream>
#include <iostream>
#include <stdexcept>

namespace etrid {

// PIMPL implementation
class RPCClient::Impl {
public:
    std::string url;
    int timeout;
    std::string username;
    std::string password;
    CURL* curl;
    uint64_t request_id;

    Impl(const std::string& u, int t)
        : url(u), timeout(t), curl(nullptr), request_id(0) {
        curl_global_init(CURL_GLOBAL_DEFAULT);
        curl = curl_easy_init();
        if (!curl) {
            throw std::runtime_error("Failed to initialize CURL");
        }
    }

    ~Impl() {
        if (curl) {
            curl_easy_cleanup(curl);
        }
        curl_global_cleanup();
    }
};

// Callback for CURL to write response data
static size_t WriteCallback(void* contents, size_t size, size_t nmemb, void* userp) {
    ((std::string*)userp)->append((char*)contents, size * nmemb);
    return size * nmemb;
}

RPCClient::RPCClient(const std::string& url, int timeout)
    : pImpl(std::make_unique<Impl>(url, timeout)) {
}

RPCClient::~RPCClient() = default;

void RPCClient::setURL(const std::string& url) {
    pImpl->url = url;
}

void RPCClient::setTimeout(int timeout) {
    pImpl->timeout = timeout;
}

void RPCClient::setAuth(const std::string& username, const std::string& password) {
    pImpl->username = username;
    pImpl->password = password;
}

bool RPCClient::testConnection() {
    try {
        auto response = sendRequest("eth_blockNumber", nlohmann::json::object());
        return response.success;
    } catch (...) {
        return false;
    }
}

RPCResponse RPCClient::sendRequest(const std::string& method, const nlohmann::json& params) {
    try {
        // Build JSON-RPC request
        nlohmann::json request = {
            {"jsonrpc", "2.0"},
            {"id", ++pImpl->request_id},
            {"method", method},
            {"params", params}
        };

        std::string request_body = request.dump();
        std::string response_body = makeHTTPRequest(request_body);

        // Parse response
        nlohmann::json response = parseResponse(response_body);

        // Check for JSON-RPC error
        if (response.contains("error")) {
            auto error = response["error"];
            std::string error_msg = error.value("message", "Unknown error");
            int error_code = error.value("code", -1);
            return RPCResponse(false, error_msg, error_code);
        }

        // Return result
        if (response.contains("result")) {
            return RPCResponse(true, response["result"]);
        }

        return RPCResponse(false, "Invalid RPC response", -1);

    } catch (const std::exception& e) {
        return RPCResponse(false, std::string("Request failed: ") + e.what(), -1);
    }
}

std::string RPCClient::makeHTTPRequest(const std::string& body) {
    std::string response_string;

    if (!pImpl->curl) {
        throw std::runtime_error("CURL not initialized");
    }

    // Set CURL options
    curl_easy_setopt(pImpl->curl, CURLOPT_URL, pImpl->url.c_str());
    curl_easy_setopt(pImpl->curl, CURLOPT_POST, 1L);
    curl_easy_setopt(pImpl->curl, CURLOPT_POSTFIELDS, body.c_str());
    curl_easy_setopt(pImpl->curl, CURLOPT_TIMEOUT, pImpl->timeout);
    curl_easy_setopt(pImpl->curl, CURLOPT_WRITEFUNCTION, WriteCallback);
    curl_easy_setopt(pImpl->curl, CURLOPT_WRITEDATA, &response_string);

    // Set headers
    struct curl_slist* headers = nullptr;
    headers = curl_slist_append(headers, "Content-Type: application/json");

    // Add authentication if provided
    if (!pImpl->username.empty()) {
        std::string userpwd = pImpl->username + ":" + pImpl->password;
        curl_easy_setopt(pImpl->curl, CURLOPT_USERPWD, userpwd.c_str());
    }

    curl_easy_setopt(pImpl->curl, CURLOPT_HTTPHEADER, headers);

    // Perform request
    CURLcode res = curl_easy_perform(pImpl->curl);

    curl_slist_free_all(headers);

    if (res != CURLE_OK) {
        throw std::runtime_error(std::string("CURL request failed: ") + curl_easy_strerror(res));
    }

    // Check HTTP status code
    long http_code = 0;
    curl_easy_getinfo(pImpl->curl, CURLINFO_RESPONSE_CODE, &http_code);
    if (http_code != 200) {
        throw std::runtime_error("HTTP error: " + std::to_string(http_code));
    }

    return response_string;
}

nlohmann::json RPCClient::parseResponse(const std::string& response) {
    try {
        return nlohmann::json::parse(response);
    } catch (const nlohmann::json::parse_error& e) {
        throw std::runtime_error(std::string("JSON parse error: ") + e.what());
    }
}

} // namespace etrid
