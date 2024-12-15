# Base image for Rust and C++
FROM rust:latest AS rust-env
RUN apt-get update && apt-get install -y build-essential cmake g++

WORKDIR /app/src

# Install system dependencies for C++ (e.g., Boost, CMake, etc.)
#RUN apt-get update && apt-get install -y \
#    build-essential \
#    cmake \
#    g++ \
#    libboost-all-dev  # Example for C++ dependencies

# Install Rust toolchain and necessary build tools
RUN rustup update
RUN rustup default stable
