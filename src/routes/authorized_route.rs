// https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/routes/user_route.rs
// https://docs.rs/warp/0.1.22/warp/filters/path/

// use warp::{
//     filters::BoxedFilter,
//     path,
//     Filter,
// };

// use super::{
//     user_api_v1_path_prefix,
// };

// pub fn authorized() -> BoxedFilter<()> {
//     warp::get()
//         .and(user_api_v1_path_prefix())
//         .and(path("authorized"))
//         .and(path::end())
//         .boxed()
// }

// 1. Login with CURL.
// $curl -X POST localhost:8000/api/user/v1/login -c cookie.txt -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'

// 2. Test it work.
// $curl -X GET localhost:8000/api/user/vi/authorized -b cookie.txt -L

// Should return this.

// <!DOCTYPE html>
// <html lang="en">

// <head>
//     <meta charset="utf-8" />
//     <title>You are authorized</title>
// </head>

// <body>
//     <h1>You are authorized</h1>
// </body>

// </html>