#!/bin/bash

set -euxo pipefail
content=$(cat kaniq/Cargo.toml | sed 's/^version = ".*"/version = "'$1'"/')
echo "$content" > kaniq/Cargo.toml
