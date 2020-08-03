extern crate gamble;
use gamble::{db::sqlite::SQLITEPOOL, models::game_with_user::GameWithUserList};

extern crate rusqlite;
use rusqlite::Result;

fn main() -> Result<()> {
    let conn = SQLITEPOOL.get().unwrap();

    let games_with_users = GameWithUserList::list(&conn)?;
    println!("{:#?}", games_with_users);

    Ok(())
}
