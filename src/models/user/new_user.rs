use rusqlite::{Connection, Result};

use serde::{Deserialize, Serialize};
use log::debug;

use crate::{security::argon::hash, utils::random::alphanumeric_key};

// These are for CLI to prototype features.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String, // Use type here?
    pub password: String,
    pub cash: f64,
    pub identity_id: String,
}

impl Default for NewUser {
    fn default() -> Self {
        NewUser {
            email: "default@email.com".into(),
            password: "password".into(),
            cash: 0.0,
            identity_id: alphanumeric_key(48),
        }
    }
}

// Use conn for param to every functions.
// Remove let conn = Connection::open("gamble.db")?; with connection pool and lazy static later.
impl NewUser {
    pub fn create(&self, conn: &Connection) -> Result<()> {
        let hashed_password = hash(&self.password.as_bytes());

        conn.execute(
            "INSERT INTO users (email, password, cash, identity_id) values (?1, ?2, ?3, ?4)",
            &[
                &self.email,
                &hashed_password,
                &self.cash.to_string(),
                &self.identity_id,
            ],
        )?;
        debug!("Save {} to gamble.db.", &self.email);
        Ok(())
    }
}
