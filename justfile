location := "./db/db.sqlite3"

alias i := init
alias r := run

init:
    sqlite3 {{ location }} < ./db/schema.sql

run:
    cargo watch --why -w src -x run

release:
    cargo build --release --frozen
