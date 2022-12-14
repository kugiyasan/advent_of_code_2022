#!/usr/bin/env bash

set -eo pipefail

# cookies.txt should contain a line with "session=<your session cookie>"
local COOKIES=$(cat cookies.txt)
local DAY=$1
local NAME=$2
local FOLDER="d$1_$2"

# Append the new workspace to Cargo.toml
sed -E -i "s/(^    \".+\"$)/\1,\n    \"$FOLDER\"/g" Cargo.toml

cargo new $FOLDER
cd $FOLDER
cargo add aoc_utils

cat > src/main.rs << EOF
use std::fs;
use aoc_utils::submit;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    println!("{:?}", &input[0..5]);
    // submit("1", false);
}
EOF

if [ -e "input" ]; then
    echo 'input file already exists' >&2
    exit 1
fi

curl -O --cookie $COOKIES "https://adventofcode.com/2022/day/$DAY/input"
