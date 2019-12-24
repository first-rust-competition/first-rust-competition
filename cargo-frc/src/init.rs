// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::util::{handle_subprocess_exit, str_map};
use clap::ArgMatches;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub fn init_command(_matches: &ArgMatches) -> Result<(), String> {
    cargo_init()?;

    trace!("Editing Cargo.toml");

    let mut cargo_toml = OpenOptions::new()
        .append(true)
        .open("./Cargo.toml")
        .map_err(str_map("Could not open Cargo.toml"))?;

    writeln!(cargo_toml, "wpilib = \"{}\"", crate_version!())
        .map_err(str_map("Could not write to Cargo.toml"))?;

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
