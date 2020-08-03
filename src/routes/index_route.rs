use warp::{filters::BoxedFilter, path, Filter};

pub fn get() -> BoxedFilter<()> {
    warp::get().and(path::end()).boxed()
}
