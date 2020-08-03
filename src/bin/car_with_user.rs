extern crate gamble;
use gamble::{
    db::sqlite::SQLITEPOOL,
    from_stdin,
    models::car_with_user::{CarWithUser, CarWithUserList},
};

extern crate rusqlite;
use rusqlite::Result;

fn main() -> Result<()> {
    let conn = SQLITEPOOL.get().unwrap();
    println!("Use [r]ead, [l]ist to show the data of a car and its owner.");

    let event = from_stdin();
    match event.as_ref() {
        "r" => {
            println!("What is the id of a car?");
            let car_id = from_stdin().parse::<i64>().unwrap();

            let car_with_user = CarWithUser::get(&conn, car_id)?;
            let car_with_user = car_with_user.get(0);
            match car_with_user {
                Some(car_with_user) => {
                    println!("{:#?}", car_with_user);
                }
                None => {
                    println!("There is no car with the id.");
                }
            }
        }
        "l" => {
            let cars_with_users = CarWithUserList::list(&conn)?;
            println!("{:#?}", cars_with_users);
        }
        _ => {
            println!("Use [r, l] to show the data of a car and its author.");
        }
    }

    Ok(())
}
