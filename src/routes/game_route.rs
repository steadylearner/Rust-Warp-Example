use warp::{
    body::{content_length_limit, json},
    filters::BoxedFilter,
    path, Filter,
};

use crate::{json_body, models::game::NewGameRequest};

use super::{
    user_api_v1_path_prefix,
    // game_api_v1_path_prefix,
};

pub fn new() -> BoxedFilter<(NewGameRequest,)> {
    warp::post()
        .and(user_api_v1_path_prefix()) // Should be game_api_v1_path_prefix later?
        .and(path("game"))
        .and(path::end())
        .and(json_body!())
        .boxed()
}

// Ok

// 1. Cash(stake_amount) only

// $curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }'
// ($curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L -H "Content-Type: application/json" -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }')

// 2. Car only

// $curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "stake_amount": null, "car_id": 1, "number_of_participants": 2 }'
// ($curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L -H "Content-Type: application/json" -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }')

// Err

// 1. without Cash and Car(both null)

// $curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "stake_amount": null, "car_id": null, "number_of_participants": 2 }'
// ($curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L -H "Content-Type: application/json" -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }')

// 2. Both Cash and Car(both with value)

// $curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L
// -H "Content-Type: application/json"
// -d '{ "stake_amount": null, "car_id": null, "number_of_participants": 2 }'
// ($curl -X POST localhost:8000/api/user/v1/game -b cookie.txt -L -H "Content-Type: application/json" -d '{ "stake_amount": 10000, "car_id": null, "number_of_participants": 2 }')

// Should write more tests to handle errors here.

// 3. Numer of participants less than 2.