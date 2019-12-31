// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::util::{handle_subprocess_exit, str_map};
use clap::ArgMatches;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

const DIGITAL_OUT_ROBOT: &str = include_str!("../../wpilib-examples/digital_out.rs");

/// Command to initialize a robot project in the current directory.
pub fn init_command(_matches: &ArgMatches) -> Result<(), String> {
    cargo_init()?;

    configure_project(Path::new("./"))?;

    Ok(())
}

/// Command to create a new robot project with the given name in the current directory.
pub fn new_command(matches: &ArgMatches) -> Result<(), String> {
    let name = matches.value_of("NAME").unwrap();

    cargo_new(name)?;

    configure_project(Path::new(name))?;

    Ok(())
}

/// Configures the cargo project located at `path` as a robot project.
/// * Adds wpilib as a dependency in `cargo.toml`
/// * Places a simple example robot in `src/main.rs`
/// * Creates `.cargo/config` to set default build target
fn configure_project(path: &Path) -> Result<(), String> {
    trace!("Editing Cargo.toml");

    let mut cargo_toml = OpenOptions::new()
        .append(true)
        .open(path.join("Cargo.toml"))
        .map_err(str_map("Could not open Cargo.toml"))?;

    writeln!(cargo_toml, "wpilib = \"{}\"", crate_version!())
        .map_err(str_map("Could not write to Cargo.toml"))?;

    trace!("Editing src/main.rs");

    let mut main = OpenOptions::new()
        .write(true)
        .open(path.join("src/main.rs"))
        .map_err(str_map("Could not open src/main.rs"))?;

    write!(main, "{}", DIGITAL_OUT_ROBOT).map_err(str_map("Could not write to src/main.rs"))?;

    trace!("Editing .cargo/config");

    fs::create_dir(path.join(".cargo")).map_err(str_map("Could not create .cargo/"))?;
    let mut config = File::create(path.join(".cargo/config"))
        .map_err(str_map("Could not create .cargo/config"))?;

    write!(config, "[build]\ntarget = \"arm-unknown-linux-gnueabi\"")
        .map_err(str_map("Could not write to .cargo/config"))?;

    Ok(())
}
/// Initializes a new default cargo project in the current directory.
fn cargo_init() -> Result<(), String> {
    info!("Initializing cargo project");

    let exit_code = subprocess::Exec::cmd("cargo")
        .arg("init")
        .arg("--bin")
        .arg("--quiet")
        .join()
        .map_err(str_map("Failed to initialize cargo project"))?;

    handle_subprocess_exit("cargo init", exit_code)
}

/// Creates a new default cargo project and containing directory with th given `name`.
fn cargo_new(name: &str) -> Result<(), String> {
    info!("Creating new cargo project");

    let exit_code = subprocess::Exec::cmd("cargo")
        .arg("new")
        .arg(name)
        .arg("--bin")
        .arg("--quiet")
        .join()
        .map_err(str_map("Failed to initialize cargo project"))?;

    handle_subprocess_exit("cargo init", exit_code)
}
