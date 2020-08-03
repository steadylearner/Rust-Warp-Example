// https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/routes/user_route.rs
// https://docs.rs/warp/0.1.22/warp/filters/path/

use crate::{
    json_body,
    models::car::{CarRefundRequest, NewCarRequest},
};

use warp::{
    body::{content_length_limit, json},
    filters::BoxedFilter,
    path, Filter,
};

use super::{
    user_api_v1_path_prefix,
    // car_api_v1_path_prefix,
};

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(user_api_v1_path_prefix()) // Should be car_api_v1_path_prefix later?
        .and(path("car"))
        .and(path::end())
        .boxed()
}

// $curl -X GET localhost:8000/api/user/v1/car -b cookie.txt -L

pub fn buy() -> BoxedFilter<(NewCarRequest,)> {
    warp::post()
        .and(user_api_v1_path_prefix()) // Should be car_api_v1_path_prefix later?
        .and(path("car"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

// $curl -X POST localhost:8000/api/user/v1/car -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "price": 10000, "color": "red" }'
// ($curl -X POST localhost:8000/api/user/v1/car -b cookie.txt -L -H "Content-Type: application/json" -d '{ "price": 10000, "color": "red" }')

// Should be improved.
pub fn refund() -> BoxedFilter<(CarRefundRequest,)> {
    warp::delete()
        .and(user_api_v1_path_prefix()) // Should be car_api_v1_path_prefix later?
        .and(path("car"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

// $curl -X DELETE localhost:8000/api/user/v1/car -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "car_id": 1 }'
// ($curl -X DELETE localhost:8000/api/user/v1/car -b cookie.txt -L -H "Content-Type: application/json" -d '{ "car_id": 1 }')

// These fails because of it is not started with /api/user/v1 and header set there?
// Should find the better way and search about it?

// pub fn buy() -> BoxedFilter<(NewCarRequest,)> {

//     warp::post()
//         .and(car_api_v1_path_prefix())
//         .and(warp::path::end())
//         .and(json_body)
//         .boxed()
// }

// $curl -X POST localhost:8000/api/car/v1 -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "price": 10000, "color": "red" }'
// ($curl -X POST localhost:8000/api/car/v1 -b cookie.txt -L -H "Content-Type: application/json" -d '{ "price": 10000, "color": "red" }')

// Should be simialr to user if you want to use the real cars.