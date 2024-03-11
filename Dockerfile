################
##### Builder (check latest version on rust and put it here)
FROM rust:1.72.0-slim as builder

WORKDIR /usr/src

# Create blank project
RUN mkdir -p /usr/src/pei_server

RUN mkdir -p /usr/src/pei_server/common
RUN mkdir -p /usr/src/pei_server/common/src
RUN mkdir -p /usr/src/pei_server/inter_services_messages
RUN mkdir -p /usr/src/pei_server/inter_services_messages/src
RUN mkdir -p /usr/src/pei_server/gateway
RUN mkdir -p /usr/src/pei_server/gateway/src
RUN mkdir -p /usr/src/pei_server/annuaire
RUN mkdir -p /usr/src/pei_server/annuaire/src


# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/pei_server/
#COPY icc_ban_cert.pem icc_ban_key.pem /usr/src/pei_server/
#COPY 0000_csr-certbot.pem 0000_key-certbot.pem /usr/src/pei_server/

COPY common/Cargo.toml /usr/src/pei_server/common/
COPY common/src /usr/src/pei_server/common/src/

COPY inter_services_messages/Cargo.toml /usr/src/pei_server/inter_services_messages/
COPY inter_services_messages/src /usr/src/pei_server/inter_services_messages/src/

COPY gateway/Cargo.toml /usr/src/pei_server/gateway/
COPY gateway/src /usr/src/pei_server/gateway/src/

COPY annuaire/Cargo.toml /usr/src/pei_server/annuaire/
COPY annuaire/src /usr/src/pei_server/annuaire/src/




# Set the working directory
WORKDIR /usr/src/pei_server

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev

# This is the actual application build.
RUN cargo build -p gateway --target x86_64-unknown-linux-musl --release
RUN cargo build -p annuaire --target x86_64-unknown-linux-musl --release



################
##### Runtime
FROM alpine:latest AS gateway 
# Copy application binary from builder image
COPY --from=builder /usr/src/pei_server/target/x86_64-unknown-linux-musl/release/gateway /usr/local/bin
#COPY --from=builder /usr/src/pei_server/icc_ban_cert.pem /usr/local/bin
#COPY --from=builder /usr/src/pei_server/icc_ban_key.pem /usr/local/bin
#COPY --from=builder /usr/src/pei_server/0000_csr-certbot.pem /usr/local/bin
#COPY --from=builder /usr/src/pei_server/0000_key-certbot.pem /usr/local/bin
# Run the application
CMD ["/usr/local/bin/gateway"]

################
##### Runtime
FROM alpine:latest AS annuaire 
# Copy application binary from builder image
COPY --from=builder /usr/src/pei_server/target/x86_64-unknown-linux-musl/release/annuaire /usr/local/bin
# Run the application
CMD ["/usr/local/bin/annuaire"]