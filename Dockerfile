FROM rust:latest

RUN apt-get update
RUN apt-get --yes install just sqlite3 clang libclang-dev python3-full

WORKDIR /app

COPY . .

ENV PATH="~/.cargo/bin:$PATH"
ENV DATABASE_URL="sqlite://./db/db.sqlite3"
RUN cargo install sqlx-cli --no-default-features --features sqlite-unbundled
RUN sqlx database create
RUN sqlx migrate run --source db/migrations

ENTRYPOINT ["just"]

EXPOSE $PORT
