location := "./db/db.sqlite3"

alias i := init
alias r := run

init:
    touch {{ location }}

run:
    cargo watch --why -w src -x run

release:
    cargo build --release --frozen
