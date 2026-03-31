use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ─── User Models ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub balance: f64,
    pub total_credits_owned: f64,
    pub avatar_url: Option<String>,
    pub kyc_verified: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserPublic {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub balance: f64,
    pub total_credits_owned: f64,
    pub kyc_verified: i64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

// ─── Carbon Project Models ───────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CarbonProject {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
    pub location: String,
    pub country: String,
    pub owner_id: String,
    pub total_credits: f64,
    pub credits_issued: f64,
    pub credits_retired: f64,
    pub verified: i64,
    pub certification: Option<String>,
    pub image_url: Option<String>,
    pub sdg_goals: Option<String>,
    pub co2_reduction_per_year: Option<f64>,
    pub project_start_date: Option<String>,
    pub project_end_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub project_type: String,
    pub location: String,
    pub country: String,
    pub total_credits: f64,
    pub certification: Option<String>,
    pub sdg_goals: Option<String>,
    pub co2_reduction_per_year: Option<f64>,
    pub project_start_date: Option<String>,
}

// ─── Carbon Credit Models ────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct CarbonCredit {
    pub id: String,
    pub project_id: String,
    pub seller_id: String,
    pub price_per_ton: f64,
    pub quantity_tons: f64,
    pub quantity_available: f64,
    pub status: String,
    pub vintage_year: i64,
    pub certification: String,
    pub serial_number: Option<String>,
    pub co2_type: String,
    pub methodology: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
    pub vintage_year: i64,
    pub certification: String,
    pub serial_number: Option<String>,
    pub methodology: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCreditRequest {
    pub project_id: String,
    pub price_per_ton: f64,
    pub quantity_tons: f64,
    pub vintage_year: i64,
    pub certification: String,
    pub methodology: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BuyRequest {
    pub credit_id: String,
    pub quantity_tons: f64,
}

// ─── Transaction Models ──────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: String,
    pub buyer_id: String,
    pub seller_id: String,
    pub credit_id: String,
    pub project_id: String,
    pub quantity_tons: f64,
    pub price_per_ton: f64,
    pub total_price: f64,
    pub tx_hash: Option<String>,
    pub certification: String,
    pub vintage_year: i64,
    pub status: String,
    pub retired: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TransactionDetail {
    pub id: String,
    pub buyer_id: String,
    pub buyer_name: String,
    pub seller_id: String,
    pub seller_name: String,
    pub project_name: String,
    pub project_type: String,
    pub quantity_tons: f64,
    pub price_per_ton: f64,
    pub total_price: f64,
    pub certification: String,
    pub vintage_year: i64,
    pub status: String,
    pub retired: i64,
    pub created_at: String,
}

// ─── Portfolio Models ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PortfolioItem {
    pub id: String,
    pub user_id: String,
    pub credit_id: String,
    pub project_id: String,
    pub quantity_tons: f64,
    pub average_buy_price: f64,
    pub total_invested: f64,
    pub retired_tons: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PortfolioWithDetails {
    pub id: String,
    pub credit_id: String,
    pub project_name: String,
    pub project_type: String,
    pub country: String,
    pub certification: String,
    pub current_price: f64,
    pub quantity_tons: f64,
    pub average_buy_price: f64,
    pub total_invested: f64,
    pub current_value: f64,
    pub pnl: f64,
    pub retired_tons: f64,
}

// ─── Market Models ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketStats {
    pub total_credits_listed: f64,
    pub total_volume_24h: f64,
    pub total_transactions: i64,
    pub avg_price: f64,
    pub highest_price: f64,
    pub lowest_price: f64,
    pub total_co2_offset: f64,
    pub total_projects: i64,
    pub verified_projects: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PriceHistory {
    pub id: String,
    pub credit_id: String,
    pub price: f64,
    pub volume: f64,
    pub recorded_at: String,
}

// ─── API Response Models ─────────────────────────────────────────────────────

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

// ─── JWT Claims ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // user id
    pub email: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

// ─── Query Params ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreditFilter {
    pub certification: Option<String>,
    pub project_type: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub vintage_year: Option<i64>,
    pub country: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RetireRequest {
    pub credit_id: String,
    pub quantity_tons: f64,
    pub retirement_reason: Option<String>,
}
