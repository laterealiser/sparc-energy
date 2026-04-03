use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use crate::models::*;
use crate::auth::require_auth;
use crate::db::DbPool;

// 1. List Contracts for a User (Client or Provider)
pub async fn list_contracts(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };

    let sql = "SELECT id, client_id, provider_id, project_id, total_amount, escrow_balance, status, created_at, updated_at 
               FROM service_contracts 
               WHERE client_id = $1 OR provider_id = $1 
               ORDER BY created_at DESC";
    
    let contracts: Vec<ServiceContract> = sqlx::query_as::<_, ServiceContract>(sql)
        .bind(&claims.sub)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(contracts))
}

// 2. Create Service Contract (Hire Expert)
pub async fn create_contract(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };

    let provider_id = body["provider_id"].as_str().unwrap_or_default();
    let amount = body["amount"].as_f64().unwrap_or(0.0);
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let sql = "INSERT INTO service_contracts (id, client_id, provider_id, total_amount, escrow_balance, status, created_at, updated_at) 
               VALUES ($1, $2, $3, $4, $5, 'active', $6, $6)";
    
    let result = sqlx::query(sql)
        .bind(&id).bind(&claims.sub).bind(&provider_id)
        .bind(amount).bind(0.0) // Initial escrow is 0 until deposit
        .bind(&now)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().json(ApiResponse::ok_msg(id, "Contract created. Please deposit to escrow.")),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse::new(&e.to_string())),
    }
}

// 3. Update Milestone Status
pub async fn update_milestone(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Unauthorized")),
    };

    let milestone_id = path.into_inner();
    let status = body["status"].as_str().unwrap_or("completed");
    let now = chrono::Utc::now().to_rfc3339();

    let sql = "UPDATE service_milestones SET status = $1, completed_at = $2 WHERE id = $3";
    
    let result = sqlx::query(sql)
        .bind(status).bind(&now).bind(&milestone_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::ok_msg(milestone_id, "Milestone updated")),
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse::new(&e.to_string())),
    }
}
