use crate::db::DbPool;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use chrono::Utc;

pub async fn run_matching_engine(pool: DbPool) {
    log::info!("⚡ Sparc Energy Matching Engine [Supabase/Postgres] started...");
    
    loop {
        if let Err(e) = match_orders(&pool).await {
            log::error!("Matching Error: {}", e);
        }
        sleep(Duration::from_secs(5)).await; // Poll every 5 seconds
    }
}

async fn match_orders(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get ALL open BIDS (Buy Orders) sorted by Price (Desc) and Time (Asc)
    let bid_sql = "SELECT id, user_id, credit_id, price, quantity, filled_quantity 
                   FROM market_orders 
                   WHERE order_type = 'bid' AND status IN ('open', 'partially_filled') 
                   ORDER BY price DESC, created_at ASC";
    
    let bids = sqlx::query_as::<_, (String, String, String, f64, f64, f64)>(bid_sql)
        .fetch_all(pool)
        .await?;

    for (bid_id, buyer_id, credit_id, bid_price, bid_qty, bid_filled) in bids {
        let bid_rem = bid_qty - bid_filled;
        if bid_rem <= 0.0 { continue; }

        // 2. Find matching ASKS (Sell Orders) for the SAME Credit ID
        let ask_sql = "SELECT id, user_id, price, quantity, filled_quantity 
                       FROM market_orders 
                       WHERE order_type = 'ask' AND credit_id = $1 
                       AND status IN ('open', 'partially_filled') 
                       AND price <= $2
                       ORDER BY price ASC, created_at ASC";

        let mut matched_asks = sqlx::query_as::<_, (String, String, f64, f64, f64)>(ask_sql)
            .bind(&credit_id)
            .bind(&bid_price)
            .fetch_all(pool)
            .await?;

        for (ask_id, seller_id, ask_price, ask_qty, ask_filled) in matched_asks {
            let ask_rem = ask_qty - ask_filled;
            if ask_rem <= 0.0 { continue; }

            // 3. Match found! Calculate Trade Quantity
            let trade_qty = bid_rem.min(ask_rem);
            let trade_id = Uuid::new_v4().to_string();
            let fee_rate = 0.025; // 2.5% platform fee

            log::info!("🌱 Matching trade: {} for {:.2} tons @ ${:.2}", credit_id, trade_qty, ask_price);

            // 4. Update Database (Atomic Trade Settlement)
            // Using a transaction for atomicity in Postgres
            let mut tx = pool.begin().await?;

            // Update Bid Order
            sqlx::query("UPDATE market_orders SET filled_quantity = filled_quantity + $1, 
                         status = CASE WHEN filled_quantity + $1 >= quantity THEN 'filled' ELSE 'partially_filled' END 
                         WHERE id = $2")
                .bind(&trade_qty).bind(&bid_id).execute(&mut *tx).await?;

            // Update Ask Order
            sqlx::query("UPDATE market_orders SET filled_quantity = filled_quantity + $1, 
                         status = CASE WHEN filled_quantity + $1 >= quantity THEN 'filled' ELSE 'partially_filled' END 
                         WHERE id = $2")
                .bind(&trade_qty).bind(&ask_id).execute(&mut *tx).await?;

            // Record the trade
            sqlx::query("INSERT INTO trades (id, buyer_id, seller_id, credit_id, bid_order_id, ask_order_id, quantity, price, total_value, platform_fee, status)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 'completed')")
                .bind(&trade_id).bind(&buyer_id).bind(&seller_id).bind(&credit_id).bind(&bid_id).bind(&ask_id)
                .bind(&trade_qty).bind(&ask_price).bind(&(trade_qty * ask_price)).bind(&(trade_qty * ask_price * fee_rate))
                .execute(&mut *tx).await?;

            // Update user balances
            sqlx::query("UPDATE users SET balance = balance - $1 WHERE id = $2")
                .bind(&(trade_qty * ask_price)).bind(&buyer_id).execute(&mut *tx).await?;
            
            sqlx::query("UPDATE users SET balance = balance + $1 WHERE id = $2")
                .bind(&(trade_qty * ask_price * (1.0 - fee_rate))).bind(&seller_id).execute(&mut *tx).await?;

            tx.commit().await?;

            if trade_qty >= bid_rem { break; }
        }
    }

    Ok(())
}
