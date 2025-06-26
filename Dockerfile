# Use the official Rust image as the base image (supports multi-platform)
FROM --platform=$TARGETPLATFORM rust:1.82-slim as builder

# Set the working directory
WORKDIR /usr/src/dbx

# Copy the manifests
COPY Cargo.toml ./
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

# Create a new stage with a minimal runtime image (supports multi-platform)
FROM --platform=$TARGETPLATFORM debian:bookworm-slim

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

# Copy the static files
COPY static/ ./static/

# Change ownership to the non-root user
RUN chown -R dbx:dbx /app

# Switch to the non-root user
USER dbx

# Expose the port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/redis/admin/ping || exit 1

# Default environment variables
ENV DATABASE_TYPE=redis
ENV HOST=0.0.0.0
ENV PORT=3000
ENV POOL_SIZE=10
ENV LOG_LEVEL=INFO

# Add labels for better image metadata
LABEL maintainer="DBX Team"
LABEL description="High-performance Redis API Gateway with HTTP and WebSocket interfaces"
LABEL version="0.1.5"
LABEL org.opencontainers.image.source="https://github.com/your-org/dbx"

# Run the binary
CMD ["./dbx-api"] 