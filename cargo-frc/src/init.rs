// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::util::{handle_subprocess_exit, str_map};
use clap::ArgMatches;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path};

const DIGITAL_OUT_ROBOT: &'static str = include_str!("../../wpilib-examples/digital_out.rs");

pub fn init_command(_matches: &ArgMatches) -> Result<(), String> {
    cargo_init()?;

    configure_project(Path::new("./"))?;

    Ok(())
}

pub fn new_command(matches: &ArgMatches) -> Result<(), String> {
    let name = matches.value_of("NAME").unwrap();

    cargo_new(name)?;

    configure_project(Path::new(name))?;

    Ok(())
}

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

    Ok(())
}

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

fn cargo_new(name: &str) -> Result<(), String> {
    info!("Initializing cargo project");

    let exit_code = subprocess::Exec::cmd("cargo")
        .arg("new")
        .arg(name)
        .arg("--bin")
        .arg("--quiet")
        .join()
        .map_err(str_map("Failed to initialize cargo project"))?;

    handle_subprocess_exit("cargo init", exit_code)
}