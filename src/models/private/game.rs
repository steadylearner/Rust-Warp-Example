// ../game.rs should be here.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewGameReply {
    pub win: bool,
    pub profit: f64,
}