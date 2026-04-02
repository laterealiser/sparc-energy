use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use std::env;

mod db;
mod models;
mod auth;
mod handlers;
mod engine;
mod storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("⚡ Sparc Energy Carbon Market Platform [Regeneration] starting...");

    // Initialize Supabase Connection Pool
    let pool = db::create_pool().await;

    // Verify Schema
    if let Err(e) = db::init_schema(&pool).await {
        log::error!("Schema verification error: {}", e);
    }

    // Spawn the Matching Engine in the background
    let engine_pool = pool.clone();
    tokio::spawn(async move {
        engine::matching::run_matching_engine(engine_pool).await;
    });

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    log::info!("🌿 Sparc Energy backend running at http://{}:{}", host, port);

    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%r %s %Dms"))
            .app_data(pool_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let err_msg = format!("Invalid JSON: {}", err);
                actix_web::error::InternalError::from_response(
                    err,
                    actix_web::HttpResponse::BadRequest()
                        .json(models::ErrorResponse::new(&err_msg)),
                )
                .into()
            }))
            // Health check
            .route("/health", web::get().to(|| async {
                actix_web::HttpResponse::Ok().json(serde_json::json!({
                    "status": "ok",
                    "platform": "Sparc Energy Carbon Market",
                    "stack": "Supabase + Mega + Rust",
                    "version": "2.0.0-PRO"
                }))
            }))
            // Auth routes (Supabase Auth logic will be primarily in frontend, 
            // but backend handles profile syncing)
            .service(
                web::scope("/api/auth")
                    .route("/me", web::get().to(handlers::auth::me))
                    .route("/kyc", web::post().to(handlers::auth::submit_kyc))
            )
            // Marketplace routes
            .service(
                web::scope("/api/credits")
                    .route("", web::get().to(handlers::credits::list_credits))
                    .route("", web::post().to(handlers::credits::list_new_credit))
                    .route("/{id}", web::get().to(handlers::credits::get_credit))
                    .route("/order", web::post().to(handlers::credits::place_order))
            )
            // Dashboard / Reports
            .service(
                web::scope("/api/dashboard")
                    .route("", web::get().to(handlers::dashboard::get_dashboard))
                    .route("/retire", web::post().to(handlers::dashboard::retire_credits))
            )
            // Project Registry
            .service(
                web::scope("/api/projects")
                    .route("", web::get().to(handlers::projects::list_projects))
                    .route("", web::post().to(handlers::projects::create_project))
                    .route("/{id}", web::get().to(handlers::projects::get_project))
            )
            // Admin routes
            .service(
                web::scope("/api/admin")
                    .route("/users", web::get().to(handlers::admin::list_users))
                    .route("/projects/{id}/approve", web::post().to(handlers::admin::approve_project))
                    .route("/stats", web::get().to(handlers::admin::admin_stats))
            )
            // Payment Webhooks / Integration
            .service(
                web::scope("/api/payments")
                    .route("/razorpay", web::post().to(handlers::market::handle_razorpay_webhook))
                    .route("/crypto", web::post().to(handlers::market::verify_crypto_tx))
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
