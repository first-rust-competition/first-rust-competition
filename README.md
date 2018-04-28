# FIRST Rust Competition

Rewrite it in rust. Not ready for any non-dev use.

## Getting started
This repository is designed to be compiled for a [RoboRIO](http://sine.ni.com/nips/cds/view/p/lang/en/nid/213308), the
processor used in the FIRST Robotics Competition. To cross compile for RoboRIO, you have to do a few things:
 1. Install [Rustup](https://www.rustup.rs/) to help manage Rust toolchains.
 2. Run `rustup target add arm-unknown-linux-gnueabi` to install the Rust stdlib for ARM-based Linux.
 3. Install some variant of `arm-linux-gnueabi-gcc`. For example, the official FRC toolchain
    (`arm-frc-linux-gnueabi-gcc`) is available [here](https://launchpad.net/~wpilib/+archive/ubuntu/toolchain), or you
    can install a generic toolchain with your package manager of choice (`sudo apt-get install gcc-arm-linux-gnueabi` on
    Ubuntu).
 4. Edit your `~/.cargo/config` file with the following information:

    ```toml
    [target.arm-unknown-linux-gnueabi]
    linker = "<path-to-arm-linux-gnueabi-gcc>"
    ```
    Mine is at `/usr/arm-frc-linux-gnueabi/`

## Building
1. Init and pull the WPILib submodule
2. Building WPILib. Verify you satisfy the [requirements](https://github.com/wpilibsuite/allwpilib#building-wpilib), then `cd HAL` and `make all`.
3. Generate rust-bindings and build the library. `cargo build`.

### Common Hitches

Rust-bindgen requires both rustc-nightly and rustfmt-nightly. You can configure this with the following:
```
rustup toolchain install nightly
rustup toolchain default nightly
cargo install rustfmt-nightly --force
```
`cargo build` should take care of building rust-bindgen for this repo only.

## TODO
1. update and merge this PR on a fork of wpilib so rust-bindgen can use C-headers
https://github.com/wpilibsuite/allwpilib/pull/535/commits/27b494baeb8dfbfc68a885644e37e089754e0e45
2. Verify the HAL works on a test RIO.
3. Write the lib.

## Credits
While writing this, I got lots of help from looking at KyleStach's [rust-wpilib](https://github.com/robotrs/rust-wpilib).
