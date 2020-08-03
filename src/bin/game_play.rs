extern crate gamble;
use gamble::{
    db::sqlite::SQLITEPOOL,
    from_stdin,
    models::{car::Car, cash, game::NewGame, user::user::User},
};

extern crate rusqlite;
use rusqlite::Result;

// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
extern crate rand;
use rand::Rng;

fn main() -> Result<()> {
    let conn = SQLITEPOOL.get().unwrap();

    // This is currently solo playing game simialr to the https://www.mafiaway.nl
    println!("What is your email?");
    let email = from_stdin();
    let user = User::get(&conn, &email)?;
    let user = user.get(0);

    // Should have some minimum value for a production project.
    // let minimum_bet = 1000f64;

    match user {
        Some(user) => {
            let User {
                cash, email, id, ..
            } = user;

            println!("How many participants?");
            let number_of_participants = from_stdin().parse::<f64>().unwrap();

            let odd = 1f64 / number_of_participants;
            let odd_to_percent = format!("{}%", odd * 100f64);
            println!("Your odd is {}.", odd_to_percent);

            if number_of_participants >= 2f64 {
                println!("Will you bet with [m]oney or a [c]ar?");
                let pay_option = from_stdin(); // m for cash, c for car.]
                if pay_option == "c" {
                    println!("What is the id of a car?");
                    let car_id = from_stdin().parse::<i64>().unwrap();

                    let car = Car::get(&conn, &car_id)?;
                    let car = car.get(0);
                    match car {
                        Some(car) => {
                            let Car {
                                user_id,
                                price: stake_amount,
                                ..
                            } = car;
                            if user_id != id {
                                println!("The user is not the owner of the car.");
                            } else {
                                println!(
                                    "The price of the car({}) will be used for the game.",
                                    &stake_amount
                                );

                                let mut rng = rand::thread_rng();
                                let from_zero_to_one = rng.gen::<f64>(); // [0, 1)

                                let condition: bool = from_zero_to_one >= odd;

                                let win = if condition {
                                    println!("You lost the gamble.");
                                    false
                                } else {
                                    println!("You won the gamble.");
                                    true
                                };

                                let profit = if !win {
                                    let loss = stake_amount * -1.0f64;
                                    loss
                                } else {
                                    let rest = number_of_participants - 1f64;
                                    let earning = stake_amount * rest;
                                    earning
                                };

                                cash::update(&conn, &profit, &email)?;

                                let new_game = NewGame {
                                    stake_amount: stake_amount.to_owned(),
                                    number_of_participants: number_of_participants as i64,
                                    win,
                                    user_id: id.to_owned(),
                                };
                                new_game.create(&conn)?;

                                println!("Save the new game result.",);
                            }
                        }
                        None => {
                            println!("There is no car with the id");
                        }
                    }
                } else {
                    println!("How much is your stake amount?");
                    let stake_amount = from_stdin().parse::<f64>().unwrap(); // Could be bet

                    if stake_amount <= 0f64 {
                        println!("You should bet more than 0 cash.");
                    } else {
                        if cash < &stake_amount {
                            println!(
                                "You need enough cash to play this game.($cargo run --bin cash)"
                            )
                        } else {
                            // Extract it to function.
                            let mut rng = rand::thread_rng();
                            let from_zero_to_one = rng.gen::<f64>(); // [0, 1)

                            // Include = because ) from_zero_to_one
                            let condition: bool = from_zero_to_one >= odd;

                            let win = if condition {
                                println!("You lost the gamble.");
                                false
                            } else {
                                println!("You won the gamble.");
                                true
                            };

                            let profit = if !win {
                                let loss = stake_amount * -1.0f64;
                                loss
                            } else {
                                let rest = number_of_participants - 1f64;
                                let earning = stake_amount * rest;
                                earning
                            };

                            cash::update(&conn, &profit, &email)?;

                            let new_game = NewGame {
                                stake_amount,
                                number_of_participants: number_of_participants as i64,
                                win,
                                user_id: id.to_owned(),
                            };
                            new_game.create(&conn)?;

                            println!("Save the new game record.",);
                        }
                    }
                };
            } else {
                println!("You need at least two players to play this.");
            }
        }
        None => {
            println!("The email is not registered.");
        }
    }

    Ok(())
}
