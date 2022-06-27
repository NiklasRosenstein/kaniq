#!/bin/bash

cd kaniq

echo '### `kaniq`'
echo '```'
cargo run -q help
echo '```'
echo

echo '### `kaniq execute`'
echo '```'
cargo run -q help execute
echo '```'
echo

echo '### `kaniq config`'
echo '```'
cargo run -q help config
echo '```'
echo

echo '### `kaniq local`'
echo '```'
cargo run -q help local
echo '```'
echo
