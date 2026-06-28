# java-spring

## Stack
- Java 21 LTS
- Spring Boot 3.3.4
- Maven 3.9

## Run locally

```bash
cd apps/java-spring/src
mvn spring-boot:run
```

App available at `http://localhost:8080`

## Run with Docker

```bash
docker build -t java-spring ./apps/java-spring
docker run --rm -p 8080:8080 java-spring
```

## Expected output

```
curl http://localhost:8080/
Hello from Java!

curl http://localhost:8080/health
{"status":"ok"}
```
