FROM rust:slim AS builder

WORKDIR /build

COPY src/Cargo.toml .
COPY src/main.rs .

RUN cargo build --release

FROM debian:bookworm-slim

RUN addgroup --system appgroup && adduser --system --ingroup appgroup appuser

WORKDIR /app
COPY --from=builder /build/target/release/app .

RUN chown appuser:appgroup app
USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

CMD ["./app"]
