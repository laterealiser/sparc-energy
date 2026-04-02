use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::*;
use crate::auth::require_auth;
use crate::db::DbPool;

// 1. Get current user profile from PostgreSql (Supabase)
pub async fn me(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(e) => return HttpResponse::Unauthorized().json(ErrorResponse::new(&e.to_string())),
    };

    let sql = "SELECT id, email, name, role, balance, kyc_status, two_factor_enabled, created_at 
               FROM users WHERE id = $1";
    
    let user = sqlx::query_as::<_, User>(sql)
        .bind(&claims.sub)
        .fetch_optional(pool.get_ref())
        .await;

    match user {
        Ok(Some(u)) => HttpResponse::Ok().json(ApiResponse::ok(u)),
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse::new("User profile not found")),
        Err(e) => {
            log::error!("Database error in /me: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Database error"))
        }
    }
}

// 2. Submit KYC application (Mega for documents)
pub async fn submit_kyc(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<KYCRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    let kyc_id = uuid::Uuid::new_v4().to_string();

    // Use Postgres for the transaction record
    let sql = "INSERT INTO kyc_applications (id, user_id, first_name, last_name, id_type, id_number, document_url, status)
               VALUES ($1, $2, $3, $4, $5, $6, $7, 'submitted')";
    
    let result = sqlx::query(sql)
        .bind(&kyc_id)
        .bind(&claims.sub)
        .bind(&body.first_name)
        .bind(&body.last_name)
        .bind(&body.id_type)
        .bind(&body.id_number)
        .bind(&body.document_url) // This URL will now point to Mega/Supabase
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            // Update user status
            let _ = sqlx::query("UPDATE users SET kyc_status = 'submitted' WHERE id = $1")
                .bind(&claims.sub)
                .execute(pool.get_ref())
                .await;

            HttpResponse::Ok().json(ApiResponse::ok_msg(
                serde_json::json!({ "application_id": kyc_id }),
                "KYC application submitted successfully. Documents moved to secure storage."
            ))
        }
        Err(e) => {
            log::error!("KYC error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to submit KYC"))
        }
    }
}
