-- =============================================================================
-- Sparc Energy Carbon Marketplace — Oracle Autonomous Database Schema
-- =============================================================================

-- 1. USERS & ROLES
CREATE TABLE users (
    id VARCHAR2(50) PRIMARY KEY,
    email VARCHAR2(100) UNIQUE NOT NULL,
    password_hash VARCHAR2(255) NOT NULL, -- Managed by Supabase primarily, but synced here
    name VARCHAR2(100) NOT NULL,
    role VARCHAR2(20) DEFAULT 'buyer' CHECK (role IN ('buyer', 'seller', 'verifier', 'admin')),
    balance NUMBER(18, 2) DEFAULT 0.00,
    kyc_status VARCHAR2(20) DEFAULT 'pending' CHECK (kyc_status IN ('pending', 'submitted', 'verified', 'rejected')),
    two_factor_enabled NUMBER(1) DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 2. PROJECTS (The primary registry)
CREATE TABLE carbon_projects (
    id VARCHAR2(50) PRIMARY KEY,
    owner_id VARCHAR2(50) NOT NULL REFERENCES users(id),
    name VARCHAR2(150) NOT NULL,
    description CLOB,
    project_type VARCHAR2(50), -- e.g., 'reforestation', 'solar', 'wind'
    location_country VARCHAR2(100),
    location_gps VARCHAR2(100), -- Format: "lat,long"
    methodology_standard VARCHAR2(50), -- e.g., 'Verra VCS', 'Gold Standard'
    certification_id VARCHAR2(100),
    total_credits NUMBER(18, 2),
    credits_issued NUMBER(18, 2) DEFAULT 0,
    verification_status VARCHAR2(20) DEFAULT 'draft' CHECK (verification_status IN ('draft', 'pending', 'approved', 'rejected')),
    verifier_id VARCHAR2(50) REFERENCES users(id),
    sdg_goals VARCHAR2(100), -- Multi-select list e.g., "13,15,17"
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 3. CREDIT BATCHES (Specific vintages)
CREATE TABLE carbon_credits (
    id VARCHAR2(50) PRIMARY KEY,
    project_id VARCHAR2(50) NOT NULL REFERENCES carbon_projects(id),
    seller_id VARCHAR2(50) NOT NULL REFERENCES users(id),
    vintage_year NUMBER(4),
    quantity_tons NUMBER(18, 2),
    quantity_available NUMBER(18, 2),
    price_per_ton NUMBER(18, 2),
    serial_number_start VARCHAR2(100),
    serial_number_end VARCHAR2(100),
    status VARCHAR2(20) DEFAULT 'active' CHECK (status IN ('active', 'sold', 'retired')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 4. ORDER BOOK (Real-time marketplace)
CREATE TABLE market_orders (
    id VARCHAR2(50) PRIMARY KEY,
    user_id VARCHAR2(50) NOT NULL REFERENCES users(id),
    credit_id VARCHAR2(50) NOT NULL REFERENCES carbon_credits(id),
    order_type VARCHAR2(10) CHECK (order_type IN ('bid', 'ask')), -- bid=buy, ask=sell
    price NUMBER(18, 2) NOT NULL,
    quantity NUMBER(18, 2) NOT NULL,
    filled_quantity NUMBER(18, 2) DEFAULT 0,
    status VARCHAR2(20) DEFAULT 'open' CHECK (status IN ('open', 'partially_filled', 'filled', 'cancelled')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 5. TRANSACTIONS (Match history)
CREATE TABLE trades (
    id VARCHAR2(50) PRIMARY KEY,
    bid_order_id VARCHAR2(50) REFERENCES market_orders(id),
    ask_order_id VARCHAR2(50) REFERENCES market_orders(id),
    buyer_id VARCHAR2(50) REFERENCES users(id),
    seller_id VARCHAR2(50) REFERENCES users(id),
    credit_id VARCHAR2(50) REFERENCES carbon_credits(id),
    quantity NUMBER(18, 2),
    price NUMBER(18, 2),
    total_value NUMBER(18, 2),
    platform_fee NUMBER(18, 2),
    tx_hash VARCHAR2(100), -- Internal or Blockchain hash
    status VARCHAR2(20) DEFAULT 'completed',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 6. RETIREMENTS (Permanent removal)
CREATE TABLE credit_retirements (
    id VARCHAR2(50) PRIMARY KEY,
    user_id VARCHAR2(50) REFERENCES users(id),
    credit_id VARCHAR2(50) REFERENCES carbon_credits(id),
    quantity NUMBER(18, 2),
    retirement_reason VARCHAR2(255),
    certificate_url VARCHAR2(255), -- Hosted on Supabase Storage
    serial_numbers_retired CLOB,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 7. KYC & COMPLIANCE
CREATE TABLE kyc_applications (
    id VARCHAR2(50) PRIMARY KEY,
    user_id VARCHAR2(50) NOT NULL REFERENCES users(id),
    first_name VARCHAR2(50),
    last_name VARCHAR2(50),
    dob DATE,
    address CLOB,
    id_type VARCHAR2(50), -- Passport, Driver License, etc.
    id_number VARCHAR2(50),
    document_url VARCHAR2(255), -- Supabase Storage link
    status VARCHAR2(20) DEFAULT 'pending',
    reviewer_comments CLOB,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 8. PAYMENTS LOG
CREATE TABLE payments (
    id VARCHAR2(50) PRIMARY KEY,
    user_id VARCHAR2(50) REFERENCES users(id),
    amount NUMBER(18, 2),
    currency VARCHAR2(10) DEFAULT 'USD',
    payment_method VARCHAR2(50), -- Razorpay, UPI, Crypto, Card
    external_ref_id VARCHAR2(100), -- Razorpay Order ID or Crypto Tx Hash
    status VARCHAR2(20) DEFAULT 'pending', -- pending, success, failed
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_credits_project ON carbon_credits(project_id);
CREATE INDEX idx_orders_status ON market_orders(status);
CREATE INDEX idx_trades_buyer ON trades(buyer_id);
CREATE INDEX idx_trades_seller ON trades(seller_id);
