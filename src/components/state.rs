// use std::sync::Arc;
// use sqlx::PgPool;
// use tokio::sync::Mutex;

// use crate::db::pool::create_pg_pool;
// // Agar state mutable bo'lsa, lekin pool uchun odatda Arc yetarli

// #[derive(Clone)] // Clone trait productionda multiple handlers uchun zarur (Axum/Actix)
// pub struct AppState {
//     pub db_pool: Arc<PgPool>, // Arc bilan shared ownership, thread-safe (sinyor darajada ref counting uchun)
//     // Qo'shimcha state: masalan, Redis pool, Config, Logger, etc.
//     // pub redis: Arc<Mutex<redis::Connection>>, // Agar kerak bo'lsa mutable access uchun Mutex
//     // pub config: Arc<AppConfig>, // Environmentdan olingan config
// }

// impl AppState {
//     pub async fn new() -> Result<Self, anyhow::Error> { // anyhow production error handling uchun (sinyor: stack traces bilan)super::
//         let db_pool = create_pg_pool().await?; // Oldingi funksiyadan chaqirish
//         Ok(Self {
//             db_pool: Arc::new(db_pool),
//         })
//     }
// }

// // Ishlatish misoli (Axum app'da):
// // let state = AppState::new().await?;
// // let app = Router::new().with_state(state);

// // Best practices: Productionda state ni DI (Dependency Injection) orqali boshqaring, masalan Tokio spawn bilan.
// // Monitoring: Prometheus metrics qo'shing pool usage uchun (sinyor: sqlx instrumented).
// // Testing: Mock pool ishlatish uchun trait abstraction (garage darajada).
