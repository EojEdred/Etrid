// ËTRID Ecosystem Directory
// Displays projects building on ËTRID blockchain

const PROJECTS = [
    // DeFi Projects
    {
        name: 'ÉtriSwap',
        category: 'defi',
        description: 'Cross-chain DEX aggregator leveraging ËTRID\'s Partition Burst Chains for optimal liquidity routing',
        status: 'Live',
        tvl: '$4.2M',
        chains: ['Ethereum', 'BSC', 'Polygon'],
        website: 'https://etriswap.io',
        logo: '🔄'
    },
    {
        name: 'Vault Protocol',
        category: 'defi',
        description: 'Multi-chain yield aggregator with automated strategy optimization across 13 blockchains',
        status: 'Live',
        tvl: '$2.8M',
        chains: ['Ethereum', 'Avalanche', 'Fantom'],
        website: 'https://vaultprotocol.io',
        logo: '🏦'
    },
    {
        name: 'ËTRID Lending',
        category: 'defi',
        description: 'Cross-chain lending and borrowing protocol with shared liquidity pools',
        status: 'Beta',
        tvl: '$1.5M',
        chains: ['ËTRID', 'Ethereum', 'BSC'],
        website: 'https://lending.etrid.org',
        logo: '💰'
    },
    {
        name: 'ChainBridge Finance',
        category: 'defi',
        description: 'Decentralized bridge aggregator for seamless asset transfers between chains',
        status: 'Live',
        tvl: '$3.1M',
        chains: ['All 13 Chains'],
        website: 'https://chainbridge.fi',
        logo: '🌉'
    },

    // NFT Projects
    {
        name: 'MultiNFT Marketplace',
        category: 'nft',
        description: 'Cross-chain NFT marketplace supporting minting and trading across multiple blockchains',
        status: 'Live',
        chains: ['Ethereum', 'Polygon', 'Solana'],
        website: 'https://multinft.market',
        logo: '🖼️'
    },
    {
        name: 'MetaVerse Land',
        category: 'nft',
        description: 'Virtual land NFTs with cross-chain interoperability and 3D metaverse integration',
        status: 'Beta',
        chains: ['ËTRID', 'Polygon'],
        website: 'https://metaverseland.io',
        logo: '🏝️'
    },
    {
        name: 'ChainCollectibles',
        category: 'nft',
        description: 'NFT collections deployed across multiple chains with unified ownership tracking',
        status: 'Live',
        chains: ['Ethereum', 'BSC', 'Avalanche'],
        website: 'https://chaincollectibles.art',
        logo: '🎨'
    },

    // Gaming
    {
        name: 'Cosmic Warriors',
        category: 'gaming',
        description: 'Play-to-earn RPG with cross-chain asset ownership and multi-chain tournaments',
        status: 'Alpha',
        chains: ['ËTRID', 'Polygon', 'BSC'],
        website: 'https://cosmicwarriors.gg',
        logo: '⚔️'
    },
    {
        name: 'Racing Realms',
        category: 'gaming',
        description: 'NFT-based racing game with cross-chain vehicle trading and staking rewards',
        status: 'Beta',
        chains: ['Polygon', 'Avalanche'],
        website: 'https://racingrealms.io',
        logo: '🏎️'
    },
    {
        name: 'BlockQuest',
        category: 'gaming',
        description: 'Blockchain-based adventure game with cross-chain item marketplace',
        status: 'Development',
        chains: ['ËTRID', 'Ethereum'],
        website: 'https://blockquest.gg',
        logo: '🎮'
    },

    // Infrastructure
    {
        name: 'ËTRID RPC Network',
        category: 'infrastructure',
        description: 'Decentralized RPC infrastructure providing high-performance blockchain access',
        status: 'Live',
        chains: ['All 13 Chains'],
        website: 'https://rpc.etrid.org',
        logo: '🔌'
    },
    {
        name: 'Oracle Bridge',
        category: 'infrastructure',
        description: 'Cross-chain oracle network for reliable off-chain data feeds',
        status: 'Live',
        chains: ['ËTRID', 'Ethereum', 'BSC', 'Polygon'],
        website: 'https://oraclebridge.network',
        logo: '📡'
    },
    {
        name: 'ChainIndex',
        category: 'infrastructure',
        description: 'Multi-chain blockchain indexer and query API for developers',
        status: 'Beta',
        chains: ['All 13 Chains'],
        website: 'https://chainindex.dev',
        logo: '📊'
    },
    {
        name: 'ValidatorHub',
        category: 'infrastructure',
        description: 'Validator monitoring and management platform for ËTRID network',
        status: 'Live',
        chains: ['ËTRID'],
        website: 'https://validator.etrid.org',
        logo: '🛡️'
    },

    // DAOs
    {
        name: 'ËTRID DAO',
        category: 'dao',
        description: 'Main governance DAO for ËTRID protocol decisions and treasury management',
        status: 'Live',
        chains: ['ËTRID'],
        website: 'https://dao.etrid.org',
        logo: '🏛️'
    },
    {
        name: 'DeFi Alliance DAO',
        category: 'dao',
        description: 'Collective of DeFi protocols coordinating cross-chain liquidity strategies',
        status: 'Live',
        chains: ['ËTRID', 'Ethereum'],
        website: 'https://defialliance.dao',
        logo: '🤝'
    },
    {
        name: 'Validator Coalition',
        category: 'dao',
        description: 'DAO for validator coordination and network upgrade proposals',
        status: 'Live',
        chains: ['ËTRID'],
        website: 'https://validators.etrid.org/dao',
        logo: '⚡'
    },

    // Social
    {
        name: 'ChainSocial',
        category: 'social',
        description: 'Decentralized social network with cross-chain identity and content ownership',
        status: 'Beta',
        chains: ['ËTRID', 'Polygon'],
        website: 'https://chainsocial.app',
        logo: '💬'
    },
    {
        name: 'CreatorDAO',
        category: 'social',
        description: 'Platform for creators to monetize content across multiple blockchain networks',
        status: 'Alpha',
        chains: ['ËTRID', 'Ethereum', 'Polygon'],
        website: 'https://creatordao.network',
        logo: '✨'
    },
];

let currentFilter = 'all';

// Render projects grid
function renderProjects() {
    const grid = document.getElementById('projects-grid');
    const filteredProjects = currentFilter === 'all'
        ? PROJECTS
        : PROJECTS.filter(p => p.category === currentFilter);

    if (filteredProjects.length === 0) {
        grid.innerHTML = `
            <div class="col-span-full text-center py-12">
                <p class="text-gray-400 text-lg">No projects found in this category</p>
                <p class="text-gray-500 text-sm mt-2">Check back soon for new projects!</p>
            </div>
        `;
        return;
    }

    grid.innerHTML = filteredProjects.map(project => `
        <div class="bg-white/5 border border-white/10 rounded-xl p-6 hover:bg-white/10 hover:border-etrid-blue/50 transition-all">
            <!-- Logo and Status -->
            <div class="flex justify-between items-start mb-4">
                <div class="text-5xl">${project.logo}</div>
                <span class="px-3 py-1 rounded-full text-xs font-semibold ${getStatusColor(project.status)}">
                    ${project.status}
                </span>
            </div>

            <!-- Project Info -->
            <h3 class="text-xl font-display font-bold mb-2">${project.name}</h3>
            <p class="text-gray-400 text-sm mb-4 line-clamp-3">${project.description}</p>

            <!-- TVL (if DeFi) -->
            ${project.tvl ? `
                <div class="mb-3">
                    <span class="text-gray-400 text-xs">TVL: </span>
                    <span class="text-etrid-blue font-semibold">${project.tvl}</span>
                </div>
            ` : ''}

            <!-- Chains -->
            <div class="mb-4">
                <div class="text-gray-400 text-xs mb-2">Supported Chains:</div>
                <div class="flex flex-wrap gap-1">
                    ${project.chains.slice(0, 3).map(chain => `
                        <span class="px-2 py-1 rounded bg-etrid-purple/20 text-etrid-purple text-xs">${chain}</span>
                    `).join('')}
                    ${project.chains.length > 3 ? `<span class="px-2 py-1 rounded bg-white/10 text-gray-400 text-xs">+${project.chains.length - 3}</span>` : ''}
                </div>
            </div>

            <!-- Visit Button -->
            <a href="${project.website}" target="_blank" class="block w-full text-center px-4 py-2 rounded-lg bg-gradient-to-r from-etrid-blue to-etrid-purple hover:opacity-90 transition-opacity text-sm font-medium">
                Visit Project →
            </a>
        </div>
    `).join('');
}

// Get status badge color
function getStatusColor(status) {
    switch (status) {
        case 'Live': return 'bg-green-500/20 text-green-400';
        case 'Beta': return 'bg-blue-500/20 text-blue-400';
        case 'Alpha': return 'bg-yellow-500/20 text-yellow-400';
        case 'Development': return 'bg-purple-500/20 text-purple-400';
        default: return 'bg-gray-500/20 text-gray-400';
    }
}

// Update ecosystem stats
function updateStats() {
    const totalProjects = PROJECTS.length;
    const activeProjects = PROJECTS.filter(p => p.status === 'Live' || p.status === 'Beta').length;
    const totalTVL = PROJECTS.reduce((sum, p) => {
        if (p.tvl) {
            const value = parseFloat(p.tvl.replace('$', '').replace('M', ''));
            return sum + value;
        }
        return sum;
    }, 0);

    document.getElementById('total-projects').textContent = totalProjects;
    document.getElementById('active-projects').textContent = activeProjects;
    document.getElementById('total-tvl').textContent = `$${totalTVL.toFixed(1)}M`;
}

// Category filter handlers
document.querySelectorAll('.category-filter').forEach(button => {
    button.addEventListener('click', () => {
        // Update active state
        document.querySelectorAll('.category-filter').forEach(btn => {
            btn.classList.remove('active', 'bg-etrid-blue/20', 'text-etrid-blue', 'border-etrid-blue/30');
            btn.classList.add('bg-white/5', 'text-gray-400');
        });

        button.classList.add('active', 'bg-etrid-blue/20', 'text-etrid-blue', 'border-etrid-blue/30');
        button.classList.remove('bg-white/5', 'text-gray-400');

        // Update filter and re-render
        currentFilter = button.dataset.category;
        renderProjects();
    });
});

// Initialize
renderProjects();
updateStats();
