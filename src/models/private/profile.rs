use serde::{Deserialize, Serialize};

// Should be ProfileReply.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub email: String,
    pub cash: f64,
}