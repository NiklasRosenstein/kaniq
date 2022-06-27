#!/bin/sh

content=$(cat Cargo.toml | sed 's/^version = ".*"/version = "'$1'"/')
echo "$content" > Cargo.toml
