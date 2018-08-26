# WPIlib

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
5. Rust-bindgen requires rust nightly. You can configure this with the following:
    ```bash
    rustup toolchain install nightly
    rustup default nightly
    ```
6. Install the [requirements of `bindgen`](https://rust-lang-nursery.github.io/rust-bindgen/requirements.html), with Clang 3.9 or higher for better C++ support.
7. Add `wpilib = ...` to `[dependencies]` in `Cargo.toml`.

## Building for Development

Setup:

1. Follow the [Getting Started](#getting-started) section.
2. Verify you satisfy the [WPILib build requirements](https://github.com/wpilibsuite/allwpilib#building-wpilib).
3. Run `make all`. This will likely take a minute or two. The process will:
    1. Init and update the WPILib submodule
    2. Build the HAL and WPILibC shared libraries to link against.
    3. Generate the rust-bindings and build the library.

After the initial `make all`, use `cargo` (with two caveats, see below) to build as normal. If the WPILib submodule updates, run `make all` again.
Pull-requests to make the build process more cross platform are welcome.

This project includes a build script that generates bindings on top of WPIlib, handles linking, *and exposes its shared libs with a symlink for cargo-frc to consume*. Because of this, the script is by default configured only to run when it needs to update the
symlink (another version of this crate has changed it). During development, to force the script to run use
`cargo build --features dev`.

Also note that using `cargo build` in the workspace root will always fail, because `wpilib` can only be built successfully on arm, whereas `cargo-frc` needs to be native. In the future, building for x86 may enable WPILib simulation.

## Roadmap

- [x] Make the official HAL headers work with rust-bindgen by making them C-compatible.
- [x] Automatically generate new bindings from the HAL headers for future proofing.
- [x] Test generated HAL bindings on a roboRIO, and adjust the headers/bindings.
- [ ] PR the new C-compatible headers to the official WPILib, and freeze the rust bindings.
- [ ] Write abstractions over the HAL.
  - [x] A way to run code when a DS packet is recieved.
  - [ ] Structs for things like solenoids / analog in / etc.
  - [ ] etc.
- [ ] Integrate with a build system to make bootstrapping a new project easy and deploying to the RIO simple. ~~Probably a fork of GradleRIO, because it seems like all build tools run on the JVM.~~ Work has begun on `cargo-frc`, the third-party cargo subcommand for this project.
- [ ] ~~Look into FFI bindings and a abstractions for [CTRE Pheonix](https://github.com/CrossTheRoadElec/Phoenix-frc-lib)
    and the [NavX](https://github.com/kauailabs/navxmxp). Both of these libraries will play very nicely with rust-bindgen's C++ support. Neither is too heavy in inheritance, neither uses templates, and neither throws exceptions. However, the question of how each of them interacts with NI's dynamic libs is yet to be seen. Getting them to behave at link-time and run-time might be hard.~~ Check out [CTRE-rs](https://github.com/auscompgeek/ctre-rs) for a CAN interface to the Talon and Victor SPX. At some point, the NavX serial protocol (for MXP and USB) will be re-implemented on top of our own serial port.
- [ ] *Re-write* Team 114's 2018 codebase *in rust.*
- [ ] Test robustness at an offseason competition.

## License

This library and its source code are released under the GPLv3.
By contributing, you license your contribution under the GPLv3.
You also agree to have your contribution included in a future
version of this library licensed under a future version of the GPL
as released by the Free Software Foundation.

## Credits

While getting the HAL to work, I got lots of help from looking at KyleStach's [rust-wpilib](https://github.com/robotrs/rust-wpilib).
