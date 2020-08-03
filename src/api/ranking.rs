#[macro_export]
macro_rules! game_ranking_list {
    () => {
        ranking_route::game_ranking_list()
            .and_then(ranking_handler::game_ranking_list)
    };
}
