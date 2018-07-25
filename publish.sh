#!/usr/bin/env sh

# Script for deploying to crates.io
# Called by travis ci


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

set -e
cd wpilib
echo "Attempting to publish wpilib."
if [ ! "v$(cargo pkgid | cut -d"#" -f2)" = "$TRAVIS_TAG" ]; then
    echo "ERROR! \$TRAVIS_TAG '$TRAVIS_TAG' != Cargo.toml#version 'v$(cargo pkgid | cut -d"#" -f2)'. Skipping Publish."
    exit 1
fi
cargo package --allow-dirty
echo "Package successful, publishing..."
cargo publish --allow-dirty --token $CRATESIO_TOKEN
cd ..

cd cargo-frc
echo "Attempting to publish cargo-frc."
if [ ! "v$(cargo pkgid | cut -d"#" -f2)" = "$TRAVIS_TAG" ]; then
    echo "ERROR! \$TRAVIS_TAG '$TRAVIS_TAG' != Cargo.toml#version 'v$(cargo pkgid | cut -d"#" -f2)'. Skipping Publish."
    exit 1
fi
cargo package
echo "Package successful, publishing..."
cargo publish --token $CRATESIO_TOKEN
cd ..
