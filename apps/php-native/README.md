# php-native

## Stack
- PHP 8.3
- No framework (PHP built-in server)

## Run locally

```bash
php -S 0.0.0.0:8080 apps/php-native/src/index.php
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t php-native ./apps/php-native
docker run --rm -p 8080:8080 php-native
```

## Expected output

```
curl http://localhost:8080/
Hello from PHP!

curl http://localhost:8080/health
{"status":"ok"}
```
