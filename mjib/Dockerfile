################
##### Builder (check latest version on rust and put it here)
FROM rust:1.85.0-slim as builder

WORKDIR /usr/src

# Create blank project
RUN mkdir -p /usr/src/mjib
RUN mkdir -p /usr/src/mjib/src/


# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/mjib/
COPY src/ /usr/src/mjib/src/

# Set the working directory
WORKDIR /usr/src/mjib

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev libsecret-1-0 git


# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM alpine:latest AS mjib 

# Copy application binary from builder image
COPY --from=builder /usr/src/mjib/target/x86_64-unknown-linux-musl/release/mjib /usr/local/bin


# Run the application
CMD ["/usr/local/bin/mjib"]