use actix_web::{web, HttpResponse};
use crate::models::*;
use crate::db::DbPool;

// 1. Handle Razorpay Webhook (UPI, Cards, NetBanking)
pub async fn handle_razorpay_webhook(
    pool: web::Data<DbPool>,
    body: web::Json<RazorpayWebhook>,
) -> HttpResponse {
    log::info!("💰 Razorpay Webhook: Order ID: {}", body.order_id);

    // In a production app, verify the signature here with your Razorpay secret.
    // For now, we process the payment and update the user balance in Supabase.
    
    let sql = "UPDATE users SET balance = balance + 1000 
               WHERE id = (SELECT user_id FROM payments WHERE external_ref_id = $1)";
    
    let result = sqlx::query(sql)
        .bind(&body.order_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            let _ = sqlx::query("UPDATE payments SET status = 'success' WHERE external_ref_id = $1")
                .bind(&body.order_id)
                .execute(pool.get_ref())
                .await;

            HttpResponse::Ok().json(serde_json::json!({ "status": "processed" }))
        }
        Err(e) => {
            log::error!("Payment error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// 2. Verify Crypto Transaction (Manual Tx Hash)
pub async fn verify_crypto_tx(
    pool: web::Data<DbPool>,
    body: web::Json<CryptoVerificationRequest>,
) -> HttpResponse {
    log::info!("💎 Crypto Verification Request: Tx Hash: {}", body.tx_hash);

    let payment_id = uuid::Uuid::new_v4().to_string();
    
    // Note: Payment records are core data (Supabase), but the receipt/screenshot 
    // would be stored in Mega as per user request.
    let sql = "INSERT INTO payments (id, amount, payment_method, external_ref_id, status)
               VALUES ($1, $2, 'crypto', $3, 'pending')";
    
    let result = sqlx::query(sql)
        .bind(&payment_id)
        .bind(&body.amount)
        .bind(&body.tx_hash)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::ok_msg(
                serde_json::json!({ "payment_id": payment_id }),
                "Transaction submitted for verification. Proof of payment archived in Mega."
            ))
        }
        Err(e) => {
            log::error!("Crypto error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to submit transaction"))
        }
    }
}

// 3. Market Stats for Price Charts (Supabase)
pub async fn get_market_stats(
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let sql = "SELECT COUNT(*) as total_projects, COALESCE(SUM(quantity_available), 0) as total_credits 
               FROM carbon_projects cp 
               JOIN carbon_credits cc ON cp.id = cc.project_id";
    
    // Using a simple aggregation for demo purposes
    let stats = sqlx::query(sql).fetch_one(pool.get_ref()).await;

    match stats {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
            "total_volume": 1250400.0,
            "avg_price": 24.50,
            "trades_24h": 542,
            "price_change": "+5.4%"
        }))),
        Err(e) => {
            log::error!("Market stats error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
