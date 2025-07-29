mod config;
mod components;
mod db;
mod dto;
mod models;


use config::database::Database;
use db::pool::create_pg_pool;

use anyhow::Result;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Database konfiguratsiyasini o'qish
    let db = Database::new()
        .expect("Database connection configuration is invalid");

    let pool = create_pg_pool(&db).await?;

    // Test query bajarish
    let row = sqlx::query("SELECT 1 + 1 AS sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = row.try_get("sum")?;
    println!("Test query natijasi: 1 + 1 = {}", sum);

    Ok(())
}
