use std::env;
use anyhow::Result;
use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct Database {
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl Database {
    pub fn new() -> Result<Self> {
        dotenv().map_err(|e| anyhow::anyhow!("Failed to load .env file: {}", e))?;

        let host = env::var("POSTGRES_HOST")
            .map_err(|_| anyhow::anyhow!("POSTGRES_HOST is not set"))?;
        let port = env::var("POSTGRES_PORT")
            .map_err(|_| anyhow::anyhow!("POSTGRES_PORT is not set"))?
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("POSTGRES_PORT must be a valid u32"))?;
        let user = env::var("POSTGRES_USER")
            .map_err(|_| anyhow::anyhow!("POSTGRES_USER is not set"))?;
        let password = env::var("POSTGRES_PASSWORD")
            .map_err(|_| anyhow::anyhow!("POSTGRES_PASSWORD is not set"))?;
        let database = env::var("POSTGRES_DB")
            .map_err(|_| anyhow::anyhow!("POSTGRES_DB is not set"))?;
        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .map_err(|_| anyhow::anyhow!("DB_MAX_CONNECTIONS is not set"))?
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("DB_MAX_CONNECTIONS must be a valid u32"))?;
        let min_connections = env::var("DB_MIN_CONNECTIONS")
            .map_err(|_| anyhow::anyhow!("DB_MIN_CONNECTIONS is not set"))?
            .parse::<u32>()
            .map_err(|_| anyhow::anyhow!("DB_MIN_CONNECTIONS must be a valid u32"))?;

        Ok(Database {
            host,
            port,
            user,
            password,
            database,
            max_connections,
            min_connections,
        })
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}
