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

# Add debian stretch backports for OpenJDK 11 and Clang 5.0
RUN set -eux; \
    echo 'deb http://deb.debian.org/debian stretch-backports main' > /etc/apt/sources.list.d/backports.list

# install dev utils
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    make \
    git \
    openjdk-11-jdk-headless \
    llvm-5.0-dev \
    libclang-5.0-dev \
    clang-5.0 \
    python \
    ;

# workaround for java cacerts issue https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=894979
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends openjdk-8-jre-headless; \
    rm /etc/ssl/certs/java/cacerts; \
    update-ca-certificates --fresh; \
    apt-get purge -y openjdk-8-jre-headless

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
