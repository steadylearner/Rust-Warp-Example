extern crate gamble;
use gamble::{
    db::sqlite::SQLITEPOOL,
    from_stdin,
    models::{cash, user::user::User},
};

extern crate rusqlite;
use rusqlite::Result;

// It should be imporved with error checking.
fn main() -> Result<()> {
    let conn = SQLITEPOOL.get().unwrap();

    println!("What is the id of a user?");
    let email = from_stdin();
    let user = User::get(&conn, &email)?;
    let user = user.get(0);
    match user {
        Some(user) => {
            let User { email, .. } = user;
            println!("Will you give the user cash or vice versa?(Use - when you want to deduct.)");
            let amount = from_stdin(); // Is this necessary?
            let amount = amount.parse::<f64>().unwrap();

            cash::update(&conn, &amount, &email)?;
        }
        None => {
            println!("The email is not registered.");
        }
    }

    Ok(())
}
