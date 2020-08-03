use warp::{
    filters::BoxedFilter,
    path, Filter,
};

use super::{
    ranking_api_v1_path_prefix,
};

pub fn game_ranking_list() -> BoxedFilter<()> {
    warp::get()
        .and(ranking_api_v1_path_prefix())
        .and(path("game"))
        .and(path::end())
        .boxed()
}

// $curl localhost:8000/api/ranking/v1/game