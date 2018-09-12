extern crate reqwest;
const TOP_STORIES: &'static str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub fn fetch_top_stories() -> Result<(), Box<std::error::Error>> {
  let mut res = reqwest::get(TOP_STORIES)?;
  println!("Status: {}", res.status());
  println!("Headers:\n{:?}", res.headers());

  std::io::copy(&mut res, &mut std::io::stdout())?;

  Ok(())
}
