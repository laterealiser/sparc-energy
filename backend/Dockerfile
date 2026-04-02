# Build stage
FROM rust:slim-bullseye AS builder

# Install build dependencies for PostgreSQL and OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential libpq-dev

WORKDIR /usr/src/app

# Create a dummy main.rs to build dependencies first (caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code and build in release mode
COPY . .
RUN touch src/main.rs
RUN cargo build --release

# Run stage
FROM debian:bullseye-slim

# Install runtime dependencies for PostgreSQL and SSL certificates
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates libpq5 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the compiled binary from the builder (Release build)
COPY --from=builder /usr/src/app/target/release/carbon-credit-backend /usr/local/bin/sparc-backend

# Set environment variables for the Render environment
ENV HOST=0.0.0.0
ENV RUST_LOG=info

# Expose the port (informative, Render handles networking via $PORT)
EXPOSE 10000

# Run the binary
CMD ["sparc-backend"]
