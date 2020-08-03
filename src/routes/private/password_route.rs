use crate::{
    json_body,
    models::private::{
        password::UpdatePasswordRequest,
    }
};

use warp::{
    body::{content_length_limit, json},
    filters::BoxedFilter,
    path, Filter,
};

use super::super::user_api_v1_path_prefix;

pub fn update_password() -> BoxedFilter<(UpdatePasswordRequest,)> {
    warp::patch()
        .and(user_api_v1_path_prefix())
        .and(path("password"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

// $curl -X POST localhost:8000/api/user/v1/login -c cookie.txt -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'

// $curl -X PATCH localhost:8000/api/user/v1/password -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "oldPassword": "random@email.com", "newPassword": "newpassword" }'
// ($curl -X PATCH localhost:8000/api/user/v1/password -b cookie.txt -L -H "Content-Type: application/json" -d '{ "oldPassword": "oldpassword", "newPassword": "newpassword" }')

