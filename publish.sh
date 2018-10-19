#!/usr/bin/env sh

# Copyright 2018 First Rust Competition Developers.
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

# Script for deploying to crates.io
# Called by travis ci

set -e

# source: https://github.com/semver/semver/issues/232, with added "v"
TAG_REGEX="^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(-(0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(\.(0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*)?(\+[0-9a-zA-Z-]+(\.[0-9a-zA-Z-]+)*)?\$"

if [ -z "$TRAVIS_TAG" ]; then
    echo "Skipping publish because \$TRAVIS_TAG is empty."
    exit
fi

if ! echo "$TRAVIS_TAG" | grep -Eq "$TAG_REGEX"; then
    echo "WARNING! Found \$TRAVIS_TAG with value $TRAVIS_TAG, but it isn't a valid version name. Skipping Publish."
    exit
fi

verify_version() {
    cd $1
    echo "Verifying version for $1"
    if [ ! "v$(cargo pkgid | cut -d"#" -f2)" = "$TRAVIS_TAG" ]; then
        echo "ERROR! \$TRAVIS_TAG '$TRAVIS_TAG' != $1 Cargo.toml#version 'v$(cargo pkgid | cut -d"#" -f2)'. Aborting."
        exit 1
    fi
    cd ..
}

publish () {
    cd $1
    echo "Attempting to publish $1"
    cargo package
    echo "Package successful, publishing $1..."
    cargo publish --token $CRATESIO_TOKEN
    cd ..
}

publish_dirty() {
    cd $1
    echo "Attempting to publish $1"
    cargo package --allow-dirty
    echo "Package successful, publishing $1..."
    cargo publish --allow-dirty --token $CRATESIO_TOKEN
    cd ..
}

verify_version wpilib
verify_version wpilib-sys
verify_version cargo-frc

publish_dirty wpilib-sys
publish wpilib
publish cargo-frc
