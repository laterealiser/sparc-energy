use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::*;
use crate::auth::*;
use crate::db::DbPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use uuid::Uuid;
use chrono::Utc;

// 1. Get current user profile from PostgreSql (Supabase)
pub async fn me(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(e) => return HttpResponse::Unauthorized().json(ErrorResponse::new(&e.to_string())),
    };

    let sql = "SELECT id, email, password_hash, name, role, balance, kyc_status, two_factor_enabled, created_at, updated_at 
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

    let kyc_id = Uuid::new_v4().to_string();

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

// 3. User Registration
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let hashed_password = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse::new("Password hashing failed")),
    };

    let sql = "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_status, created_at, updated_at) 
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)";
    
    let result = sqlx::query(sql)
        .bind(&id).bind(&body.email).bind(&hashed_password)
        .bind(&body.name).bind(&body.role).bind(10000.0).bind("pending")
        .bind(&now).bind(&now)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => {
            // If the role is professional, create a profile entry
            if body.role == "pdd_writer" || body.role == "auditor" {
                let prof_sql = "INSERT INTO professional_profiles (user_id, title, rating, completed_projects, verified, created_at, updated_at) 
                                VALUES ($1, $2, 0.0, 0, 0, $3, $4)";
                let title = if body.role == "pdd_writer" { "Carbon Project Developer" } else { "Independent Auditor" };
                let _ = sqlx::query(prof_sql)
                    .bind(&id).bind(title).bind(&now).bind(&now)
                    .execute(pool.get_ref())
                    .await;
            }

            let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            let timestamp = Utc::now().timestamp();
            let claims = Claims {
                sub: id.clone(),
                email: body.email.clone(),
                iat: timestamp,
                exp: timestamp + (24 * 3600),
                role: body.role.clone(),
            };
            
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap_or_default();
            
            HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                "token": token,
                "user": { "id": id, "name": body.name, "email": body.email, "role": body.role, "balance": 10000.0 }
            })))
        }
        Err(e) => {
            log::error!("Registration database error: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse::new("User with this email already exists"))
        }
    }
}

// 4. User Login
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let sql = "SELECT id, email, password_hash, name, role, balance, kyc_status, two_factor_enabled, created_at, updated_at FROM users WHERE email = $1";
    let user = sqlx::query_as::<_, User>(sql)
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await;

    match user {
        Ok(Some(u)) => {
            if verify(&body.password, &u.password_hash).unwrap_or(false) {
                let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
                let timestamp = Utc::now().timestamp();
                let claims = Claims {
                    sub: u.id.clone(),
                    email: u.email.clone(),
                    iat: timestamp,
                    exp: timestamp + (24 * 3600),
                    role: u.role.clone(),
                };
                
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap_or_default();
                
                HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                    "token": token,
                    "user": u
                })))
            } else {
                HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid email or password"))
            }
        }
        _ => HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid email or password")),
    }
}

// 5. List Professionals (PDD Writers, Auditors)
pub async fn list_professionals(
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let sql = "SELECT u.id, u.name, u.email, u.role, p.title, p.bio, p.skills, p.hourly_rate, 
                      p.rating, p.completed_projects, p.accreditation_id, p.verified
               FROM users u
               JOIN professional_profiles p ON u.id = p.user_id
               WHERE u.role IN ('pdd_writer', 'auditor')";
    
    let result = sqlx::query(sql)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let mut professionals = Vec::new();
            for row in rows {
                use sqlx::Row;
                professionals.push(serde_json::json!({
                    "id": row.get::<String, _>("id"),
                    "name": row.get::<String, _>("name"),
                    "role": row.get::<String, _>("role"),
                    "title": row.get::<String, _>("title"),
                    "bio": row.get::<Option<String>, _>("bio"),
                    "skills": row.get::<Option<String>, _>("skills"),
                    "hourly_rate": row.get::<Option<f64>, _>("hourly_rate"),
                    "rating": row.get::<f64, _>("rating"),
                    "completed_projects": row.get::<i32, _>("completed_projects"),
                    "accreditation_id": row.get::<Option<String>, _>("accreditation_id"),
                    "verified": row.get::<i32, _>("verified") == 1
                }));
            }
            HttpResponse::Ok().json(ApiResponse::ok(professionals))
        }
        Err(e) => {
            log::error!("List professionals error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to fetch professionals"))
        }
    }
}
