################
##### Builder (check latest version on rust and put it here)
FROM rust:1.69.0-slim as builder

WORKDIR /usr/src

# Create blank project
#RUN USER=root cargo new pei_server
RUN mkdir -p /usr/src/pei_server

RUN mkdir -p /usr/src/pei_server/icc_common
RUN mkdir -p /usr/src/pei_server/icc_common/src
RUN mkdir -p /usr/src/pei_server/inter_services_messages
RUN mkdir -p /usr/src/pei_server/inter_services_messages/src
RUN mkdir -p /usr/src/pei_server/icc_gateway
RUN mkdir -p /usr/src/pei_server/icc_gateway/src
RUN mkdir -p /usr/src/pei_server/icc_annuaire
RUN mkdir -p /usr/src/pei_server/icc_annuaire/src


# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/pei_server/
COPY icc_ban_cert.pem icc_ban_key.pem /usr/src/pei_server/
COPY 0000_csr-certbot.pem 0000_key-certbot.pem /usr/src/pei_server/

COPY icc_common/Cargo.toml /usr/src/pei_server/icc_common/
COPY icc_common/src /usr/src/pei_server/icc_common/src/

COPY inter_services_messages/Cargo.toml /usr/src/pei_server/inter_services_messages/
COPY inter_services_messages/src /usr/src/pei_server/inter_services_messages/src/

COPY icc_gateway/Cargo.toml /usr/src/pei_server/icc_gateway/
COPY icc_gateway/src /usr/src/pei_server/icc_gateway/src/

COPY icc_annuaire/Cargo.toml /usr/src/pei_server/icc_annuaire/
COPY icc_annuaire/src /usr/src/pei_server/icc_annuaire/src/




# Set the working directory
WORKDIR /usr/src/pei_server

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev

# This is the actual application build.
RUN cargo build -p icc_gateway --target x86_64-unknown-linux-musl --release
RUN cargo build -p icc_annuaire --target x86_64-unknown-linux-musl --release



################
##### Runtime
FROM alpine:latest AS gateway_runtime 
# Copy application binary from builder image
COPY --from=builder /usr/src/pei_server/target/x86_64-unknown-linux-musl/release/icc_gateway /usr/local/bin
COPY --from=builder /usr/src/pei_server/icc_ban_cert.pem /usr/local/bin
COPY --from=builder /usr/src/pei_server/icc_ban_key.pem /usr/local/bin
COPY --from=builder /usr/src/pei_server/0000_csr-certbot.pem /usr/local/bin
COPY --from=builder /usr/src/pei_server/0000_key-certbot.pem /usr/local/bin
# Run the application
CMD ["/usr/local/bin/icc_gateway"]

################
##### Runtime
FROM alpine:latest AS annuaire_runtime 
# Copy application binary from builder image
COPY --from=builder /usr/src/pei_server/target/x86_64-unknown-linux-musl/release/icc_annuaire /usr/local/bin
# Run the application
CMD ["/usr/local/bin/icc_annuaire"]