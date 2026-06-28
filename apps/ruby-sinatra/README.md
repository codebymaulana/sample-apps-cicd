# ruby-sinatra

## Stack
- Ruby 3.3
- Sinatra 4.1.1

## Run locally

```bash
cd apps/ruby-sinatra/src
bundle install
bundle exec ruby app.rb
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t ruby-sinatra ./apps/ruby-sinatra
docker run --rm -p 8080:8080 ruby-sinatra
```

## Expected output

```
curl http://localhost:8080/
Hello from Ruby!

curl http://localhost:8080/health
{"status":"ok"}
```
