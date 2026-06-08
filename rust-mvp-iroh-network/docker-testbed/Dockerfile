# 1. Compilation

# Rust image: https://hub.docker.com/_/rust
# Stable version of Rust
# slim a lightweight variant of the official Rust image, sufficient for the testbed
FROM rust:1.96.0-slim as builder

# WORKDIR create the /app folder and changes the working directory to it for all commands
WORKDIR /app

# Copy all project file to /app inside the container
COPY . .

# --release Build with optimizations
# https://doc.rust-lang.org/cargo/commands/cargo-build.html
RUN cargo build --release

# 2. Final image

# References for the image: https://kerkour.com/rust-docker-small-secure-images
FROM debian::bookworm-slim AS runtime