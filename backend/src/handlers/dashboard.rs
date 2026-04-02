use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use crate::models::*;
use crate::auth::require_auth;

pub async fn get_dashboard(
    pool: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    // User info
    let user: Option<UserPublic> = sqlx::query_as(
        "SELECT id, email, name, role, balance, total_credits_owned, kyc_verified, created_at FROM users WHERE id = $1"
    )
    .bind(&claims.sub)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    // Portfolio
    let portfolio: Vec<PortfolioWithDetails> = sqlx::query_as(
        "SELECT p.id, p.credit_id, cp.name as project_name, cp.project_type, cp.country,
                cc.certification, cc.price_per_ton as current_price,
                p.quantity_tons, p.average_buy_price, p.total_invested,
                (p.quantity_tons * cc.price_per_ton) as current_value,
                ((p.quantity_tons * cc.price_per_ton) - p.total_invested) as pnl,
                p.retired_tons
         FROM portfolio p
         JOIN carbon_credits cc ON p.credit_id = cc.id
         JOIN carbon_projects cp ON p.project_id = cp.id
         WHERE p.user_id = $1 AND p.quantity_tons > 0
         ORDER BY p.updated_at DESC"
    )
    .bind(&claims.sub)
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    // Recent transactions
    let transactions: Vec<TransactionDetail> = sqlx::query_as(
        "SELECT t.id, t.buyer_id, buyer.name as buyer_name, t.seller_id, seller.name as seller_name,
                cp.name as project_name, cp.project_type,
                t.quantity_tons, t.price_per_ton, t.total_price,
                t.certification, t.vintage_year, t.status, t.retired, t.created_at
         FROM transactions t
         JOIN users buyer ON t.buyer_id = buyer.id
         JOIN users seller ON t.seller_id = seller.id
         JOIN carbon_projects cp ON t.project_id = cp.id
         WHERE t.buyer_id = $1 OR t.seller_id = $2
         ORDER BY t.created_at DESC LIMIT 20"
    )
    .bind(&claims.sub).bind(&claims.sub)
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    // Portfolio summary
    let total_invested: f64 = portfolio.iter().map(|p| p.total_invested).sum();
    let total_current_value: f64 = portfolio.iter().map(|p| p.current_value).sum();
    let total_pnl = total_current_value - total_invested;
    let total_retired: f64 = portfolio.iter().map(|p| p.retired_tons).sum();

    HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
        "user": user,
        "portfolio": portfolio,
        "transactions": transactions,
        "summary": {
            "total_invested": total_invested,
            "total_current_value": total_current_value,
            "total_pnl": total_pnl,
            "total_pnl_pct": if total_invested > 0.0 { (total_pnl / total_invested) * 100.0 } else { 0.0 },
            "total_credits": portfolio.iter().map(|p| p.quantity_tons).sum::<f64>(),
            "total_retired": total_retired,
            "co2_offset_tons": total_retired
        }
    })))
}

pub async fn retire_credits(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    body: web::Json<RetireRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    let now = chrono::Utc::now().to_rfc3339();

    let portfolio: Option<(String, f64)> = sqlx::query_as(
        "SELECT id, quantity_tons FROM portfolio WHERE user_id = $1 AND credit_id = $2"
    )
    .bind(&claims.sub).bind(&body.credit_id)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    let (port_id, qty) = match portfolio {
        Some(p) => p,
        None => return HttpResponse::NotFound().json(ErrorResponse::new("Credit not in portfolio")),
    };

    if body.quantity_tons > qty {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            &format!("Only {:.2} tons available to retire", qty)
        ));
    }

    let _ = sqlx::query(
        "UPDATE portfolio SET quantity_tons = quantity_tons - $1, retired_tons = retired_tons + $2, updated_at = $3 WHERE id = $4"
    )
    .bind(body.quantity_tons).bind(body.quantity_tons).bind(&now).bind(&port_id)
    .execute(pool.as_ref()).await;

    let _ = sqlx::query(
        "UPDATE users SET total_credits_owned = total_credits_owned - $1, updated_at = $2 WHERE id = $3"
    )
    .bind(body.quantity_tons).bind(&now).bind(&claims.sub)
    .execute(pool.as_ref()).await;

    let _ = sqlx::query(
        "UPDATE transactions SET retired = 1 WHERE buyer_id = $1 AND credit_id = $2"
    )
    .bind(&claims.sub).bind(&body.credit_id)
    .execute(pool.as_ref()).await;

    HttpResponse::Ok().json(ApiResponse::ok_msg(
        serde_json::json!({
            "retired_tons": body.quantity_tons,
            "reason": body.retirement_reason,
            "certificate": format!("SPARC-RET-{}", &uuid::Uuid::new_v4().to_string()[..8].to_uppercase())
        }),
        &format!("{:.2} tons of CO2e successfully retired. Certificate issued.", body.quantity_tons)
    ))
}
