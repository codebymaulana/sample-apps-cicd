FROM php:8.3-cli-alpine

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app

COPY src/index.php .

RUN chown appuser:appgroup index.php
USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget -qO- http://localhost:8080/health || exit 1

CMD ["php", "-S", "0.0.0.0:8080", "index.php"]
