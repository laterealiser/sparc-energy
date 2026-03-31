use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use crate::auth::require_auth;

pub async fn list_projects(pool: web::Data<SqlitePool>) -> HttpResponse {
    let projects: Vec<CarbonProject> = sqlx::query_as(
        "SELECT * FROM carbon_projects ORDER BY verified DESC, created_at DESC"
    )
    .fetch_all(pool.as_ref())
    .await
    .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(projects))
}

pub async fn get_project(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let project: Option<CarbonProject> = sqlx::query_as(
        "SELECT * FROM carbon_projects WHERE id = ?"
    )
    .bind(&project_id)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    match project {
        Some(p) => {
            // Also get credits for this project
            let credits: Vec<CreditWithProject> = sqlx::query_as(
                "SELECT cc.id, cc.project_id, cp.name as project_name, cp.project_type, cp.country,
                        cc.seller_id, u.name as seller_name, cc.price_per_ton, cc.quantity_tons,
                        cc.quantity_available, cc.status, cc.vintage_year, cc.certification,
                        cc.serial_number, cc.methodology, cc.created_at
                 FROM carbon_credits cc
                 JOIN carbon_projects cp ON cc.project_id = cp.id
                 JOIN users u ON cc.seller_id = u.id
                 WHERE cc.project_id = ? AND cc.status = 'active'"
            )
            .bind(&project_id)
            .fetch_all(pool.as_ref())
            .await
            .unwrap_or_default();

            HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                "project": p,
                "credits": credits
            })))
        }
        None => HttpResponse::NotFound().json(ErrorResponse::new("Project not found")),
    }
}

pub async fn create_project(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
    body: web::Json<CreateProjectRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(&body.name).bind(&body.description)
    .bind(&body.project_type).bind(&body.location).bind(&body.country)
    .bind(&claims.sub).bind(body.total_credits).bind(0.0).bind(0)
    .bind(&body.certification).bind(&body.sdg_goals)
    .bind(body.co2_reduction_per_year).bind(&body.project_start_date)
    .bind(&now).bind(&now)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(ApiResponse::ok_msg(
            serde_json::json!({ "id": id }),
            "Project created, pending verification"
        )),
        Err(e) => {
            log::error!("Create project error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to create project"))
        }
    }
}
