pub mod error_handler;

// pub mod hello_handler;
// pub mod hi_handler;
// pub mod authorized_handler;

pub mod index_handler; // Should be a single page app.
pub mod user_handler;
pub mod car_handler;
pub mod game_handler;
pub mod ranking_handler;

pub mod private;

// Will be used at error_hanlder.rs and others synchronously.
use warp::{
    reject::{
        Reject
    },
};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct UNAUTHORIZED;
impl Reject for UNAUTHORIZED {}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct INTERNAL_SERVER_ERROR;
impl Reject for INTERNAL_SERVER_ERROR {}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct NOT_ACCEPTABLE;
impl Reject for NOT_ACCEPTABLE {}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct BAD_REQUEST;
impl Reject for BAD_REQUEST {}