FROM rust:1.73-alpine3.17 as builder
RUN apk update && apk upgrade
RUN apk add --no-cache sqlite
WORKDIR /workspace
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY prepare.sql prepare.sql
RUN sqlite3 decorum.db < prepare.sql
COPY src src
COPY .sqlx .sqlx
RUN cargo build --release

FROM alpine:3.17 as runner
WORKDIR /workspace
COPY --from=builder /workspace/target/release/decorum-api .
COPY --from=builder /workspace/decorum.db .
VOLUME ["/workspace/decorum.db"]
COPY .env .
ENTRYPOINT ["/workspace/scraper"]
