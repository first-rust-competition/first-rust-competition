# A container with all the build dependencies:
# * Rust nightly
# * g++ > 5.0
# * arm-frc-linux-gnueabi-gcc > 5.0

# Begin with rust-nightly image but based on ubuntu xenial
FROM ubuntu:xenial

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
# end rust nightly file

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
    gcc-multilib \
    ;

# install frc arm compiler and g++
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends software-properties-common; \
    apt-add-repository ppa:wpilib/toolchain; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
    frc-toolchain \
    g++ \
    ;

# add arm target to rust
RUN rustup target add arm-unknown-linux-gnueabi

ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABI_LINKER arm-frc-linux-gnueabi-gcc

COPY . ./first-rust-competition


# TODO: rebase for proper build testing
