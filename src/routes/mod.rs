// pub mod hello_route;
// pub mod hi_route;
// pub mod authorized_route;

use warp::{
    filters::BoxedFilter,
    path, Filter
};

pub mod index_route;
pub mod user_route;

pub mod private;

pub mod car_route;
pub mod game_route;
pub mod ranking_route;

// Compare it with this that chain /String param.
// https://github.com/steadylearner/Rust-Full-Stack/blob/master/microservices_with_docker/warp_client/src/routes/user_route.rs

// Should I make a function or macro later only to substitute user, car, game etc part?
// https://docs.rs/warp/0.2.2/warp/macro.path.htmal#path-prefixes
pub fn user_api_v1_path_prefix() -> BoxedFilter<()> {
    path!("api" / "user" / "v1" / ..).boxed()
}

pub fn ranking_api_v1_path_prefix() -> BoxedFilter<()> {
    path!("api" / "ranking" / "v1" / ..).boxed()
}
