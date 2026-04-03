use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::*;
use crate::auth::require_auth;
use crate::db::DbPool;

// 1. List Users (Admin only, from Supabase)
pub async fn list_users(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse::new("Admin access required"));
    }

    let users: Vec<User> = sqlx::query_as::<_, User>(
        "SELECT id, email, name, role, balance, kyc_status, two_factor_enabled, created_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(users))
}

// 2. Verify/Approve Project (Admin action on Supabase)
pub async fn approve_project(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse::new("Admin access required"));
    }

    let project_id = path.into_inner();
    let now = chrono::Utc::now().to_rfc3339();
    
    let result = sqlx::query(
        "UPDATE carbon_projects SET verified = 1, updated_at = $1 WHERE id = $2"
    )
    .bind(&now).bind(&project_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().json(ApiResponse::ok_msg(
            serde_json::json!({ "project_id": project_id }),
            "Project verified successfully on registry"
        )),
        _ => HttpResponse::NotFound().json(ErrorResponse::new("Project not found")),
    }
}

// 3. Global Stats for Admin Panel
pub async fn admin_stats(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse::new("Admin access required"));
    }

    let total_users: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool.get_ref()).await.unwrap_or((0,));
    let total_projects: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM carbon_projects")
        .fetch_one(pool.get_ref()).await.unwrap_or((0,));
    let total_transactions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM transactions")
        .fetch_one(pool.get_ref()).await.unwrap_or((0,));
    
    let pending_projects: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM carbon_projects WHERE verified = 0"
    )
    .fetch_one(pool.get_ref()).await.unwrap_or((0,));

    HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
        "total_users": total_users.0,
        "total_projects": total_projects.0,
        "total_transactions": total_transactions.0,
        "pending_projects": pending_projects.0
    })))
}

// 4. Verify/Approve User KYC
pub async fn verify_kyc(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse::new("Admin access required"));
    }

    let user_id = path.into_inner();
    let now = chrono::Utc::now().to_rfc3339();
    
    let result = sqlx::query(
        "UPDATE users SET kyc_status = 'verified', updated_at = $1 WHERE id = $2"
    )
    .bind(&now).bind(&user_id)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().json(ApiResponse::ok_msg(
            serde_json::json!({ "user_id": user_id }),
            "User KYC verified successfully"
        )),
        _ => HttpResponse::NotFound().json(ErrorResponse::new("User not found")),
    }
}
