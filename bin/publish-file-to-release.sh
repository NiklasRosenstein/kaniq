#!/bin/bash

set -euo pipefail

GITHUB_TOKEN="$1"
RELEASE_NAME="$2"
LOCAL_FILE="$3"
TARGET_FILE="$4"

function gh-curl() {
    curl -v --fail-with-body -H "Accept: application/vnd.github.v3+json" -H "Authorization: token $GITHUB_TOKEN" "$@"
}

release_id=$(gh-curl "https://api.github.com/repos/NiklasRosenstein/kaniq/releases/tags/$RELEASE_NAME" | jq .id)

gh-curl -X POST "https://uploads.github.com/repos/NiklasRosenstein/kaniq/releases/$release_id/assets?name=${TARGET_FILE}" \
    --header 'Content-Type: application/octet-stream' \
    --upload-file "${LOCAL_FILE}"
