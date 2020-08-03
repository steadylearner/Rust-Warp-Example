#[macro_export]
macro_rules! update_password {
    () => {
        password_route::update_password()
            .and(user_session_filter())
            .and_then(password_handler::update_password)
    };
}