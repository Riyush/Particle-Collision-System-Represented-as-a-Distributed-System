# Base image for Rust and C++
FROM rust:latest AS rust-env

WORKDIR /app

COPY worker_node /app

# Install Rust toolchain and necessary build tools
RUN rustup update
#RUN rustup default stable

CMD ["cargo", "run", "--bin", "worker_node"]