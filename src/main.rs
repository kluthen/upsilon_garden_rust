#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate rand;
extern crate postgres;


extern crate rocket;
extern crate rocket_contrib;



// loads .env : configuraiton stuff.
mod config;
mod db;
mod web;
mod engine; 

#[allow(dead_code)]
fn test_db() {
    let mut gard = engine::garden::Garden::new();
    gard.name = "Kluth".to_string();
    gard.repsert();

    println!("Main: Test: Garden inserted ? {:?}", gard);

    let gard_2 = engine::garden::Garden::by_id(&gard.id).unwrap();
    println!("Main: Fetched Garden: {:?}", gard_2);

    engine::garden::Garden::drop(&gard.id);

    if let None = engine::garden::Garden::by_id(&gard.id) {
        println!("Main: Test: Correctly dropped garden.");
    }
}


fn main() {    
    env_logger::init();
    test_db();

    web::server::setup();
    
    info!("starting up");

}