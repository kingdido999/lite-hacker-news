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

#[cfg(test)]
mod tests {
    use super::hackernews;

    #[test]
    fn it_fetches_top_stories() {
        let top_stories = hackernews::fetch_top_stories().unwrap();
        assert!(top_stories.len() > 0);
    }
}
