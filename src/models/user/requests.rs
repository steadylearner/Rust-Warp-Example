use serde::{Deserialize, Serialize};

// These are for a web server.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserRequest {
    pub email: String,
    pub password: String,
}

// Could be update password request or just use NewUserRequest instead.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCashRequest {
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

