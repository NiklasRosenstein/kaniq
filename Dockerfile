FROM rust:alpine as build
RUN apk add musl-dev git
WORKDIR /kaniq
COPY kaniq/ ./
ENV CARGO_HOME /root/.cargo
RUN --mount=type=bind,source=bin,dst=bin \
    --mount=type=bind,source=.git,dst=.git \
    bin/update-crate-version.sh $(git describe --tags)
RUN --mount=type=cache,dst=/root/.cargo/ \
    --mount=type=cache,dst=./target/ : \
    && time cargo build --release \
    && cp target/release/kaniq /usr/local/bin/kaniq

FROM gcr.io/kaniko-project/executor:debug as kaniq
COPY --from=build /usr/local/bin/kaniq /kaniko/kaniq
