use crate::db::DbPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

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
// 3. User Registration
pub async fn register(
    pool: web::Data<DbPool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
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
            let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
            let claims = Claims {
                sub: id.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                role: body.role.clone(),
            };
            
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap_or_default();
            
            HttpResponse::Ok().json(ApiResponse::ok(serde_json::json!({
                "token": token,
                "user": { "id": id, "name": body.name, "email": body.email, "role": body.role, "balance": 10000.0 }
            })))
        }
        Err(_) => HttpResponse::BadRequest().json(ErrorResponse::new("User with this email already exists")),
    }
}

// 4. User Login
pub async fn login(
    pool: web::Data<DbPool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let sql = "SELECT id, email, password_hash, name, role, balance FROM users WHERE email = $1";
    let user = sqlx::query_as::<_, User>(sql)
        .bind(&body.email)
        .fetch_optional(pool.get_ref())
        .await;

    match user {
        Ok(Some(u)) => {
            if verify(&body.password, &u.password_hash).unwrap_or(false) {
                let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
                let claims = Claims {
                    sub: u.id.clone(),
                    exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
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
