use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use crate::models::*;

pub async fn get_market_stats(pool: web::Data<PgPool>) -> HttpResponse {
    let total_credits: (f64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(quantity_available), 0) FROM carbon_credits WHERE status = 'active'"
    )
    .fetch_one(pool.as_ref()).await.unwrap_or((0.0,));

    let volume_24h: (f64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_price), 0) FROM transactions WHERE created_at > datetime('now', '-24 hours')"
    )
    .fetch_one(pool.as_ref()).await.unwrap_or((0.0,));

    let total_txns: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM transactions")
        .fetch_one(pool.as_ref()).await.unwrap_or((0,));

    let price_stats: Option<(f64, f64, f64)> = sqlx::query_as(
        "SELECT AVG(price_per_ton), MAX(price_per_ton), MIN(price_per_ton) FROM carbon_credits WHERE status = 'active'"
    )
    .fetch_optional(pool.as_ref()).await.unwrap_or(None);

    let co2_offset: (f64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(quantity_tons), 0) FROM transactions WHERE retired = 1"
    )
    .fetch_one(pool.as_ref()).await.unwrap_or((0.0,));

    let total_projects: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM carbon_projects")
        .fetch_one(pool.as_ref()).await.unwrap_or((0,));

    let verified_projects: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM carbon_projects WHERE verified = 1"
    )
    .fetch_one(pool.as_ref()).await.unwrap_or((0,));

    let (avg_price, highest_price, lowest_price) = price_stats.unwrap_or((0.0, 0.0, 0.0));

    let stats = MarketStats {
        total_credits_listed: total_credits.0,
        total_volume_24h: volume_24h.0,
        total_transactions: total_txns.0,
        avg_price,
        highest_price,
        lowest_price,
        total_co2_offset: co2_offset.0,
        total_projects: total_projects.0,
        verified_projects: verified_projects.0,
    };

    HttpResponse::Ok().json(ApiResponse::ok(stats))
}

pub async fn get_recent_trades(pool: web::Data<PgPool>) -> HttpResponse {
    let trades: Vec<TransactionDetail> = sqlx::query_as(
        "SELECT t.id, t.buyer_id, buyer.name as buyer_name, t.seller_id, seller.name as seller_name,
                cp.name as project_name, cp.project_type,
                t.quantity_tons, t.price_per_ton, t.total_price,
                t.certification, t.vintage_year, t.status, t.retired, t.created_at
         FROM transactions t
         JOIN users buyer ON t.buyer_id = buyer.id
         JOIN users seller ON t.seller_id = seller.id
         JOIN carbon_projects cp ON t.project_id = cp.id
         ORDER BY t.created_at DESC LIMIT 50"
    )
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(trades))
}

pub async fn get_leaderboard(pool: web::Data<PgPool>) -> HttpResponse {
    let leaders: Vec<serde_json::Value> = sqlx::query_as::<_, (String, String, f64, f64)>(
        "SELECT id, name, total_credits_owned, balance FROM users WHERE role != 'admin' ORDER BY total_credits_owned DESC LIMIT 20"
    )
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default()
    .into_iter()
    .enumerate()
    .map(|(i, (id, name, credits, balance))| serde_json::json!({
        "rank": i + 1,
        "id": id,
        "name": name,
        "total_credits": credits,
        "balance": balance
    }))
    .collect();

    HttpResponse::Ok().json(ApiResponse::ok(leaders))
}
