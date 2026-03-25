alias r := run
alias d := docker

default: config release

run:
    cargo watch --why -w src -x run

release:
    cargo run --release

config:
    #!/usr/bin/env bash
    cd bin
    python3 -m venv .venv
    . .venv/bin/activate
    pip install -r requirements.txt
    python3 configure.py
    cd ..

docker port:
    docker build --build-arg PORT={{ port }} -t dhesinta .
    docker run -p {{ port }}:{{ port }} -it dhesinta
