#!/bin/bash

cd kaniq
../bin/update-crate-version.sh $(git describe --tags)
cargo build --release

function gh-curl() {
    curl -v --fail-with-body -H "Accept: application/vnd.github.v3+json" -H "Authorization: token $GITHUB_TOKEN" "$@"
}

if [ "${BITRISE_GIT_TAG:-}" != "" ]; then

    echo "Uploading to GitHub Release"

    # Get the release by tag name.
    release_id=$(gh-curl "https://api.github.com/repos/NiklasRosenstein/kaniq/releases/tags/$BITRISE_GIT_TAG" | jq .id)

    # Upload the file to the release.
    gh-curl -X POST "https://uploads.github.com/repos/NiklasRosenstein/kaniq/releases/$release_id/assets?name=kaniq-darwin-`arch`-$BITRISE_GIT_TAG" \
    --header 'Content-Type: application/octet-stream' \
    --upload-file target/release/kaniq

else
    echo "Skip uploading to GitHub Release (no tag)."
fi
