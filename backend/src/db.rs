use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::fs;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Ensure the database file directory exists
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent).ok();
            }
        }
    }

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let schema = include_str!("../migrations/001_init.sql");
    
    // Execute each statement
    for statement in schema.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            sqlx::query(stmt).execute(pool).await?;
        }
    }
    
    log::info!("Database migrations completed successfully");
    Ok(())
}

pub async fn seed_data(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    use uuid::Uuid;
    use chrono::Utc;
    use bcrypt::{hash, DEFAULT_COST};

    // Check if already seeded
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    
    if count.0 > 0 {
        log::info!("Database already seeded, skipping...");
        return Ok(());
    }

    log::info!("Seeding database with sample data...");
    let now = Utc::now().to_rfc3339();

    // Create admin user
    let admin_id = Uuid::new_v4().to_string();
    let admin_hash = hash("Admin@123", DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&admin_id).bind("admin@sparcenergy.com").bind(&admin_hash)
    .bind("Sparc Admin").bind("admin").bind(1000000.0).bind(1)
    .bind(&now).bind(&now)
    .execute(pool).await?;

    // Create seller users
    let seller1_id = Uuid::new_v4().to_string();
    let seller_hash = hash("Seller@123", DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&seller1_id).bind("greenforest@reforestation.com").bind(&seller_hash)
    .bind("Amazon Reforestation Ltd").bind("seller").bind(250000.0).bind(1)
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let seller2_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&seller2_id).bind("solar@renewableindia.com").bind(&seller_hash)
    .bind("Renewable India Power").bind("seller").bind(180000.0).bind(1)
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let seller3_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&seller3_id).bind("wind@nordicclean.com").bind(&seller_hash)
    .bind("Nordic Clean Energy").bind("seller").bind(320000.0).bind(1)
    .bind(&now).bind(&now)
    .execute(pool).await?;

    // Demo buyer
    let buyer_id = Uuid::new_v4().to_string();
    let buyer_hash = hash("Demo@123", DEFAULT_COST).unwrap();
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, balance, kyc_verified, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&buyer_id).bind("demo@sparcenergy.com").bind(&buyer_hash)
    .bind("Demo Investor").bind("buyer").bind(50000.0).bind(1)
    .bind(&now).bind(&now)
    .execute(pool).await?;

    // Create projects
    let p1_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&p1_id)
    .bind("Amazon Reforestation Initiative")
    .bind("Large-scale reforestation project in the Brazilian Amazon, protecting 50,000 hectares of tropical rainforest and supporting local indigenous communities.")
    .bind("reforestation")
    .bind("Amazon Basin, Pará State")
    .bind("Brazil")
    .bind(&seller1_id)
    .bind(500000.0)
    .bind(350000.0)
    .bind(1)
    .bind("Verra VCS")
    .bind("13,15,17")
    .bind(85000.0)
    .bind("2020-01-01")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let p2_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&p2_id)
    .bind("Rajasthan Solar Farm")
    .bind("800 MW utility-scale solar power plant in Rajasthan providing clean electricity to 600,000 homes and displacing coal-fired generation.")
    .bind("solar")
    .bind("Jodhpur, Rajasthan")
    .bind("India")
    .bind(&seller2_id)
    .bind(200000.0)
    .bind(180000.0)
    .bind(1)
    .bind("Gold Standard")
    .bind("7,9,13")
    .bind(42000.0)
    .bind("2021-06-01")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let p3_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&p3_id)
    .bind("North Sea Wind Offshore")
    .bind("Offshore wind farm in the North Sea generating 1.2 GW of clean energy for Northern Europe, replacing fossil fuel generation.")
    .bind("wind")
    .bind("North Sea, 80km offshore")
    .bind("Norway")
    .bind(&seller3_id)
    .bind(300000.0)
    .bind(280000.0)
    .bind(1)
    .bind("Gold Standard")
    .bind("7,8,13")
    .bind(65000.0)
    .bind("2019-03-01")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let p4_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_projects (id, name, description, project_type, location, country, owner_id, total_credits, credits_issued, verified, certification, sdg_goals, co2_reduction_per_year, project_start_date, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&p4_id)
    .bind("Gujarat Mangrove Conservation")
    .bind("Protection and restoration of 12,000 hectares of mangrove ecosystems along Gujarat coast, providing blue carbon sequestration.")
    .bind("blue_carbon")
    .bind("Gulf of Khambhat, Gujarat")
    .bind("India")
    .bind(&seller2_id)
    .bind(150000.0)
    .bind(90000.0)
    .bind(1)
    .bind("Verra VCS")
    .bind("14,15,13")
    .bind(28000.0)
    .bind("2022-01-01")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    // Create carbon credit listings
    let c1_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&c1_id).bind(&p1_id).bind(&seller1_id)
    .bind(18.50).bind(50000.0).bind(50000.0)
    .bind("active").bind(2023).bind("Verra VCS")
    .bind("VCS-BRA-2023-001").bind("VM0007")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let c2_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&c2_id).bind(&p2_id).bind(&seller2_id)
    .bind(22.75).bind(30000.0).bind(30000.0)
    .bind("active").bind(2024).bind("Gold Standard")
    .bind("GS-IND-2024-001").bind("AMS-I.D")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let c3_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&c3_id).bind(&p3_id).bind(&seller3_id)
    .bind(31.20).bind(25000.0).bind(25000.0)
    .bind("active").bind(2024).bind("Gold Standard")
    .bind("GS-NOR-2024-001").bind("AMS-I.D")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let c4_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&c4_id).bind(&p4_id).bind(&seller2_id)
    .bind(14.80).bind(20000.0).bind(20000.0)
    .bind("active").bind(2023).bind("Verra VCS")
    .bind("VCS-IND-2023-002").bind("VM0033")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    let c5_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO carbon_credits (id, project_id, seller_id, price_per_ton, quantity_tons, quantity_available, status, vintage_year, certification, serial_number, methodology, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&c5_id).bind(&p1_id).bind(&seller1_id)
    .bind(16.40).bind(40000.0).bind(40000.0)
    .bind("active").bind(2022).bind("Verra VCS")
    .bind("VCS-BRA-2022-001").bind("VM0007")
    .bind(&now).bind(&now)
    .execute(pool).await?;

    // Add price history for charts
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let credits = vec![
        (&c1_id, 18.50_f64),
        (&c2_id, 22.75_f64),
        (&c3_id, 31.20_f64),
        (&c4_id, 14.80_f64),
        (&c5_id, 16.40_f64),
    ];

    for (credit_id, base_price) in &credits {
        let mut price = *base_price * 0.75;
        for i in 0..30 {
            let change: f64 = rng.gen_range(-0.8..1.2);
            price = (price + change).max(5.0);
            let ts = Utc::now() - chrono::Duration::days(30 - i);
            let hist_id = Uuid::new_v4().to_string();
            let volume: f64 = rng.gen_range(500.0..5000.0);
            sqlx::query(
                "INSERT INTO price_history (id, credit_id, price, volume, recorded_at) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&hist_id).bind(credit_id).bind(price).bind(volume)
            .bind(ts.to_rfc3339())
            .execute(pool).await?;
        }
    }

    log::info!("✅ Database seeded successfully with Sparc Energy sample data");
    Ok(())
}
