use crate::{
    json_body,
    models::user::{
        requests::{NewUserRequest, LoginRequest, UpdateCashRequest, UpdateUserRequest}
    },
};

use warp::{
    body::{content_length_limit, json},
    filters::BoxedFilter,
    path, Filter,
};

use super::user_api_v1_path_prefix;

// https://github.com/axios/axios/issues/569
pub fn register() -> BoxedFilter<(NewUserRequest,)> {

    // warp::any()
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(warp::path::end())
        .and(json_body!())
        .boxed()
}

// 1. Without CORS (Need to test with a separate frontend.) - POST (warp::post())

// $curl -X POST localhost:8000/api/user/v1 -H "Content-Type: application/json"
// -d '{ "email": "random@email.com", "password": "password" }'
// ($curl -X POST localhost:8000/api/user/v1 -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }')

// 2. CORS with React - any here and OPTIONS in CURL or axios request.
// (https://github.com/seanmonstar/warp/blob/master/tests/cors.rs)

// $curl -i -X OPTIONS localhost:8000/api/user/v1 -H "origin: *" -H "access-control-request-method: POST" -H "Content-Type: application/json" -d '{ "email": "steady@learner.com", "password": "password" }'

// Retruns this simialr to tests/cors_test.rs (Is this preflight?)

// HTTP/1.1 200 OK
// access-control-allow-headers:
// access-control-allow-methods: POST
// access-control-allow-origin: *
// content-length: 0

pub fn list() -> BoxedFilter<()> {
    warp::get()
        .and(user_api_v1_path_prefix())
        .and(warp::path::end())
        .boxed()
}

// $curl localhost:8000/api/user/v1

pub fn do_login() -> BoxedFilter<(LoginRequest,)> {
    warp::post()
        .and(user_api_v1_path_prefix())
        .and(path("login"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

// $curl -X POST localhost:8000/api/user/v1 -c cookie-file.txt -H "Content-Type: application/json"
// -d '{ "email": "random@email.com", "password": "password" }'
// ($curl -X POST localhost:8000/api/user/v1/login -c cookie.txt -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }')

pub fn update_cash() -> BoxedFilter<(UpdateCashRequest,)> {
    warp::patch()
        .and(user_api_v1_path_prefix())
        .and(path("cash"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

// $curl -X PATCH localhost:8000/api/user/v1/cash -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "amount": 1000000 }'
// ($curl -X PATCH localhost:8000/api/user/v1/cash -b cookie.txt -L -H "Content-Type: application/json" -d '{ "amount": 100000 }')

pub fn delete_user() -> BoxedFilter<()> {
    warp::delete()
        .and(user_api_v1_path_prefix())
        .and(path::end())
        .boxed()
}

// $curl -X DELETE localhost:8000/api/user/v1 -b cookie.txt -L

pub fn logout() -> BoxedFilter<()> {
    // Does path_prefix is necesary here?
    warp::get()
        .and(user_api_v1_path_prefix())
        .and(path("logout"))
        .and(path::end())
        .boxed()
}

// $curl -X GET localhost:8000/api/user/v1/logout -b cookie.txt -L
