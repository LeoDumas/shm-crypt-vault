FROM mcr.microsoft.com/devcontainers/base:ubuntu-22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    gdb \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    libprotobuf-dev \
    git \
    ca-certificates \
    && apt-get clean

USER vscode
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN rustup component add rust-analyzer rust-src clippy