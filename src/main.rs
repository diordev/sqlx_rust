mod config;
mod components;
mod db;
mod dto;
mod models;


use config::database::Database;
use db::pool::create_pg_pool;
use dto::user::{UserResponse};

use anyhow::Result;

use crate::models::user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    // Database konfiguratsiyasini o'qish
    let db = Database::new()
        .expect("Database connection configuration is invalid");

    let pool = create_pg_pool(&db).await?;

    // SELECT query — barcha foydalanuvchilarni olish
    let users: Vec<User> = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    // Foydalanuvchilarni aylanish
    for user in users.iter() {
        let row = UserResponse {
            id: user.id.to_string(), // uuid -> string
            username: user.username.clone(),
            email: user.email.clone(),
            created_at: user.created_at.to_rfc3339(), // datetime -> string
            updated_at: user.updated_at.to_rfc3339(),
        };
        println!("Row: {:?}", row);
    }

    Ok(())
}
