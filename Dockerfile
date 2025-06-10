# syntax=docker/dockerfile:1.4
FROM python:3.13-slim AS builder
LABEL authors="thirantos"

RUN apt-get update && apt-get install -y --no-install-recommends \
  curl build-essential pkg-config git ca-certificates \
  libfontconfig1-dev libfreetype6-dev libgraphite2-dev \
  libharfbuzz-dev libicu-dev libpng-dev libssl-dev zlib1g-dev patchelf\
  && rm -rf /var/lib/apt/lists/*


# Install Rust using rustup and set it up in PATH
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH="${CARGO_HOME}/bin:${PATH}"

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain nightly



RUN pip install maturin

WORKDIR /build
COPY . .

ENV CXXFLAGS="-std=c++17"
ENV CXX="g++"

RUN maturin build --release --strip --manylinux

FROM scratch AS export-stage
COPY --from=builder /build/target/wheels /dist
