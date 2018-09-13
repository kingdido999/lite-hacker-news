# Lite Hacker News

As a heavy user of [Hacker News](https://news.ycombinator.com/), I've been wasting quite a lot of time waiting for articles to be opened and loaded before I can consume them. The goal of this project is to build a blazing fast Hacker News reader with articles fully cached while learning some [Rust](https://www.rust-lang.org).

## TODO

- [x] Fetch top stories via [HackerNews API](https://github.com/HackerNews/API)
- [ ] Render top stories in HTML
- [ ] Subscribe and update top stories
- [ ] Save article contents in DB
- [ ] Load and cache the content of stories in client-side

## Development

```bash
cargo run
```