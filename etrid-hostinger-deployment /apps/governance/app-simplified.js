// ËTRID Governance Portal - Simplified Standalone Version
// Works without blockchain connection until mainnet launch

// Consensus Day Configuration
const CONSENSUS_DAY = new Date('2026-12-01T00:00:00Z');

// Global state
let account = null;

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    console.log('🚀 Initializing ËTRID Governance Portal (Standalone Mode)...');
    initializeApp();
});

// Initialize application
function initializeApp() {
    // Load mock data immediately
    loadMockData();

    // Start countdown timer
    startCountdown();

    // Setup event listeners
    setupEventListeners();

    // Show info banner
    showInfoBanner();
}

// Show info banner about standalone mode
function showInfoBanner() {
    const banner = document.getElementById('connectionBanner');
    const statusText = document.getElementById('connectionStatus');

    banner.classList.remove('hidden');
    statusText.textContent = 'Preview Mode - Mainnet launching Q1 2026';
}

// Load mock governance data
function loadMockData() {
    console.log('📊 Loading governance preview data...');

    document.getElementById('activeProposals').textContent = '3';
    document.getElementById('totalVoters').textContent = '12,847';
    document.getElementById('votingPower').textContent = '0';
    document.getElementById('inflation').textContent = '2.5%';

    // Show message in proposals tab
    const proposalsTab = document.getElementById('proposals-tab');
    if (proposalsTab) {
        proposalsTab.innerHTML = `
            <div class="bg-etrid-dark border border-white/10 rounded-xl p-8 text-center">
                <svg class="w-16 h-16 mx-auto mb-4 text-etrid-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
                <h3 class="text-xl font-semibold mb-2">Preview Mode</h3>
                <p class="text-gray-400 mb-4">
                    The governance portal will be fully active when ËTRID mainnet launches in Q1 2026.
                </p>
                <p class="text-sm text-gray-500">
                    Next Consensus Day: December 1st, 2026
                </p>
            </div>
        `;
    }
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

    // Show preparation period
    document.getElementById('currentPhase').textContent = 'Preparation Period';
}

// Setup event listeners
function setupEventListeners() {
    // Connect Wallet button
    document.getElementById('connectWallet').addEventListener('click', showWalletInfo);

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

// Show wallet info (preview mode)
function showWalletInfo() {
    showNotification(
        'Wallet connection will be available when mainnet launches in Q1 2026. Install Polkadot.js extension to prepare!',
        'info'
    );
}

// Show notification
function showNotification(message, type = 'info') {
    // Create notification element
    const notification = document.createElement('div');
    notification.className = `fixed bottom-4 right-4 px-6 py-4 rounded-lg shadow-lg text-white z-50 max-w-md`;

    const colors = {
        success: 'bg-green-500',
        error: 'bg-red-500',
        warning: 'bg-yellow-500',
        info: 'bg-blue-500'
    };

    notification.classList.add(colors[type] || colors.info);
    notification.innerHTML = `
        <div class="flex items-start gap-3">
            <svg class="w-5 h-5 flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
            </svg>
            <span class="text-sm">${message}</span>
        </div>
    `;

    document.body.appendChild(notification);

    // Animate in
    setTimeout(() => {
        notification.style.animation = 'slideIn 0.3s ease-out';
    }, 10);

    // Remove after 6 seconds
    setTimeout(() => {
        notification.style.animation = 'slideOut 0.3s ease-in';
        setTimeout(() => notification.remove(), 300);
    }, 6000);
}

// Add animation styles
const style = document.createElement('style');
style.textContent = `
    @keyframes slideIn {
        from {
            transform: translateX(400px);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
    @keyframes slideOut {
        from {
            transform: translateX(0);
            opacity: 1;
        }
        to {
            transform: translateX(400px);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

console.log('✅ Governance portal initialized in preview mode');
