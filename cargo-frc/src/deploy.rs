// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::config::FrcConfig;
use crate::util::*;
use clap::ArgMatches;
use ref_slice::*;
use std::env;
use std::ffi::OsStr;
use std::fmt;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use subprocess::ExitStatus;

pub fn deploy_command(matches: &ArgMatches<'_>, config: &FrcConfig) -> Result<(), String> {
    cargo_build(matches, config)?;

    let addresses = if let Some(addr) = config.rio_address_override.clone() {
        vec![addr]
    } else {
        make_addresses(
            config
                .team_number
                .ok_or("No RIO address or team number specified")?,
        )
    };
    let mut executable_path = config.target_dir.clone();
    executable_path.push(DEPLOY_TARGET_TRIPLE);
    if matches.is_present("release") {
        executable_path.push("release");
    } else {
        executable_path.push("debug");
    }
    executable_path.push(&config.executable);
    info!("Attempting to deploy executable {:?}", executable_path);

    for addr in &addresses {
        info!("Searching for rio at {}", addr);
        let canonical = &format!("admin@{}", addr);
        if test_ssh_address(canonical)? {
            do_deploy(canonical, &executable_path)?;
            return Ok(());
        }
    }
    Err("No tested address responded to ssh".to_string())
}

fn make_addresses(team_number: u64) -> Vec<String> {
    vec![
        format!("roborio-{}-FRC.local", team_number),
        format!("10.{}.{}.2", team_number / 100, team_number % 100),
        "172.22.11.2".to_string(),

        // The following are addresses that exist in uncommon development environments
        // Some teams run a shop wifi network that acts as a practice field. 
        // This setup involves the following mDNS entries
        format!("roborio-{}-FRC", team_number),
        format!("roborio-{}-FRC.lan", team_number),
        format!("roborio-{}-FRC.frc-field.local", team_number),
    ]
}

fn test_ssh_address(address: &str) -> Result<bool, String> {
    debug!("ssh -oBatchMode=yes {} \"exit\"", address);
    let mut process = subprocess::Exec::cmd("ssh")
        .arg("-oBatchMode=yes")
        .arg("-oStrictHostKeyChecking=no")
        .arg(address)
        .arg("\"exit\"")
        .popen()
        .map_err(str_map("'ssh' subprocess failed"))?;
    let ret = match process
        .wait_timeout(Duration::from_secs(2))
        .map_err(str_map("'ssh' subprocess failed to wait"))?
    {
        Some(ExitStatus::Exited(0)) => Ok(true),
        _ => Ok(false),
    };
    process.kill().map_err(str_map(
        "'ssh' subprocess timed out and could not be killed",
    ))?;
    ret
}

const DEPLOY_SCRIPT_CANONICAL_PATH: &str = "/home/lvuser/cargo-frc-script.sh";
const EXECUTABLE_TEMPORARY_PATH: &str = "/home/lvuser/rust-program-temp";

fn do_deploy(rio_address: &str, executable_path: &Path) -> Result<(), String> {
    let executable_path = executable_path
        .canonicalize()
        .map_err(str_map("Could not canonicalize executable path"))?;
    let mut script = tempfile::NamedTempFile::new()
        .map_err(str_map("Could not create temporary script file"))?;
    let executable_name = executable_path
        .file_name()
        .ok_or("executable_path does not point to a file")?
        .to_str()
        .ok_or("executable path is not valid Unicode; `as_str()` failed.")?;
    script
        .as_file_mut()
        .write_all(
            format!(
                r#"#!/bin/bash
    . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null
    mv {} /home/lvuser/{exec_name}
    echo "/home/lvuser/{exec_name}" > /home/lvuser/robotCommand
    chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand
    sync
    ldconfig
    . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null"#,
                EXECUTABLE_TEMPORARY_PATH,
                exec_name = executable_name
            )
            .as_bytes(),
        )
        .map_err(str_map("Could not write to temporary deploy script file"))?;
    script
        .as_file_mut()
        .sync_all()
        .map_err(str_map("'sync_all()' on script file failed"))?;
    let script_path = script
        .as_ref()
        .canonicalize()
        .map_err(str_map("Could not canonicalize script path"))?;
    info!("scp-ing deploy script...");
    scp(
        ref_slice(&script_path),
        rio_address,
        DEPLOY_SCRIPT_CANONICAL_PATH,
    )?;

    info!("scp-ing executable...");
    scp(
        ref_slice(&executable_path),
        rio_address,
        EXECUTABLE_TEMPORARY_PATH,
    )?;

    info!("Deploying libs...");
    deploy_libs(rio_address)?;

    info!("ssh-ing to execute deploy script...");
    ssh(
        &rio_address,
        &format!("sh {}", DEPLOY_SCRIPT_CANONICAL_PATH),
    )?;
    Ok(())
}

/// Only call this with addresses checked with `test_ssh_address` first
fn scp<T: AsRef<OsStr> + fmt::Debug>(
    local_paths: &[T],
    target_address: &str,
    remote_path: &str,
) -> Result<(), String> {
    let mut builder = subprocess::Exec::cmd("scp")
        .arg("-oBatchMode=yes")
        .arg("-oStrictHostKeyChecking=no");
    for arg in local_paths.iter() {
        builder = builder.arg(arg);
    }
    builder = builder.arg(format!("{}:{}", target_address, remote_path));
    trace!("scp subprocess builder struct: {:?}", builder);
    debug!("Running scp command: \"{}\"", builder.to_cmdline_lossy());
    handle_subprocess("scp", builder.join())?;
    Ok(())
}

/// Only call this with addresses checked with `test_ssh_address` first
fn ssh<T: AsRef<OsStr>>(target_address: &T, command: &str) -> Result<(), String> {
    let builder = subprocess::Exec::cmd("ssh")
        .arg("-oBatchMode=yes")
        .arg("-oStrictHostKeyChecking=no")
        .arg(target_address)
        .arg(command);
    trace!("ssh process builder struct: {:?}", builder);
    debug!("Running ssh -command: \"{}\"", builder.to_cmdline_lossy());
    handle_subprocess("ssh", builder.join())?;
    Ok(())
}

const DEPLOY_TARGET_TRIPLE: &str = "arm-unknown-linux-gnueabi";

fn cargo_build(matches: &ArgMatches<'_>, config: &FrcConfig) -> Result<(), String> {
    info!("Building the project...");
    let mut args = vec![
        "build",
        "--quiet",
        "--target",
        DEPLOY_TARGET_TRIPLE,
        "--bin",
    ];
    args.push(
        config
            .executable
            .to_str()
            .ok_or("Executable name is not valid Unicode.")?,
    );
    if matches.is_present("release") {
        args.push("--release");
    }
    debug!("Using cargo args {:?}", args);
    let exit_code = subprocess::Exec::cmd("cargo")
        .args(&args)
        .join()
        .map_err(str_map("'cargo build' subprocess failed"))?;
    handle_subprocess_exit("cargo build", exit_code)
}

const LIBS_TO_DEPLOY: &[&str] = &["wpiHal", "wpiutil" /* "ntcore.so", "cscore"*/];

fn deploy_libs(target_address: &str) -> Result<(), String> {
    debug!("Attempting to deploy libs: {:?}", LIBS_TO_DEPLOY);
    let mut symlink_path = env::temp_dir();
    symlink_path.push("frc-libs");
    symlink_path
        .canonicalize()
        .map_err(str_map("Could not canonicalize symlinked athena lib path"))?;
    let libs: Vec<_> = LIBS_TO_DEPLOY
        .iter()
        .map(|name| {
            let mut d = symlink_path.clone();
            d.push(format!("lib{}.so", name));
            d
        })
        .collect();
    trace!("Deploying libs at paths {:?}", libs);
    scp(&libs[..], target_address, "/usr/local/frc/third-party/lib")?;
    Ok(())
}
