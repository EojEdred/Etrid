// ËTRID Events Calendar
// Displays upcoming and past community events

const EVENTS = [
    {
        title: 'Developer Workshop: Building DeFi on ËTRID',
        type: 'workshop',
        date: '2025-11-15',
        endDate: '2025-11-15',
        location: 'Online',
        attendees: '200',
        icon: '💻',
        description: 'Learn to build decentralized finance applications on ËTRID. Hands-on workshop with live coding.',
        url: 'https://events.etrid.org/defi-workshop'
    },
    {
        title: 'ËTRID Berlin Meetup',
        type: 'meetup',
        date: '2025-11-20',
        endDate: '2025-11-20',
        location: 'Berlin, Germany',
        attendees: '150',
        icon: '🍺',
        description: 'Monthly community meetup with presentations, networking, and drinks. All levels welcome!',
        url: 'https://meetup.com/etrid-berlin'
    },
    {
        title: 'Winter Hackathon 2025',
        type: 'hackathon',
        date: '2025-12-01',
        endDate: '2025-12-03',
        location: 'Online',
        attendees: '500+',
        icon: '🏆',
        description: '72-hour cross-chain hackathon with $150K in prizes. Build the future of multichain apps!',
        url: 'https://hack.etrid.org/winter-2025'
    },
    {
        title: 'Cross-Chain Security Workshop',
        type: 'workshop',
        date: '2025-12-10',
        endDate: '2025-12-10',
        location: 'Online',
        attendees: '100',
        icon: '🔐',
        description: 'Learn security best practices for cross-chain smart contracts and bridge protocols.',
        url: 'https://events.etrid.org/security-workshop'
    },
    {
        title: 'ËTRID at ETHDenver 2026',
        type: 'conference',
        date: '2026-02-20',
        endDate: '2026-02-23',
        location: 'Denver, Colorado',
        attendees: '1,000+',
        icon: '⛷️',
        description: 'Join the ËTRID team at ETHDenver! Special booth, side events, and networking opportunities.',
        url: 'https://ethdenver.com'
    },
    {
        title: 'Singapore Developer Meetup',
        type: 'meetup',
        date: '2026-01-15',
        endDate: '2026-01-15',
        location: 'Singapore',
        attendees: '200',
        icon: '🦁',
        description: 'Connect with ËTRID developers in Southeast Asia. Technical talks and networking.',
        url: 'https://meetup.com/etrid-singapore'
    },
    {
        title: 'Consensus Day 2026',
        type: 'conference',
        date: '2026-03-15',
        endDate: '2026-03-17',
        location: 'Austin, Texas',
        attendees: '5,000+',
        icon: '🎉',
        description: 'Annual governance event for voting on fiscal policy and network upgrades. Three days of talks, workshops, and networking.',
        url: 'https://consensus.etrid.org'
    },
    {
        title: 'Paris Blockchain Week - ËTRID Day',
        type: 'conference',
        date: '2026-04-10',
        endDate: '2026-04-11',
        location: 'Paris, France',
        attendees: '800',
        icon: '🗼',
        description: 'Full-day ËTRID track at Paris Blockchain Week featuring ecosystem projects and technical deep dives.',
        url: 'https://pbw.etrid.org'
    },
    {
        title: 'NFT Workshop: Cross-Chain Collections',
        type: 'workshop',
        date: '2026-01-25',
        endDate: '2026-01-25',
        location: 'Online',
        attendees: '150',
        icon: '🖼️',
        description: 'Build NFT collections that work across multiple blockchains using ËTRID infrastructure.',
        url: 'https://events.etrid.org/nft-workshop'
    },
    {
        title: 'Tokyo Blockchain Summit',
        type: 'conference',
        date: '2026-05-05',
        endDate: '2026-05-07',
        location: 'Tokyo, Japan',
        attendees: '2,000+',
        icon: '🗾',
        description: 'ËTRID keynote and exhibition booth at Japan\'s largest blockchain conference.',
        url: 'https://tokyo-blockchain-summit.jp'
    },
    {
        title: 'NYC Validator Meetup',
        type: 'meetup',
        date: '2025-12-15',
        endDate: '2025-12-15',
        location: 'New York City',
        attendees: '75',
        icon: '🗽',
        description: 'Meetup for current and aspiring validators. Discuss network performance, staking strategies, and infrastructure.',
        url: 'https://meetup.com/etrid-nyc'
    },
    {
        title: 'Spring DeFi Hackathon',
        type: 'hackathon',
        date: '2026-04-20',
        endDate: '2026-04-22',
        location: 'Hybrid',
        attendees: '600+',
        icon: '🌸',
        description: 'Build innovative DeFi protocols on ËTRID. $200K prize pool with categories for lending, DEXs, and derivatives.',
        url: 'https://hack.etrid.org/spring-2026'
    },
];

let currentFilter = 'all';

// Render upcoming events
function renderEvents() {
    const container = document.getElementById('events-grid');
    const today = new Date();

    // Filter events
    let filteredEvents = EVENTS.filter(event => new Date(event.date) >= today);

    if (currentFilter !== 'all') {
        filteredEvents = filteredEvents.filter(e => e.type === currentFilter);
    }

    // Sort by date
    filteredEvents.sort((a, b) => new Date(a.date) - new Date(b.date));

    if (filteredEvents.length === 0) {
        container.innerHTML = `
            <div class="text-center py-12">
                <p class="text-gray-400 text-lg">No upcoming ${currentFilter === 'all' ? '' : currentFilter} events</p>
                <p class="text-gray-500 text-sm mt-2">Check back soon for new events!</p>
            </div>
        `;
        return;
    }

    container.innerHTML = filteredEvents.map(event => `
        <div class="bg-white/5 border border-white/10 rounded-xl p-6 hover:bg-white/10 hover:border-etrid-blue/50 transition-all">
            <div class="flex items-start gap-6">
                <!-- Event Icon & Date -->
                <div class="flex-shrink-0">
                    <div class="w-16 h-16 rounded-xl bg-gradient-to-br from-etrid-blue to-etrid-purple flex items-center justify-center text-3xl mb-2">
                        ${event.icon}
                    </div>
                    <div class="text-center">
                        <div class="text-xs text-gray-400">${formatMonth(event.date)}</div>
                        <div class="text-xl font-bold">${formatDay(event.date)}</div>
                    </div>
                </div>

                <!-- Event Details -->
                <div class="flex-1">
                    <div class="flex items-start justify-between mb-2">
                        <div>
                            <h3 class="text-xl font-bold mb-1">${event.title}</h3>
                            <div class="flex flex-wrap gap-2 mb-3">
                                <span class="px-2 py-1 rounded-full text-xs font-semibold ${getTypeColor(event.type)}">
                                    ${event.type.toUpperCase()}
                                </span>
                                ${event.location === 'Online' ?
                                    '<span class="px-2 py-1 rounded-full bg-blue-500/20 text-blue-400 text-xs font-semibold">ONLINE</span>' :
                                    ''
                                }
                            </div>
                        </div>
                    </div>

                    <p class="text-gray-400 text-sm mb-4">${event.description}</p>

                    <div class="flex flex-wrap items-center gap-4 text-sm text-gray-300">
                        <div class="flex items-center gap-2">
                            <span class="text-etrid-blue">📅</span>
                            <span>${formatDateRange(event.date, event.endDate)}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-etrid-purple">📍</span>
                            <span>${event.location}</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-etrid-blue">👥</span>
                            <span>${event.attendees} attendees</span>
                        </div>
                    </div>

                    <div class="mt-4">
                        <a href="${event.url}" target="_blank" class="inline-block px-6 py-2 rounded-lg bg-etrid-blue/20 text-etrid-blue hover:bg-etrid-blue/30 transition-colors text-sm font-medium">
                            Register / Learn More →
                        </a>
                    </div>
                </div>
            </div>
        </div>
    `).join('');
}

// Event type color coding
function getTypeColor(type) {
    switch (type) {
        case 'conference': return 'bg-purple-500/20 text-purple-400';
        case 'hackathon': return 'bg-green-500/20 text-green-400';
        case 'meetup': return 'bg-blue-500/20 text-blue-400';
        case 'workshop': return 'bg-yellow-500/20 text-yellow-400';
        case 'online': return 'bg-cyan-500/20 text-cyan-400';
        default: return 'bg-gray-500/20 text-gray-400';
    }
}

// Date formatting functions
function formatMonth(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', { month: 'short' }).toUpperCase();
}

function formatDay(dateString) {
    const date = new Date(dateString);
    return date.getDate();
}

function formatDateRange(startDate, endDate) {
    const start = new Date(startDate);
    const end = new Date(endDate);

    if (startDate === endDate) {
        return start.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
    } else {
        const startStr = start.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
        const endStr = end.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
        return `${startStr} - ${endStr}`;
    }
}

// Filter button handlers
document.querySelectorAll('.event-filter').forEach(button => {
    button.addEventListener('click', () => {
        // Update active state
        document.querySelectorAll('.event-filter').forEach(btn => {
            btn.classList.remove('active', 'bg-etrid-blue/20', 'text-etrid-blue', 'border-etrid-blue/30');
            btn.classList.add('bg-white/5', 'text-gray-400');
        });

        button.classList.add('active', 'bg-etrid-blue/20', 'text-etrid-blue', 'border-etrid-blue/30');
        button.classList.remove('bg-white/5', 'text-gray-400');

        // Update filter
        currentFilter = button.dataset.type;
        renderEvents();
    });
});

// Initialize
renderEvents();
