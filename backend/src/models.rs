use serde::{Deserialize, Serialize};

// ─── User & KYC Models ───────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub balance: f64,
    pub kyc_status: String,
    pub two_factor_enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserPublic {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct KYCRequest {
    pub first_name: String,
    pub last_name: String,
    pub id_type: String,
    pub id_number: String,
    pub document_url: String, // Link to Supabase Storage
}

// ─── Project & Credit Registry ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct CarbonProject {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
    pub country: String,
    pub methodology_standard: String,
    pub verification_status: String,
    pub total_credits: f64,
    pub credits_issued: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
    pub location: String,
    pub country: String,
    pub total_credits: f64,
    pub certification: String,
    pub sdg_goals: String,
    pub co2_reduction_per_year: f64,
    pub project_start_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct CarbonCredit {
    pub id: String,
    pub project_id: String,
    pub seller_id: String,
    pub vintage_year: i32,
    pub quantity_tons: f64,
    pub quantity_available: f64,
    pub price_per_ton: f64,
    pub serial_number_start: String,
    pub serial_number_end: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCreditRequest {
    pub project_id: String,
    pub vintage_year: i32,
    pub quantity_tons: f64,
    pub price_per_ton: f64,
    pub certification: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct CreditWithProject {
    pub id: String,
    pub project_id: String,
    pub project_name: String,
    pub project_type: String,
    pub country: String,
    pub seller_id: String,
    pub seller_name: String,
    pub price_per_ton: f64,
    pub quantity_tons: f64,
    pub quantity_available: f64,
    pub status: String,
    pub vintage_year: i32,
    pub certification: String,
    pub serial_number_start: String,
    pub serial_number_end: String,
    pub created_at: String,
}

// ─── Marketplace (Order Book) ────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MarketOrder {
    pub id: String,
    pub user_id: String,
    pub credit_id: String,
    pub order_type: String, // "bid" or "ask"
    pub price: f64,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub status: String, // "open", "filled", "cancelled"
}

#[derive(Debug, Deserialize)]
pub struct PlaceOrderRequest {
    pub credit_id: String,
    pub order_type: String,
    pub price: f64,
    pub quantity: f64,
}

// ─── Trade & Settlement ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Trade {
    pub id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub quantity: f64,
    pub price: f64,
    pub total_value: f64,
    pub tx_hash: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RetirementRecord {
    pub id: String,
    pub user_id: String,
    pub credit_id: String,
    pub quantity: f64,
    pub certificate_url: String,
    pub serial_numbers: String,
}

#[derive(Debug, Deserialize)]
pub struct RetireRequest {
    pub credit_id: String,
    pub quantity_tons: f64,
    pub retirement_reason: Option<String>,
}

// ─── Payments ────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RazorpayWebhook {
    pub order_id: String,
    pub payment_id: String,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct CryptoVerificationRequest {
    pub tx_hash: String,
    pub amount: f64,
}

// ─── API Response Wrappers ───────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { success: true, data: Some(data), message: None }
    }
    pub fn ok_msg(data: T, msg: &str) -> Self {
        Self { success: true, data: Some(data), message: Some(msg.to_string()) }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

impl ErrorResponse {
    pub fn new(msg: &str) -> Self {
        Self { success: false, error: msg.to_string() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}
