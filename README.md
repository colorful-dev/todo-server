# Todo-Server

Todo-server impl by using [actix.rs](https://actix.rs)

## Why actix.rs

### 1. It's blazing fast

[Benchmark](https://www.techempower.com/benchmarks/#section=data-r21&hw=ph&test=fortune)

From now on, Actix is the second fastest web framework in the world.

### 2. It's easy to use

Don't need register routes in main function always.

```rust
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
```

## 3. Debug

### need install deps

docker  
docker-compose  
postgres (in lib/postgres)  

```bash
cargo install diesel_cli --no-default-features --features "postgres"
cargo install cargo-watch
brew install httpie
# in this workspace
diesel migration run
```

### after install deps

```bash
# start server (require cargo-watch) 
make start

# test server (require httpie)
make test
```
