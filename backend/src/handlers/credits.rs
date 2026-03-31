use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use crate::auth::require_auth;

pub async fn list_credits(
    pool: web::Data<SqlitePool>,
    query: web::Query<CreditFilter>,
) -> HttpResponse {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;

    let mut sql = String::from(
        "SELECT cc.id, cc.project_id, cp.name as project_name, cp.project_type, cp.country,
                cc.seller_id, u.name as seller_name, cc.price_per_ton, cc.quantity_tons,
                cc.quantity_available, cc.status, cc.vintage_year, cc.certification,
                cc.serial_number, cc.methodology, cc.created_at
         FROM carbon_credits cc
         JOIN carbon_projects cp ON cc.project_id = cp.id
         JOIN users u ON cc.seller_id = u.id
         WHERE cc.status = 'active' AND cc.quantity_available > 0"
    );

    if let Some(cert) = &query.certification {
        sql.push_str(&format!(" AND cc.certification = '{}'", cert.replace('\'', "''")));
    }
    if let Some(ptype) = &query.project_type {
        sql.push_str(&format!(" AND cp.project_type = '{}'", ptype.replace('\'', "''")));
    }
    if let Some(min_p) = query.min_price {
        sql.push_str(&format!(" AND cc.price_per_ton >= {}", min_p));
    }
    if let Some(max_p) = query.max_price {
        sql.push_str(&format!(" AND cc.price_per_ton <= {}", max_p));
    }
    if let Some(vy) = query.vintage_year {
        sql.push_str(&format!(" AND cc.vintage_year = {}", vy));
    }
    if let Some(country) = &query.country {
        sql.push_str(&format!(" AND cp.country = '{}'", country.replace('\'', "''")));
    }

    sql.push_str(&format!(" ORDER BY cc.created_at DESC LIMIT {} OFFSET {}", limit, offset));

    let credits: Vec<CreditWithProject> = sqlx::query_as(&sql)
        .fetch_all(pool.as_ref())
        .await
        .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(credits))
}

pub async fn get_credit(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> HttpResponse {
    let credit_id = path.into_inner();
    let credit: Option<CreditWithProject> = sqlx::query_as(
        "SELECT cc.id, cc.project_id, cp.name as project_name, cp.project_type, cp.country,
                cc.seller_id, u.name as seller_name, cc.price_per_ton, cc.quantity_tons,
                cc.quantity_available, cc.status, cc.vintage_year, cc.certification,
                cc.serial_number, cc.methodology, cc.created_at
         FROM carbon_credits cc
         JOIN carbon_projects cp ON cc.project_id = cp.id
         JOIN users u ON cc.seller_id = u.id
         WHERE cc.id = ?"
    )
    .bind(&credit_id)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    match credit {
        Some(c) => HttpResponse::Ok().json(ApiResponse::ok(c)),
        None => HttpResponse::NotFound().json(ErrorResponse::new("Credit not found")),
    }
}

pub async fn buy_credit(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    body: web::Json<BuyRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    // Get credit details
    let credit: Option<CarbonCredit> = sqlx::query_as(
        "SELECT * FROM carbon_credits WHERE id = ? AND status = 'active'"
    )
    .bind(&body.credit_id)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    let credit = match credit {
        Some(c) => c,
        None => return HttpResponse::NotFound().json(ErrorResponse::new("Credit listing not found")),
    };

    if credit.seller_id == claims.sub {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Cannot buy your own listing"));
    }

    if body.quantity_tons <= 0.0 {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Quantity must be greater than 0"));
    }

    if body.quantity_tons > credit.quantity_available {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            &format!("Only {:.2} tons available", credit.quantity_available)
        ));
    }

    let total_cost = body.quantity_tons * credit.price_per_ton;

    // Check buyer balance
    let buyer: Option<(f64,)> = sqlx::query_as("SELECT balance FROM users WHERE id = ?")
        .bind(&claims.sub)
        .fetch_optional(pool.as_ref())
        .await
        .unwrap_or(None);

    let (buyer_balance,) = match buyer {
        Some(b) => b,
        None => return HttpResponse::NotFound().json(ErrorResponse::new("Buyer not found")),
    };

    if buyer_balance < total_cost {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            &format!("Insufficient balance. Need ${:.2}, have ${:.2}", total_cost, buyer_balance)
        ));
    }

    let tx_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let tx_hash = format!("0x{}", hex::encode(&Uuid::new_v4().as_bytes()[..]));

    // Use a transaction for atomicity
    let mut db_tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse::new("Database error")),
    };

    // Deduct from buyer
    let _ = sqlx::query("UPDATE users SET balance = balance - ?, updated_at = ? WHERE id = ?")
        .bind(total_cost).bind(&now).bind(&claims.sub)
        .execute(&mut *db_tx).await;

    // Add to seller
    let _ = sqlx::query("UPDATE users SET balance = balance + ?, updated_at = ? WHERE id = ?")
        .bind(total_cost * 0.975).bind(&now).bind(&credit.seller_id) // 2.5% platform fee
        .execute(&mut *db_tx).await;

    // Update credit availability
    let new_available = credit.quantity_available - body.quantity_tons;
    let new_status = if new_available <= 0.0 { "sold" } else { "active" };
    let _ = sqlx::query(
        "UPDATE carbon_credits SET quantity_available = ?, status = ?, updated_at = ? WHERE id = ?"
    )
    .bind(new_available).bind(new_status).bind(&now).bind(&credit.id)
    .execute(&mut *db_tx).await;

    // Record transaction
    let _ = sqlx::query(
        "INSERT INTO transactions (id, buyer_id, seller_id, credit_id, project_id, quantity_tons, price_per_ton, total_price, tx_hash, certification, vintage_year, status, retired, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&tx_id).bind(&claims.sub).bind(&credit.seller_id)
    .bind(&credit.id).bind(&credit.project_id)
    .bind(body.quantity_tons).bind(credit.price_per_ton).bind(total_cost)
    .bind(&tx_hash).bind(&credit.certification).bind(credit.vintage_year)
    .bind("completed").bind(0).bind(&now)
    .execute(&mut *db_tx).await;

    // Update buyer's total credits
    let _ = sqlx::query(
        "UPDATE users SET total_credits_owned = total_credits_owned + ?, updated_at = ? WHERE id = ?"
    )
    .bind(body.quantity_tons).bind(&now).bind(&claims.sub)
    .execute(&mut *db_tx).await;

    // Update portfolio
    let port_exists: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM portfolio WHERE user_id = ? AND credit_id = ?"
    )
    .bind(&claims.sub).bind(&credit.id)
    .fetch_optional(&mut *db_tx).await
    .unwrap_or(None);

    if let Some((port_id,)) = port_exists {
        let _ = sqlx::query(
            "UPDATE portfolio SET quantity_tons = quantity_tons + ?, total_invested = total_invested + ?, updated_at = ? WHERE id = ?"
        )
        .bind(body.quantity_tons).bind(total_cost).bind(&now).bind(&port_id)
        .execute(&mut *db_tx).await;
    } else {
        let new_port_id = Uuid::new_v4().to_string();
        let _ = sqlx::query(
            "INSERT INTO portfolio (id, user_id, credit_id, project_id, quantity_tons, average_buy_price, total_invested, retired_tons, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&new_port_id).bind(&claims.sub).bind(&credit.id).bind(&credit.project_id)
        .bind(body.quantity_tons).bind(credit.price_per_ton).bind(total_cost).bind(0.0)
        .bind(&now).bind(&now)
        .execute(&mut *db_tx).await;
    }

    if db_tx.commit().await.is_err() {
        return HttpResponse::InternalServerError().json(ErrorResponse::new("Transaction failed"));
    }

    HttpResponse::Ok().json(ApiResponse::ok_msg(
        serde_json::json!({
            "tx_id": tx_id,
            "tx_hash": tx_hash,
            "quantity_tons": body.quantity_tons,
            "price_per_ton": credit.price_per_ton,
            "total_cost": total_cost,
            "certification": credit.certification,
            "message": "Purchase successful! Carbon credits added to your portfolio."
        }),
        "Purchase completed successfully"
    ))
}

pub async fn list_new_credit(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    body: web::Json<CreateCreditRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    // Verify project ownership
    let project: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM carbon_projects WHERE id = ? AND (owner_id = ? OR ? = 'admin')"
    )
    .bind(&body.project_id).bind(&claims.sub).bind(&claims.role)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    if project.is_none() {
        return HttpResponse::Forbidden().json(ErrorResponse::new("Project not found or access denied"));
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let serial = format!("{}-{}-{}", body.certification.to_uppercase().replace(' ', "-"),
                         body.vintage_year, &id[..8].to_uppercase());

    let result = sqlx::query(
        "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(&body.project_id).bind(&claims.sub)
    .bind(body.price_per_ton).bind(body.quantity_tons).bind(body.quantity_tons)
    .bind("active").bind(body.vintage_year).bind(&body.certification)
    .bind(&serial).bind(&body.methodology).bind(&now).bind(&now)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(ApiResponse::ok_msg(
            serde_json::json!({ "id": id, "serial_number": serial }),
            "Credit listing created successfully"
        )),
        Err(e) => {
            log::error!("Create credit error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to create listing"))
        }
    }
}

pub async fn get_price_history(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> HttpResponse {
    let credit_id = path.into_inner();
    let history: Vec<PriceHistory> = sqlx::query_as(
        "SELECT * FROM price_history WHERE credit_id = ? ORDER BY recorded_at ASC LIMIT 90"
    )
    .bind(&credit_id)
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(history))
}

// Needed for tx_hash generation
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
