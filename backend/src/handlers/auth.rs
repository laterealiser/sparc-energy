use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::*;
use crate::auth::{generate_token, require_auth};

pub async fn register(
    pool: web::Data<SqlitePool>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    // Validate input
    if body.name.trim().is_empty() || body.email.trim().is_empty() || body.password.len() < 6 {
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "Name, email are required and password must be at least 6 characters"
        ));
    }

    // Check if user exists
    let existing: Option<(String,)> = sqlx::query_as(
        "SELECT id FROM users WHERE email = ?"
    )
    .bind(&body.email)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    if existing.is_some() {
        return HttpResponse::Conflict().json(ErrorResponse::new("Email already registered"));
    }

    let password_hash = match hash(&body.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json(ErrorResponse::new("Server error")),
    };

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let role = body.role.as_deref().unwrap_or("buyer").to_string();

    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id).bind(&body.email).bind(&password_hash)
    .bind(&body.name).bind(&role).bind(10000.0).bind(0)
    .bind(&now).bind(&now)
    .execute(pool.as_ref())
    .await;

    match result {
        Ok(_) => {
            let token = generate_token(&id, &body.email, &role).unwrap();
            let user = UserPublic {
                id,
                email: body.email.clone(),
                name: body.name.clone(),
                role,
                balance: 10000.0,
                total_credits_owned: 0.0,
                kyc_verified: 0,
                created_at: now,
            };
            HttpResponse::Created().json(ApiResponse::ok_msg(AuthResponse { token, user }, "Account created successfully"))
        }
        Err(e) => {
            log::error!("Register error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse::new("Failed to create account"))
        }
    }
}

pub async fn login(
    pool: web::Data<SqlitePool>,
    body: web::Json<LoginRequest>,
) -> HttpResponse {
    let user: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = ?"
    )
    .bind(&body.email)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    let user = match user {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid email or password")),
    };

    match verify(&body.password, &user.password_hash) {
        Ok(true) => {}
        _ => return HttpResponse::Unauthorized().json(ErrorResponse::new("Invalid email or password")),
    }

    let token = generate_token(&user.id, &user.email, &user.role).unwrap();
    let user_public = UserPublic {
        id: user.id,
        email: user.email,
        name: user.name,
        role: user.role,
        balance: user.balance,
        total_credits_owned: user.total_credits_owned,
        kyc_verified: user.kyc_verified,
        created_at: user.created_at,
    };

    HttpResponse::Ok().json(ApiResponse::ok_msg(AuthResponse { token, user: user_public }, "Login successful"))
}

pub async fn me(
    pool: web::Data<SqlitePool>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match require_auth(&req) {
        Ok(c) => c,
        Err(e) => return HttpResponse::Unauthorized().json(ErrorResponse::new(&e.to_string())),
    };

    let user: Option<UserPublic> = sqlx::query_as(
        "SELECT id, email, name, role, balance, total_credits_owned, kyc_verified, created_at FROM users WHERE id = ?"
    )
    .bind(&claims.sub)
    .fetch_optional(pool.as_ref())
    .await
    .unwrap_or(None);

    match user {
        Some(u) => HttpResponse::Ok().json(ApiResponse::ok(u)),
        None => HttpResponse::NotFound().json(ErrorResponse::new("User not found")),
    }
}
