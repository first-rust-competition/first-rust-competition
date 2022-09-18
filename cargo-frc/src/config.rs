// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::toolchain::Toolchain;
use crate::util::str_map;
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FrcConfig {
    pub team_number: Option<u64>,
    pub rio_address_override: Option<String>,
    pub target_dir: PathBuf,
    pub executable: PathBuf,
    pub toolchain_year: Option<Toolchain>,
}

pub fn get_config() -> Result<FrcConfig, String> {
    debug!("Reading from `cargo read-manifest`");
    let manifest: Value = serde_json::from_str(
        &subprocess::Exec::cmd("cargo")
            .arg("read-manifest")
            .capture()
            .map_err(str_map("Failed to capture cargo read-manifest"))?
            .stdout_str(),
    )
    .map_err(str_map("Failed to capture cargo read-manifest stdout"))?;
    let err = "FRC Config not found in package manifest.
        Config should be in [package.metadata.frc] in Cargo.toml";
    let mut target_dir = PathBuf::new();
    target_dir.push(match manifest["manifest_path"] {
        Value::String(ref x) => Ok(x),
        _ => Err("Could not read manifest_path from Cargo.toml."),
    }?);
    target_dir.pop(); // remove Cargo.toml from the path
    let frc = manifest
        .get("metadata")
        .ok_or(err)?
        .get("frc")
        .ok_or(err)?
        .as_object()
        .ok_or(err)?;
    // executable
    let target_dir_json = frc
        .get("target-dir")
        .ok_or("target-dir not specified in FRC Config.")?;
    let target_dir_string = match target_dir_json {
        Value::String(x) => Ok(x),
        _ => Err("target-dir must be a string"),
    }?;
    target_dir.push(target_dir_string);
    target_dir = target_dir
        .canonicalize()
        .map_err(str_map("Could not canonicalize target_dir path"))?;

    let executable_name = match frc.get("executable-name") {
        Some(Value::String(x)) => {
            info!("Using executable name {}", x);
            Ok(x.clone())
        }
        _ => {
            warn!("Executable name not specified, using package name instead.");
            match manifest["name"] {
                Value::String(ref x) => Ok(x.clone()),
                _ => Err("Could not find package name"),
            }
        }
    }?;
    let mut executable = PathBuf::new();
    executable.push(executable_name);

    // rio address
    let team_number = match frc.get("team-number") {
        Some(Value::Number(x)) => x.as_u64(),
        _ => {
            warn!("No team number found, or it is not an unsigned integer.");
            None
        }
    };
    let rio_address_override = match frc.get("rio-address") {
        Some(Value::String(x)) => Some(x),
        _ => {
            info!("No rio address override found, or it is not a string.");
            None
        }
    }
    .cloned();
    if team_number == None && rio_address_override == None {
        error!("Neither a team number or rio address was specified.");
    };

    debug!("year val: {:?}", frc.get("year"));
    // debug!("team-number val: {:?}", frc.get("team-number"));

    let toolchain_year = frc
        .get("year")
        .and_then(|v| v.as_u64())
        .map(|i| i.to_string())
        .and_then(|s| Toolchain::from_year(s.as_str()));

    let tmp = FrcConfig {
        team_number,
        rio_address_override,
        target_dir,
        executable,
        toolchain_year,
    };
    debug!("Using config: {:?}.", tmp);

    Ok(tmp)
}
