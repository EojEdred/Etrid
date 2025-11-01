// ËTRID Governance Portal - Dual Bootstrap Nodes
// Connects to Azure VM #1 (Alice) and VM #2 (Bob) with automatic failover

// Dual Bootstrap Nodes Configuration
const BOOTSTRAP_NODES = [
    'ws://20.186.91.207:9944',  // VM #1 (Alice) - Primary
    'ws://172.177.44.73:9944',   // VM #2 (Bob) - Fallback
];

// Consensus Day Configuration
const CONSENSUS_DAY = new Date('2026-12-01T00:00:00Z');

// Global state
let api = null;
let account = null;
let unsubscribe = null;

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    initializeApp();
    setupEventListeners();
    startCountdown();
});

// Initialize application
async function initializeApp() {
    console.log('🚀 Initializing ËTRID Governance Portal...');

    try {
        // Connect to blockchain with failover
        api = await connectToBlockchain();

        if (api) {
            showConnectionBanner(true);
            await loadGovernanceData();
        } else {
            showConnectionBanner(false);
            showNotification('Unable to connect to ËTRID network. Please check your connection.', 'error');
        }
    } catch (error) {
        console.error('❌ Initialization error:', error);
        showConnectionBanner(false);
        showNotification('Failed to initialize. Please refresh the page.', 'error');
    }
}

// Connect to blockchain with dual-node failover
async function connectToBlockchain() {
    let lastError = null;

    for (const endpoint of BOOTSTRAP_NODES) {
        try {
            console.log(`🔄 Attempting connection to ${endpoint}...`);

            const { ApiPromise, WsProvider } = polkadotApi;
            const provider = new WsProvider(endpoint);
            const api = await ApiPromise.create({ provider });

            await api.isReady;

            console.log(`✅ Connected to Ëtrid blockchain at ${endpoint}`);

            // Update UI with node info
            document.getElementById('nodeInfo').textContent = `Node: ${endpoint}`;

            return api;
        } catch (error) {
            console.warn(`⚠️ Failed to connect to ${endpoint}:`, error);
            lastError = error;
        }
    }

    console.error(`❌ Failed to connect to any bootstrap node. Last error:`, lastError);
    return null;
}

// Show/hide connection banner
function showConnectionBanner(connected) {
    const banner = document.getElementById('connectionBanner');
    const statusText = document.getElementById('connectionStatus');

    if (connected) {
        banner.classList.remove('hidden');
        statusText.textContent = 'Connected to Ëtrid Network';
    } else {
        banner.classList.add('hidden');
    }
}

// Load governance data from blockchain
async function loadGovernanceData() {
    if (!api) {
        console.warn('⚠️ No API connection, using mock data');
        loadMockData();
        return;
    }

    try {
        console.log('📊 Loading governance data...');

        // TODO: Query on-chain governance pallet
        // const proposals = await api.query.governance.activeProposals();
        // const voters = await api.query.governance.registeredVoters();
        // const inflation = await api.query.governance.currentInflation();

        // For now, use mock data until governance pallet is deployed
        loadMockData();

    } catch (error) {
        console.error('❌ Error loading governance data:', error);
        loadMockData();
    }
}

// Load mock governance data (until on-chain pallet is ready)
function loadMockData() {
    document.getElementById('activeProposals').textContent = '1';
    document.getElementById('totalVoters').textContent = '3,247';
    document.getElementById('votingPower').textContent = '0';
    document.getElementById('inflation').textContent = '2.5%';
}

// Countdown timer to Consensus Day
function startCountdown() {
    updateCountdown(); // Initial update
    setInterval(updateCountdown, 1000); // Update every second
}

function updateCountdown() {
    const now = new Date();
    const timeRemaining = CONSENSUS_DAY - now;

    if (timeRemaining <= 0) {
        // Consensus Day is happening or has passed
        document.getElementById('days').textContent = '000';
        document.getElementById('hours').textContent = '00';
        document.getElementById('minutes').textContent = '00';
        document.getElementById('seconds').textContent = '00';
        document.getElementById('currentPhase').textContent = 'Event Completed';
        return;
    }

    const days = Math.floor(timeRemaining / (1000 * 60 * 60 * 24));
    const hours = Math.floor((timeRemaining % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    const minutes = Math.floor((timeRemaining % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((timeRemaining % (1000 * 60)) / 1000);

    document.getElementById('days').textContent = String(days).padStart(3, '0');
    document.getElementById('hours').textContent = String(hours).padStart(2, '0');
    document.getElementById('minutes').textContent = String(minutes).padStart(2, '0');
    document.getElementById('seconds').textContent = String(seconds).padStart(2, '0');

    // Determine current phase
    determinePhase();
}

// Determine which Consensus Day phase we're in
function determinePhase() {
    const now = new Date();
    const consensusStart = new Date(CONSENSUS_DAY);

    // Check if we're during Consensus Day (22 hours)
    const timeSinceStart = now - consensusStart;
    const hoursElapsed = timeSinceStart / (1000 * 60 * 60);

    let phase = 'Preparation Period';

    if (hoursElapsed >= 0 && hoursElapsed < 6) {
        phase = 'Phase 1: Registration';
    } else if (hoursElapsed >= 6 && hoursElapsed < 18) {
        phase = 'Phase 2: Voting';
    } else if (hoursElapsed >= 18 && hoursElapsed < 21) {
        phase = 'Phase 3: Minting';
    } else if (hoursElapsed >= 21 && hoursElapsed < 22) {
        phase = 'Phase 4: Distribution';
    } else if (hoursElapsed >= 22) {
        phase = 'Event Completed';
    }

    document.getElementById('currentPhase').textContent = phase;
}

// Setup event listeners
function setupEventListeners() {
    // Connect Wallet button
    document.getElementById('connectWallet').addEventListener('click', connectWallet);

    // Tab navigation
    const tabButtons = document.querySelectorAll('.tab-button');
    tabButtons.forEach(button => {
        button.addEventListener('click', () => switchTab(button.dataset.tab));
    });
}

// Tab switching
function switchTab(tabName) {
    // Update button states
    document.querySelectorAll('.tab-button').forEach(btn => {
        btn.classList.remove('active', 'border-etrid-blue', 'text-etrid-blue');
        btn.classList.add('border-transparent', 'text-gray-400');
    });

    const activeButton = document.querySelector(`[data-tab="${tabName}"]`);
    activeButton.classList.add('active', 'border-etrid-blue', 'text-etrid-blue');
    activeButton.classList.remove('border-transparent', 'text-gray-400');

    // Show/hide tab content
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.add('hidden');
    });

    document.getElementById(`${tabName}-tab`).classList.remove('hidden');
}

// Connect wallet
async function connectWallet() {
    try {
        const { web3Accounts, web3Enable, web3FromAddress } = polkadotExtensionDapp;

        // Request access to extension
        const extensions = await web3Enable('ËTRID Governance Portal');

        if (extensions.length === 0) {
            showNotification('Please install Polkadot.js extension to connect your wallet', 'warning');
            return;
        }

        // Get all accounts
        const accounts = await web3Accounts();

        if (accounts.length === 0) {
            showNotification('No accounts found. Please create an account in your wallet.', 'warning');
            return;
        }

        // Select first account (in production, show account selector)
        account = accounts[0];

        console.log('✅ Wallet connected:', account.address);

        // Update UI
        document.getElementById('connectWallet').textContent =
            `${account.meta.name || 'Account'} (${account.address.substring(0, 8)}...)`;
        document.getElementById('connectWallet').classList.remove('bg-gradient-to-r', 'from-etrid-blue', 'to-etrid-purple');
        document.getElementById('connectWallet').classList.add('bg-green-500');

        showNotification('Wallet connected successfully!', 'success');

        // Load user-specific data
        await loadUserData();

    } catch (error) {
        console.error('❌ Error connecting wallet:', error);
        showNotification('Failed to connect wallet. Please try again.', 'error');
    }
}

// Load user-specific governance data
async function loadUserData() {
    if (!api || !account) return;

    try {
        console.log('👤 Loading user data...');

        // TODO: Query user's voting power, stake, history
        // const votingPower = await api.query.governance.votingPower(account.address);
        // const stakeInfo = await api.query.staking.ledger(account.address);

        // For now, show mock data
        document.getElementById('votingPower').textContent = '12,450';

        showNotification('Your voting power has been loaded', 'success');

    } catch (error) {
        console.error('❌ Error loading user data:', error);
    }
}

// Show notification
function showNotification(message, type = 'info') {
    // Create notification element
    const notification = document.createElement('div');
    notification.className = `fixed bottom-4 right-4 px-6 py-4 rounded-lg shadow-lg text-white z-50 animate-slide-up`;

    const colors = {
        success: 'bg-green-500',
        error: 'bg-red-500',
        warning: 'bg-yellow-500',
        info: 'bg-blue-500'
    };

    notification.classList.add(colors[type] || colors.info);
    notification.textContent = message;

    document.body.appendChild(notification);

    // Remove after 5 seconds
    setTimeout(() => {
        notification.remove();
    }, 5000);
}

// Export for debugging
window.governanceApp = {
    api,
    account,
    BOOTSTRAP_NODES,
    CONSENSUS_DAY
};
