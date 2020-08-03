// use warp::{
//     Reply,
//     // reject::{custom, Reject},
//     Rejection,
//     http::{Uri},
// };

// use crate::{
//     session::UserSession,
// };

// pub async fn authorized(user_session: Option<UserSession>) -> Result<impl Reply, Rejection> {
//     if let Some(_user_session) = user_session {
//         Ok(warp::redirect(Uri::from_static("/authorized.html")))
//     } else {
//         // Should be not allowed here
//         Err(warp::reject::not_found())
//     }
// }
