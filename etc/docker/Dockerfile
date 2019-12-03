FROM rust AS builder

# build time optimization from:
# http://whitfin.io/speeding-up-rust-docker-builds/

# create a new empty shell project
RUN USER=root cargo new --bin parsley
WORKDIR /parsley

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# build for release
RUN rm ./target/release/deps/parsley*
RUN cargo build --release

RUN mkdir -p /build-out
RUN cp target/release/pdf_printer /build-out/

FROM debian:stretch-slim

COPY --from=builder /build-out/pdf_printer /

# No command or entrypoint since we are using this to just deploy the binary!
