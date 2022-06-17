# Wpilib-rs

[![Crates.io](https://img.shields.io/crates/v/wpilib.svg)](https://crates.io/crates/wpilib/)
[![Docs.rs](https://docs.rs/wpilib/badge.svg)](https://docs.rs/wpilib)

A `wpilib` monorepo for programming FRC robots in Rust and `cargo-frc` for deploying said code.

## Getting started

Parts of this repository are designed to be compiled for a [RoboRIO](http://sine.ni.com/nips/cds/view/p/lang/en/nid/213308), the
processor used in the FIRST Robotics Competition.
To cross-compile your code and run Rust on your RoboRIO, follow the instructions in [WPILib's README](wpilib/README.md).

To deploy code you write using `wpilib`, use [cargo-frc](cargo-frc).

## Other Rust projects

If you want to go further with Rust development for FRC, check out these other community projects:

- [ctre-rs](https://github.com/auscompgeek/ctre-rs) for functionality found in CTRE Phoenix.
- [nt-rs](https://gitlab.com/Redrield/nt-rs) for using NetworkTables.
- [navx-rs](https://github.com/Eaglestrike/navX-rs) for interfacing with Kauai Labs's gyroscope.

## Building locally

In order to build locally, verify that you have the dependencies for building [`wpilib`](https://github.com/wpilibsuite/allwpilib#requirements). After cloning this repository, run `cargo x init` to clone, build, and copy `allwpilib`.

## License

The contents of this repository are distributed under the terms of both the
MIT license and the Apache License (Version 2.0). By contributing, you agree
to license your contribution under these terms.

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), for details.
