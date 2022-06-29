#!/bin/bash

set -euo pipefail

cd kaniq

echo '### `kaniq`'
echo '```'
cargo run -q -- -h
echo '```'
echo

echo '### `kaniq auth`'
echo '```'
cargo run -q auth -h
echo '```'
echo

echo '### `kaniq execute`'
echo '```'
cargo run -q execute -h
echo '```'
echo

echo '### `kaniq run`'
echo '```'
cargo run -q run -h
echo '```'
echo
