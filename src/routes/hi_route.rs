// use warp::{
//     filters::BoxedFilter,
//     path,
//     Filter,
// };

// pub fn hi() -> BoxedFilter<(String, )> {
//     warp::get()
//         .and(path("hi"))
//         .and(path::param::<String>())
//         .boxed()
// }
