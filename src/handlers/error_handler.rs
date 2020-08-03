// I should mix 1.(For a single page app?) and 2.

// Refer to these.
// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// Err(reject::custom(DivideByZero))

use std::convert::Infallible;

use warp::{
    http::StatusCode,
    Rejection,
    Reply,
    reject,
};

use serde_derive::Serialize;

use super::{
    UNAUTHORIZED,
    INTERNAL_SERVER_ERROR,
    NOT_ACCEPTABLE,
    BAD_REQUEST
};

/// An API error serializable to JSON.
// 1. JSON with error message and code example.

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

// https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
// https://docs.rs/warp/0.2.2/warp/reject/index.html
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (
            StatusCode::NOT_FOUND,
            "NOT_FOUND"
        ) // return template here?
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "METHOD_NOT_ALLOWED"
        )
    } else if let Some(_) = err.find::<UNAUTHORIZED>() {
        (
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED"
        )
    } else if let Some(_) = err.find::<NOT_ACCEPTABLE>() {
        (
            StatusCode::NOT_ACCEPTABLE,
            "NOT_ACCEPTABLE"
        )
    } else if let Some(_) = err.find::<BAD_REQUEST>() {
        (
            StatusCode::BAD_REQUEST,
            "BAD_REQUEST"
        )
    } else if let Some(_) = err.find::<INTERNAL_SERVER_ERROR>() {
        // Is this necesary here?
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_SERVER_ERROR"
        )
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "UNHANDLED_REJECTION"
        )
    };

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

// 2. With templates.

// Create custom error pages.
// fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
//     match err.status() {
//         StatusCode::NOT_FOUND => {
//             eprintln!("Got a 404: {:?}", err);
//             // We have a custom 404 page!
//             Response::builder().status(StatusCode::NOT_FOUND).html(|o| {
//                 templates::error(
//                     o,
//                     StatusCode::NOT_FOUND,
//                     "The resource you requested could not be located.",
//                 )
//             })
//         }
//         code => {
//             eprintln!("Got a {}: {:?}", code.as_u16(), err);
//             Response::builder()
//                 .status(code)
//                 .html(|o| templates::error(o, code, "Something went wrong."))
//         }
//     }
// }

    // let code;
    // let message;

    // if err.is_not_found() {
    //     code = StatusCode::NOT_FOUND;
    //     message = "NOT_FOUND"; // Put template here?
    // } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
    //     // We can handle a specific error, here METHOD_NOT_ALLOWED,
    //     // and render it however we want
    //     code = StatusCode::METHOD_NOT_ALLOWED;
    //     message = "METHOD_NOT_ALLOWED"; // How to make this error?s
    // } else if let Some(_) = err.find::<UNAUTHORIZED>() {
    //     code = StatusCode::UNAUTHORIZED;
    //     message = "UNAUTHORIZED";
    // } else if let Some(_) = err.find::<INTERNAL_SERVER_ERROR>() {
    //     code = StatusCode::INTERNAL_SERVER_ERROR;
    //     message = "INTERNAL_SERVER_ERROR";
    // } else if let Some(_) = err.find::<NOT_ACCEPTABLE>() {
    //     code = StatusCode::NOT_ACCEPTABLE;
    //     message = "NOT_ACCEPTABLE";
    // } else if let Some(_) = err.find::<BAD_REQUEST>() {
    //     code = StatusCode::BAD_REQUEST;
    //     message = "BAD_REQUEST";
    // } else {
    //     // We should have expected this... Just log and say its a 500
    //     eprintln!("unhandled rejection: {:?}", err);
    //     code = StatusCode::INTERNAL_SERVER_ERROR;
    //     message = "UNHANDLED_REJECTION";
    // }