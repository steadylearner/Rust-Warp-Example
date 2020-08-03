#[macro_export]
macro_rules! index {
    () => {
        index_route::get().and_then(index_handler::get)
    };
}
