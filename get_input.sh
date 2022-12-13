#!/usr/bin/env bash

set -eo pipefail

# cookies.txt should contain a line with "session=<your session cookie>"
COOKIES=$(cat cookies.txt)
DAY=$1
NAME=$2
FOLDER="d$1_$2"
MAINFILE="$FOLDER/src/main.rs"

curl -O --cookie $COOKIES "https://adventofcode.com/2022/day/$DAY/input"

cargo new $FOLDER
mv input $FOLDER

# Append the new workspace to Cargo.toml
sed -E -i "s/(^    \".+\"$)/\1,\n    \"$FOLDER\"/g" Cargo.toml

cat > $MAINFILE << EOF
use std::fs;

fn main() {
    let path = "input";
    // let path = "example";
    let buf = fs::read_to_string(path).unwrap();

    let input: Vec<_> = buf.trim().split('\n').collect();

    println!("{:?}", &input[0..5]);
}
EOF
