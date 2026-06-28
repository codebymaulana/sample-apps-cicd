# go-http

## Stack
- Go 1.22+
- Standard library `net/http` only

## Run locally

```bash
cd apps/go-http/src
go run main.go
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t go-http ./apps/go-http
docker run --rm -p 8080:8080 go-http
```

## Expected output

```
curl http://localhost:8080/
Hello from Go!

curl http://localhost:8080/health
{"status":"ok"}
```
