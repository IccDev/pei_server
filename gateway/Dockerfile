################
##### Builder (check latest version on rust and put it here)
FROM rust:1.78.0-slim as builder

WORKDIR /usr/src

# Create blank project
RUN mkdir -p /usr/src/gateway
RUN mkdir -p /usr/src/gateway/src/


# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/gateway/
COPY src/ /usr/src/gateway/src/

# Set the working directory
WORKDIR /usr/src/gateway

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev libsecret-1-0 git


# This is the actual application build.
RUN cargo build -p gateway --target x86_64-unknown-linux-musl --release
#RUN cargo build -p gateway --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM alpine:latest AS gateway 

# Copy application binary from builder image
COPY --from=builder /usr/src/gateway/target/x86_64-unknown-linux-musl/release/gateway /usr/local/bin

#EXPOSE 3030

# Run the application
CMD ["/usr/local/bin/gateway"]