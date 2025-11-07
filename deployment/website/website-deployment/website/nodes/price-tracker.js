// ËTRID Price Tracker - Global Price Syncing Utility
// Ensures all prices across the site stay synchronized with live ÉTR price

class EtridPriceTracker {
    constructor() {
        this.currentPrice = 2.45; // Default fallback
        this.priceChangePercent = 0;
        this.lastUpdate = null;
        this.updateInterval = null;
        this.listeners = [];

        // CoinGecko API endpoint
        this.apiEndpoint = 'https://api.coingecko.com/api/v3/simple/price';
        this.tokenId = 'etrid'; // Replace with actual CoinGecko ID when listed

        this.init();
    }

    async init() {
        // Fetch initial price
        await this.fetchPrice();

        // Update every 30 seconds
        this.updateInterval = setInterval(() => this.fetchPrice(), 30000);

        // Update on page visibility change
        document.addEventListener('visibilitychange', () => {
            if (!document.hidden) {
                this.fetchPrice();
            }
        });
    }

    async fetchPrice() {
        try {
            // Uncomment when ÉTR is listed on CoinGecko
            /*
            const response = await fetch(
                `${this.apiEndpoint}?ids=${this.tokenId}&vs_currencies=usd&include_24hr_change=true`
            );
            const data = await response.json();

            if (data[this.tokenId]) {
                this.currentPrice = data[this.tokenId].usd;
                this.priceChangePercent = data[this.tokenId].usd_24h_change || 0;
                this.lastUpdate = new Date();
                this.notifyListeners();
            }
            */

            // Mock data for now (remove when live)
            // Simulate price fluctuation
            const mockChange = (Math.random() - 0.5) * 0.1; // ±5%
            this.currentPrice = 2.45 + mockChange;
            this.priceChangePercent = (Math.random() - 0.5) * 10; // ±5%
            this.lastUpdate = new Date();
            this.notifyListeners();

        } catch (error) {
            console.error('Error fetching ÉTR price:', error);
            // Keep using last known price
        }
    }

    // Subscribe to price updates
    onPriceUpdate(callback) {
        this.listeners.push(callback);
        // Immediately call with current price
        callback(this.currentPrice, this.priceChangePercent);
    }

    // Notify all listeners of price change
    notifyListeners() {
        this.listeners.forEach(callback => {
            callback(this.currentPrice, this.priceChangePercent);
        });
    }

    // Convert ÉTR to USD
    etrToUsd(etrAmount) {
        return (etrAmount * this.currentPrice).toFixed(2);
    }

    // Convert USD to ÉTR
    usdToEtr(usdAmount) {
        return (usdAmount / this.currentPrice).toFixed(2);
    }

    // Format ÉTR amount
    formatEtr(amount) {
        return `${Number(amount).toLocaleString()} ÉTR`;
    }

    // Format USD amount
    formatUsd(amount) {
        return `$${Number(amount).toLocaleString()}`;
    }

    // Get current price
    getPrice() {
        return this.currentPrice;
    }

    // Get price change
    getPriceChange() {
        return this.priceChangePercent;
    }

    // Cleanup
    destroy() {
        if (this.updateInterval) {
            clearInterval(this.updateInterval);
        }
        this.listeners = [];
    }
}

// Create global instance
window.etridPrice = new EtridPriceTracker();

// Utility functions for easy access
window.etrToUsd = (amount) => window.etridPrice.etrToUsd(amount);
window.usdToEtr = (amount) => window.etridPrice.usdToEtr(amount);
window.formatEtr = (amount) => window.etridPrice.formatEtr(amount);
window.formatUsd = (amount) => window.etridPrice.formatUsd(amount);

// Auto-update all price elements on the page
function updatePriceElements() {
    // Update elements with data-etr-amount attribute
    document.querySelectorAll('[data-etr-amount]').forEach(el => {
        const etrAmount = parseFloat(el.dataset.etrAmount);
        const usdAmount = window.etridPrice.etrToUsd(etrAmount);

        if (el.dataset.format === 'both') {
            el.textContent = `${window.formatEtr(etrAmount)} (~${window.formatUsd(usdAmount)})`;
        } else if (el.dataset.format === 'usd') {
            el.textContent = window.formatUsd(usdAmount);
        } else {
            el.textContent = window.formatEtr(etrAmount);
        }
    });

    // Update elements with data-usd-amount attribute
    document.querySelectorAll('[data-usd-amount]').forEach(el => {
        const usdAmount = parseFloat(el.dataset.usdAmount);
        const etrAmount = window.etridPrice.usdToEtr(usdAmount);

        if (el.dataset.format === 'both') {
            el.textContent = `${window.formatUsd(usdAmount)} (~${window.formatEtr(etrAmount)})`;
        } else if (el.dataset.format === 'etr') {
            el.textContent = window.formatEtr(etrAmount);
        } else {
            el.textContent = window.formatUsd(usdAmount);
        }
    });

    // Update current price displays
    document.querySelectorAll('[data-price="etr-current"]').forEach(el => {
        el.textContent = `$${window.etridPrice.getPrice().toFixed(2)}`;
    });

    // Update price change displays
    document.querySelectorAll('[data-price="etr-change"]').forEach(el => {
        const change = window.etridPrice.getPriceChange();
        const isPositive = change >= 0;
        el.textContent = `${isPositive ? '+' : ''}${change.toFixed(2)}%`;
        el.className = `${el.className.split(' ')[0]} ${isPositive ? 'text-green-400' : 'text-red-400'}`;
    });
}

// Subscribe to price updates
window.etridPrice.onPriceUpdate((price, change) => {
    updatePriceElements();

    // Dispatch custom event for other components
    window.dispatchEvent(new CustomEvent('etrPriceUpdate', {
        detail: { price, change }
    }));
});

// Initial update when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', updatePriceElements);
} else {
    updatePriceElements();
}

// Export for use in other modules
export { EtridPriceTracker };
