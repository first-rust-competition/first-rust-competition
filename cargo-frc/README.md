# Cargo FRC

[![Crates.io](https://img.shields.io/crates/v/cargo-frc.svg)](https://crates.io/crates/cargo-frc/)

A cargo subcommand for deploying your rust code.

## Installation

In this package, run `cargo install`, assuming `cargo`'s bin is in your `PATH`.

## Usage

Add a config section like

```toml
[package.metadata.frc]
team-number = 114
rio-address = "10.1.14.2"
target-dir = "target"
executable-name = "digital_out"
```

to your `Cargo.toml`.

* `team-number` - used to deduce the IP of the roborio if an override is not provided. Currently, this will try mDNS, USB, and the conventional static IP.
* `rio-address` - set the one and only IP `cargo frc` will try to deploy to
* `target-dir` - set the relative or absolute path of `cargo`'s `target` directory. This is usually just `"./target"`, but will vary for cargo workspaces.
* `executable-name` - set the name of the binary to deploy to the RIO. If not specified, the `package.name` key is used instead.

You can then run `cargo frc deploy` in your robot code package.

## Roadmap

- [x] MVP to streamline `wpilib` testing and development
- [ ] Test windows support, decide whether to add it
- [x] Deploy shared libraries
- [ ] Automatic debugging support with `gdb` and its server

## License

The contents of this repository are distributed under the terms of both the
MIT license and the Apache License (Version 2.0). By contributing, you agree
to license your contribution under these terms.

See [LICENSE-APACHE](../LICENSE-APACHE), [LICENSE-MIT](../LICENSE-MIT), for details.
