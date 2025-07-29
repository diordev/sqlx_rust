use dotenvy::dotenv;
use std::env;

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
    pub fn new() -> Option<Self> {
        dotenv().ok();

        let host = env::var("POSTGRES_HOST").ok()?;
        let port = env::var("POSTGRES_PORT").ok()?.parse::<u32>().ok()?;
        let user = env::var("POSTGRES_USER").ok()?;
        let password = env::var("POSTGRES_PASSWORD").ok()?;
        let database = env::var("POSTGRES_DB").ok()?;
        let max_connections = env::var("DB_MAX_CONNECTIONS").ok()?.parse::<u32>().ok()?;
        let min_connections = env::var("DB_MIN_CONNECTIONS").ok()?.parse::<u32>().ok()?;

        Some(Database {
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
