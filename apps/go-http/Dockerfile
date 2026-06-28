FROM golang:1.22-alpine AS builder

WORKDIR /build

COPY src/go.mod .
COPY src/main.go .

RUN go build -o app .

FROM alpine:3.20

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app
COPY --from=builder /build/app .

RUN chown appuser:appgroup app
USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget -qO- http://localhost:8080/health || exit 1

CMD ["./app"]
