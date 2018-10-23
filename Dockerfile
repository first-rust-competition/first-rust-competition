# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# A container with all the build dependencies:
# * Rust nightly
# * arm-frc-linux-gnueabi-gcc > 5.0
# * libclang / clang / llvm
# * JDK
# * Python 2.7
#
# Check the apt-get commands for the canonical list

# Begin with rust-nightly image but based on ubuntu xenial
FROM ubuntu:xenial

# install dev utils
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    make \
    git \
    default-jdk \
    llvm-5.0-dev \
    libclang-5.0-dev \
    clang-5.0 \
    python \
    ;

# install frc arm compiler
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends software-properties-common; \
    apt-add-repository ppa:wpilib/toolchain; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    frc-toolchain \
    ;

# begin rust nightly
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    gcc \
    libc6-dev \
    wget \
    ; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    \
    apt-get remove -y --auto-remove \
    wget \
    ; \
    rm -rf /var/lib/apt/lists/*;
# end rust nightly

# add arm target and clippy
RUN set -eux; \
    rustup target add arm-unknown-linux-gnueabi; \
    rustup component add clippy-preview; \
    cargo clippy --version; \
    rustup component add rustfmt-preview; \
    cargo fmt --version;

# configure the linker
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER arm-frc-linux-gnueabi-gcc

COPY . ./first-rust-competition
