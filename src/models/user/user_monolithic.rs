// use rusqlite::{params, Connection, Result, NO_PARAMS};

// use chrono::naive::NaiveDateTime;

// use crate::{security::argon::hash, utils::random::alphanumeric_key};

// use serde::{Deserialize, Serialize};

// use log::debug;

// // These are for CLI to prototype features.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct NewUser {
//     pub email: String, // Use type here?
//     pub password: String,
//     pub cash: f64,
//     pub identity_id: String,
// }

// impl Default for NewUser {
//     fn default() -> Self {
//         NewUser {
//             email: "default@email.com".into(),
//             password: "password".into(),
//             cash: 0.0,
//             identity_id: alphanumeric_key(48),
//         }
//     }
// }

// // Use conn for param to every functions.
// // Remove let conn = Connection::open("gamble.db")?; with connection pool and lazy static later.
// impl NewUser {
//     pub fn create(&self, conn: &Connection) -> Result<()> {
//         let hashed_password = hash(&self.password.as_bytes());

//         conn.execute(
//             "INSERT INTO users (email, password, cash, identity_id) values (?1, ?2, ?3, ?4)",
//             &[
//                 &self.email,
//                 &hashed_password,
//                 &self.cash.to_string(),
//                 &self.identity_id,
//             ],
//         )?;
//         println!("Save {} to gamble.db.", &self.email);
//         Ok(())
//     }
// }

// // These are for a web server.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct NewUserRequest {
//     pub email: String,
//     pub password: String,
// }

// // Could be UpdateUserPassordRequest later.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UpdateUserRequest {
//     pub email: String,
//     pub password: String,
// }

// // Should include it to UpdateUserRequest?
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UpdateCashRequest {
//     pub amount: f64,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LoginRequest {
//     pub email: String,
//     pub password: String,
// }

// // Should be synchronize with SQLite commands at build.rs
// #[derive(Debug, Serialize, Deserialize)]
// pub struct User {
//     pub id: i64,
//     pub email: String,
//     pub password: String,
//     pub cash: f64,
//     //
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub identity_id: String,
// }

// // Use id instead of web app later if necessary.
// // String to &str and use .to_owned() later.s
// impl User {
//     pub fn get(conn: &Connection, email: String) -> Result<Vec<User>> {
//         let mut stmt = conn.prepare("SELECT * FROM users WHERE email = (?1);")?;

//         let result = stmt.query_map(params![&email], |row| {
//             Ok(User {
//                 id: row.get(0)?,
//                 email: row.get(1)?,
//                 password: row.get(2)?,
//                 cash: row.get(3)?,
//                 created_at: row.get(4)?,
//                 updated_at: row.get(5)?,
//                 identity_id: row.get(6)?,
//             })
//         })?;

//         let mut user = Vec::new();
//         for u in result {
//             user.push(u?);
//         }
//         debug!("{:#?}", user);

//         Ok(user)
//     }

//     pub fn get_by_identity_id(conn: &Connection, identity_id: &str) -> Result<Vec<User>> {
//         let mut stmt = conn.prepare("SELECT * FROM users WHERE identity_id = (?1);")?;

//         let result = stmt.query_map(params![&identity_id.to_owned()], |row| {
//             Ok(User {
//                 id: row.get(0)?,
//                 email: row.get(1)?,
//                 password: row.get(2)?,
//                 cash: row.get(3)?,
//                 created_at: row.get(4)?,
//                 updated_at: row.get(5)?,
//                 identity_id: row.get(6)?,
//             })
//         })?;

//         let mut user = Vec::new();
//         for u in result {
//             user.push(u?);
//         }
//         debug!("{:#?}", user);

//         Ok(user)
//     }

//     pub fn delete(conn: &Connection, email: &str) -> Result<()> {
//         conn.execute(
//             "DELETE FROM users WHERE email = (?1);",
//             &[&email.to_owned()],
//         )?;

//         Ok(())
//     }

//     pub fn update_password(conn: &Connection, email: &str, new_password: &str) -> Result<()> {
//         let hashed_new_password = hash(&new_password.as_bytes());

//         conn.execute(
//             "UPDATE users
//                 SET password = (?1), updated_at = datetime('now','localtime')
//                 WHERE email = (?2);",
//             &[&hashed_new_password, &email.to_owned()],
//         )?;

//         Ok(())
//     }

//     // Should use Result later.
//     pub fn is_registered(conn: &Connection, email: &str) -> Option<String> {
//         // Should set updated_at here or default behavior for that at SQL in build.rs.
//         // Search more.
//         // https://stackoverflow.com/questions/14461851/how-to-have-an-automatic-timestamp-in-sqlite
//         let user = User::get(&conn, email.into()).unwrap(); // Remove this unwrap later with correct error handler
//         let user = user.get(0);

//         match user {
//             Some(user) => {
//                 let User {
//                     password: hashed, ..
//                 } = user;
//                 Some(hashed.to_owned())
//             }
//             None => None,
//         }
//     }

//     pub fn set_identity_id(conn: &Connection, email: &str, identity_id: &str) -> Result<()> {
//         conn.execute(
//             "UPDATE users
//                 SET identity_id = (?1)
//                 WHERE email = (?2);",
//             &[&identity_id.to_owned(), &email.to_owned()],
//         )?;

//         Ok(())
//     }

//     pub fn remove_identity_id(conn: &Connection, previous_identity_id: &str) -> Result<()> {
//         conn.execute(
//             "UPDATE users
//                 SET identity_id = ''
//                 WHERE identity_id = (?1);",
//             &[&previous_identity_id.to_owned()],
//         )?;

//         Ok(())
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserPublic {
//     pub email: String,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserList(pub Vec<User>);

// impl UserList {
//     pub fn list(conn: &Connection) -> Result<Vec<User>> {
//         let mut stmt = conn.prepare("SELECT * FROM users;")?;

//         let result = stmt.query_map(NO_PARAMS, |row| {
//             Ok(User {
//                 id: row.get(0)?,
//                 email: row.get(1)?,
//                 password: row.get(2)?,
//                 cash: row.get(3)?,
//                 created_at: row.get(4)?,
//                 updated_at: row.get(5)?,
//                 identity_id: row.get(6)?,
//             })
//         })?;

//         let mut users = Vec::new();
//         for u in result {
//             users.push(u?);
//         }

//         Ok(users)
//     }

//     pub fn list_public(conn: &Connection) -> Result<Vec<UserPublic>> {
//         let mut stmt = conn.prepare("SELECT * FROM users;")?;

//         let result = stmt.query_map(NO_PARAMS, |row| {
//             Ok(UserPublic {
//                 email: row.get(1)?,
//                 created_at: row.get(4)?,
//                 updated_at: row.get(5)?,
//             })
//         })?;

//         let mut users_public = Vec::new();
//         for u in result {
//             users_public.push(u?);
//         }

//         Ok(users_public)
//     }
// }
