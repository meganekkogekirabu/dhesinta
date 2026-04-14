FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update
RUN apt-get install -y libclang-19-dev
RUN rm -rf /var/lib/apt/lists/*
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

ENV DATABASE_URL="sqlite://./db/db.sqlite3"
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo install sqlx-cli --locked
RUN sqlx database create
RUN sqlx migrate run --source ./db/migrations/
RUN cargo build --release

FROM debian:trixie-slim AS runtime
ENV DATABASE_URL="sqlite://./db/db.sqlite3"
WORKDIR /app
COPY --from=builder /app/target/release/dhesinta /usr/local/bin/
COPY --from=builder /app/db/ db/
ENTRYPOINT [ "/usr/local/bin/dhesinta" ]
