extern crate gamble;
use gamble::{
    db::sqlite::SQLITEPOOL,
    from_stdin,
    models::user::{
        new_user::{NewUser}, 
        user::{User, UserList}
    },
};

extern crate rusqlite;
use rusqlite::Result;

fn main() -> Result<()> {
    println!("Use [c, r, (u), d, l, p] to manage users.");
    // let conn = Connection::open("gamble.db")?;
    let conn = SQLITEPOOL.get().unwrap();

    let event = from_stdin();
    match event.as_ref() {
        "c" => {
            println!("What is the email for the account?");
            let email = from_stdin();
            println!("What is the password for it?");
            let password = from_stdin();

            let new_user = NewUser {
                email,
                password,
                ..Default::default()
            };

            new_user.create(&conn)?;
        }
        "r" => {
            println!("Which email you want to read?");
            let email = from_stdin();
            let user = User::get(&conn, &email)?;
            println!("{:#?}", user);
        }
        // "u" => {
        //     println!("Which email you want to update its password?");
        //     let email = from_stdin();
        //     println!("What is the new password?");
        //     let password = from_stdin();
        //     println!("The password of {} will be updated.", email);
        //     User::update(&conn, email, password)?;
        // }
        "d" => {
            println!("Which email you want to delete?");
            let email = from_stdin();
            println!("{} will be deleted.", &email);
            User::delete(&conn, &email)?;
        }
        "l" => {
            let users = UserList::list(&conn)?;
            println!("{:#?}", users);
        }
        "p" => {
            println!("Which email you want to update its password?");
            let email = from_stdin();
            println!("What is the new password?");
            let password = from_stdin();
            println!("The password of {} will be updated.", &email);
            User::update_password(&conn, &email, &password)?;
        }
        _ => {
            println!("Use [c, r, u, d, l] to manage users.");
        }
    }

    Ok(())
}
