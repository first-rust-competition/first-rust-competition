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
Setup:
1. Follow the [Getting Started](#getting-started) section.\
2. Rust-bindgen requires both rustc-nightly and rustfmt-nightly. You can configure this with the following:
    ```
    rustup toolchain install nightly
    rustup toolchain default nightly
    cargo install rustfmt-nightly --force
    ```
    The rest of the build process will take care of compiling rust-bindgen.
3. Verify you satisfy the [WPILib build requirements](https://github.com/wpilibsuite/allwpilib#building-wpilib).

Run `make all`. This will likely take a minute or two.\
The process will
1. Init and update the WPILib submodule
2. Build the HAL and WPILibC shared libraries to link against.
3. Generate the rust-bindings and build the library.

After the initial `make all`, use `cargo` to build as normal. If the WPILib submodule updates, run `make all` again.
Pull-requests to make the build process more cross platform are welcome.

## Roadmap
- [x] Make the official HAL headers work with rust-bindgen by making them C-compatible.
- [x] Automatically generate new bindings from the HAL headers for future proofing.
- [ ] Test generated HAL bindings on a roboRIO, and adjust the headers/bindings.
- [ ] PR the new C-compatible headers to the official WPILib, and freeze the rust bindings.
- [ ] Write abstractions over the HAL.
    - [ ] A way to run code when a DS packet is recieved.
    - [ ] Structs for things like solenoids / analog in / etc.
    - [ ] etc.
- [ ] Integrate with a build system to make bootstrapping a new project easy and deploying to the RIO simple. Probably a fork of GradleRIO, because it seems like all build tools run on the JVM.
- [ ] Look into FFI bindings and a abstractions for [CTRE Pheonix](https://github.com/CrossTheRoadElec/Phoenix-frc-lib)
    and the [NavX](https://github.com/kauailabs/navxmxp).
- [ ] *Re-write* Team 114's 2018 codebase *in rust.*
- [ ] Test robustness at an offseason competition.

## License

This library is released under the GPLv3.
By contributing, you license your contribution under the GPLv3.
You also agree to have your contribution included in a future
version of this library licensed under a future version of the GPL
as released by the Free Software Foundation.

## Credits
While getting the HAL to work, I got lots of help from looking at KyleStach's [rust-wpilib](https://github.com/robotrs/rust-wpilib).
