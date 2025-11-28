#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <iomanip>
#include "rpc_client.h"
#include "commands.h"
#include "types.h"

// Version information
#define ETRCPP_VERSION "1.0.0"
#define ETRCPP_BUILD "MVP-MAINNET"

namespace etrid {

// Print formatted JSON response
void printResponse(const RPCResponse& response, bool pretty = true) {
    if (response.success) {
        if (pretty) {
            std::cout << response.result.dump(2) << std::endl;
        } else {
            std::cout << response.result.dump() << std::endl;
        }
    } else {
        std::cerr << "Error [" << response.error_code << "]: "
                  << response.error_message << std::endl;
    }
}

// Print help message
void printHelp() {
    std::cout << "ËTRID C++ CLI (etrcpp) v" << ETRCPP_VERSION << "\n"
              << "Usage: etrcpp [options] <command> [parameters]\n\n"
              << "Options:\n"
              << "  -rpcconnect=<ip>    Connect to ËTRID node on <ip> (default: 127.0.0.1)\n"
              << "  -rpcport=<port>     Connect to ËTRID node on <port> (default: 9944)\n"
              << "  -rpcuser=<user>     Username for RPC authentication\n"
              << "  -rpcpassword=<pw>   Password for RPC authentication\n"
              << "  -timeout=<n>        Connection timeout in seconds (default: 30)\n"
              << "  -h, --help          Show this help message\n"
              << "  -version            Show version information\n\n"
              << "Account Commands:\n"
              << "  account create [name]          Create a new account\n"
              << "  account list                   List all accounts\n"
              << "  account info <address>         Get account information\n"
              << "  account import <key> [name]    Import account from private key\n\n"
              << "Stake Commands:\n"
              << "  stake <address> <amount>       Stake tokens\n"
              << "  unstake <address> [amount]     Unstake tokens (0 = all)\n"
              << "  stakeinfo <address>            Get stake information\n"
              << "  validators                     List all validators\n\n"
              << "Query Commands:\n"
              << "  balance <address>              Query account balance\n"
              << "  block <height|hash>            Query block information\n"
              << "  transaction <hash>             Query transaction details\n"
              << "  blockchaininfo                 Get blockchain information\n"
              << "  networkinfo                    Get network information\n\n"
              << "Transaction Commands:\n"
              << "  send <from> <to> <amount> [fee]    Send transaction\n"
              << "  sendraw <hex>                       Send raw transaction\n\n"
              << "Consensus Commands:\n"
              << "  consensusday                   Get current consensus day\n"
              << "  consensusdayinfo <day>         Get consensus day information\n"
              << "  vote <validator> <proposal> <yes|no>  Submit consensus vote\n\n"
              << "Examples:\n"
              << "  etrcpp account create my-account\n"
              << "  etrcpp balance 0x1234567890123456789012345678901234567890\n"
              << "  etrcpp send 0x... 0x... 1000000 1000\n"
              << "  etrcpp stake 0x... 10000000\n"
              << "  etrcpp consensusday\n"
              << std::endl;
}

// Print version information
void printVersion() {
    std::cout << "etrcpp version " << ETRCPP_VERSION << " (" << ETRCPP_BUILD << ")\n"
              << "ËTRID C++ Command-Line Interface\n"
              << "Copyright (c) 2025 ËTRID Foundation\n"
              << std::endl;
}

// Parse command-line arguments
struct CLIOptions {
    std::string rpc_host = "127.0.0.1";
    std::string rpc_port = "9944";
    std::string rpc_user;
    std::string rpc_password;
    int timeout = 30;
    std::vector<std::string> command_args;
};

CLIOptions parseArguments(int argc, char* argv[]) {
    CLIOptions options;

    for (int i = 1; i < argc; ++i) {
        std::string arg = argv[i];

        if (arg == "-h" || arg == "--help") {
            printHelp();
            exit(0);
        } else if (arg == "-version") {
            printVersion();
            exit(0);
        } else if (arg.find("-rpcconnect=") == 0) {
            options.rpc_host = arg.substr(12);
        } else if (arg.find("-rpcport=") == 0) {
            options.rpc_port = arg.substr(9);
        } else if (arg.find("-rpcuser=") == 0) {
            options.rpc_user = arg.substr(9);
        } else if (arg.find("-rpcpassword=") == 0) {
            options.rpc_password = arg.substr(13);
        } else if (arg.find("-timeout=") == 0) {
            options.timeout = std::stoi(arg.substr(9));
        } else {
            options.command_args.push_back(arg);
        }
    }

    return options;
}

// Execute command
int executeCommand(Commands& commands, const std::vector<std::string>& args) {
    if (args.empty()) {
        std::cerr << "Error: No command specified. Use -h for help." << std::endl;
        return 1;
    }

    std::string command = args[0];
    RPCResponse response;

    try {
        // Account commands
        if (command == "account") {
            if (args.size() < 2) {
                std::cerr << "Error: account command requires subcommand" << std::endl;
                return 1;
            }
            std::string subcmd = args[1];

            if (subcmd == "create") {
                std::string name = (args.size() > 2) ? args[2] : "";
                response = commands.accountCreate(name);
            } else if (subcmd == "list") {
                response = commands.accountList();
            } else if (subcmd == "info") {
                if (args.size() < 3) {
                    std::cerr << "Error: account info requires address" << std::endl;
                    return 1;
                }
                response = commands.accountInfo(args[2]);
            } else if (subcmd == "import") {
                if (args.size() < 3) {
                    std::cerr << "Error: account import requires private key" << std::endl;
                    return 1;
                }
                std::string name = (args.size() > 3) ? args[3] : "";
                response = commands.accountImport(args[2], name);
            } else {
                std::cerr << "Error: Unknown account subcommand: " << subcmd << std::endl;
                return 1;
            }
        }
        // Stake commands
        else if (command == "stake") {
            if (args.size() < 3) {
                std::cerr << "Error: stake requires address and amount" << std::endl;
                return 1;
            }
            response = commands.stakeTokens(args[1], std::stoull(args[2]));
        }
        else if (command == "unstake") {
            if (args.size() < 2) {
                std::cerr << "Error: unstake requires address" << std::endl;
                return 1;
            }
            uint64_t amount = (args.size() > 2) ? std::stoull(args[2]) : 0;
            response = commands.unstakeTokens(args[1], amount);
        }
        else if (command == "stakeinfo") {
            if (args.size() < 2) {
                std::cerr << "Error: stakeinfo requires address" << std::endl;
                return 1;
            }
            response = commands.stakeInfo(args[1]);
        }
        else if (command == "validators") {
            response = commands.listValidators();
        }
        // Query commands
        else if (command == "balance") {
            if (args.size() < 2) {
                std::cerr << "Error: balance requires address" << std::endl;
                return 1;
            }
            response = commands.queryBalance(args[1]);
        }
        else if (command == "block") {
            if (args.size() < 2) {
                std::cerr << "Error: block requires height or hash" << std::endl;
                return 1;
            }
            response = commands.queryBlock(args[1]);
        }
        else if (command == "transaction") {
            if (args.size() < 2) {
                std::cerr << "Error: transaction requires hash" << std::endl;
                return 1;
            }
            response = commands.queryTransaction(args[1]);
        }
        else if (command == "blockchaininfo") {
            response = commands.getBlockchainInfo();
        }
        else if (command == "networkinfo") {
            response = commands.getNetworkInfo();
        }
        // Transaction commands
        else if (command == "send") {
            if (args.size() < 4) {
                std::cerr << "Error: send requires from, to, and amount" << std::endl;
                return 1;
            }
            uint64_t fee = (args.size() > 4) ? std::stoull(args[4]) : 1000;
            response = commands.sendTransaction(args[1], args[2], std::stoull(args[3]), fee);
        }
        else if (command == "sendraw") {
            if (args.size() < 2) {
                std::cerr << "Error: sendraw requires raw transaction hex" << std::endl;
                return 1;
            }
            response = commands.sendRawTransaction(args[1]);
        }
        // Consensus commands
        else if (command == "consensusday") {
            response = commands.consensusDay();
        }
        else if (command == "consensusdayinfo") {
            if (args.size() < 2) {
                std::cerr << "Error: consensusdayinfo requires day number" << std::endl;
                return 1;
            }
            response = commands.consensusDayInfo(std::stoull(args[1]));
        }
        else if (command == "vote") {
            if (args.size() < 4) {
                std::cerr << "Error: vote requires validator, proposal, and yes/no" << std::endl;
                return 1;
            }
            bool vote = (args[3] == "yes" || args[3] == "true" || args[3] == "1");
            response = commands.submitVote(args[1], args[2], vote);
        }
        else {
            std::cerr << "Error: Unknown command: " << command << std::endl;
            std::cerr << "Use -h for help." << std::endl;
            return 1;
        }

        // Print response
        printResponse(response);
        return response.success ? 0 : 1;

    } catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << std::endl;
        return 1;
    }
}

} // namespace etrid

int main(int argc, char* argv[]) {
    using namespace etrid;

    try {
        // Parse arguments
        CLIOptions options = parseArguments(argc, argv);

        if (options.command_args.empty()) {
            printHelp();
            return 0;
        }

        // Build RPC URL
        std::string rpc_url = "http://" + options.rpc_host + ":" + options.rpc_port;

        // Create RPC client
        auto rpc_client = std::make_shared<RPCClient>(rpc_url, options.timeout);

        // Set authentication if provided
        if (!options.rpc_user.empty()) {
            rpc_client->setAuth(options.rpc_user, options.rpc_password);
        }

        // Create commands handler
        Commands commands(rpc_client);

        // Execute command
        return executeCommand(commands, options.command_args);

    } catch (const std::exception& e) {
        std::cerr << "Fatal error: " << e.what() << std::endl;
        return 1;
    }
}
