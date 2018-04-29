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
    Mine is at `/usr/bin/arm-frc-linux-gnueabi-gcc` on Ubuntu.

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

## Roadmap
- [x] Make the official HAL headers work with rust-bindgen by making them C-compatible.
- [x] Automatically generate new bindings from the HAL headers for future proofing.
- [ ] Test generated HAL bindings on a roboRIO, and adjust the headers/bindings.
- [ ] PR the new C-compatible headers to the official WPILib, and freeze the rust bindings.
- [ ] Write abstractions over the HAL.
    - [ ] A way to run code when a DS packet is recieved.
    - [ ] Structs for things like solenoids / analog in / etc.
    - [ ] etc.
- [ ] Look into FFI bindings and a abstractions for [CTRE Pheonix](https://github.com/CrossTheRoadElec/Phoenix-frc-lib) and the [NavX](https://github.com/kauailabs/navxmxp).
- [ ] *Re-write* Team 114's 2018 codebase *in rust.*
- [ ] Test robustness at an offseason competition.

## Credits
While writing this, I got lots of help from looking at KyleStach's [rust-wpilib](https://github.com/robotrs/rust-wpilib).
