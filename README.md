# CI/CD Playground — Multi-Language Hello World Apps

A learning repo for CI/CD pipelines. Each app is a minimal HTTP server with `GET /` and `GET /health`, containerised and wired to a GitHub Actions pipeline that builds, tests, and pushes to GHCR.

## Apps

| App | Language | Framework |
|-----|----------|-----------|
| [python-flask](apps/python-flask/) | Python 3.12 | Flask + Gunicorn |
| [node-express](apps/node-express/) | Node.js 22 | Express 4 |
| [go-http](apps/go-http/) | Go 1.22 | net/http |
| [java-spring](apps/java-spring/) | Java 21 | Spring Boot 3 |
| [ruby-sinatra](apps/ruby-sinatra/) | Ruby 3.3 | Sinatra |
| [php-native](apps/php-native/) | PHP 8.3 | none |
| [rust-actix](apps/rust-actix/) | Rust stable | Actix-web 4 |
| [dotnet-minimal](apps/dotnet-minimal/) | .NET 8 | ASP.NET Core Minimal API |

## CI/CD Pipeline (per app)

Each workflow (`.github/workflows/<name>.yml`) runs on push/PR to `main` when files under that app change:

1. **build** — `docker build`, saves image as artifact
2. **lint** — language-specific linter (ruff / eslint / golangci-lint / checkstyle / rubocop / phpcs / cargo clippy / dotnet format)
3. **test** — runs the container, `curl /health`, asserts HTTP 200
4. **push** — logs in to GHCR, pushes `:<sha>` and `:latest` tags (main branch only)

## Secrets required

| Secret | Purpose |
|--------|---------|
| `GHCR_USERNAME` | GitHub username |
| `GHCR_TOKEN` | PAT with `write:packages` |
