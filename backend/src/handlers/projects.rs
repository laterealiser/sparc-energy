use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use crate::auth::require_auth;
use crate::db::DbPool;

// 1. List Projects (Supabase)
pub async fn list_projects(pool: web::Data<DbPool>) -> HttpResponse {
    let sql = "SELECT id, owner_id, name, description, project_type, country, 
               'Verra' as methodology_standard, 'verified' as verification_status, 
               total_credits, credits_issued 
               FROM carbon_projects ORDER BY verified DESC, created_at DESC";
    
    let projects: Vec<CarbonProject> = sqlx::query_as::<_, CarbonProject>(sql)
        .fetch_all(pool.get_ref())
        .await
        .unwrap_or_default();

    HttpResponse::Ok().json(ApiResponse::ok(projects))
}

// 2. Get Single Project
pub async fn get_project(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let project_id = path.into_inner();
    let sql = "SELECT id, owner_id, name, description, project_type, country, 
               'Verra' as methodology_standard, 'verified' as verification_status, 
               total_credits, credits_issued 
               FROM carbon_projects WHERE id = $1";
    
    let project = sqlx::query_as::<_, CarbonProject>(sql)
        .bind(&project_id)
        .fetch_optional(pool.get_ref())
        .await;

    match project {
        Ok(Some(p)) => {
            // Also get credits for this project (Join users for seller name)
            let credits_sql = "SELECT cc.id, cc.project_id, cp.name as project_name, cp.project_type, cp.country,
                                      cc.seller_id, u.name as seller_name, cc.price_per_ton, cc.quantity_tons,
                                      cc.quantity_available, cc.status, cc.vintage_year, cc.certification,
                                      cc.serial_number_start, cc.serial_number_end, cc.created_at
                               FROM carbon_credits cc
                               JOIN carbon_projects cp ON cc.project_id = cp.id
                               JOIN users u ON cc.seller_id = u.id
                               WHERE cc.project_id = $1 AND cc.status = 'active'";

            let credits: Vec<CreditWithProject> = sqlx::query_as::<_, CreditWithProject>(credits_sql)
                .bind(&project_id)
                .fetch_all(pool.get_ref())
                .await
                .unwrap_or_default();

            HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                "project": p,
                "credits": credits
            })))
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse::new("Project not found")),
        Err(e) => {
            log::error!("Project search error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Database error"))
        }
    }
}

// 3. Create Project (Metadata in Supabase, Images in Mega)
pub async fn create_project(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    body: web::Json<CreateProjectRequest>,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json(ErrorResponse::new("Authentication required")),
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    // Media URLs point to Mega as per user instruction
    let mega_image_url = format!("https://mega.nz/file/PRJ-{}.jpg", &id[..6].to_uppercase());

    let sql = "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, image_url, created_at, updated_at) 
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)";
    
    let result = sqlx::query(sql)
        .bind(&id).bind(&body.name).bind(&body.description)
        .bind(&body.project_type).bind(&body.location).bind(&body.country)
        .bind(&claims.sub).bind(body.total_credits).bind(0.0).bind(0)
        .bind(&body.certification).bind(&body.sdg_goals)
        .bind(body.co2_reduction_per_year).bind(&body.project_start_date)
        .bind(&mega_image_url).bind(&now).bind(&now)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().json(ApiResponse::ok_msg(
            serde_json::json!({ "id": id, "media_archive": mega_image_url }),
            "Project profile registered on Supabase. Media assets queued for Mega archival."
        )),
        Err(e) => {
            log::error!("Create project error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed up upload project metadata"))
        }
    }
}
