################
##### Builder (check latest version on rust and put it here)
FROM rust:1.69.0-slim as builder

WORKDIR /usr/src

# Create blank project
#RUN USER=root cargo new pei_server
RUN mkdir -p /usr/src/pei_server

RUN mkdir -p /usr/src/pei_server/icc_gateway
RUN mkdir -p /usr/src/pei_server/icc_gateway/src
RUN mkdir -p /usr/src/pei_server/icc_common
RUN mkdir -p /usr/src/pei_server/icc_common/src
RUN mkdir -p /usr/src/pei_server/icc_users
RUN mkdir -p /usr/src/pei_server/icc_users/src
RUN mkdir -p /usr/src/pei_server/inter_services_messages


# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/pei_server/

COPY icc_common/Cargo.toml /usr/src/pei_server/icc_common/
COPY icc_common/src /usr/src/pei_server/icc_common/src/

COPY inter_services_messages/Cargo.toml /usr/src/pei_server/inter_services_messages/
COPY inter_services_messages/src /usr/src/pei_server/inter_services_messages/src/

COPY icc_gateway/Cargo.toml icc_gateway/Cargo.lock /usr/src/pei_server/icc_gateway/
COPY icc_gateway/src icc_gateway/Cargo.lock /usr/src/pei_server/icc_gateway/src/

COPY icc_users/Cargo.toml icc_users/Cargo.lock /usr/src/pei_server/icc_users/
COPY icc_users/src icc_users/Cargo.lock /usr/src/pei_server/icc_users/src/


# Set the working directory
WORKDIR /usr/src/pei_server

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev

# This is the actual application build.
RUN cargo build -p icc_gateway --target x86_64-unknown-linux-musl --release
RUN cargo build -p icc_users --target x86_64-unknown-linux-musl --release



################
##### Runtime
FROM alpine:latest AS gateway_runtime 
# Copy application binary from builder image
COPY --from=builder /usr/src/pei_server/target/x86_64-unknown-linux-musl/release/icc_gateway /usr/local/bin
EXPOSE 4010
# Run the application
CMD ["/usr/local/bin/icc_gateway"]



################
##### Runtime
FROM alpine:latest AS users_runtime 
# Copy application binary from builder image
COPY --from=builder /usr/src/pei_server/target/x86_64-unknown-linux-musl/release/icc_users /usr/local/bin
EXPOSE 4011
# Run the application
CMD ["/usr/local/bin/icc_users"]