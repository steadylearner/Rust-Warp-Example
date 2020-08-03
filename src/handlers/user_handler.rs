use warp::http::{header, Response, StatusCode};
use warp::{
    reply,
    reject::{
        // https://docs.rs/warp/0.1.6/warp/reject/index.html
        custom,
        not_found,
    },
    redirect,
    Rejection,
    Reply,
    http::{Uri},
};

use log::{debug, error, warn};

use bincode;

use crate::{
    db::sqlite::SQLITEPOOL,
    models::{
        cash,
        user::{
            new_user::{NewUser},
            requests::{NewUserRequest, LoginRequest, UpdateCashRequest, UpdateUserRequest},
            // responses::{LoginSuccessResponse},
            user::{User, UserList},
        },
        private::{
            profile::Profile,
        },
    },
    security::argon::verify,
    session::UserSession,
    utils::random::alphanumeric_key,
    temporary_redirect_to_home,
    redirect_to_login,
};

use super::{
    UNAUTHORIZED,
    INTERNAL_SERVER_ERROR,
    NOT_ACCEPTABLE,
};

pub async fn register(new_user_request: NewUserRequest) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            let new_user = NewUser {
                email: new_user_request.email,
                password: new_user_request.password,
                ..Default::default()
            };
            println!("{:#?}", &new_user);
            // Shoud verify email(unique, length, regex etc) and password(security) here.

            if let Err(e) = new_user.create(&conn) {
                error!("{:#?}", e);
                Err(custom(NOT_ACCEPTABLE))
            } else {
                // debug!("Register success and redirect a user to /login with the frontend(React).");
                // Ok(redirect_to_login!())
                let response = Response::builder()
                    .body(b"".to_vec());

                Ok(response)
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn list() -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            match UserList::list_public(&conn) {
                Ok(public_user_list) => {
                    Ok(reply::json(&public_user_list))
                },
                Err(e) => {
                    error!("{:#?}", e);
                    Err(custom(INTERNAL_SERVER_ERROR))
                }
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

// Refer to https://github.com/kaj/warp-diesel-ructe-sample/tree/master/src to save session_id to user and database
// Use do here temporaily because I will make login page?
// Should I make custom session struct?
pub async fn do_login(login_request: LoginRequest) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            let LoginRequest { email, password } = login_request;
            let password_from_database = User::is_registered(&conn, &email);

            if let Some(hash) = password_from_database {
                if verify(&hash, password.as_bytes()) {
                    let identity_id = alphanumeric_key(48);
                    debug!("New identity_id for {} is {}.", &email, &identity_id);

                    // Should set identified_at field with NaiveDateTime to compare when user did login later.
                    User::set_identity_id(&conn, &email, &identity_id).unwrap(); // Remove this unwrap later with correct error handler

                    let cookie = format!("EXAUTH={}; SameSite=Strict; HttpOpnly", &identity_id);

                    // This helps the user to navigate after login, but it is not identity_id
                    // Save it to the user database also? Use it to see how users behave etc.
                    let session_id = alphanumeric_key(48);
                    let body = session_id.into_bytes();
                    
                    // let body = LoginSuccessResponse {
                    //    session_id,
                    // };
                    // let encoded: Vec<u8> = bincode::serialize(&body).unwrap();
                    
                    let response = Response::builder()
                        .status(StatusCode::OK) 
                        .header(
                            header::SET_COOKIE,
                            cookie,
                        )
                        .body(body.to_vec());
                     
                        // .body(encoded);
                    
                    Ok(response)
                } else {
                    debug!("The password is not correct.");
                    Err(custom(UNAUTHORIZED))
                }
            } else {
                // Should handle this correctly.
                warn!("There is no hashed password for the user {}. Something severe problem happend. Where to send this user to where?", &email);
                // Password is none. Where to send the user?
                Err(warp::reject::not_found())
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn update_cash(
    update_cash_request: UpdateCashRequest,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UpdateCashRequest { amount } = update_cash_request;
                debug!("The cash will be updated with {}.", &amount);
                let UserSession { email, .. } = user_session;

                if let Err(e) = cash::update(&conn, &amount, &email) {
                    error!("{:#?}", e);
                    Err(custom(INTERNAL_SERVER_ERROR)) // Custom error here?
                } else {
                    debug!("The cash of the user is updated.");
                    // Ok(redirect(Uri::from_static("/profile"))) // Redirect is handled at the React frontend.
                    Ok(reply())
                }
            } else {
                debug!("Fail to update the cash without authorization. Should redirect a user to /login.");
                // Ok(redirect_to_login!())
                Ok(reply()) // Use this to make it compile. Should handle it correctly later.
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn delete_user(user_session: Option<UserSession>) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { email, .. } = user_session;

                if let Err(e) = User::delete(&conn, &email) {
                    error!("{:#?}", e);
                    Err(custom(INTERNAL_SERVER_ERROR)) // Custom error here?
                } else {
                    debug!("The user is deleted and redirect to / without any session data.");
                    Ok(temporary_redirect_to_home!())
                }
            } else {
                debug!("Fail to delete the user without authorization. Should redirect a user to /.");
                Ok(temporary_redirect_to_home!())
            }
        },
        Err(e) => {
            error!("{:#?}", e);
            Err(custom(INTERNAL_SERVER_ERROR))
        }
    };
    response
}

pub async fn logout(user_session: Option<UserSession>) -> Result<impl Reply, Rejection> {

    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                if let Err(e) = User::remove_identity_id(&conn, &user_session.identity_id) {
                    error!("{:#?}", e);
                    Err(custom(INTERNAL_SERVER_ERROR)) // Custom error here?
                } else {
                    debug!("Logout success and redirect a user to / without any session data .");
                    Ok(temporary_redirect_to_home!())
                }
            } else {
                debug!("Fail to logout without autorization. Should redirect a user to /.");
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
