# Shorty
This is an url shortener written in Rust based on the tutorial 
[here](https://matthias-endler.de/2017/rust-url-shortener/) 
and extended to store urls in a PostgreSQL DB
## Usage
### Install and run
```bash
cp .env.example .env
rustup override set nightly
cargo build
docker-compose up -d db
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
cargo run
```

### Basic usage

#### Store short url
``` bash
curl --data "url=https://www.example.com" http://localhost:8000/
Response: J0
```

#### Store named url
```bash
curl --data "url=https://www.example.com" --data "name=example" http://localhost:8000/named
Response: example
```

#### Use short url
```bash 
curl http://localhost:8000/J0
```
or
```bash 
curl http://localhost:8000/example
```
