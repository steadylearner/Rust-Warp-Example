use warp::{
    Reply, reply,
    reject::{
        custom,
        // Reject
    },
    // redirect,
    Rejection,
    // http::{Uri},
};
// use warp::http::{header, Response, StatusCode};

use crate::{
    models::{
        // user::{
        //     NewUser,
        //     NewUserRequest,
        //     LoginRequest,
        //     UpdateUserRequest,
        //     UpdateCashRequest,
        //     User,
        //     UserList,
        // },
        car::{
            NewCar,
            NewCarRequest,
            Car,
            CarRefundRequest,
            // CarPublic,
            CarPublicList,
        },
        // cash,
    },
    db::sqlite::SQLITEPOOL,
    session::UserSession,
    // redirect_to_login,
};

use super::{
    UNAUTHORIZED,
    INTERNAL_SERVER_ERROR,
    NOT_ACCEPTABLE,
};

use log::{debug};

pub async fn list(
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;

                match CarPublicList::list(&conn, &user_id) {
                    Ok(cars) => {
                        Ok(reply::json(&cars))
                    },
                    Err(e) => {
                        error!("{:#?}", e);
                        Err(custom(INTERNAL_SERVER_ERROR))
                    }
                }
            } else {
                debug!("Fail to buy a car without authorization. Should redirect a user to /login.");
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn buy(
    new_car_request: NewCarRequest,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(mut conn) => {
            if let Some(user_session) = user_session {
                let UserSession { cash, user_id, .. } = user_session;
                let NewCarRequest { price, color } = new_car_request;

                if &cash >= &price {
                    let new_car = NewCar {
                        price,
                        color,
                        user_id,
                    };
                    if let Err(e) = new_car.create(&mut conn) {
                        error!("{:#?}", e);
                        Err(custom(INTERNAL_SERVER_ERROR))
                    } else {
                        // Should handle it correctly.
                        debug!("The user bought a car.\n");
                        Ok(reply::html("Redirect the user where he can see a new car.\n"))
                    }
                } else {
                    // Should handle it correctly.
                    debug!("The user need more money to buy a car.\n");
                    Ok(reply::html("Redirect the user to deposit more money.\n"))
                }
            } else {
                debug!("Fail to buy a car without authorization. Should redirect a user to /login.");
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!()) // Should rebuild it with Warp API?
                Err(custom(UNAUTHORIZED))
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn refund(
    car_refund_request: CarRefundRequest,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
        let response = match SQLITEPOOL.get() {
        Ok(mut conn) => {
            if let Some(user_session) = user_session {
                let UserSession { user_id, .. } = user_session;
                let CarRefundRequest { car_id, } = car_refund_request;

                let car = Car::get(&conn, &car_id).unwrap();
                let car = car.get(0);

                if let Some(car) = car {
                    let Car { user_id: author_id, .. } = car;
                    if &user_id != author_id {
                        Err(custom(UNAUTHORIZED))
                    } else {
                        if let Err(e) = car.refund(&mut conn) {
                            eprintln!("{:#?}", e);
                            Err(custom(INTERNAL_SERVER_ERROR))
                        } else {
                            // Should handle it correctly.
                            debug!("The user refunded the car.");
                            Ok(reply::html("Redirect the user to see the money from the acar.\n"))
                        }
                        // Ok(reply::html("The user refunded the car.".into())) // It has a type relevant problem currently.
                    }
                } else {
                    // Should handle it correctly.
                    debug!("The car is not registered.");
                    Err(custom(NOT_ACCEPTABLE))
                }
            } else {
                debug!("Fail to refund a car without authorization. Should redirect a user to /login.");
                // currently shows expected opaque type, found a different opaque type error
                // Ok(redirect_to_login!())
                Err(custom(UNAUTHORIZED))
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}
