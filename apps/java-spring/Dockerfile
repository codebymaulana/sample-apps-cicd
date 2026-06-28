FROM maven:3.9-eclipse-temurin-21 AS builder

WORKDIR /build

COPY src/pom.xml .
RUN mvn dependency:go-offline -q

COPY src/main ./main

RUN mvn package -DskipTests -q

FROM eclipse-temurin:21-jre-alpine

RUN addgroup -S appgroup && adduser -S appuser -G appgroup

WORKDIR /app
COPY --from=builder /build/target/java-spring-0.0.1-SNAPSHOT.jar app.jar

RUN chown appuser:appgroup app.jar
USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 \
  CMD wget -qO- http://localhost:8080/health || exit 1

CMD ["java", "-jar", "app.jar"]
