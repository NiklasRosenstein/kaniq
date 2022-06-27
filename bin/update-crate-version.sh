#!/bin/sh

set -euxo pipefail
content=$(cat Cargo.toml | sed 's/^version = ".*"/version = "'$1'"/')
echo "$content" > Cargo.toml
