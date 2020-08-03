// https://serde.rs/container-attrs.html
use serde::{Deserialize, Serialize};

// Serialize to send the data to a client(used at the client side)
// Deserialize to use the data from a client
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}