# dotnet-minimal

## Stack
- .NET 8 LTS (C#)
- ASP.NET Core Minimal API

## Run locally

```bash
cd apps/dotnet-minimal/src
dotnet run
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t dotnet-minimal ./apps/dotnet-minimal
docker run --rm -p 8080:8080 dotnet-minimal
```

## Expected output

```
curl http://localhost:8080/
Hello from .NET!

curl http://localhost:8080/health
{"status":"ok"}
```
