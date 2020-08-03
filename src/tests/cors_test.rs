use warp::http::method::Method;
use warp::Filter;

#[tokio::main]
async fn main() {
    let a = warp::path!("api" / "user" / "v1")
        .and(warp::post())
        .map(|| "")
        .with(warp::cors().allow_any_origin().allow_method(Method::POST));
    let filter = a;
    let response = warp::test::request()
        .path("/api/user/v1")
        .method("OPTIONS")
        .header("origin", "*")
        .header("access-control-request-method", "POST")
        .reply(&filter)
        .await;

    println!("{:#?}", response);
}

Response {
    status: 200,
    version: HTTP/1.1,
    headers: {
        "access-control-allow-headers": "",
        "access-control-allow-methods": "POST",
        "access-control-allow-origin": "*",
    },
    body: b"",
}

