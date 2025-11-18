-- Ëtrid Mobile Wallet Backend Database Schema
-- PostgreSQL 14+

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm"; -- For text search optimization

-- ============================================================================
-- USERS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    address VARCHAR(48) UNIQUE NOT NULL, -- SS58 address
    email VARCHAR(255) UNIQUE,
    phone VARCHAR(20) UNIQUE,
    kyc_status VARCHAR(20) DEFAULT 'pending' CHECK (kyc_status IN ('pending', 'verified', 'rejected')),
    kyc_level INTEGER DEFAULT 0 CHECK (kyc_level IN (0, 1, 2)), -- 0=none, 1=basic, 2=full
    kyc_submitted_at TIMESTAMP,
    kyc_verified_at TIMESTAMP,
    email_verified BOOLEAN DEFAULT FALSE,
    phone_verified BOOLEAN DEFAULT FALSE,
    two_factor_enabled BOOLEAN DEFAULT FALSE,
    two_factor_secret VARCHAR(255),
    password_hash VARCHAR(255), -- Optional for email/password login
    profile_image_url TEXT,
    preferences JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_users_address ON users(address);
CREATE INDEX idx_users_email ON users(email) WHERE email IS NOT NULL;
CREATE INDEX idx_users_kyc_status ON users(kyc_status);

-- ============================================================================
-- TRANSACTIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tx_hash VARCHAR(66) UNIQUE NOT NULL,
    block_number BIGINT,
    block_hash VARCHAR(66),
    from_address VARCHAR(48) NOT NULL,
    to_address VARCHAR(48) NOT NULL,
    amount NUMERIC(36, 18) NOT NULL,
    asset VARCHAR(20) NOT NULL, -- ETR, BTC, ETH, etc.
    fee NUMERIC(36, 18) DEFAULT 0,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'failed')),
    tx_type VARCHAR(20) NOT NULL CHECK (tx_type IN ('transfer', 'stake', 'unstake', 'vote', 'bridge', 'swap')),
    metadata JSONB DEFAULT '{}',
    error_message TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    confirmed_at TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_transactions_user_id ON transactions(user_id);
CREATE INDEX idx_transactions_tx_hash ON transactions(tx_hash);
CREATE INDEX idx_transactions_from_address ON transactions(from_address);
CREATE INDEX idx_transactions_to_address ON transactions(to_address);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_created_at ON transactions(created_at DESC);
CREATE INDEX idx_transactions_block_number ON transactions(block_number DESC);

-- ============================================================================
-- STAKING POSITIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS staking_positions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    validator_address VARCHAR(48) NOT NULL,
    validator_name VARCHAR(255),
    amount NUMERIC(36, 18) NOT NULL,
    rewards_earned NUMERIC(36, 18) DEFAULT 0,
    rewards_claimed NUMERIC(36, 18) DEFAULT 0,
    apy DECIMAL(5, 2), -- e.g., 12.50 for 12.5%
    start_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    end_date TIMESTAMP,
    unbonding_period_days INTEGER DEFAULT 28,
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'unbonding', 'withdrawn', 'slashed')),
    auto_compound BOOLEAN DEFAULT FALSE,
    last_reward_update TIMESTAMP,
    tx_hash VARCHAR(66),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_staking_positions_user_id ON staking_positions(user_id);
CREATE INDEX idx_staking_positions_validator ON staking_positions(validator_address);
CREATE INDEX idx_staking_positions_status ON staking_positions(status);
CREATE INDEX idx_staking_positions_created_at ON staking_positions(created_at DESC);

-- ============================================================================
-- GOVERNANCE VOTES TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS governance_votes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    proposal_id INTEGER NOT NULL,
    proposal_hash VARCHAR(66),
    support BOOLEAN NOT NULL, -- true=YES, false=NO
    conviction INTEGER DEFAULT 0 CHECK (conviction BETWEEN 0 AND 6), -- 0=0.1x, 6=6x
    voting_power NUMERIC(36, 18) NOT NULL,
    balance_locked NUMERIC(36, 18) NOT NULL,
    tx_hash VARCHAR(66) UNIQUE,
    delegated_from VARCHAR(48), -- If voting with delegated tokens
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, proposal_id) -- One vote per user per proposal
);

CREATE INDEX idx_governance_votes_user_id ON governance_votes(user_id);
CREATE INDEX idx_governance_votes_proposal_id ON governance_votes(proposal_id);
CREATE INDEX idx_governance_votes_created_at ON governance_votes(created_at DESC);

-- ============================================================================
-- ATM WITHDRAWALS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS atm_withdrawals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    withdrawal_code VARCHAR(20) UNIQUE NOT NULL,
    amount_usd DECIMAL(10, 2) NOT NULL,
    amount_crypto NUMERIC(36, 18) NOT NULL,
    asset VARCHAR(20) NOT NULL, -- ETR, BTC, ETH
    fee DECIMAL(10, 2) NOT NULL,
    exchange_rate DECIMAL(15, 6) NOT NULL,
    atm_partner VARCHAR(50) NOT NULL CHECK (atm_partner IN ('Coinme', 'Bitcoin Depot', 'CoinFlip')),
    atm_location_id VARCHAR(100),
    atm_address TEXT,
    atm_lat DECIMAL(10, 8),
    atm_lng DECIMAL(11, 8),
    tx_hash VARCHAR(66),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'ready', 'completed', 'expired', 'failed', 'cancelled')),
    expires_at TIMESTAMP NOT NULL, -- Usually 15-30 minutes
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    partner_transaction_id VARCHAR(255),
    metadata JSONB DEFAULT '{}'
);

CREATE INDEX idx_atm_withdrawals_user_id ON atm_withdrawals(user_id);
CREATE INDEX idx_atm_withdrawals_code ON atm_withdrawals(withdrawal_code);
CREATE INDEX idx_atm_withdrawals_status ON atm_withdrawals(status);
CREATE INDEX idx_atm_withdrawals_expires_at ON atm_withdrawals(expires_at);

-- ============================================================================
-- GPU RENTALS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS gpu_rentals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    gpu_id VARCHAR(100) NOT NULL,
    gpu_name VARCHAR(255) NOT NULL, -- e.g., "NVIDIA RTX 4090"
    provider VARCHAR(50) NOT NULL CHECK (provider IN ('Vast.ai', 'RunPod', 'Internal')),
    duration_hours INTEGER NOT NULL,
    price_per_hour NUMERIC(36, 18) NOT NULL,
    total_cost NUMERIC(36, 18) NOT NULL,
    ssh_host VARCHAR(255),
    ssh_port INTEGER,
    ssh_username VARCHAR(100),
    ssh_password VARCHAR(255), -- Encrypted
    jupyter_url TEXT,
    vram_gb INTEGER,
    gpu_count INTEGER DEFAULT 1,
    cpu_cores INTEGER,
    ram_gb INTEGER,
    disk_gb INTEGER,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'provisioning', 'active', 'completed', 'cancelled', 'failed')),
    start_time TIMESTAMP,
    end_time TIMESTAMP,
    payment_tx_hash VARCHAR(66),
    provider_instance_id VARCHAR(255),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_gpu_rentals_user_id ON gpu_rentals(user_id);
CREATE INDEX idx_gpu_rentals_status ON gpu_rentals(status);
CREATE INDEX idx_gpu_rentals_provider ON gpu_rentals(provider);

-- ============================================================================
-- BRIDGE TRANSFERS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS bridge_transfers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    from_chain VARCHAR(50) NOT NULL, -- BTC, ETH, BSC, etc.
    to_chain VARCHAR(50) NOT NULL, -- FLARE (Ëtrid FlareChain)
    from_address VARCHAR(255) NOT NULL,
    to_address VARCHAR(48) NOT NULL,
    from_asset VARCHAR(20) NOT NULL,
    to_asset VARCHAR(20) NOT NULL,
    amount_from NUMERIC(36, 18) NOT NULL,
    amount_to NUMERIC(36, 18) NOT NULL,
    exchange_rate DECIMAL(20, 10) NOT NULL,
    bridge_fee NUMERIC(36, 18) NOT NULL,
    from_tx_hash VARCHAR(255),
    to_tx_hash VARCHAR(66),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'confirming', 'minting', 'completed', 'failed', 'refunded')),
    confirmations_required INTEGER DEFAULT 6,
    confirmations_current INTEGER DEFAULT 0,
    error_message TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP
);

CREATE INDEX idx_bridge_transfers_user_id ON bridge_transfers(user_id);
CREATE INDEX idx_bridge_transfers_status ON bridge_transfers(status);
CREATE INDEX idx_bridge_transfers_from_chain ON bridge_transfers(from_chain);

-- ============================================================================
-- VALIDATORS TABLE (for staking)
-- ============================================================================
CREATE TABLE IF NOT EXISTS validators (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    address VARCHAR(48) UNIQUE NOT NULL,
    name VARCHAR(255),
    description TEXT,
    website_url TEXT,
    commission_rate DECIMAL(5, 2) NOT NULL, -- e.g., 5.00 for 5%
    total_stake NUMERIC(36, 18) DEFAULT 0,
    own_stake NUMERIC(36, 18) DEFAULT 0,
    delegator_count INTEGER DEFAULT 0,
    apy DECIMAL(5, 2), -- Current APY
    is_active BOOLEAN DEFAULT TRUE,
    is_slashed BOOLEAN DEFAULT FALSE,
    uptime_percentage DECIMAL(5, 2),
    blocks_produced INTEGER DEFAULT 0,
    last_active TIMESTAMP,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_validators_address ON validators(address);
CREATE INDEX idx_validators_is_active ON validators(is_active);
CREATE INDEX idx_validators_apy ON validators(apy DESC NULLS LAST);

-- ============================================================================
-- PROPOSALS TABLE (for governance)
-- ============================================================================
CREATE TABLE IF NOT EXISTS proposals (
    id SERIAL PRIMARY KEY,
    proposal_hash VARCHAR(66) UNIQUE NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    proposer_address VARCHAR(48) NOT NULL,
    proposal_type VARCHAR(50) NOT NULL, -- runtime_upgrade, treasury_spend, etc.
    voting_threshold VARCHAR(20), -- Simple_majority, Super_majority_approve, etc.
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'passed', 'rejected', 'cancelled', 'executed')),
    yes_votes NUMERIC(36, 18) DEFAULT 0,
    no_votes NUMERIC(36, 18) DEFAULT 0,
    total_turnout NUMERIC(36, 18) DEFAULT 0,
    submission_deposit NUMERIC(36, 18),
    voting_starts_at TIMESTAMP NOT NULL,
    voting_ends_at TIMESTAMP NOT NULL,
    execution_block INTEGER,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_proposals_status ON proposals(status);
CREATE INDEX idx_proposals_voting_ends_at ON proposals(voting_ends_at DESC);

-- ============================================================================
-- NOTIFICATIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL, -- tx_confirmed, proposal_ending, rewards_received, etc.
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    data JSONB DEFAULT '{}',
    is_read BOOLEAN DEFAULT FALSE,
    sent_via VARCHAR(20)[], -- ['push', 'email', 'sms']
    sent_at TIMESTAMP,
    read_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_is_read ON notifications(is_read);
CREATE INDEX idx_notifications_created_at ON notifications(created_at DESC);

-- ============================================================================
-- API KEYS TABLE (for developer access)
-- ============================================================================
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_hash VARCHAR(255) UNIQUE NOT NULL,
    key_prefix VARCHAR(20) NOT NULL, -- First 8 chars for identification
    name VARCHAR(255) NOT NULL,
    permissions TEXT[] DEFAULT '{}', -- ['read', 'write', 'admin']
    is_active BOOLEAN DEFAULT TRUE,
    last_used_at TIMESTAMP,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_api_keys_key_hash ON api_keys(key_hash);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

-- ============================================================================
-- PRICE HISTORY TABLE (for analytics)
-- ============================================================================
CREATE TABLE IF NOT EXISTS price_history (
    id BIGSERIAL PRIMARY KEY,
    asset VARCHAR(20) NOT NULL,
    price_usd DECIMAL(20, 10) NOT NULL,
    volume_24h DECIMAL(20, 2),
    market_cap DECIMAL(20, 2),
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_price_history_asset_timestamp ON price_history(asset, timestamp DESC);

-- ============================================================================
-- ANALYTICS EVENTS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS analytics_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    event_name VARCHAR(100) NOT NULL,
    event_properties JSONB DEFAULT '{}',
    user_agent TEXT,
    ip_address INET,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_analytics_events_user_id ON analytics_events(user_id);
CREATE INDEX idx_analytics_events_event_name ON analytics_events(event_name);
CREATE INDEX idx_analytics_events_created_at ON analytics_events(created_at DESC);

-- ============================================================================
-- TRIGGERS FOR updated_at
-- ============================================================================
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_staking_positions_updated_at BEFORE UPDATE ON staking_positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_gpu_rentals_updated_at BEFORE UPDATE ON gpu_rentals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_validators_updated_at BEFORE UPDATE ON validators
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_proposals_updated_at BEFORE UPDATE ON proposals
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- VIEWS FOR COMMON QUERIES
-- ============================================================================

-- User portfolio view
CREATE OR REPLACE VIEW user_portfolio AS
SELECT
    u.id as user_id,
    u.address,
    COUNT(DISTINCT sp.id) as active_staking_positions,
    COALESCE(SUM(sp.amount), 0) as total_staked,
    COALESCE(SUM(sp.rewards_earned), 0) as total_rewards,
    COUNT(DISTINCT gv.id) as governance_votes_count,
    COUNT(DISTINCT t.id) as transaction_count
FROM users u
LEFT JOIN staking_positions sp ON u.id = sp.user_id AND sp.status = 'active'
LEFT JOIN governance_votes gv ON u.id = gv.user_id
LEFT JOIN transactions t ON u.id = t.user_id AND t.status = 'confirmed'
GROUP BY u.id, u.address;

-- Active validators view
CREATE OR REPLACE VIEW active_validators_stats AS
SELECT
    v.*,
    COUNT(DISTINCT sp.user_id) as unique_delegators,
    COALESCE(SUM(sp.amount), 0) as delegated_stake
FROM validators v
LEFT JOIN staking_positions sp ON v.address = sp.validator_address AND sp.status = 'active'
WHERE v.is_active = TRUE
GROUP BY v.id;

-- ============================================================================
-- GRANT PERMISSIONS (adjust user as needed)
-- ============================================================================
-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO etrid_api_user;
-- GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO etrid_api_user;
