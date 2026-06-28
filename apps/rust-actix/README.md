# rust-actix

## Stack
- Rust (latest stable)
- Actix-web 4.9.0

## Run locally

```bash
cd apps/rust-actix/src
cargo run
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t rust-actix ./apps/rust-actix
docker run --rm -p 8080:8080 rust-actix
```

## Expected output

```
curl http://localhost:8080/
Hello from Rust!

curl http://localhost:8080/health
{"status":"ok"}
```
