#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_derive;

pub mod hackernews;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
