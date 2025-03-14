# 1) Build Stage
FROM rust:latest as builder
WORKDIR /app
COPY . .
# Ensure migrations folder is included
RUN cargo build --release

# 2) Runtime Stage
FROM debian:stable-slim
WORKDIR /app
COPY --from=builder /app/target/release/hackademy_sqlite /usr/local/bin/hackademy_sqlite
COPY .env ./
# Copy migrations if needed at runtime; or embed them with sqlx
COPY migrations ./migrations

EXPOSE 3000
CMD ["hackademy_sqlite"]