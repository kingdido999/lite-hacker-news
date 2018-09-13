extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize)]
pub struct Item {
  id: u64,

  // `type` is a reserved keyword
  #[serde(rename(deserialize = "type"))]
  _type: String,

  by: String,
  time: u64,
  text: Option<String>,
  url: Option<String>,
  score: Option<u32>,
  title: Option<String>,
}

pub fn fetch_top_stories() -> Result<Vec<u64>, Box<std::error::Error>> {
  let client = reqwest::Client::new();
  let res = client
    .get("https://hacker-news.firebaseio.com/v0/topstories.json")
    .send()?
    .text()?;
  let top_stories: Vec<u64> = serde_json::from_str(&res)?;
  Ok(top_stories)
}

pub fn fetch_item_by_id(id: u64) -> Result<Item, Box<std::error::Error>> {
  let client = reqwest::Client::new();
  let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
  let res = client.get(&*url).send()?.text()?;
  let item: Item = serde_json::from_str(&res)?;
  Ok(item)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_fetches_top_stories() {
    let top_stories = fetch_top_stories().unwrap();
    assert!(top_stories.len() > 0);
  }

  #[test]
  fn it_fetches_item_by_id() {
    let id = 8863;
    let item = fetch_item_by_id(id).unwrap();
    assert_eq!(item.id, id);
    assert_eq!(item._type, String::from("story"));
    assert_eq!(item.by, String::from("dhouston"));
    assert_eq!(
      item.title.unwrap(),
      String::from("My YC app: Dropbox - Throw away your USB drive")
    );
    assert_eq!(
      item.url.unwrap(),
      String::from("http://www.getdropbox.com/u/2/screencast.html")
    );
  }
}
