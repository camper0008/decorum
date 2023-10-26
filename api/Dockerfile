FROM rust:1.73-alpine3.17 as builder
WORKDIR /workspace
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY prepare.sql prepare.sql
RUN sqlite3 decorum.db < prepare.sql
COPY src src
RUN cargo build --release

FROM alpine:3.17 as runner
WORKDIR /workspace
COPY --from=builder /workspace/target/release/decorum-api .
COPY --from=builder /workspace/decorum.db .
COPY .env .
ENTRYPOINT ["/workspace/scraper"]
