# Rust as the base image
FROM rust:latest as build

# Create a new empty shell project
RUN USER=root cargo new --bin annuaire
WORKDIR /annuaire

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

RUN apt-get update && apt-get install -y musl-dev libpq-dev git gcc libpq-dev
# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/annuaire*
RUN cargo build --release

# The final base image
FROM debian:buster-slim AS annuaire

# Copy from the previous build
COPY --from=build /annuaire/target/release/annuaire /usr/src/annuaire
# COPY --from=build /holodeck/target/release/holodeck/target/x86_64-unknown-linux-musl/release/holodeck .

# Run the binary
CMD ["/usr/src/annuaire"]