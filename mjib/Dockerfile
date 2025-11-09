################
##### Builder
FROM rust:bookworm AS builder

WORKDIR /usr/src/mjib

# Copy source and config
COPY Cargo.toml Cargo.lock ./
COPY .cargo/ .cargo/
COPY src/ src/

# Install required libraries to build
RUN apt-get update && apt-get install -y libpq-dev libpq5

# Build the Rust project in release mode
RUN cargo build --release


################
##### Runtime
FROM debian:bookworm-slim AS mjib

# Install runtime dependencies (needed for mjib binary)
RUN apt-get update && apt-get install -y --no-install-recommends libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Copy application binary from builder
COPY --from=builder /usr/src/mjib/target/release/mjib /usr/local/bin/mjib

# Run the application
CMD ["/usr/local/bin/mjib"]