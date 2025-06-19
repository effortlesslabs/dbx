# Use the official Rust image as the base image
FROM rust:1.82-slim as builder

# Set the working directory
WORKDIR /usr/src/dbx

# Copy the manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY api/ ./api/

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached if dependencies don't change)
RUN cargo build --release

# Remove the dummy main.rs and copy the real source code
RUN rm src/main.rs
COPY api/src/ ./api/src/

# Build the application
RUN cargo build --release --bin dbx-api

# Create a new stage with a minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false dbx

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/dbx/target/release/dbx-api /app/dbx-api

# Copy the example environment file
COPY api/env.example /app/.env.example

# Change ownership to the non-root user
RUN chown -R dbx:dbx /app

# Switch to the non-root user
USER dbx

# Expose the port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Run the binary
CMD ["./dbx-api"] 