# Base image for Rust and C++
FROM rust:latest AS rust-env


# Set environment variables to avoid prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# ensure clang is default compiler -might have to be changed to paths
ENV CC=clang CXX=clang++

WORKDIR /app/src

# Install system dependencies for C++ (e.g., Boost, CMake, etc.)
# Update package lists and install basic development tools
RUN apt-get update && apt-get install -y \
    software-properties-common \
    build-essential \
    curl \
    wget \
    ninja-build \
    clang-19 \
    lld-19 \
    clang-tidy-19 \
    && apt-get clean

# Install the latest CMake (3.28+) directly from Kitware
RUN wget https://github.com/Kitware/CMake/releases/download/v3.28.0/cmake-3.28.0-linux-x86_64.tar.gz && \
    tar -xvzf cmake-3.28.0-linux-x86_64.tar.gz && \
    cp -r cmake-3.28.0-linux-x86_64/bin/* /usr/local/bin/ && \
    cp -r cmake-3.28.0-linux-x86_64/share/* /usr/local/share/ && \
    rm -rf cmake-3.28.0-linux-x86_64.tar.gz

# Symlink clang and clang++ to clang-19 and clang++-19
RUN ln -sf /usr/bin/clang-19 /usr/bin/clang && \
    ln -sf /usr/bin/clang++-19 /usr/bin/clang++

RUN update-alternatives --install /usr/bin/cc cc /usr/bin/clang-19 100
RUN update-alternatives --install /usr/bin/c++ c++ /usr/bin/clang++-19 100

#Clean up and reduce image size
RUN apt-get clean && rm -rf /var/lib/apt/lists/* llvm.sh cmake-3.28.0-linux-x86_64.tar.gz

# Install Rust toolchain and necessary build tools
RUN rustup update
RUN rustup default stable
