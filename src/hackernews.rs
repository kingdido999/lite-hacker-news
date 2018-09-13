extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::error::Error;

pub struct HackerNews {
  client: reqwest::Client,
}

impl HackerNews {
  pub fn new() -> HackerNews {
    HackerNews {
      client: reqwest::Client::new(),
    }
  }

  pub fn fetch_top_story_ids(&self) -> Result<Vec<u64>, Box<Error>> {
    let res = self
      .client
      .get("https://hacker-news.firebaseio.com/v0/topstories.json")
      .send()?
      .text()?;
    let top_stories: Vec<u64> = serde_json::from_str(&res)?;
    Ok(top_stories)
  }

  pub fn fetch_item_by_id(&self, id: u64) -> Result<Item, Box<Error>> {
    let url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);
    let res = self.client.get(&*url).send()?.text()?;
    let item: Item = serde_json::from_str(&res)?;
    Ok(item)
  }

  pub fn fetch_top_stories(&self) -> Result<Vec<Item>, Box<Error>> {
    // TODO: limit the number of stories to fetch
    let top_story_ids = self.fetch_top_story_ids().unwrap();
    let top_stories: Vec<Item> = top_story_ids
      .into_iter()
      .map(|id| self.fetch_item_by_id(id).unwrap())
      .collect();

    Ok(top_stories)
  }
}

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

#[cfg(test)]
mod tests {
  use super::HackerNews;

  #[test]
  fn it_fetches_top_story_ids() {
    let hn = HackerNews::new();
    let top_stories = hn.fetch_top_story_ids().unwrap();
    assert!(top_stories.len() > 0);
  }

  #[test]
  fn it_fetches_item_by_id() {
    let hn = HackerNews::new();
    let id = 8863;
    let item = hn.fetch_item_by_id(id).unwrap();
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

  #[test]
  fn it_fetches_top_stories() {
    let hn = HackerNews::new();
    let top_stories = hn.fetch_top_stories().unwrap();
    assert!(top_stories.len() > 0);
  }
}
