use warp::{
    reply,
    reject::{
        // https://docs.rs/warp/0.1.6/warp/reject/index.html
        custom,
        not_found
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
            profile::Profile,
        }
    },
    session::UserSession,
};

use super::super::{
    UNAUTHORIZED,
    INTERNAL_SERVER_ERROR,
};

pub async fn get(user_session: Option<UserSession>,) -> Result<impl Reply, Rejection> {
    let response = match SQLITEPOOL.get() {
        Ok(conn) => {
            if let Some(user_session) = user_session {
                let UserSession { email, .. } = user_session;

                let response = match User::get(&conn, &email) {
                    Ok(user) => {
                        let user = user.get(0);
                        let response = if let Some(user) = user {
                            let User { email, cash, .. } = user;
                            let profile = Profile {
                                email: email.to_string(),
                                cash: *cash,
                            };
                            // Content-Type: application/json
                            // Should make it return with this header.
                            Ok(reply::json(&profile))
                        } else {
                            Err(not_found())
                        };
                        response
                    },
                    Err(e) => {
                        error!("{:#?}", e);
                        Err(custom(INTERNAL_SERVER_ERROR))
                    }
                };

                response
            } else {
                debug!("Failed without autorization. Should redirect a user to /login at React Frontend.");
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

// I get this error from the client.
// XML Parsing Error: syntax error
// Location: http://localhost:1234/api/user/v1/login
// Line Number 1, Column 1:

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type
// Fix it with header Content-Type: application/json
