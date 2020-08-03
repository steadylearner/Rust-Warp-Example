use std::error::Error;
use warp::{self, Filter};

#[macro_use]
extern crate lazy_static;

use pretty_env_logger;
#[macro_use]
extern crate log;

mod api;
mod handlers;
mod routes;

mod models;
mod security;
mod template_setup;
mod utils;

mod db;
mod session;

mod server;

use self::{
    db::sqlite::setup,
    server::{end, routes_info},
    handlers::error_handler::handle_rejection,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    if let Err(e) = setup() {
        eprintln!("{:#?}", e);
        ::std::process::exit(1);
    }

    routes_info();

    Ok(warp::serve(end().recover(handle_rejection))
        .run(([0, 0, 0, 0], 8000))
        .await)
}
