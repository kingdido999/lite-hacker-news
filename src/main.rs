#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate dotenv;
extern crate job_scheduler;
extern crate r2d2;
extern crate reqwest;
extern crate rocket;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod job;
mod pg_pool;

use dotenv::dotenv;
use std::env;
use std::thread;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    thread::spawn(|| {
        job::run();
    });

    rocket::ignite()
        .manage(pg_pool::init(&database_url))
        .mount("/", routes![index])
        .launch();
}
