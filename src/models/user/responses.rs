use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSuccessResponse {
    pub session_id: String,
}
