#!/usr/bin/env python3

import argparse
import os
import rtoml
import sys

CARGO_TOML = '''\
[package]
name = "<name>"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
advent = { path = "../advent" }
'''

MAIN_RS = '''\
use advent::prelude::*;

#[part_one]
fn part_one(_: String) -> &'static str {
    "incomplete"
}

#[part_two]
fn part_two(_: String) -> &'static str {
    "incomplete"
}

harness!();

'''

def add_to_workspace(name: str):
    with open('Cargo.toml', 'a+') as f:
        f.seek(0)
        t = rtoml.load(f)

        members = set(t['workspace']['members'])
        members.add(name)
        members = list(members)
        members.sort()
        t['workspace']['members'] = members

        f.truncate(0)
        f.write(rtoml.dumps(t, pretty=True))

def add_new_question(name: str) -> int:
    if os.path.exists(name):
        print(f"ERROR: {name} already exists")
        return 1

    os.mkdir(name)
    os.mkdir(os.path.join(name, 'src'))

    with open(os.path.join(name, 'Cargo.toml'), 'w') as f:
        f.write(CARGO_TOML.replace('<name>', name))

    with open(os.path.join(name, 'src/main.rs'), 'w') as f:
        f.write(MAIN_RS)

    add_to_workspace(name)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("name")
    args = parser.parse_args()
    return add_new_question(args.name)


if __name__ == "__main__":
    sys.exit(main())
