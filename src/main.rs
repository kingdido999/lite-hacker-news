#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate reqwest;
extern crate rocket;
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
        hackernews::fetch_top_stories();
    }
}
