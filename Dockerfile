FROM rust:1.79-bookworm AS dev

RUN apt-get update && apt-get install -y git && rustup component add clippy rls rust-analysis rustfmt

FROM dev AS build

WORKDIR /build

COPY ./src /build/src
COPY ./Cargo.toml ./Cargo.lock /build/

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 AS runtime

COPY --from=build /build/target/release/terac /usr/local/bin/terac

CMD ["/usr/local/bin/terac"]
