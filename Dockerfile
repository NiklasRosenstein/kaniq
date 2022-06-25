FROM rust:alpine as kaniq
RUN apk add musl-dev
WORKDIR /kaniq
COPY kaniq/ ./
ENV CARGO_HOME /root/.cargo
RUN --mount=type=cache,dst=/root/.cargo/ --mount=type=cache,dst=./target/ : \
    && time cargo build --release \
    && cp target/release/kaniq /usr/local/bin/kaniku

FROM gcr.io/kaniko-project/executor:debug as kaniku
COPY --from=kaniq /usr/local/bin/kaniku /kaniko/kaniku
