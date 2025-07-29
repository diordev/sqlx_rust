use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserSchema {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String, // UUID
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}
