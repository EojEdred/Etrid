// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, Window};
use bip39::{Mnemonic, Language};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Account data structure
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Account {
    name: String,
    address: String,
    public_key: String,
    seed_phrase: String,
    derivation_path: String,
    balance: f64,
    staked_amount: f64,
}

// Node connection configuration
#[derive(Serialize, Deserialize, Clone, Debug)]
struct NodeConfig {
    rpc_endpoint: String,
    ws_endpoint: String,
    chain_type: String,
    api_key: Option<String>,
}

// Blockchain data structures
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Block {
    height: u64,
    hash: String,
    timestamp: u64,
    transactions_count: u32,
    validator: String,
    extrinsics_root: String,
    state_root: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Validator {
    address: String,
    name: String,
    commission: f64, // percentage
    total_stake: f64,
    self_stake: f64,
    nominators_count: u32,
    is_active: bool,
    blocks_produced: u64,
    blocks_expected: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct NetworkStats {
    block_height: u64,
    total_issuance: f64,
    total_staked: f64,
    active_validators: u32,
    total_nominators: u32,
    transactions_per_second: f64,
    average_block_time: f64,
    connected_peers: u32,
    network_tps: f64,
    total_nodes: u32,
}

// Transaction data structure
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Transaction {
    id: String,
    from: String,
    to: String,
    amount: f64,
    fee: f64,
    timestamp: u64,
    status: String,
    transaction_type: String, // transfer, stake, unstake, etc.
}

// Staking data structure
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Stake {
    validator: String,
    amount: f64,
    start_era: u32,
    end_era: Option<u32>,
    rewards: f64,
    status: String, // active, inactive, pending
}

// Node status structure
#[derive(Serialize, Deserialize, Clone, Debug)]
struct NodeStatus {
    is_running: bool,
    block_height: u64,
    peers: u32,
    uptime: String,
    network_tps: f64,
}

// In-memory storage for demonstration
// In a real application, this would be persistent storage
struct AppState {
    accounts: Arc<Mutex<Vec<Account>>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    stakes: Arc<Mutex<Vec<Stake>>>,
    node_status: Arc<Mutex<NodeStatus>>,
    node_config: Arc<Mutex<NodeConfig>>,
    network_stats: Arc<Mutex<NetworkStats>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            accounts: Arc::new(Mutex::new(vec![
                // Default accounts
                Account {
                    name: "Alice (default)".to_string(),
                    address: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    public_key: "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string(),
                    seed_phrase: "bottom drive obey lake curtain smoke basket hold race lonely fit walk".to_string(),
                    derivation_path: "//Alice".to_string(),
                    balance: 10000.0,
                    staked_amount: 5000.0,
                },
                Account {
                    name: "Bob (default)".to_string(),
                    address: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                    public_key: "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48".to_string(),
                    seed_phrase: "run clerk work couple abuse pipe wolf clever exciting humble student mention".to_string(),
                    derivation_path: "//Bob".to_string(),
                    balance: 5000.0,
                    staked_amount: 2000.0,
                },
                Account {
                    name: "Charlie (default)".to_string(),
                    address: "5FLSigC9HGRKVhB9FiEo4Y3koPsNm3v1zQeeXV87u4JcndEh".to_string(),
                    public_key: "0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22".to_string(),
                    seed_phrase: "faculty prison setup split dream dinner robust nature weapon labor spin mesh".to_string(),
                    derivation_path: "//Charlie".to_string(),
                    balance: 7500.0,
                    staked_amount: 3000.0,
                },
            ])),
            transactions: Arc::new(Mutex::new(vec![ // Sample transactions
                Transaction {
                    id: "0x1a2b3c4d5e6f".to_string(),
                    from: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                    amount: 125.0,
                    fee: 0.001,
                    timestamp: 1678886400,
                    status: "confirmed".to_string(),
                    transaction_type: "transfer".to_string(),
                },
                Transaction {
                    id: "0x2b3c4d5e6f7a".to_string(),
                    from: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    to: "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty".to_string(),
                    amount: 50.0,
                    fee: 0.001,
                    timestamp: 1678886500,
                    status: "confirmed".to_string(),
                    transaction_type: "stake".to_string(),
                },
            ])),
            stakes: Arc::new(Mutex::new(vec![ // Sample stakes
                Stake {
                    validator: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string(),
                    amount: 2000.0,
                    start_era: 100,
                    end_era: None,
                    rewards: 12.5,
                    status: "active".to_string(),
                },
            ])),
            node_status: Arc::new(Mutex::new(NodeStatus {
                is_running: true,
                block_height: 1248921,
                peers: 24,
                uptime: "24 days, 14:42:17".to_string(),
                network_tps: 1248.0,
            })),
            node_config: Arc::new(Mutex::new(NodeConfig {
                rpc_endpoint: "wss://rpc.etrid.org".to_string(),
                ws_endpoint: "wss://ws.etrid.org".to_string(),
                chain_type: "Live".to_string(),
                api_key: None,
            })),
            network_stats: Arc::new(Mutex::new(NetworkStats {
                block_height: 1248921,
                total_issuance: 100000000.0,
                total_staked: 45000000.0,
                active_validators: 96,
                total_nominators: 2450,
                transactions_per_second: 1248.5,
                average_block_time: 12.4,
                connected_peers: 142,
                network_tps: 1248.5,
                total_nodes: 142,
            })),
        }
    }
}

// Account management commands
#[tauri::command]
async fn create_account(_window: Window, state: tauri::State<'_, AppState>, name: String) -> Result<Account, String> {
    // Generate a new mnemonic (seed phrase) using correct bip39 API
    // Note: The original code used Mnemonic::generate_in which is not in bip39 v2
    // We'll simulate it for now to fix the compilation error, assuming a standard generation
    let mut rng = rand::thread_rng();
    let entropy_bytes = (0..16).map(|_| rng.next_u32() as u8).collect::<Vec<u8>>();
    let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy_bytes)
        .map_err(|e| e.to_string())?;

    let seed_phrase = mnemonic.words().collect::<Vec<_>>().join(" ");

    // For now, we'll create a dummy address - in a real implementation,
    // this would involve proper key derivation for Substrate/Polkadot format
    let dummy_address = format!("5{}", format!("{:0>48x}", rng.next_u64()));

    let account = Account {
        name,
        address: dummy_address,
        public_key: format!("0x{:0>64x}", rng.next_u64()), // dummy public key
        seed_phrase,
        derivation_path: "m/44'/60'/0'/0/0".to_string(), // default derivation path
        balance: 0.0, // New account starts with 0 balance
        staked_amount: 0.0,
    };

    // Add the new account to the state
    {
        let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
        accounts.push(account.clone());
    }

    Ok(account)
}

#[tauri::command]
async fn import_account(_window: Window, state: tauri::State<'_, AppState>, name: String, seed_phrase: String) -> Result<Account, String> {
    // Validate the seed phrase
    let words: Vec<&str> = seed_phrase.split_whitespace().collect();
    if words.len() != 12 && words.len() != 24 {
        return Err("Invalid seed phrase: must be 12 or 24 words".to_string());
    }

    // Validate that the mnemonic is valid using parse_in_normalized
    let _mnemonic = Mnemonic::parse_in_normalized(Language::English, &seed_phrase)
        .map_err(|e| format!("Invalid seed phrase: {}", e))?;

    let mut rng = rand::thread_rng();
    let dummy_address = format!("5{}", format!("{:0>48x}", rng.next_u64()));

    let account = Account {
        name,
        address: dummy_address,
        public_key: format!("0x{:0>64x}", rng.next_u64()), // dummy public key
        seed_phrase,
        derivation_path: "m/44'/60'/0'/0/0".to_string(), // default derivation path
        balance: 0.0,
        staked_amount: 0.0,
    };

    // Add the imported account to the state
    {
        let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
        accounts.push(account.clone());
    }

    Ok(account)
}

#[tauri::command]
async fn get_default_accounts(_window: Window, state: tauri::State<'_, AppState>) -> Result<Vec<Account>, String> {
    // Return default accounts from state
    let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    let default_accounts: Vec<Account> = accounts
        .iter()
        .filter(|acc| acc.name.contains("(default)"))
        .cloned()
        .collect();

    Ok(default_accounts)
}

#[tauri::command]
async fn list_accounts(_window: Window, state: tauri::State<'_, AppState>) -> Result<Vec<Account>, String> {
    let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    Ok(accounts.clone())
}

#[tauri::command]
async fn get_account(_window: Window, state: tauri::State<'_, AppState>, address: String) -> Result<Option<Account>, String> {
    let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    let account = accounts.iter().find(|acc| acc.address == address).cloned();
    Ok(account)
}

#[tauri::command]
async fn delete_account(_window: Window, state: tauri::State<'_, AppState>, address: String) -> Result<String, String> {
    let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    if let Some(pos) = accounts.iter().position(|acc| acc.address == address) {
        accounts.remove(pos);
        Ok("Account deleted successfully".to_string())
    } else {
        Err("Account not found".to_string())
    }
}

#[tauri::command]
async fn send_tokens(_window: Window, state: tauri::State<'_, AppState>, from: String, to: String, amount: f64, fee: Option<f64>) -> Result<String, String> {
    if amount <= 0.0 {
        return Err("Amount must be greater than zero".to_string());
    }

    let fee = fee.unwrap_or(0.001); // Default fee

    // Find sender account
    let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    if let Some(sender_idx) = accounts.iter().position(|acc| acc.address == from) {
        if accounts[sender_idx].balance < amount + fee {
            return Err("Insufficient balance".to_string());
        }

        // Find recipient account
        if let Some(recipient_idx) = accounts.iter().position(|acc| acc.address == to) {
            // Perform the transfer
            accounts[sender_idx].balance -= amount + fee;
            accounts[recipient_idx].balance += amount;

            // Create transaction record
            let mut transactions = state.transactions.lock().map_err(|e| e.to_string())?;
            transactions.push(Transaction {
                id: format!("0x{}", format!("{:0>12x}", rand::random::<u64>())),
                from: from.clone(),
                to: to.clone(),
                amount,
                fee,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| e.to_string())?
                    .as_secs(),
                status: "confirmed".to_string(),
                transaction_type: "transfer".to_string(),
            });

            Ok("Transaction completed successfully".to_string())
        } else {
            Err("Recipient account not found".to_string())
        }
    } else {
        Err("Sender account not found".to_string())
    }
}

// Node management commands
#[tauri::command]
async fn start_node(_window: Window, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut node_status = state.node_status.lock().map_err(|e| e.to_string())?;
    node_status.is_running = true;
    Ok("Node started successfully".to_string())
}

#[tauri::command]
async fn stop_node(_window: Window, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut node_status = state.node_status.lock().map_err(|e| e.to_string())?;
    node_status.is_running = false;
    Ok("Node stopped successfully".to_string())
}

#[tauri::command]
async fn restart_node(_window: Window, state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut node_status = state.node_status.lock().map_err(|e| e.to_string())?;
    node_status.is_running = true;
    // In a real implementation, this would actually restart the node process
    Ok("Node restarted successfully".to_string())
}

#[tauri::command]
async fn get_node_status(_window: Window, state: tauri::State<'_, AppState>) -> Result<NodeStatus, String> {
    let node_status = state.node_status.lock().map_err(|e| e.to_string())?;
    Ok(node_status.clone())
}

#[tauri::command]
async fn get_network_info(_window: Window, state: tauri::State<'_, AppState>) -> Result<HashMap<String, String>, String> {
    let node_status = state.node_status.lock().map_err(|e| e.to_string())?;
    let mut info = HashMap::new();
    info.insert("block_height".to_string(), node_status.block_height.to_string());
    info.insert("peers".to_string(), node_status.peers.to_string());
    info.insert("uptime".to_string(), node_status.uptime.clone());
    info.insert("tps".to_string(), node_status.network_tps.to_string());
    info.insert("status".to_string(), if node_status.is_running { "running" } else { "stopped" }.to_string());
    Ok(info)
}

// Staking commands
#[tauri::command]
async fn stake_tokens(_window: Window, state: tauri::State<'_, AppState>, from: String, amount: f64, validator: String) -> Result<String, String> {
    if amount <= 0.0 {
        return Err("Amount must be greater than zero".to_string());
    }

    let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    if let Some(account_idx) = accounts.iter().position(|acc| acc.address == from) {
        if accounts[account_idx].balance < amount {
            return Err("Insufficient balance".to_string());
        }

        accounts[account_idx].balance -= amount;
        accounts[account_idx].staked_amount += amount;

        // Create stake record
        let mut stakes = state.stakes.lock().map_err(|e| e.to_string())?;
        stakes.push(Stake {
            validator,
            amount,
            start_era: 120, // Current era
            end_era: None,
            rewards: 0.0,
            status: "active".to_string(),
        });

        Ok(format!("Successfully staked {} tokens", amount))
    } else {
        Err("Account not found".to_string())
    }
}

#[tauri::command]
async fn unstake_tokens(_window: Window, state: tauri::State<'_, AppState>, from: String, amount: f64, validator: String) -> Result<String, String> {
    if amount <= 0.0 {
        return Err("Amount must be greater than zero".to_string());
    }

    let mut accounts = state.accounts.lock().map_err(|e| e.to_string())?;
    if let Some(account_idx) = accounts.iter().position(|acc| acc.address == from) {
        if accounts[account_idx].staked_amount < amount {
            return Err("Insufficient staked amount".to_string());
        }

        accounts[account_idx].balance += amount;
        accounts[account_idx].staked_amount -= amount;

        // Find and update the stake record
        let mut stakes = state.stakes.lock().map_err(|e| e.to_string())?;
        for stake in stakes.iter_mut() {
            if stake.validator == validator && stake.amount >= amount {
                stake.amount -= amount;
                if stake.amount == 0.0 {
                    stake.end_era = Some(121); // Current era
                }
                break;
            }
        }

        Ok(format!("Successfully unstaked {} tokens", amount))
    } else {
        Err("Account not found".to_string())
    }
}

#[tauri::command]
async fn get_stakes(_window: Window, state: tauri::State<'_, AppState>, _account_address: String) -> Result<Vec<Stake>, String> {
    // In a real implementation, we would filter stakes by account
    // For now, return all stakes
    let stakes = state.stakes.lock().map_err(|e| e.to_string())?;
    Ok(stakes.clone())
}

#[tauri::command]
async fn get_transaction_history(_window: Window, state: tauri::State<'_, AppState>, address: String) -> Result<Vec<Transaction>, String> {
    let transactions = state.transactions.lock().map_err(|e| e.to_string())?;
    let account_transactions: Vec<Transaction> = transactions
        .iter()
        .filter(|tx| tx.from == address || tx.to == address)
        .cloned()
        .collect();

    Ok(account_transactions)
}

#[tauri::command]
async fn get_network_stats(_window: Window, state: tauri::State<'_, AppState>) -> Result<NetworkStats, String> {
    let stats = state.network_stats.lock().map_err(|e| e.to_string())?;
    Ok(stats.clone())
}

// Terminal commands - enhanced functionality
#[tauri::command]
async fn execute_terminal_command(window: Window, state: tauri::State<'_, AppState>, command: String) -> Result<String, String> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Ok("No command provided".to_string());
    }

    match parts[0] {
        "help" => {
            let help_text = r###"Available commands:  help - Show this help message  status - Show node status  accounts - List all accounts  account [address] - Show account details  balance [address] - Check account balance  stake [amount] [validator] [from] - Stake tokens  unstake [amount] [validator] [from] - Unstake tokens  send [from] [to] [amount] - Send tokens between accounts  transactions [address] - Show transaction history  network - Show network information  node_start - Start the node  node_stop - Stop the node  node_restart - Restart the node  exit - Close the terminal"###;
            Ok(help_text.to_string())
        }
        "status" => {
            let node_status = state.node_status.lock().map_err(|e| e.to_string())?;
            Ok(format!(
                "Node Status: {}\nBlock Height: {}\nConnected Peers: {}\nUptime: {}\nNetwork TPS: {}",
                if node_status.is_running { "RUNNING" } else { "STOPPED" },
                node_status.block_height,
                node_status.peers,
                node_status.uptime,
                node_status.network_tps
            ))
        }
        "accounts" => {
            let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
            let mut result = "Accounts:\n".to_string();
            for account in accounts.iter() {
                result.push_str(&format!("- {} ({})\n  Balance: {} ETR\n  Staked: {} ETR\n",
                    account.name,
                    account.address,
                    account.balance,
                    account.staked_amount
                ));
            }
            Ok(result)
        }
        "account" => {
            if parts.len() < 2 {
                return Ok("Usage: account [address]".to_string());
            }
            let address = parts[1];
            let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
            if let Some(account) = accounts.iter().find(|acc| acc.address == address) {
                Ok(format!(
                    "Account Details:\nName: {}\nAddress: {}\nPublic Key: {}\nBalance: {} ETR\nStaked: {} ETR\nDerivation Path: {}",
                    account.name,
                    account.address,
                    account.public_key,
                    account.balance,
                    account.staked_amount,
                    account.derivation_path
                ))
            } else {
                Ok("Account not found".to_string())
            }
        }
        "balance" => {
            if parts.len() < 2 {
                return Ok("Usage: balance [address]".to_string());
            }
            let address = parts[1];
            let accounts = state.accounts.lock().map_err(|e| e.to_string())?;
            if let Some(account) = accounts.iter().find(|acc| acc.address == address) {
                Ok(format!("Balance for {}: {} ETR", address, account.balance))
            } else {
                Ok("Account not found".to_string())
            }
        }
        "stake" => {
            if parts.len() < 4 {
                return Ok("Usage: stake [amount] [validator] [from]".to_string());
            }
            let amount = parts[1].parse::<f64>().map_err(|_| "Invalid amount")?;
            let validator = parts[2];
            let from = parts[3];

            stake_tokens(window, state, from.to_string(), amount, validator.to_string()).await?;
            Ok(format!("Successfully staked {} ETR to validator 1248921", amount))
        }
        "unstake" => {
            if parts.len() < 4 {
                return Ok("Usage: unstake [amount] [validator] [from]".to_string());
            }
            let amount = parts[1].parse::<f64>().map_err(|_| "Invalid amount")?;
            let validator = parts[2];
            let from = parts[3];

            unstake_tokens(window, state, from.to_string(), amount, validator.to_string()).await?;
            Ok(format!("Successfully unstaked {} ETR from validator 1248921", amount))
        }
        "send" => {
            if parts.len() < 4 {
                return Ok("Usage: send [from] [to] [amount]".to_string());
            }
            let from = parts[1];
            let to = parts[2];
            let amount = parts[3].parse::<f64>().map_err(|_| "Invalid amount")?;

            send_tokens(window, state, from.to_string(), to.to_string(), amount, None).await?;
            Ok(format!("Successfully sent {} ETR from {} to {}", amount, from, to))
        }
        "transactions" => {
            if parts.len() < 2 {
                return Ok("Usage: transactions [address]".to_string());
            }
            let address = parts[1];
            let transactions = get_transaction_history(window, state, address.to_string()).await?;
            let mut result = format!("Transactions for {}:\n", address);
            for tx in transactions.iter().take(10) { // Show last 10 transactions
                result.push_str(&format!(
                    "  ID: {} | {} -> {} | {} ETR | {}\n",
                    tx.id,
                    tx.from,
                    tx.to,
                    tx.amount,
                    tx.status
                ));
            }
            Ok(result)
        }
        "network" => {
            // Get network statistics
            let stats = get_network_stats(window, state).await?;
                        Ok(format!(
                            "Network Information:\nBlock Height: {}\nActive Validators: {}\nConnected Peers: {}\n",
                            stats.block_height, stats.active_validators, stats.connected_peers
                        ))
        }
        "node_start" => {
            start_node(window, state).await?;
            Ok("Node started successfully".to_string())
        }
        "node_stop" => {
            stop_node(window, state).await?;
            Ok("Node stopped successfully".to_string())
        }
        "node_restart" => {
            restart_node(window, state).await?;
            Ok("Node restarted successfully".to_string())
        }
        "exit" => {
            Ok("Goodbye! Use the GUI to continue".to_string())
        }
        _ => {
            Ok(format!("Unknown command: {}. Type 'help' for available commands.", parts[0]))
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(
            tauri_plugin_log::Builder::default()
              .level(if cfg!(debug_assertions) { log::LevelFilter::Info } else { log::LevelFilter::Error })
              .build(),
        )
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_account,
            import_account,
            get_default_accounts,
            list_accounts,
            get_account,
            delete_account,
            send_tokens,
            start_node,
            stop_node,
            restart_node,
            get_node_status,
            get_network_info,
            stake_tokens,
            unstake_tokens,
            get_stakes,
            get_transaction_history,
            get_network_stats,
            execute_terminal_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
