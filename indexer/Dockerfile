# Use the official Rust image as a base
FROM rust:latest

# Install the necessary system dependencies
RUN apt-get update && apt-get install -y \
    libdw-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Set the command to run the application
CMD ["cargo", "run", "--release", "--", "-c", "config.yaml"]
