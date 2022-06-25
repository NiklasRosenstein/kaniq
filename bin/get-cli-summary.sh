#!/bin/bash

cd kaniq

echo '### `kaniku`'
echo '```'
cargo run -q help
echo '```'
echo

echo '### `kaniku execute`'
echo '```'
cargo run -q help execute
echo '```'
echo

echo '### `kaniku config`'
echo '```'
cargo run -q help config
echo '```'
echo

echo '### `kaniku local`'
echo '```'
cargo run -q help local
echo '```'
echo
