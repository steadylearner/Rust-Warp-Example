// Should it be here and used at main.rs or used at build.rs?

// https://docs.rs/r2d2_sqlite/0.3.0/r2d2_sqlite/
// https://www.reddit.com/r/rust/comments/6z7gs2/using_rusqlite_from_multiple_threads/

use r2d2;
use r2d2_sqlite::SqliteConnectionManager;

use rusqlite::Result;
use rusqlite::NO_PARAMS;

// https://github.com/steadylearner/Rust-Full-Stack/tree/master/auth/javascript/express/db/sql

// https://docs.rs/r2d2-sqlite3/0.1.1/r2d2_sqlite3/
pub type SqlitePool = r2d2::Pool<SqliteConnectionManager>;

lazy_static! {
    pub static ref SQLITEPOOL: SqlitePool = {
        let sqlite_database = "gamble.db";
        let manager = SqliteConnectionManager::file(&sqlite_database);
        let pool = r2d2::Pool::builder().build(manager).unwrap();

        pool
    };
}

// https://doc.rust-lang.org/stable/rust-by-example/custom_types/constants.html
const USER: &str = "CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    cash FLOAT CHECK (cash >= 0.0),
    created_at DATE DEFAULT (datetime('now','localtime')),
    updated_at DATE DEFAULT (datetime('now','localtime')),
    identity_id TEXT NOT NULL UNIQUE
)";

const CAR: &str = "CREATE TABLE IF NOT EXISTS cars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    price FLOAT CHECK (price >= 0.0),
    color TEXT NOT NULL,
    user_id INTEGER NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (id)
)";

const GAME: &str = "CREATE TABLE IF NOT EXISTS games (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    stake_amount FLOAT CHECK (stake_amount > 0.0),
    number_of_participants INTEGER NOT NULL CHECK (number_of_participants >= 2),
    win INTEGER,
    created_at DATE DEFAULT (datetime('now','localtime')),
    user_id INTEGER NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users (id)
)";

pub fn setup() -> Result<()> {
    match SQLITEPOOL.get() {
        Ok(conn) => {
            // Give limit to cash to be >= 0.0
            // https://www.sqlitetutorial.net/sqlite-check-constraint/
            conn.execute(USER, NO_PARAMS)?;

            // I can also use products instead of cars with more datas.
            // https://www.sqlitetutorial.net/sqlite-foreign-key/
            conn.execute(CAR, NO_PARAMS)?;

            // SQLite don't have boolean type.
            conn.execute(GAME, NO_PARAMS)?;
        },
        Err(e) => {
            eprintln!("{:#?}", e);
        }
    }

    Ok(())
}

// Can I resue conn? Test it later or there are restrctions from SQLite.

// for i in 0..10i32 {
//         let pool = pool.clone();
//         thread::spawn(move || {
//             let conn = pool.get().unwrap();
//             let mut stmt = conn.prepare("INSERT INTO foo (bar) VALUES (?)").unwrap();
//             stmt.bind(1, 42).unwrap();
//         });
//     }