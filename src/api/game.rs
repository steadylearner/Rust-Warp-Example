#[macro_export]
macro_rules! new_game {
    () => {
        game_route::new()
            .and(user_session_filter())
            .and_then(game_handler::new)
    };
}
