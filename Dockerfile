FROM rust:alpine as build
RUN apk add musl-dev git
WORKDIR /kaniq
COPY kaniq/ ./
ENV CARGO_HOME /root/.cargo
ARG GIT_VERSION
RUN --mount=type=bind,source=bin,dst=bin \
    set -eou pipefail \
    && bin/update-crate-version.sh ${GIT_VERSION} \
    && cat Cargo.toml
RUN --mount=type=cache,dst=/root/.cargo/ \
    --mount=type=cache,dst=./target/ : \
    && time cargo build --release \
    && cp target/release/kaniq /usr/local/bin/kaniq

FROM build as publish
RUN apk add curl
COPY bin/publish-file-to-release.sh publish.sh
ENTRYPOINT [ "/bin/bash", "/kaniq/publish.sh" ]

FROM gcr.io/kaniko-project/executor:debug as kaniq
COPY --from=build /usr/local/bin/kaniq /kaniko/kaniq
