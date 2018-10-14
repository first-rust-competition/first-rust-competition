# FIRST Rust Competition

[![Build Status](https://travis-ci.org/Lytigas/first-rust-competition.svg?branch=master)](https://travis-ci.org/Lytigas/first-rust-competition)
[![Crates.io](https://img.shields.io/crates/v/wpilib.svg)](https://crates.io/crates/wpilib/)
[![Docs.rs](https://docs.rs/wpilib/badge.svg)](https://docs.rs/wpilib)

A monorepo for `wpilib` for programming FRC robots and `cargo-frc` for deploying said code. Currently a pre-alpha WIP.

## Getting started

Parts of this repository are designed to be compiled for a [RoboRIO](http://sine.ni.com/nips/cds/view/p/lang/en/nid/213308), the
processor used in the FIRST Robotics Competition. To cross compile for RoboRIO, you have to do a few things.
Detailed instructions can be found in [WPILib's README](wpilib/README.md).

Examples can be found in [wpilib-examples](wpilib-examples).

To deploy code you write using `wpilib`, use [cargo-frc](cargo-frc).

## Other Rust Projects

If you want to go further with Rust development for FRC, check out these other community projects:

* [ctre-rs](https://github.com/auscompgeek/ctre-rs) for functionality found in CTRE Phoenix.
* [nt-rs](https://gitlab.com/Redrield/nt-rs) for using NetworkTables.
* [navx-rs](https://github.com/Eaglestrike/navX-rs) for interfacing with Kauai Labs's gyroscope.

## Building

Verify you can build `wpilib`, then run `make all`. `cargo-frc` should build out of the box, but you should `cargo install` it
to [use it properly](cargo-frc/README.md).

For a full list of requirements, see the [Dockerfile](Dockerfile) used for Travis CI.

## License

The contents of this repo are released under the GPLv3.
By contributing, you license your contribution under the GPLv3.
You also agree to have your contribution included in a future
version of this library licensed under a future version of the GPL
as released by the Free Software Foundation.
