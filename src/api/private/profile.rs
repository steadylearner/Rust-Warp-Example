#[macro_export]
macro_rules! get_profile {
    () => {
        profile_route::get()
            .and(user_session_filter())
            .and_then(profile_handler::get)
    };
}