# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# A container with all the build dependencies:
# * Rust stable
# * make
# * git
# * libclang / clang / llvm
# * OpenJDK 11
# * Python 2.7
# * arm-frc2019-linux-gnueabi-gcc/g++
#
# Check the apt-get commands for the canonical list

# Begin with rust-stable image
FROM rust:latest

# install dev utils
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    make \
    git \
    rename \
    openjdk-11-jdk-headless \
    llvm-7-dev \
    libclang-7-dev \
    clang-7 \
    python \
    ;

# add the frc2019 compiler
RUN set -eux; \
    wget -qO- https://github.com/wpilibsuite/toolchain-builder/releases/download/v2019-3/FRC-2019-Linux-Toolchain-6.3.0.tar.gz \
    | tar xvz -C /

ENV PATH /frc2019/roborio/bin/:$PATH

# add arm target and clippy/rustfmt
RUN set -eux; \
    rustup target add arm-unknown-linux-gnueabi; \
    rustup component add clippy-preview; \
    cargo clippy --version; \
    rustup component add rustfmt-preview; \
    cargo fmt --version

# configure the linker
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER arm-frc2019-linux-gnueabi-gcc

COPY . ./first-rust-competition
