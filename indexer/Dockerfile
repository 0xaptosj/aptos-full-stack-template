# Use the official Rust image to build the binary
FROM rust:latest AS builder
# FROM gcr.io/gcp-runtimes/rust:1.75 AS builder

# Install the necessary system dependencies
RUN apt-get update && apt-get install -y \
    libdw-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./
COPY .cargo .cargo

# Copy the actual source code
COPY src src

# Build the project
RUN cargo build --release

# Use a minimal base image to run the binary
# FROM debian:bullseye-slim
# FROM ubuntu:latest
# FROM gcr.io/distroless/base-debian12
FROM debian:12

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libdw1 \
    ca-certificates \
    openssl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the compiled binary from the builder image
COPY --from=builder /usr/src/app/target/release/indexer .

# Copy the config file
COPY config.yaml .

# Expose the port your application is using
EXPOSE 8080

# Set the command to run the application
CMD ["./indexer", "-c", "config.yaml"]
