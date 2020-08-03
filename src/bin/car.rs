extern crate gamble;
use gamble::{
    from_stdin,
    models::{
        car::{Car, CarList, NewCar},
        user::user::User,
    },
};

extern crate rusqlite;
use rusqlite::{Connection, Result};

// People should sell it to another user or return it to the site and get money also.
// For this prototype, just suppose that there are sufficient number of cars.(Virtual Cars for the game.)
// Verify first that user have enough cash.

fn main() -> Result<()> {
    let mut conn = Connection::open("gamble.db")?;
    println!("Use [b]uy, [r]efund or [l]ist to manage cars.");

    let event = from_stdin();
    match event.as_ref() {
        "b" => {
            // Use id or identity_id etc later from a web app if you want.
            println!("What is the email of a car buyer?");
            let email = from_stdin();

            let user = User::get(&conn, &email)?;
            let user = user.get(0);
            match user {
                Some(user) => {
                    let User { cash, id, .. } = user;
                    println!("What is the price of the car?");
                    let price = from_stdin().parse::<f64>().unwrap();
                    // Build a web app or
                    // a SQL level limit to be a cash can't be less than 0.
                    if cash >= &price {
                        println!("What is the color of the car?");
                        let color = from_stdin();

                        let new_car = NewCar {
                            price,
                            color,
                            user_id: id.to_owned(),
                        };
                        new_car.create(&mut conn)?;
                        println!("The user buyed a car with ${}.", &price);
                    } else {
                        println!("The user doesn't have enough money to buy a car.");
                    }
                }
                None => {
                    println!("The email is not registered.");
                }
            }
        }
        "r" => {
            println!("What is the email of a user who wants to refund a car?");
            let email = from_stdin();

            let user = User::get(&conn, &email)?;
            let user = user.get(0);
            match user {
                Some(user) => {
                    let User { id, .. } = user;
                    println!("What is the id of the car?");
                    let car_id = from_stdin().parse::<i64>().unwrap();

                    // Make it work only when user_if is equal to id.
                    let car = Car::get(&conn, &car_id)?;
                    let car = car.get(0);
                    match car {
                        Some(car) => {
                            let Car { user_id, price, .. } = car;
                            if user_id != id {
                                println!("The user is not the author of the car.")
                            } else {
                                car.refund(&mut conn)?;
                                println!("The user get ${} instead of the car.", &price);
                            }
                        }
                        None => {
                            println!("The car is not registered.");
                        }
                    }
                }
                None => {
                    println!("The email is not registered.");
                }
            }
        }
        "l" => {
            let cars = CarList::list(&conn)?;
            println!("{:#?}", cars);
        }
        _ => {
            println!("Use [b, r, l] to mangage cars.");
        }
    }

    Ok(())
}
