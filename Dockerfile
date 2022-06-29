
# We build based on alpine so that we can have a fully statically linked binary using musl.
FROM rust:alpine as build
RUN apk add musl-dev git

WORKDIR /kaniq
COPY kaniq/ ./
ENV CARGO_HOME /root/.cargo

# Fetch dependencies; this greatly improves CI if no source files change.
RUN --mount=type=cache,dst=${CARGO_HOME} cargo fetch

# Bump the version in Cargo.toml.
ARG GIT_VERSION
RUN --mount=type=bind,source=bin,dst=bin set -eou pipefail \
    && bin/update-crate-version.sh ${GIT_VERSION} \
    && cat Cargo.toml

# Execute the build and copy the binary to /usr/local/bin.
RUN --mount=type=cache,dst=${CARGO_HOME} \
    --mount=type=cache,dst=./target/ : \
    && time cargo build --release \
    && cp target/release/kaniq /usr/local/bin/kaniq

FROM gcr.io/kaniko-project/executor:debug as kaniq
COPY --from=build /usr/local/bin/kaniq /kaniko/kaniq
