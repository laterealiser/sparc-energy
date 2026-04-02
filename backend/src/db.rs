use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (postgresql://postgres:[password]@db.[project-id].supabase.co:5432/postgres)");

    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Supabase PostgreSQL.")
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
