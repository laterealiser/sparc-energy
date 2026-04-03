-- Sparc Energy - Phase 2: Professional Services Migration

-- 1. Extend Users Role Type (Manual check in Rust, but here define professional_profiles)
CREATE TABLE IF NOT EXISTS professional_profiles (
    user_id TEXT PRIMARY KEY REFERENCES users(id),
    title TEXT NOT NULL, -- e.g. "Senior PDD Writer", "VCS Auditor"
    bio TEXT,
    skills TEXT, -- Comma-separated or JSONB if using Postgres
    hourly_rate DOUBLE PRECISION,
    rating DOUBLE PRECISION DEFAULT 0.0,
    completed_projects INTEGER DEFAULT 0,
    accreditation_id TEXT, -- Verra/GS ID
    verified INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 2. Service Contracts & Escrow
CREATE TABLE IF NOT EXISTS service_contracts (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL REFERENCES users(id),
    provider_id TEXT NOT NULL REFERENCES users(id),
    project_id TEXT REFERENCES carbon_projects(id),
    total_amount DOUBLE PRECISION NOT NULL,
    escrow_balance DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'escrowed', 'completed', 'disputed'
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- 3. Milestones for Escrow Release
CREATE TABLE IF NOT EXISTS service_milestones (
    id TEXT PRIMARY KEY,
    contract_id TEXT NOT NULL REFERENCES service_contracts(id),
    description TEXT NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'paid'
    due_date TEXT,
    completed_at TEXT
);

-- 4. Registry Wallet Connections (Verra/GS)
CREATE TABLE IF NOT EXISTS registry_wallets (
    user_id TEXT PRIMARY KEY REFERENCES users(id),
    registry_name TEXT NOT NULL, -- 'Verra', 'GoldStandard'
    wallet_address TEXT NOT NULL,
    verified INTEGER NOT NULL DEFAULT 0,
    last_sync_at TEXT
);
