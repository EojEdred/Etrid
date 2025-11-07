// ËTRID Node Deployment Platform
// One-click node deployment with ÉTR token payments

// Constants (from Ivory Papers Vol III - Original Specifications)
const APY_RATE = 0.03; // ~3% annual validator reward pool from circulating supply
const VALIDATOR_MIN_STAKE = 64; // 64 ÉTR minimum stake for VALIDITY Nodes

let selectedNodeType = null;
let walletConnected = false;
let walletAddress = null;
let currentStep = 1;
let deploymentConfig = {};

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
});

function setupEventListeners() {
    // Node type cards
    document.querySelectorAll('.node-type-card .deploy-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            e.stopPropagation();
            const card = e.target.closest('.node-type-card');
            selectedNodeType = card.dataset.type;
            openDeploymentWizard();
        });
    });

    // Wallet connection
    document.getElementById('connect-wallet-btn').addEventListener('click', () => {
        document.getElementById('wallet-modal').classList.remove('hidden');
    });

    document.getElementById('close-wallet-modal').addEventListener('click', () => {
        document.getElementById('wallet-modal').classList.add('hidden');
    });

    // Wallet options
    document.querySelectorAll('.wallet-option').forEach(btn => {
        btn.addEventListener('click', () => {
            const walletType = btn.dataset.wallet;
            connectWallet(walletType);
        });
    });

    // Wizard close
    document.getElementById('close-wizard').addEventListener('click', closeWizard);
}

// Wallet Connection
async function connectWallet(walletType) {
    try {
        let address = null;

        switch (walletType) {
            case 'metamask':
                address = await connectMetaMask();
                break;
            case 'walletconnect':
                address = await connectWalletConnect();
                break;
            case 'polkadot':
                address = await connectPolkadot();
                break;
            case 'trust':
                address = await connectTrustWallet();
                break;
        }

        if (address) {
            walletConnected = true;
            walletAddress = address;

            // Update UI
            document.getElementById('connect-wallet-btn').textContent = `${address.substring(0, 6)}...${address.substring(address.length - 4)}`;
            document.getElementById('connect-wallet-btn').classList.add('bg-green-500/20', 'text-green-400');

            // Close modal
            document.getElementById('wallet-modal').classList.add('hidden');

            // Show success message
            showNotification('Wallet connected successfully!', 'success');
        }
    } catch (error) {
        console.error('Wallet connection error:', error);
        showNotification('Failed to connect wallet. Please try again.', 'error');
    }
}

async function connectMetaMask() {
    if (typeof window.ethereum === 'undefined') {
        showNotification('Please install MetaMask first', 'error');
        window.open('https://metamask.io/download/', '_blank');
        return null;
    }

    const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
    return accounts[0];
}

async function connectWalletConnect() {
    // WalletConnect v2 implementation
    showNotification('WalletConnect integration coming soon', 'info');
    // For demo, return mock address
    return '0x' + Math.random().toString(16).substring(2, 42);
}

async function connectPolkadot() {
    try {
        const { web3Accounts, web3Enable } = await import('https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.1/+esm');

        const extensions = await web3Enable('ËTRID Node Deployment');

        if (extensions.length === 0) {
            showNotification('Please install Polkadot.js extension', 'error');
            window.open('https://polkadot.js.org/extension/', '_blank');
            return null;
        }

        const accounts = await web3Accounts();

        if (accounts.length === 0) {
            showNotification('No accounts found in Polkadot.js extension', 'error');
            return null;
        }

        return accounts[0].address;
    } catch (error) {
        console.error('Polkadot wallet error:', error);
        // For demo, return mock Substrate address
        return '5' + Math.random().toString(36).substring(2, 47).toUpperCase();
    }
}

async function connectTrustWallet() {
    if (typeof window.ethereum === 'undefined') {
        showNotification('Please install Trust Wallet browser extension', 'error');
        return null;
    }

    const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
    return accounts[0];
}

// Deployment Wizard
function openDeploymentWizard() {
    currentStep = 1;
    document.getElementById('deployment-wizard').classList.remove('hidden');
    renderWizardStep();
}

function closeWizard() {
    document.getElementById('deployment-wizard').classList.add('hidden');
    deploymentConfig = {};
    currentStep = 1;
}

function renderWizardStep() {
    const content = document.getElementById('wizard-content');

    switch (currentStep) {
        case 1:
            content.innerHTML = renderConfigurationStep();
            setupConfigurationHandlers();
            break;
        case 2:
            content.innerHTML = renderPaymentStep();
            setupPaymentHandlers();
            break;
        case 3:
            content.innerHTML = renderDeploymentStep();
            setupDeploymentHandlers();
            break;
    }

    updateStepIndicators();
}

function updateStepIndicators() {
    const indicators = document.querySelectorAll('.step-indicator');
    indicators.forEach((indicator, index) => {
        const stepNum = index + 1;
        const circle = indicator.querySelector('div');
        const text = indicator.querySelector('span');

        if (stepNum < currentStep) {
            circle.classList.add('bg-green-500');
            circle.classList.remove('bg-etrid-blue', 'bg-white/10');
            text?.classList.remove('text-gray-400');
        } else if (stepNum === currentStep) {
            circle.classList.add('bg-etrid-blue');
            circle.classList.remove('bg-white/10', 'bg-green-500');
            text?.classList.remove('text-gray-400');
        } else {
            circle.classList.add('bg-white/10');
            circle.classList.remove('bg-etrid-blue', 'bg-green-500');
            text?.classList.add('text-gray-400');
        }
    });
}

// Step 1: Configuration
function renderConfigurationStep() {
    const nodeTypeText = {
        validator: 'Validator Node',
        full: 'Full Node',
        local: 'Local Setup'
    };

    if (selectedNodeType === 'local') {
        return renderLocalSetup();
    }

    return `
        <div class="space-y-6">
            <div>
                <h3 class="text-xl font-bold mb-4">Configure Your ${nodeTypeText[selectedNodeType]}</h3>
                <p class="text-gray-400 text-sm mb-6">Choose your deployment settings</p>
            </div>

            <!-- Node Name -->
            <div>
                <label class="block text-sm font-medium mb-2">Node Name</label>
                <input type="text" id="node-name" placeholder="My ËTRID ${nodeTypeText[selectedNodeType]}" class="w-full px-4 py-3 rounded-lg bg-white/5 border border-white/10 focus:border-etrid-blue focus:outline-none">
            </div>

            <!-- Cloud Provider -->
            <div>
                <label class="block text-sm font-medium mb-2">Cloud Provider</label>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                    <button class="provider-option p-4 rounded-lg bg-white/5 border-2 border-white/10 hover:border-etrid-blue transition-all" data-provider="azure">
                        <div class="text-3xl mb-2">☁️</div>
                        <div class="text-sm font-medium">Azure</div>
                    </button>
                    <button class="provider-option p-4 rounded-lg bg-white/5 border-2 border-white/10 hover:border-etrid-blue transition-all" data-provider="aws">
                        <div class="text-3xl mb-2">📦</div>
                        <div class="text-sm font-medium">AWS</div>
                    </button>
                    <button class="provider-option p-4 rounded-lg bg-white/5 border-2 border-white/10 hover:border-etrid-blue transition-all" data-provider="gcp">
                        <div class="text-3xl mb-2">🌐</div>
                        <div class="text-sm font-medium">Google Cloud</div>
                    </button>
                    <button class="provider-option p-4 rounded-lg bg-white/5 border-2 border-white/10 hover:border-etrid-blue transition-all" data-provider="digitalocean">
                        <div class="text-3xl mb-2">🌊</div>
                        <div class="text-sm font-medium">DigitalOcean</div>
                    </button>
                </div>
            </div>

            <!-- Region -->
            <div>
                <label class="block text-sm font-medium mb-2">Region</label>
                <select id="region-select" class="w-full px-4 py-3 rounded-lg bg-white/5 border border-white/10 focus:border-etrid-blue focus:outline-none">
                    <option value="">Select a region...</option>
                    <option value="us-east-1">US East (N. Virginia)</option>
                    <option value="us-west-2">US West (Oregon)</option>
                    <option value="eu-west-1">EU West (Ireland)</option>
                    <option value="eu-central-1">EU Central (Frankfurt)</option>
                    <option value="ap-southeast-1">Asia Pacific (Singapore)</option>
                    <option value="ap-northeast-1">Asia Pacific (Tokyo)</option>
                </select>
            </div>

            <!-- Instance Type -->
            <div>
                <label class="block text-sm font-medium mb-2">Instance Type</label>
                <div class="space-y-2">
                    <label class="instance-option flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="instance-type" value="standard" class="w-4 h-4" checked>
                        <div class="flex-1">
                            <div class="font-semibold">Standard</div>
                            <div class="text-xs text-gray-400">4 vCPU, 16 GB RAM, 500 GB SSD</div>
                        </div>
                        <div class="text-etrid-blue font-semibold">${selectedNodeType === 'validator' ? '500' : '200'} ÉTR/mo</div>
                    </label>
                    <label class="instance-option flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="instance-type" value="high-performance" class="w-4 h-4">
                        <div class="flex-1">
                            <div class="font-semibold">High Performance</div>
                            <div class="text-xs text-gray-400">8 vCPU, 32 GB RAM, 1 TB NVMe</div>
                        </div>
                        <div class="text-etrid-blue font-semibold">${selectedNodeType === 'validator' ? '800' : '350'} ÉTR/mo</div>
                    </label>
                </div>
            </div>

            ${selectedNodeType === 'validator' ? `
            <!-- Validator Stake -->
            <div>
                <label class="block text-sm font-medium mb-2">Initial Stake (minimum 64 ÉTR)</label>
                <input type="number" id="validator-stake" placeholder="64" min="64" class="w-full px-4 py-3 rounded-lg bg-white/5 border border-white/10 focus:border-etrid-blue focus:outline-none">
                <p class="text-xs text-gray-400 mt-2">Minimum 64 ÉTR stake required for VALIDITY Nodes</p>
            </div>
            ` : ''}

            <!-- Action Buttons -->
            <div class="flex gap-3 pt-4">
                <button onclick="closeWizard()" class="flex-1 px-6 py-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-colors font-medium">
                    Cancel
                </button>
                <button id="next-to-payment" class="flex-1 px-6 py-3 rounded-lg bg-gradient-to-r from-etrid-blue to-etrid-purple hover:opacity-90 transition-opacity font-medium">
                    Continue to Payment →
                </button>
            </div>
        </div>
    `;
}

function renderLocalSetup() {
    return `
        <div class="space-y-6">
            <div>
                <h3 class="text-xl font-bold mb-4">Local Node Setup</h3>
                <p class="text-gray-400 text-sm mb-6">Run ËTRID on your own hardware</p>
            </div>

            <!-- Operating System -->
            <div>
                <label class="block text-sm font-medium mb-2">Operating System</label>
                <div class="grid grid-cols-3 gap-3">
                    <button class="os-option p-4 rounded-lg bg-white/5 border-2 border-etrid-blue transition-all" data-os="linux">
                        <div class="text-3xl mb-2">🐧</div>
                        <div class="text-sm font-medium">Linux</div>
                    </button>
                    <button class="os-option p-4 rounded-lg bg-white/5 border-2 border-white/10 hover:border-etrid-blue transition-all" data-os="macos">
                        <div class="text-3xl mb-2">🍎</div>
                        <div class="text-sm font-medium">macOS</div>
                    </button>
                    <button class="os-option p-4 rounded-lg bg-white/5 border-2 border-white/10 hover:border-etrid-blue transition-all" data-os="windows">
                        <div class="text-3xl mb-2">🪟</div>
                        <div class="text-sm font-medium">Windows</div>
                    </button>
                </div>
            </div>

            <!-- Installation Method -->
            <div>
                <label class="block text-sm font-medium mb-2">Installation Method</label>
                <div class="space-y-2">
                    <label class="flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-etrid-blue cursor-pointer">
                        <input type="radio" name="install-method" value="docker" class="w-4 h-4" checked>
                        <div class="flex-1">
                            <div class="font-semibold">Docker (Recommended)</div>
                            <div class="text-xs text-gray-400">Easiest setup with automatic updates</div>
                        </div>
                    </label>
                    <label class="flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="install-method" value="binary" class="w-4 h-4">
                        <div class="flex-1">
                            <div class="font-semibold">Binary</div>
                            <div class="text-xs text-gray-400">Direct binary installation</div>
                        </div>
                    </label>
                    <label class="flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="install-method" value="source" class="w-4 h-4">
                        <div class="flex-1">
                            <div class="font-semibold">Build from Source</div>
                            <div class="text-xs text-gray-400">For advanced users</div>
                        </div>
                    </label>
                </div>
            </div>

            <!-- Node Type -->
            <div>
                <label class="block text-sm font-medium mb-2">Node Type</label>
                <div class="space-y-2">
                    <label class="flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="local-node-type" value="full" class="w-4 h-4" checked>
                        <div class="flex-1">
                            <div class="font-semibold">Full Node</div>
                            <div class="text-xs text-gray-400">Sync entire blockchain, run RPC</div>
                        </div>
                    </label>
                    <label class="flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="local-node-type" value="validator" class="w-4 h-4">
                        <div class="flex-1">
                            <div class="font-semibold">Validator</div>
                            <div class="text-xs text-gray-400">Participate in consensus (requires stake)</div>
                        </div>
                    </label>
                </div>
            </div>

            <!-- Setup Script -->
            <div class="bg-etrid-dark border border-white/10 rounded-lg p-6">
                <div class="flex justify-between items-start mb-4">
                    <h4 class="font-semibold">One-Line Installation</h4>
                    <button id="copy-install-script" class="text-etrid-blue hover:text-etrid-purple text-sm">Copy</button>
                </div>
                <pre id="install-script" class="text-sm text-gray-300 overflow-x-auto bg-black/30 p-4 rounded"><code>curl -sSL https://get.etrid.org/node | bash</code></pre>
            </div>

            <!-- Action Buttons -->
            <div class="flex gap-3 pt-4">
                <button onclick="closeWizard()" class="flex-1 px-6 py-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-colors font-medium">
                    Close
                </button>
                <button id="download-setup-script" class="flex-1 px-6 py-3 rounded-lg bg-gradient-to-r from-etrid-blue to-etrid-purple hover:opacity-90 transition-opacity font-medium">
                    Download Full Setup Script →
                </button>
            </div>
        </div>
    `;
}

function setupConfigurationHandlers() {
    if (selectedNodeType === 'local') {
        // Copy install script
        document.getElementById('copy-install-script')?.addEventListener('click', () => {
            const script = document.getElementById('install-script').textContent;
            navigator.clipboard.writeText(script);
            showNotification('Install script copied!', 'success');
        });

        // Download full setup script
        document.getElementById('download-setup-script')?.addEventListener('click', () => {
            downloadLocalSetupScript();
        });

        // OS selection
        document.querySelectorAll('.os-option').forEach(btn => {
            btn.addEventListener('click', () => {
                document.querySelectorAll('.os-option').forEach(b => {
                    b.classList.remove('border-etrid-blue');
                    b.classList.add('border-white/10');
                });
                btn.classList.add('border-etrid-blue');
                btn.classList.remove('border-white/10');
            });
        });

        return;
    }

    // Provider selection
    document.querySelectorAll('.provider-option').forEach(btn => {
        btn.addEventListener('click', () => {
            document.querySelectorAll('.provider-option').forEach(b => {
                b.classList.remove('border-etrid-blue');
                b.classList.add('border-white/10');
            });
            btn.classList.add('border-etrid-blue');
            btn.classList.remove('border-white/10');
            deploymentConfig.provider = btn.dataset.provider;
        });
    });

    // Next button
    document.getElementById('next-to-payment')?.addEventListener('click', () => {
        // Validate configuration
        const nodeName = document.getElementById('node-name').value;
        const region = document.getElementById('region-select').value;
        const instanceType = document.querySelector('input[name="instance-type"]:checked')?.value;

        if (!nodeName || !region || !instanceType || !deploymentConfig.provider) {
            showNotification('Please complete all fields', 'error');
            return;
        }

        deploymentConfig.nodeName = nodeName;
        deploymentConfig.region = region;
        deploymentConfig.instanceType = instanceType;

        if (selectedNodeType === 'validator') {
            const stake = document.getElementById('validator-stake').value;
            if (!stake || stake < VALIDATOR_MIN_STAKE) {
                showNotification(`Validator requires minimum ${VALIDATOR_MIN_STAKE} ÉTR stake`, 'error');
                return;
            }
            deploymentConfig.stake = stake;
        }

        currentStep = 2;
        renderWizardStep();
    });
}

// Step 2: Payment
function renderPaymentStep() {
    const pricing = {
        validator: { standard: 500, 'high-performance': 800 },
        full: { standard: 200, 'high-performance': 350 }
    };

    const price = pricing[selectedNodeType][deploymentConfig.instanceType];
    const discount = walletConnected ? 0.10 : 0;
    const finalPrice = price * (1 - discount);

    // Get current ÉTR price for USD conversion
    const currentEtrPrice = window.etridPrice ? window.etridPrice.getPrice() : 2.45;
    const priceUSD = (price * currentEtrPrice).toFixed(2);
    const finalPriceUSD = (finalPrice * currentEtrPrice).toFixed(2);

    return `
        <div class="space-y-6">
            <div>
                <h3 class="text-xl font-bold mb-4">Payment</h3>
                <p class="text-gray-400 text-sm mb-6">Complete payment to deploy your node</p>
            </div>

            <!-- Order Summary -->
            <div class="bg-white/5 border border-white/10 rounded-lg p-6">
                <h4 class="font-semibold mb-4">Order Summary</h4>
                <div class="space-y-3 text-sm">
                    <div class="flex justify-between">
                        <span class="text-gray-400">Node Type</span>
                        <span class="font-medium">${selectedNodeType === 'validator' ? 'Validator' : 'Full Node'}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">Instance</span>
                        <span class="font-medium">${deploymentConfig.instanceType === 'standard' ? 'Standard' : 'High Performance'}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">Region</span>
                        <span class="font-medium">${deploymentConfig.region}</span>
                    </div>
                    <div class="flex justify-between">
                        <span class="text-gray-400">Billing</span>
                        <span class="font-medium">Monthly</span>
                    </div>
                    ${walletConnected ? `
                    <div class="flex justify-between text-green-400">
                        <span>ÉTR Payment Discount</span>
                        <span>-10% (-${(price * 0.10).toFixed(0)} ÉTR)</span>
                    </div>
                    ` : ''}
                    <div class="border-t border-white/10 pt-3">
                        <div class="flex justify-between text-lg font-bold mb-1">
                            <span>Total</span>
                            <span class="text-etrid-blue">${finalPrice} ÉTR/month</span>
                        </div>
                        <div class="flex justify-between text-sm text-gray-400">
                            <span></span>
                            <span data-etr-amount="${finalPrice}" data-format="usd">~$${finalPriceUSD}/month</span>
                        </div>
                    </div>
                </div>
            </div>

            ${!walletConnected ? `
            <div class="bg-yellow-500/10 border border-yellow-500/30 rounded-lg p-4">
                <p class="text-sm text-yellow-400">
                    💡 Connect your wallet to pay with ÉTR and get 10% discount!
                </p>
            </div>
            ` : ''}

            <!-- Payment Method -->
            <div>
                <label class="block text-sm font-medium mb-3">Payment Method</label>
                <div class="space-y-2">
                    ${walletConnected ? `
                    <label class="payment-method flex items-center gap-3 p-4 rounded-lg bg-gradient-to-r from-etrid-blue/10 to-etrid-purple/10 border-2 border-etrid-blue cursor-pointer">
                        <input type="radio" name="payment-method" value="etr-wallet" class="w-4 h-4" checked>
                        <div class="flex-1">
                            <div class="font-semibold">Pay with ÉTR (Connected Wallet)</div>
                            <div class="text-xs text-gray-400">${walletAddress?.substring(0, 10)}...${walletAddress?.substring(walletAddress.length - 8)}</div>
                        </div>
                        <div class="text-green-400 text-xs font-semibold">-10% DISCOUNT</div>
                    </label>
                    ` : ''}

                    <label class="payment-method flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 ${walletConnected ? 'border-white/10' : 'border-etrid-blue'} cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="payment-method" value="etr-qr" class="w-4 h-4" ${!walletConnected ? 'checked' : ''}>
                        <div class="flex-1">
                            <div class="font-semibold">Pay with ÉTR (QR Code)</div>
                            <div class="text-xs text-gray-400">Scan QR code with your wallet</div>
                        </div>
                    </label>

                    <label class="payment-method flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="payment-method" value="crypto" class="w-4 h-4">
                        <div class="flex-1">
                            <div class="font-semibold">Pay with Crypto</div>
                            <div class="text-xs text-gray-400">BTC, ETH, USDT, USDC</div>
                        </div>
                    </label>

                    <label class="payment-method flex items-center gap-3 p-4 rounded-lg bg-white/5 border-2 border-white/10 cursor-pointer hover:border-etrid-blue transition-all">
                        <input type="radio" name="payment-method" value="card" class="w-4 h-4">
                        <div class="flex-1">
                            <div class="font-semibold">Credit/Debit Card</div>
                            <div class="text-xs text-gray-400">Visa, Mastercard, Amex</div>
                        </div>
                    </label>
                </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex gap-3 pt-4">
                <button id="back-to-config" class="flex-1 px-6 py-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-colors font-medium">
                    ← Back
                </button>
                <button id="proceed-to-payment" class="flex-1 px-6 py-3 rounded-lg bg-gradient-to-r from-etrid-blue to-etrid-purple hover:opacity-90 transition-opacity font-medium">
                    Proceed to Payment →
                </button>
            </div>
        </div>
    `;
}

function setupPaymentHandlers() {
    document.getElementById('back-to-config')?.addEventListener('click', () => {
        currentStep = 1;
        renderWizardStep();
    });

    document.getElementById('proceed-to-payment')?.addEventListener('click', () => {
        const paymentMethod = document.querySelector('input[name="payment-method"]:checked')?.value;

        if (!paymentMethod) {
            showNotification('Please select a payment method', 'error');
            return;
        }

        deploymentConfig.paymentMethod = paymentMethod;
        processPayment(paymentMethod);
    });
}

async function processPayment(method) {
    switch (method) {
        case 'etr-wallet':
            await payWithConnectedWallet();
            break;
        case 'etr-qr':
            showQRPayment();
            break;
        case 'crypto':
            showCryptoPayment();
            break;
        case 'card':
            showCardPayment();
            break;
    }
}

async function payWithConnectedWallet() {
    try {
        showNotification('Processing payment...', 'info');

        // Simulate payment processing
        await new Promise(resolve => setTimeout(resolve, 2000));

        showNotification('Payment successful!', 'success');

        currentStep = 3;
        renderWizardStep();
    } catch (error) {
        showNotification('Payment failed. Please try again.', 'error');
    }
}

function showQRPayment() {
    const pricing = {
        validator: { standard: 500, 'high-performance': 800 },
        full: { standard: 200, 'high-performance': 350 }
    };

    const price = pricing[selectedNodeType][deploymentConfig.instanceType];
    const paymentAddress = 'etrid1qpzry9x8gf2tvdw0s3jn54khce6mua7lmu9qqqqqqqqqqqqqqqqqqqqqqqqq';

    const content = document.getElementById('wizard-content');
    content.innerHTML = `
        <div class="space-y-6 text-center">
            <h3 class="text-xl font-bold">Scan to Pay</h3>
            <p class="text-gray-400 text-sm">Send ${price} ÉTR to complete deployment</p>

            <!-- QR Code Placeholder -->
            <div class="bg-white p-8 rounded-xl inline-block mx-auto">
                <div class="w-64 h-64 bg-gray-200 flex items-center justify-center">
                    <div class="text-gray-600 text-sm">QR Code<br/>would appear here</div>
                </div>
            </div>

            <!-- Payment Address -->
            <div class="bg-white/5 border border-white/10 rounded-lg p-4">
                <div class="text-sm text-gray-400 mb-2">Payment Address</div>
                <div class="flex items-center gap-2">
                    <code class="flex-1 text-sm font-mono break-all">${paymentAddress}</code>
                    <button onclick="navigator.clipboard.writeText('${paymentAddress}')" class="text-etrid-blue hover:text-etrid-purple">📋</button>
                </div>
            </div>

            <div class="bg-blue-500/10 border border-blue-500/30 rounded-lg p-4">
                <p class="text-sm text-blue-400">
                    ⏱️ Waiting for payment confirmation...<br/>
                    This usually takes 1-2 minutes.
                </p>
            </div>

            <button onclick="window.location.reload()" class="px-6 py-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-colors">
                Cancel
            </button>
        </div>
    `;

    // Simulate payment detection
    setTimeout(() => {
        showNotification('Payment received!', 'success');
        setTimeout(() => {
            currentStep = 3;
            renderWizardStep();
        }, 1000);
    }, 5000);
}

function showCryptoPayment() {
    showNotification('Crypto payment gateway loading...', 'info');
    setTimeout(() => {
        showNotification('Payment successful!', 'success');
        currentStep = 3;
        renderWizardStep();
    }, 3000);
}

function showCardPayment() {
    showNotification('Card payment gateway loading...', 'info');
    setTimeout(() => {
        showNotification('Payment successful!', 'success');
        currentStep = 3;
        renderWizardStep();
    }, 3000);
}

// Step 3: Deployment
function renderDeploymentStep() {
    return `
        <div class="space-y-6 text-center">
            <div class="w-20 h-20 rounded-full bg-gradient-to-r from-etrid-blue to-etrid-purple mx-auto flex items-center justify-center text-4xl animate-pulse">
                🚀
            </div>

            <h3 class="text-2xl font-bold">Deploying Your Node...</h3>
            <p class="text-gray-400">This will take 2-3 minutes</p>

            <!-- Progress Steps -->
            <div class="max-w-md mx-auto space-y-3 text-left">
                <div id="deploy-step-1" class="deployment-step flex items-center gap-3 p-3 rounded-lg bg-white/5">
                    <div class="w-6 h-6 rounded-full bg-green-500 flex items-center justify-center text-xs">✓</div>
                    <span class="text-sm">Payment confirmed</span>
                </div>
                <div id="deploy-step-2" class="deployment-step flex items-center gap-3 p-3 rounded-lg bg-white/5">
                    <div class="w-6 h-6 rounded-full bg-etrid-blue flex items-center justify-center">
                        <div class="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                    </div>
                    <span class="text-sm">Provisioning VM instance...</span>
                </div>
                <div id="deploy-step-3" class="deployment-step flex items-center gap-3 p-3 rounded-lg bg-white/10 opacity-50">
                    <div class="w-6 h-6 rounded-full bg-white/20"></div>
                    <span class="text-sm">Installing ËTRID node software...</span>
                </div>
                <div id="deploy-step-4" class="deployment-step flex items-center gap-3 p-3 rounded-lg bg-white/10 opacity-50">
                    <div class="w-6 h-6 rounded-full bg-white/20"></div>
                    <span class="text-sm">Bootstrapping node...</span>
                </div>
                <div id="deploy-step-5" class="deployment-step flex items-center gap-3 p-3 rounded-lg bg-white/10 opacity-50">
                    <div class="w-6 h-6 rounded-full bg-white/20"></div>
                    <span class="text-sm">Starting blockchain sync...</span>
                </div>
            </div>

            <div id="deployment-complete" class="hidden">
                <div class="bg-green-500/10 border border-green-500/30 rounded-lg p-6 mb-6">
                    <h4 class="text-xl font-bold text-green-400 mb-2">🎉 Deployment Successful!</h4>
                    <p class="text-sm text-gray-300">Your node is now running and syncing with the network</p>
                </div>

                <div class="bg-white/5 border border-white/10 rounded-lg p-6 text-left mb-6">
                    <h5 class="font-semibold mb-4">Node Details</h5>
                    <div class="space-y-2 text-sm">
                        <div class="flex justify-between">
                            <span class="text-gray-400">Node ID</span>
                            <code class="font-mono">etrid-${Math.random().toString(36).substring(7)}</code>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-gray-400">IP Address</span>
                            <code class="font-mono">${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}</code>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-gray-400">RPC Endpoint</span>
                            <code class="font-mono">wss://node-${Math.random().toString(36).substring(7)}.etrid.org:9944</code>
                        </div>
                        <div class="flex justify-between">
                            <span class="text-gray-400">Status</span>
                            <span class="text-green-400">● Syncing</span>
                        </div>
                    </div>
                </div>

                <div class="flex gap-3">
                    <button onclick="window.open('https://dashboard.etrid.org', '_blank')" class="flex-1 px-6 py-3 rounded-lg bg-gradient-to-r from-etrid-blue to-etrid-purple hover:opacity-90 transition-opacity font-medium">
                        Open Dashboard
                    </button>
                    <button onclick="closeWizard()" class="flex-1 px-6 py-3 rounded-lg bg-white/5 border border-white/10 hover:bg-white/10 transition-colors font-medium">
                        Done
                    </button>
                </div>
            </div>
        </div>
    `;
}

function setupDeploymentHandlers() {
    // Simulate deployment progress
    simulateDeployment();
}

async function simulateDeployment() {
    const steps = [
        { id: 'deploy-step-2', duration: 30000 },
        { id: 'deploy-step-3', duration: 40000 },
        { id: 'deploy-step-4', duration: 30000 },
        { id: 'deploy-step-5', duration: 20000 },
    ];

    for (const step of steps) {
        await new Promise(resolve => setTimeout(resolve, step.duration));

        const stepEl = document.getElementById(step.id);
        stepEl.classList.remove('opacity-50');
        stepEl.querySelector('.w-6').innerHTML = '<div class="text-xs">✓</div>';
        stepEl.querySelector('.w-6').classList.remove('bg-white/20', 'bg-etrid-blue');
        stepEl.querySelector('.w-6').classList.add('bg-green-500');
    }

    // Show completion
    document.getElementById('deployment-complete').classList.remove('hidden');
}

// Local Setup Script Download
function downloadLocalSetupScript() {
    const os = document.querySelector('.os-option.border-etrid-blue')?.dataset.os || 'linux';
    const installMethod = document.querySelector('input[name="install-method"]:checked')?.value || 'docker';
    const nodeType = document.querySelector('input[name="local-node-type"]:checked')?.value || 'full';

    const script = generateSetupScript(os, installMethod, nodeType);

    const blob = new Blob([script], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `etrid-node-setup-${os}.sh`;
    a.click();
    URL.revokeObjectURL(url);

    showNotification('Setup script downloaded!', 'success');
}

function generateSetupScript(os, method, nodeType) {
    // This would generate the actual setup script based on OS and method
    return `#!/bin/bash
# ËTRID Node Setup Script
# OS: ${os}
# Method: ${method}
# Node Type: ${nodeType}

set -e

echo "🚀 Setting up ËTRID ${nodeType} node..."

# Add actual setup commands here
# This is a placeholder

echo "✅ Setup complete!"
echo "Run: systemctl start etrid-node"
`;
}

// Utility Functions
function showNotification(message, type = 'info') {
    const colors = {
        success: 'bg-green-500',
        error: 'bg-red-500',
        info: 'bg-blue-500',
        warning: 'bg-yellow-500'
    };

    const notification = document.createElement('div');
    notification.className = `fixed top-20 right-4 ${colors[type]} text-white px-6 py-3 rounded-lg shadow-lg z-50 animate-fade-in`;
    notification.textContent = message;

    document.body.appendChild(notification);

    setTimeout(() => {
        notification.classList.add('animate-fade-out');
        setTimeout(() => notification.remove(), 300);
    }, 3000);
}

// Rewards Calculator
function setupRewardsCalculator() {
    const stakeInput = document.getElementById('stake-amount');

    if (!stakeInput) return;

    function calculateRewards() {
        const stakeAmount = parseFloat(stakeInput.value) || VALIDATOR_MIN_STAKE;

        // Ensure minimum stake
        if (stakeAmount < VALIDATOR_MIN_STAKE) {
            stakeInput.value = VALIDATOR_MIN_STAKE;
            return;
        }

        // Calculate rewards
        const dailyReward = (stakeAmount * APY_RATE) / 365;
        const monthlyReward = dailyReward * 30;
        const annualReward = stakeAmount * APY_RATE;

        // Update displays
        document.getElementById('daily-reward').textContent = `${dailyReward.toFixed(2)} ÉTR`;
        document.getElementById('monthly-reward').textContent = `${monthlyReward.toLocaleString(undefined, {maximumFractionDigits: 0})} ÉTR`;
        document.getElementById('annual-reward').textContent = `${annualReward.toLocaleString(undefined, {maximumFractionDigits: 0})} ÉTR`;

        // Update USD values using price tracker
        const dailyRewardEl = document.getElementById('daily-reward-usd');
        const monthlyRewardEl = document.getElementById('monthly-reward-usd');
        const annualRewardEl = document.getElementById('annual-reward-usd');

        if (dailyRewardEl) dailyRewardEl.dataset.etrAmount = dailyReward.toFixed(2);
        if (monthlyRewardEl) monthlyRewardEl.dataset.etrAmount = monthlyReward.toFixed(0);
        if (annualRewardEl) annualRewardEl.dataset.etrAmount = annualReward.toFixed(0);

        // Trigger price update
        if (window.etridPrice) {
            const price = window.etridPrice.getPrice();
            if (dailyRewardEl) dailyRewardEl.textContent = `~${window.formatUsd(dailyReward * price)}`;
            if (monthlyRewardEl) monthlyRewardEl.textContent = `~${window.formatUsd(monthlyReward * price)}`;
            if (annualRewardEl) annualRewardEl.textContent = `~${window.formatUsd(annualReward * price)}`;
        }
    }

    // Listen for input changes
    stakeInput.addEventListener('input', calculateRewards);

    // Listen for price updates
    window.addEventListener('etrPriceUpdate', calculateRewards);

    // Initial calculation
    calculateRewards();
}

// Initialize rewards calculator when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setupRewardsCalculator);
} else {
    setupRewardsCalculator();
}
