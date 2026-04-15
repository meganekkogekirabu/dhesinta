#!/usr/bin/env python3
# -*- coding: utf8 -*-

import random
import string
import sys
from dataclasses import asdict, dataclass
from pathlib import Path

from tomli_w import dumps

cwd = Path.cwd().parent
config_file = cwd / "config.toml"


@dataclass
class NetConfig:
    hostname: str
    port: int


@dataclass
class Config:
    db_url: str
    net: NetConfig
    secret_key: str


BOLD = "\033[1m"
YELLOW = "\033[33m"
BLUE = "\033[34m"
RESET = "\033[0m"
UP = "\033[F"
CLEAR = "\033[K"


def prompt(field: str, default: str) -> str:
    response = (
        input(f"{BOLD}{YELLOW}?{RESET} {BOLD}{field}{RESET} [default {default}]: ")
        or default
    )
    _ = sys.stdout.write(UP + CLEAR)
    print(f"{YELLOW}?{RESET} {field}: {BLUE}{response}{RESET}")
    return response


hostname = prompt("Hostname", "0.0.0.0")
port = int(prompt("Port", "3000"))
net = NetConfig(hostname, port)

db_url = prompt("Database URL", "sqlite://./db/db.sqlite3")
secret_key = [random.choice(string.ascii_letters + string.digits) for _ in range(32)]
secret_key = "".join(secret_key)

config = Config(db_url, net, secret_key)
config = asdict(config)

toml = dumps(config)

with open(config_file, "w") as file:
    _ = file.write(toml)
