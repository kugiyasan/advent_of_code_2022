#!/usr/bin/env bash

set -eo pipefail

# cookies.txt should contain a line with "session=<your session cookie>"
COOKIES=$(cat cookies.txt)
DAY=$1

curl -O --cookie $COOKIES "https://adventofcode.com/2022/day/$DAY/input"
