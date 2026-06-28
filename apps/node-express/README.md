# node-express

## Stack
- Node.js 22 LTS
- Express 4.21.2

## Run locally

```bash
cd apps/node-express/src
npm ci --omit=dev
node index.js
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t node-express ./apps/node-express
docker run --rm -p 8080:8080 node-express
```

## Expected output

```
curl http://localhost:8080/
Hello from Node.js!

curl http://localhost:8080/health
{"status":"ok"}
```
