use warp::{
    filters::BoxedFilter,
    path, Filter,
};

use super::super::user_api_v1_path_prefix;

pub fn get() -> BoxedFilter<()> {
    warp::get()
        .and(user_api_v1_path_prefix())
        .and(path("profile"))
        .and(path::end())
        .boxed()
}

// $curl -X POST localhost:8000/api/user/v1/login -c cookie.txt -H "Content-Type: application/json" -d '{ "email": "random@email.com", "password": "password" }'
// $curl -X GET localhost:8000/api/user/v1/profile -b cookie.txt -L
