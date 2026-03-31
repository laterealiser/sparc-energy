use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::SqlitePool;
use crate::models::*;
use crate::auth::require_auth;

pub async fn list_users(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };
    if claims.role != "admin" {
        return HttpResponse::Forbidden().json(ErrorResponse::new("Admin access required"));
    }

    let users: Vec<UserPublic> = sqlx::query_as(
        "SELECT id, email, name, role, balance, total_credits_owned, kyc_verified, created_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(users))
}

pub async fn approve_project(
    pool: web::Data<SqlitePool>,
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
        "UPDATE carbon_projects SET verified = 1, updated_at = ? WHERE id = ?"
    )
    .bind(&now).bind(&project_id)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => HttpResponse::Ok().json(ApiResponse::ok_msg(
            serde_json::json!({ "project_id": project_id }),
            "Project verified successfully"
        )),
        _ => HttpResponse::NotFound().json(ErrorResponse::new("Project not found")),
    }
}

pub async fn admin_stats(
    pool: web::Data<SqlitePool>,
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
        .fetch_one(pool.as_ref()).await.unwrap_or((0,));
    let total_projects: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM carbon_projects")
        .fetch_one(pool.as_ref()).await.unwrap_or((0,));
    let total_transactions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM transactions")
        .fetch_one(pool.as_ref()).await.unwrap_or((0,));
    let total_volume: (f64,) = sqlx::query_as("SELECT COALESCE(SUM(total_price), 0) FROM transactions")
        .fetch_one(pool.as_ref()).await.unwrap_or((0.0,));
    let pending_projects: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM carbon_projects WHERE verified = 0"
    )
    .fetch_one(pool.as_ref()).await.unwrap_or((0,));

    HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
        "total_users": total_users.0,
        "total_projects": total_projects.0,
        "total_transactions": total_transactions.0,
        "total_volume": total_volume.0,
        "pending_projects": pending_projects.0
    })))
}
