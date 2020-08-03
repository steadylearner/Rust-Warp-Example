extern crate gamble;
use gamble::{
    db::sqlite::SQLITEPOOL,
    from_stdin,
    models::game::{
        Game,
        GameList, // To show records and use rankings later?
    },
};

extern crate rusqlite;
use rusqlite::Result;

// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
extern crate rand;

fn main() -> Result<()> {
    let conn = SQLITEPOOL.get().unwrap();

    println!("Use [r, l] to see game records.");
    let event = from_stdin();
    match event.as_ref() {
        "r" => {
            println!("What is the id of a game?");
            let id = from_stdin();

            let game_list = Game::get(&conn, id)?;
            println!("{:#?}", game_list);
        }
        "l" => {
            let game_list = GameList::list(&conn)?;
            println!("{:#?}", game_list);
        }
        _ => {
            println!("Use [r, l] to see game records.");
        }
    }

    Ok(())
}
