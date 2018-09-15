#![feature(plugin)]
#![feature(custom_attribute)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate serde_derive;

pub mod hackernews;
pub mod job;

use std::thread;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    thread::spawn(|| {
        job::run();
    });

    rocket::ignite().mount("/", routes![index]).launch();
}
