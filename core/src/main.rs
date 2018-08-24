// nightly only features
#![feature(integer_atomics)]
#![feature(const_fn)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate quick_error;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate serde;
extern crate rocket;
extern crate rocket_contrib;
extern crate yahoo_weather;
extern crate rzw;
extern crate toml;

pub mod error;
pub mod json;
pub mod model;
pub mod core;
pub mod sse;
pub mod devices;
pub mod weather;
pub mod zwave;
pub mod api;
pub mod log;


use core::Reactor;
use model::{SimpleEvent, Setting};
use rocket::config::Config;


fn main() {
    // load the settings
    let config = Setting::from_file();

    // setting up the reactor
    let reactor = Reactor::<SimpleEvent>::new();

    // start the logger
    log::start(&reactor).unwrap();

    // start the device handler
    devices::start(&reactor).unwrap();

    // start the weather service
    weather::start(&reactor, config.get_location()).unwrap();

    // start the zwave service
    zwave::start(&reactor, config.get_zwave()).unwrap();

    // configure the rocket webserver
    let config = Config::build(config.get_env())
        .port(config.get_port())
        .workers(100)
        .unwrap();

    // start the rocket webserver
    rocket::custom(config, true)
        .manage(reactor)
        .mount("/", routes![
            api::index, 
            api::api, 
            api::get_cmd, 
            api::post_cmd, 
            api::opt_cmd, 
            api::events, 
            api::files])
        .launch();
}


