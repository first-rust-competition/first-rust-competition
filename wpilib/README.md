# WPIlib

[![Crates.io](https://img.shields.io/crates/v/wpilib.svg)](https://crates.io/crates/wpilib/)
[![Docs.rs](https://docs.rs/wpilib/badge.svg)](https://docs.rs/wpilib)

Program your FRC robot in rust.

## Getting started

This repository is designed to be compiled for a [RoboRIO](http://sine.ni.com/nips/cds/view/p/lang/en/nid/213308), the
processor used in the FIRST Robotics Competition. To cross compile for RoboRIO, you have to do a few things:

1. Install [Rustup](https://www.rustup.rs/) to help manage Rust toolchains.
2. Only the latest rust stable is guaranteed to work. Ensure you are up to date:
    ```bash
    rustup update stable
    ```
3. Install some variant of `arm-linux-gnueabi-gcc`. For example, the official FRC toolchain
    (`arm-frc-linux-gnueabi-gcc`) is available for various platforms [here](http://first.wpi.edu/FRC/roborio/toolchains/), or you
    can install a generic toolchain with your package manager of choice (`sudo apt-get install gcc-arm-linux-gnueabi` on
    Ubuntu).
4. Edit your `~/.cargo/config` file with the following information:
    ```toml
    [target.arm-unknown-linux-gnueabi]
    linker = "<path-to-arm-linux-gnueabi-gcc>"
    ```
    Mine is at `/usr/bin/arm-frc-linux-gnueabi-gcc` on Ubuntu.
    You can also use the name of the command, provided it's in the PATH.
5. Run `rustup target add arm-unknown-linux-gnueabi` to install the Rust stdlib for ARM-based Linux.
6. Add `wpilib = ...` to `[dependencies]` in `Cargo.toml`.
7. Do deploy code, check out [cargo-frc](https://crates.io/crates/cargo-frc)

## Building for Development

Setup:

1. Follow the [Getting Started](#getting-started) section.
2. Verify you satisfy the [WPILib build requirements](https://github.com/wpilibsuite/allwpilib#building-wpilib).
3. Either install `arm-frc-linux-gnueabi-*` from the official FRC toolchain, or acquire a different arm compiler and `export CXX_FRC="/path/to or name of arm C++ compiler"`. This is necessary to load compiler headers.
4. Run `make all`. This will likely take a minute or two. The process will:
    1. Init and update the WPILib submodule
    2. Build the HAL and WPILibC shared libraries to link against.
    3. Generate the rust-bindings and build the library.

After the initial `make all`, use `cargo` (with two caveats, see below) to build as normal. If the WPILib submodule updates, run `make all` again.
Pull-requests to make the build process more cross platform are welcome. If you want to contribute but can't get the build working, you can use the CI docker image by invoking the `.ci/pull-request.sh` script.

This project includes a build script that generates bindings on top of WPIlib, handles linking, *and exposes its shared libs with a symlink for cargo-frc to consume*. Because of this, the script is by default configured only to run when it needs to update the
symlink (another version of this crate has changed it). During development, to force the script to run use
`cargo build --features dev`.

Also note that using `cargo build` in the workspace root will always fail, because `wpilib` can only be built successfully on arm, whereas `cargo-frc` needs to be native. In the future, building for x86 may enable WPILib simulation.

## Roadmap

- [x] Make the official HAL headers work with rust-bindgen by making them C-compatible.
- [x] Automatically generate new bindings from the HAL headers for future proofing.
- [x] Test generated HAL bindings on a roboRIO, and adjust the headers/bindings.
- [ ] PR the new C-compatible headers to the official WPILib, and freeze the rust bindings.
- [x] Write abstractions over the HAL.
  - [x] A way to run code when a DS packet is recieved.
  - [x] Structs for things like solenoids / analog in / etc.
  - [x] Quadrature Encoders
  - [ ] etc.
- [x] Integrate with a build system to make bootstrapping a new project easy and deploying to the RIO simple. ~~Probably a fork of GradleRIO, because it seems like all build tools run on the JVM.~~ Work has begun on `cargo-frc`, the third-party cargo subcommand for this project.
- [ ] ~~Look into FFI bindings and a abstractions for [CTRE Pheonix](https://github.com/CrossTheRoadElec/Phoenix-frc-lib)
    and the [NavX](https://github.com/kauailabs/navxmxp). Both of these libraries will play very nicely with rust-bindgen's C++ support. Neither is too heavy in inheritance, neither uses templates, and neither throws exceptions. However, the question of how each of them interacts with NI's dynamic libs is yet to be seen. Getting them to behave at link-time and run-time might be hard.~~ Check out [CTRE-rs](https://github.com/auscompgeek/ctre-rs) for a CAN interface to the Talon and Victor SPX. At some point, the NavX serial protocol (for MXP and USB) will be re-implemented on top of our own serial port.
- [ ] Use rust in the 2019 FRC season

## License

This library is distributed under the terms of both the
MIT license and the Apache License (Version 2.0). By contributing, you agree
to license your contribution under these terms.

See [LICENSE-APACHE](../LICENSE-APACHE), [LICENSE-MIT](../LICENSE-MIT), for details.

## Credits

While getting the HAL to work, I got lots of help from looking at KyleStach's [rust-wpilib](https://github.com/robotrs/rust-wpilib).
