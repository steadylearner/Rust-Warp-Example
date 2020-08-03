#[macro_export]
macro_rules! buy_a_car {
    () => {
        car_route::buy()
            .and(user_session_filter())
            .and_then(car_handler::buy)
    };
}

#[macro_export]
macro_rules! list_cars {
    () => {
        car_route::list()
            .and(user_session_filter())
            .and_then(car_handler::list)
    };
}

#[macro_export]
macro_rules! refund_a_car {
    () => {
        car_route::refund()
            .and(user_session_filter())
            .and_then(car_handler::refund)
    };
}
