use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{Pool, Postgres, ConnectOptions};
use std::env;
use std::str::FromStr;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (postgresql://postgres.project-id:[password]@aws-0-region.pooler.supabase.com:6543/postgres)");

    // Log the DB host (masked) for debugging
    if let Ok(url) = url::Url::parse(&database_url) {
        log::info!("Connecting to Database: {} at port {}", url.host_str().unwrap_or("unknown"), url.port().unwrap_or(5432));
        log::info!("Database Username: {}", url.username());
    }

    // Supabase Transaction Pooler (port 6543) requires statement_cache_capacity(0)
    let options = PgConnectOptions::from_str(&database_url)
        .expect("Failed to parse DATABASE_URL. Ensure special characters like $ are percent-encoded as %24.")
        .statement_cache_capacity(0);

    let pool_result = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(45))
        .connect_with(options)
        .await;

    match pool_result {
        Ok(pool) => pool,
        Err(e) => {
            log::error!("CRITICAL: Failed to connect to Supabase PostgreSQL.");
            log::error!("ERROR DETAIL: {}", e);
            panic!("Database connection failed during startup. Check your Render Environment Variables.");
        }
    }
}

pub async fn init_schema(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Check if tables exist
    let row: (i64,) = sqlx::query_as("SELECT count(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_name = 'users'")
        .fetch_one(pool)
        .await?;

    if row.0 == 0 {
        log::warn!("Tables not found. Please run backend/migrations/001_init.sql in your Supabase SQL Editor.");
    } else {
        log::info!("Supabase (PostgreSQL) connection verified.");
    }

    Ok(())
}
