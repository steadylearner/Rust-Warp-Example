// These are to reuse at handlers/
use serde::{Serialize, Deserialize};

// Use other macro name later.
#[macro_export]
macro_rules! temporary_redirect_to_home {
    () => {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::SET_COOKIE,
                "EXAUTH=; Max-Age=0; SameSite=Strict; HttpOpnly",
            )
            .body(b"".to_vec());
    };
}

// Should find and read the documentaion here.
// https://docs.rs/http/0.2.1/http/response/struct.Builder.html
// https://docs.rs/warp/0.1.12/warp/reply/index.html
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/auth/javascript/express/__tests__/routes/login.js
// $curl -X POST localhost:8000/api/user/v1/login -c cookie-file.txt -H "Content-Type: application/json" -d '{ "email": "whatever@email.com", "password": "randompassword" }'
// $cat cookie-file.txt to see identity_id are the same.

// Should return statusCode, statusText etc?
#[macro_export]
macro_rules! set_identity_id {
    // https://doc.rust-lang.org/1.7.0/book/macros.html#hygiene
    ($id:expr) => {
        Response::builder()
            .status(StatusCode::FOUND)
            // I don't need this because React client will handle this.
            // .header(header::LOCATION, "/") // Should be at /profile, email or (user)name to show the profile.
            .header(
                header::SET_COOKIE,
                format!("EXAUTH={}; SameSite=Strict; HttpOpnly", $id),
            )
            .body(LoginSuccessResponse {
                identity_id: $id,
            });
            // .body(b"".to_vec());
    };
}

// React frontend will handle this also.
#[macro_export]
macro_rules! redirect_to_login {
    () => {
        redirect(Uri::from_static("/login"))
    };
}

// #[macro_export]
// macro_rules! redirect_to_profile {
//     () => {
//         redirect(Uri::from_static("/profile"))
//     };
// }
