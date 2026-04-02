use sqlx::postgres::{PgPoolOptions, PgConnectOptions};
use sqlx::{Pool, Postgres, ConnectOptions};
use std::env;
use std::str::FromStr;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> DbPool {
    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            log::error!("❌ CRITICAL ERROR: DATABASE_URL environment variable is MISSING on Render!");
            log::error!("Please add DATABASE_URL in Render Dashboard > Environment.");
            // Return a dummy pool or wait to allow logs to flush
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            std::process::exit(1);
        }
    };

    log::info!("🔍 Checking Database URL format...");
    if database_url.contains("pooler.supabase.com") && !database_url.contains(".loldpnnmjqttgvsxcgnr") {
        log::warn!("⚠️ WARNING: Your DATABASE_URL is missing the '.project-id' prefix in the username. This WILL fail on Supabase Pooler.");
    }

    let options = match PgConnectOptions::from_str(&database_url) {
        Ok(opt) => opt.statement_cache_capacity(0),
        Err(e) => {
            log::error!("❌ CRITICAL ERROR: Could not parse DATABASE_URL!");
            log::error!("DETAIL: {}", e);
            log::error!("HINT: Ensure special characters like $ are written as %24");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            std::process::exit(1);
        }
    };

    log::info!("🚀 Attempting to connect to Supabase (Pooler Mode)...");
    
    match PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(std::time::Duration::from_secs(45))
        .connect_with(options)
        .await 
    {
        Ok(pool) => {
            log::info!("✅ Supabase (PostgreSQL) connection established!");
            pool
        },
        Err(e) => {
            log::error!("❌ CRITICAL ERROR: Database Connection Failed!");
            log::error!("DB ERROR: {}", e);
            log::error!("HINT: If 'Tenant not found', update your Render Environment Variable to use 'postgres.loldpnnmjqttgvsxcgnr'");
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            std::process::exit(1);
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
