FROM rust:1.62.1 AS builder

# create a new empty shell project
RUN USER=root cargo new --bin testnet
WORKDIR /testnet

# copy over manifests and source to build image
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# build for release
RUN cargo build --all --release

# final base
FROM ubuntu:22.04

# copy build artifacts from build stage
COPY --from=builder /testnet/target/release/testnet .

ENV RUST_LOG=debug

ENTRYPOINT ["./testnet"]
