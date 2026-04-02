use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use crate::models::*;
use crate::auth::require_auth;
use crate::db::DbPool;

// 1. List Carbon Credits available on the marketplace (Postgres)
pub async fn list_credits(
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let sql = "SELECT id, project_id, seller_id, vintage_year, quantity_tons, 
               quantity_available, price_per_ton, serial_number_start, serial_number_end, status
               FROM carbon_credits
               WHERE status = 'active'";

    match sqlx::query_as::<_, CarbonCredit>(sql).fetch_all(pool.get_ref()).await {
        Ok(credits) => HttpResponse::Ok().json(ApiResponse::ok(credits)),
        Err(e) => {
            log::error!("Database error in list_credits: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Database error"))
        }
    }
}

// 2. Place a Market Order (Bid or Ask)
pub async fn place_order(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<PlaceOrderRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    // KYC check (Must be verified to trade)
    let kyc_sql = "SELECT kyc_status FROM users WHERE id = $1";
    let kyc_status = match sqlx::query_scalar::<_, String>(kyc_sql)
        .bind(&claims.sub)
        .fetch_one(pool.get_ref())
        .await {
            Ok(s) => s,
            Err(e) => {
                log::error!("User search error: {}", e);
                return HttpResponse::Forbidden().json(ErrorResponse::new("User not found or database error"))
            },
        };

    if kyc_status != "verified" {
        return HttpResponse::Forbidden().json(ErrorResponse::new("KYC verification required for trading. Please submit documents via Mega/Supabase."));
    }

    let order_id = Uuid::new_v4().to_string();
    let sql = "INSERT INTO market_orders (id, user_id, credit_id, order_type, price, quantity, status)
               VALUES ($1, $2, $3, $4, $5, $6, 'open')";
    
    let result = sqlx::query(sql)
        .bind(&order_id) 
        .bind(&claims.sub) 
        .bind(&body.credit_id) 
        .bind(&body.order_type) 
        .bind(&body.price) 
        .bind(&body.quantity)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Ok().json(ApiResponse::ok_msg(
                serde_json::json!({ "order_id": order_id }),
                "Order placed successfully in Supabase marketplace"
            ))
        }
        Err(e) => {
            log::error!("Order error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to place order"))
        }
    }
}

// 3. Register a New Carbon Credit Batch
pub async fn list_new_credit(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateCreditRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    // Serial Number Generation logic
    let id = Uuid::new_v4().to_string();
    let serial_prefix = format!("{}-{}-{}", body.certification.to_uppercase(), body.vintage_year, &id[..4]);
    let serial_start = format!("{}-000001", serial_prefix);
    let serial_end = format!("{}-{:06}", serial_prefix, body.quantity_tons as i32);

    let sql = "INSERT INTO carbon_credits (id, project_id, seller_id, vintage_year, quantity_tons, quantity_available, price_per_ton, serial_number_start, serial_number_end, status)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'active')";
    
    let result = sqlx::query(sql)
        .bind(&id)
        .bind(&body.project_id)
        .bind(&claims.sub)
        .bind(&body.vintage_year)
        .bind(&body.quantity_tons)
        .bind(&body.quantity_tons)
        .bind(&body.price_per_ton)
        .bind(&serial_start)
        .bind(&serial_end)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            HttpResponse::Created().json(ApiResponse::ok_msg(
                serde_json::json!({ "id": id, "serial_range": format!("{} to {}", serial_start, serial_end) }),
                "Credit listing registered in Supabase. Verification documents should be uploaded to Mega."
            ))
        }
        Err(e) => {
            log::error!("Create credit error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to register listing"))
        }
    }
}

pub async fn get_credit(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let credit_id = path.into_inner();
    let sql = "SELECT id, project_id, seller_id, vintage_year, quantity_tons, 
               quantity_available, price_per_ton, serial_number_start, serial_number_end, status 
               FROM carbon_credits WHERE id = $1";
    
    let credit = sqlx::query_as::<_, CarbonCredit>(sql)
        .bind(&credit_id)
        .fetch_optional(pool.get_ref())
        .await;

    match credit {
        Ok(Some(c)) => HttpResponse::Ok().json(ApiResponse::ok(c)),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse::new("Credit not found")),
        Err(e) => {
            log::error!("Database error in get_credit: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Database error"))
        }
    }
}
