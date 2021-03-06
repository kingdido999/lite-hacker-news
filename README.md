# Lite Hacker News

As a heavy user of [Hacker News](https://news.ycombinator.com/), I've been wasting quite a lot of time waiting for articles to be opened and loaded before I can consume them. The goal of this project is to build a blazing fast Hacker News reader with articles fully cached while learning some [Rust](https://www.rust-lang.org).

## TODO

- [x] Fetch top stories via [HackerNews API](https://github.com/HackerNews/API)
- [ ] Run a job to fetch and save stories in DB periodically
- [ ] Render top stories in HTML
- [ ] Load and cache the content of stories in client-side

## Development

Create a `.env` file out of the example and modify username and password for your local [PostgreSQL](https://www.postgresql.org/) instance:

```bash
cp .env.example .env
```

Start the project:

```bash
cargo run
```