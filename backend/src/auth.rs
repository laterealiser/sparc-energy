use actix_web::{HttpMessage, HttpRequest, web};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::models::Claims;

pub const JWT_SECRET: &str = "sparc_energy_super_secret_jwt_key_2024_carbon_market";
pub const JWT_EXPIRY_HOURS: usize = 24 * 7; // 7 days

pub fn generate_token(user_id: &str, email: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        iat: now,
        exp: now + (JWT_EXPIRY_HOURS * 3600),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

pub fn extract_claims(req: &HttpRequest) -> Option<Claims> {
    let auth_header = req.headers().get("Authorization")?;
    let auth_str = auth_header.to_str().ok()?;
    let token = auth_str.strip_prefix("Bearer ")?;
    verify_token(token).ok()
}

pub fn require_auth(req: &HttpRequest) -> Result<Claims, actix_web::Error> {
    extract_claims(req).ok_or_else(|| {
        actix_web::error::ErrorUnauthorized("Missing or invalid authentication token")
    })
}

pub fn require_role(req: &HttpRequest, role: &str) -> Result<Claims, actix_web::Error> {
    let claims = require_auth(req)?;
    if claims.role != role && claims.role != "admin" {
        return Err(actix_web::error::ErrorForbidden("Insufficient permissions"));
    }
    Ok(claims)
}
