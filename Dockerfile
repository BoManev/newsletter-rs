# builder
FROM rust:1.71.1 AS builder

WORKDIR /app
RUN apt update && apt install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# runtime

FROM rust:1.71.1 as runtime

WORKDIR /app
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]
