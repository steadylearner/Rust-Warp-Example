#[macro_export]
macro_rules! register_user {
    () => {
        user_route::register().and_then(user_handler::register)
    };
}

#[macro_export]
macro_rules! list_users {
    () => {
        user_route::list().and_then(user_handler::list)
    };
}

#[macro_export]
macro_rules! do_login {
    () => {
        user_route::do_login().and_then(user_handler::do_login)
    };
}

// #[macro_export]
// macro_rules! update_password {
//     () => {
//         user_route::update_password()
//             .and(user_session_filter())
//             .and_then(user_handler::update_password)
//     };
// }

#[macro_export]
macro_rules! update_cash {
    () => {
        user_route::update_cash()
            .and(user_session_filter())
            .and_then(user_handler::update_cash)
    };
}

#[macro_export]
macro_rules! delete_user {
    () => {
        user_route::delete_user()
            .and(user_session_filter())
            .and_then(user_handler::delete_user)
    };
}

#[macro_export]
macro_rules! logout {
    () => {
        user_route::logout()
            .and(user_session_filter())
            .and_then(user_handler::logout)
    };
}
