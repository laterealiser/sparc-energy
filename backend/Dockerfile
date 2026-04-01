# Build stage
FROM rust:1.80-slim-bullseye AS builder

# Install build dependencies for SQLite and OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev libsqlite3-dev build-essential

WORKDIR /usr/src/app

# Create a dummy main.rs to build dependencies first (caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm -rf src

# Copy the actual source code and build
COPY . .
RUN touch src/main.rs
RUN cargo build

# Run stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates sqlite3 libsqlite3-0 && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the compiled binary from the builder
COPY --from=builder /usr/src/app/target/debug/carbon-credit-backend /usr/local/bin/sparc-backend

# Set environment variables for the Render environment
ENV HOST=0.0.0.0
# Render injects the PORT environment variable automatically (default 10000)
ENV DATABASE_URL=sqlite://./sparc_energy.db

# Expose the port (informative, Render handles networking via $PORT)
EXPOSE 10000 8080

# Run the binary
CMD ["sparc-backend"]
