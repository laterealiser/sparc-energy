-- Carbon Credit Market Platform - Database Schema

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'buyer',
    balance DOUBLE PRECISION NOT NULL DEFAULT 10000.0,
    total_credits_owned DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    avatar_url TEXT,
    kyc_verified INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS carbon_projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    project_type TEXT NOT NULL,
    location TEXT NOT NULL,
    country TEXT NOT NULL,
    owner_id TEXT NOT NULL REFERENCES users(id),
    total_credits DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    credits_issued DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    credits_retired DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    verified INTEGER NOT NULL DEFAULT 0,
    certification TEXT,
    image_url TEXT,
    sdg_goals TEXT,
    co2_reduction_per_year DOUBLE PRECISION,
    project_start_date TEXT,
    project_end_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS carbon_credits (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES carbon_projects(id),
    seller_id TEXT NOT NULL REFERENCES users(id),
    price_per_ton DOUBLE PRECISION NOT NULL,
    quantity_tons DOUBLE PRECISION NOT NULL,
    quantity_available DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    vintage_year INTEGER NOT NULL,
    certification TEXT NOT NULL,
    serial_number TEXT UNIQUE,
    co2_type TEXT NOT NULL DEFAULT 'CO2e',
    methodology TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS market_orders (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    credit_id TEXT NOT NULL REFERENCES carbon_credits(id),
    order_type TEXT NOT NULL,
    quantity_tons DOUBLE PRECISION NOT NULL,
    filled_quantity DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    price_per_ton DOUBLE PRECISION NOT NULL,
    total_amount DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS trades (
    id TEXT PRIMARY KEY,
    buyer_id TEXT NOT NULL REFERENCES users(id),
    seller_id TEXT NOT NULL REFERENCES users(id),
    credit_id TEXT NOT NULL REFERENCES carbon_credits(id),
    bid_order_id TEXT NOT NULL REFERENCES market_orders(id),
    ask_order_id TEXT NOT NULL REFERENCES market_orders(id),
    quantity DOUBLE PRECISION NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    total_value DOUBLE PRECISION NOT NULL,
    platform_fee DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'completed',
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    buyer_id TEXT NOT NULL REFERENCES users(id),
    seller_id TEXT NOT NULL REFERENCES users(id),
    credit_id TEXT NOT NULL REFERENCES carbon_credits(id),
    project_id TEXT NOT NULL REFERENCES carbon_projects(id),
    quantity_tons DOUBLE PRECISION NOT NULL,
    price_per_ton DOUBLE PRECISION NOT NULL,
    total_price DOUBLE PRECISION NOT NULL,
    tx_hash TEXT UNIQUE,
    certification TEXT NOT NULL,
    vintage_year INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'completed',
    retired INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS portfolio (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    credit_id TEXT NOT NULL REFERENCES carbon_credits(id),
    project_id TEXT NOT NULL REFERENCES carbon_projects(id),
    quantity_tons DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    average_buy_price DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    total_invested DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    retired_tons DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(user_id, credit_id)
);

CREATE TABLE IF NOT EXISTS price_history (
    id TEXT PRIMARY KEY,
    credit_id TEXT NOT NULL REFERENCES carbon_credits(id),
    price DOUBLE PRECISION NOT NULL,
    volume DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    recorded_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS watchlist (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    credit_id TEXT NOT NULL REFERENCES carbon_credits(id),
    added_at TEXT NOT NULL,
    UNIQUE(user_id, credit_id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_credits_status ON carbon_credits(status);
CREATE INDEX IF NOT EXISTS idx_credits_seller ON carbon_credits(seller_id);
CREATE INDEX IF NOT EXISTS idx_transactions_buyer ON transactions(buyer_id);
CREATE INDEX IF NOT EXISTS idx_transactions_seller ON transactions(seller_id);
CREATE INDEX IF NOT EXISTS idx_portfolio_user ON portfolio(user_id);
CREATE INDEX IF NOT EXISTS idx_price_history_credit ON price_history(credit_id);
