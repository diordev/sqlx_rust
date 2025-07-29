use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use anyhow::{Result, Context};
use tracing::info;

use crate::config::database::Database;

pub async fn create_pg_pool(db: &Database) -> Result<PgPool> {

    let db_url = db.connection_string();
    info!("Creating PostgreSQL connection pool...");

    let pool = PgPoolOptions::new()
        .max_connections(db.max_connections) // Productionda 10-50, CPU va DB instanc limitiga qarab
        .min_connections(db.min_connections) // Minimal connectionlar, idle connectionlarni saqlash uchun
        .acquire_timeout(Duration::from_secs(30)) // Connection olish vaqti, load balancer timeout bilan moslashtir
        .idle_timeout(Some(Duration::from_secs(600))) // 10 min idle bo'lsa yopish, resurs tejash
        .max_lifetime(Some(Duration::from_secs(1800))) // 30 min max lifetime, stale connectionlarni yangilash
        .test_before_acquire(true) // Har connectionni test qilish, broken pool oldini olish
        .connect(&db_url)
        .await
        .context("Failed to create PostgreSQL connection pool")?;

    info!("PostgreSQL connection pool created successfully with {} max connections.", db.max_connections);

    Ok(pool)
}
