extern crate reqwest;
extern crate serde;
extern crate serde_json;

const TOP_STORIES: &'static str = "https://hacker-news.firebaseio.com/v0/topstories.json";

pub fn fetch_top_stories() -> Result<Vec<u64>, Box<std::error::Error>> {
  let client = reqwest::Client::new();
  let res = client.get(TOP_STORIES).send()?.text()?;
  let top_stories: Vec<u64> = serde_json::from_str(&res)?;
  Ok(top_stories)
}
