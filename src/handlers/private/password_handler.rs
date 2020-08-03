use warp::{
    reply,
    reject::{
        custom,
    },
    Rejection,
    Reply,
};

use log::{debug, error};

use crate::{
    db::sqlite::SQLITEPOOL,
    models::{
        user::{
            user::{User},
        },
        private::{
            password::UpdatePasswordRequest,
        },
    },
    security::argon::verify,
    session::UserSession,
};

use super::super::{
    UNAUTHORIZED,
    INTERNAL_SERVER_ERROR,
    NOT_ACCEPTABLE,
};

pub async fn update_password(
    update_password_request: UpdatePasswordRequest,
    user_session: Option<UserSession>,
) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { email, password, .. } = user_session;

                let UpdatePasswordRequest {
                    old_password,
                    new_password,
                } = update_password_request;

                // Should use argon here.
                let correct_password = verify(&password, &old_password.as_bytes());
                if correct_password == false {
                    error!("The password({}) given by the user is not correct.", &old_password);
                    Err(custom(NOT_ACCEPTABLE))
                } else {
                    if let Err(e) = User::update_password(&conn, &email, &new_password) {
                        error!("{:#?}", e);
                        Err(custom(INTERNAL_SERVER_ERROR))
                    } else {
                        debug!("Could update the password.");
                        Ok(reply())
                    }
                }
            } else {
                debug!("Fail to update the password without authorization. Should redirect a user to /login.");
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
