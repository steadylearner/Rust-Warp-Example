use crate::{db::sqlite::SQLITEPOOL, models::user::user::User};
use warp::filters::{cookie, BoxedFilter};
use warp::{self, Filter};

// It is difficult with Warp sometimes. Then, use Hyper directly.
// use hyper::HeaderMap;

// pub fn get_header(name: &str) -> Option<String> {
//     let mut map = HeaderMap::new();
//     let value = map.get(name);
//     value
// }

#[derive(Debug)]
pub struct UserSession {
    pub identity_id: String,
    pub email: String,
    pub password: String,
    pub cash: f64,
    pub user_id: i64,
}

pub fn create_user_session(identity_id: &str) -> Option<UserSession> {
    let conn = SQLITEPOOL.get().unwrap();

    let user = User::get_by_identity_id(&conn, &identity_id);
    let user_session = match user {
        Ok(user) => {
            let user = user.get(0);
            // debug!("create_user_session match user {:#?}", &user);
            match user {
                Some(user) => {
                    let User {
                        email,
                        password,
                        cash,
                        id: user_id,
                        ..
                    } = user;
                    let user_session = UserSession {
                        identity_id: identity_id.into(),
                        email: email.to_owned(),
                        password: password.to_owned(),
                        cash: cash.to_owned(),
                        user_id: user_id.to_owned(),
                    };
                    Some(user_session)
                }
                None => None,
            }
        }
        Err(e) => {
            error!("{:#?}", e);
            None
        }
    };
    // debug!("create_user_session user_session {:#?}", &user_session);
    user_session
}

// https://docs.rs/warp/0.2.2/warp/filters/any/fn.any.html
pub fn user_session_filter() -> BoxedFilter<(Option<UserSession>,)> {
    // Handling session is just to read the private data from the browser.
    // Then, use it inside the server.

    cookie::optional("EXAUTH") // It returns filter
        .map(move |key: Option<String>| {
            let key = key.as_ref().map(|s| &**s);
            // Current problem is here. Should find the reason.
            // println!("{:#?}", &key); Why this is none?
            // When I test it with /api/user/v1 it works.
            // But, it fails when I test it with /api/car/v1

            let user_session = if let Some(identity_id) = key {
                create_user_session(identity_id)
            } else {
                debug!("{}", "Fail to find identity_key from EXAUTH.");
                None
            };
            // debug!("user_session filter user_session {:#?}", &user_session);

            user_session
        })
        .boxed()
}
