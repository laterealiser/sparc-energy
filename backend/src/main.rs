use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use std::env;

mod db;
mod models;
mod auth;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // DATABASE_URL is now REQUIRED for production security
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set in production");

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    log::info!("⚡ Sparc Energy Carbon Market Platform starting...");
    log::info!("📦 Connecting to database...");

    let pool = db::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");

    db::run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    db::seed_data(&pool)
        .await
        .expect("Failed to seed database");

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
                    "version": "1.0.0"
                }))
            }))
            // Auth routes
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(handlers::auth::register))
                    .route("/login", web::post().to(handlers::auth::login))
                    .route("/me", web::get().to(handlers::auth::me))
            )
            // Credits routes
            .service(
                web::scope("/api/credits")
                    .route("", web::get().to(handlers::credits::list_credits))
                    .route("", web::post().to(handlers::credits::list_new_credit))
                    .route("/{id}", web::get().to(handlers::credits::get_credit))
                    .route("/{id}/history", web::get().to(handlers::credits::get_price_history))
                    .route("/buy", web::post().to(handlers::credits::buy_credit))
            )
            // Projects routes
            .service(
                web::scope("/api/projects")
                    .route("", web::get().to(handlers::projects::list_projects))
                    .route("", web::post().to(handlers::projects::create_project))
                    .route("/{id}", web::get().to(handlers::projects::get_project))
            )
            // Market routes
            .service(
                web::scope("/api/market")
                    .route("/stats", web::get().to(handlers::market::get_market_stats))
                    .route("/trades", web::get().to(handlers::market::get_recent_trades))
                    .route("/leaderboard", web::get().to(handlers::market::get_leaderboard))
            )
            // Dashboard routes
            .service(
                web::scope("/api/dashboard")
                    .route("", web::get().to(handlers::dashboard::get_dashboard))
                    .route("/retire", web::post().to(handlers::dashboard::retire_credits))
            )
            // Admin routes
            .service(
                web::scope("/api/admin")
                    .route("/users", web::get().to(handlers::admin::list_users))
                    .route("/stats", web::get().to(handlers::admin::admin_stats))
                    .route("/projects/{id}/approve", web::post().to(handlers::admin::approve_project))
            )
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
