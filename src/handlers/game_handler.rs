use crate::{
    models::{
        car::Car,
        cash,
        game::{
            NewGame,
            NewGameRequest,
            find_game_result_and_profit,
        },
        private::{
            game::NewGameReply,
        }
    },
    session::UserSession,
};

use warp::{
    reply,
    Rejection,
    Reply,
    reject::{
        // https://docs.rs/warp/0.1.6/warp/reject/index.html
        custom,
        // not_found
    },
    // redirect,
    // http::{Uri},
};

use crate::{
    db::sqlite::SQLITEPOOL,
    // redirect_to_login
};

use log::{debug, error, warn};

use super::{
    UNAUTHORIZED,
    INTERNAL_SERVER_ERROR,
    BAD_REQUEST,
    // NOT_ACCEPTABLE,
};

// Could make it compile following the request of a freelance client.
// But, you should handle errors better. Extract them with fuctnions etc.
pub async fn new(
    new_game_request: NewGameRequest,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    if !new_game_request.is_logically_valid() {
        debug!("Fail to play the game because it is logically invalid.");
        Err(custom(BAD_REQUEST))
    } else {
        if let Some(user_session) = user_session {
            match SQLITEPOOL.get() {
                Ok(conn) => {
                    let UserSession { user_id, email, .. } = user_session;

                    let NewGameRequest {
                        stake_amount,
                        car_id,
                        number_of_participants,
                    } = new_game_request;

                    if let Some(stake_amount) = stake_amount {
                        debug!("Finally play the game with the cash.");

                        let (win, profit) = find_game_result_and_profit(number_of_participants, &stake_amount);
                        let new_game_reply = NewGameReply {
                            win,
                            profit,
                        };

                        let new_game = NewGame {
                            stake_amount,
                            number_of_participants: number_of_participants as i64,
                            win,
                            user_id: user_id.to_owned(),
                        };

                        if let Err(e) = new_game.create(&conn) {
                            error!("{:#?}", e);
                            Err(custom(INTERNAL_SERVER_ERROR))
                        } else {
                            if let Err(e) = cash::update(&conn, &profit, &email) {
                                error!("{:#?}", e);
                                // Should I destroy or revert game creation here?
                                // Handle it correctly later.
                                Err(custom(INTERNAL_SERVER_ERROR))
                            } else {
                                debug!("Save the new car game record played with cash.");
                                // Ok(reply())
                                Ok(reply::json(&new_game_reply))
                            }
                        }
                    } else {
                        if let Some(car_id) = car_id {
                            match Car::get(&conn, &car_id) {
                                Ok(car) => {
                                    let car = car.get(0);
                                    if let Some(car) = car {
                                        let Car {
                                            user_id: author_id,
                                            price: stake_amount,
                                            ..
                                        } = car;

                                        if &user_id != author_id {
                                            debug!("The user is not the owner of the car.");
                                            Err(custom(UNAUTHORIZED))
                                        } else {
                                            debug!("Finally play the game with the car.");

                                            let (win, profit) = find_game_result_and_profit(number_of_participants, &stake_amount);
                                            let new_game_reply = NewGameReply {
                                                win,
                                                profit,
                                            };

                                            cash::update(&conn, &profit, &email).unwrap();

                                            let new_game = NewGame {
                                                stake_amount: stake_amount.to_owned(),
                                                number_of_participants: number_of_participants as i64,
                                                win,
                                                user_id: user_id.to_owned(),
                                            };

                                            if let Err(e) = new_game.create(&conn) {
                                                error!("{:#?}", e);
                                                Err(custom(INTERNAL_SERVER_ERROR))
                                            } else {
                                                if let Err(e) = cash::update(&conn, &profit, &email) {
                                                    error!("{:#?}", e);
                                                    // Should I destroy or revert game creation here?
                                                    // Handle it correctly later.
                                                    Err(custom(INTERNAL_SERVER_ERROR))
                                                } else {
                                                    debug!("Save the new car game record played with a car.");
                                                    if !win {
                                                        if let Err(e) = Car::delete(&conn, &car_id) {
                                                            error!("{:#?}", e);
                                                            // Should I destroy or revert game creation and cash here?
                                                            // Handle it correctly later.
                                                            Err(custom(INTERNAL_SERVER_ERROR))
                                                        } else {
                                                            debug!("The user couldn't win the game. So, the car was detroyed.");
                                                            Ok(reply::json(&new_game_reply))
                                                        }
                                                    } else {
                                                        debug!("The user win the game. He will get the prize and car won't be deleted.");
                                                        Ok(reply::json(&new_game_reply))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        // Should handle it correctly
                                        warn!("The car is not registered. But, the user could send the request?");
                                        // Ok(reply::html("Should send the user to buy a car?"))
                                        Err(custom(BAD_REQUEST))
                                    }
                                },
                                Err(e) => {
                                    error!("{:#?}", e);
                                    Err(custom(INTERNAL_SERVER_ERROR))
                                }
                            }
                        } else {
                            debug!("The user should have offered either cash or a car to play the game.");
                            Err(custom(BAD_REQUEST))
                        }
                    }
                },
                Err(e) => {
                    error!("{:#?}", e);
                    Err(custom(INTERNAL_SERVER_ERROR))
                }
            }
        } else {
            debug!("Fail to play the game without authorization. Should redirect a user to /login.");
            // Ok(redirect_to_login!()) // Should handle type not match error.
            Err(custom(UNAUTHORIZED))
        }
    }
}
