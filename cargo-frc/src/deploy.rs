use super::config::FrcConfig;
use clap::ArgMatches;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use subprocess;
use subprocess::ExitStatus;
use tempfile;
use util::*;

pub fn deploy_command(matches: &ArgMatches, config: &FrcConfig) -> Result<(), String> {
    // println!("{}", test_ssh_address("demo@test.rebex.net")?);
    cargo_build(matches, config)?;

    let addresses = if let Some(addr) = config.rio_address_override.clone() {
        vec![addr]
    } else {
        make_addresses(config
            .team_number
            .ok_or("No RIO address or team number specified")?)
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

    //test
    // do_deploy("admin@10.1.14.2", &executable_path)?;

    for addr in addresses.iter() {
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

const DEPLOY_SCRIPT_CANONICAL_PATH: &'static str = "/home/lvuser/cargo-frc-script.sh";
const EXECUTABLE_TEMPORARY_PATH: &'static str = "/home/lvuser/rust-program-temp";

fn do_deploy(rio_address: &str, executable_path: &Path) -> Result<(), String> {
    let executable_path = executable_path
        .canonicalize()
        .map_err(str_map("Could not canonicalize executable path"))?;
    let mut script =
        tempfile::NamedTempFile::new().map_err(str_map("Could not create temporary script file"))?;
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
            ).as_bytes(),
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
    scp(&script_path, rio_address, DEPLOY_SCRIPT_CANONICAL_PATH)?;

    info!("scp-ing executable...");
    scp(&executable_path, rio_address, EXECUTABLE_TEMPORARY_PATH)?;

    info!("ssh-ing to execute deploy script...");
    ssh(
        &rio_address,
        &format!("sh {}", DEPLOY_SCRIPT_CANONICAL_PATH),
    )?;
    Ok(())
}

/// Only call this with addresses checked with `test_ssh_address` first
fn scp<T: AsRef<OsStr>>(
    local_path: &T,
    target_address: &str,
    remote_path: &str,
) -> Result<(), String> {
    debug!(
        "scp -oBatchMode=yes, $local_path {}:{}",
        target_address, remote_path,
    );
    let handle = subprocess::Exec::cmd("scp")
        .arg("-oBatchMode=yes")
        .arg("-oStrictHostKeyChecking=no")
        .arg(local_path)
        .arg(format!("{}:{}", target_address, remote_path))
        .join();
    handle_subprocess("scp", handle)?;
    Ok(())
}

/// Only call this with addresses checked with `test_ssh_address` first
fn ssh<T: AsRef<OsStr>>(target_address: &T, command: &str) -> Result<(), String> {
    debug!("ssh -oBatchMode=yes $target_address \"{}\"", command);
    let handle = subprocess::Exec::cmd("ssh")
        .arg("-oBatchMode=yes")
        .arg("-oStrictHostKeyChecking=no")
        .arg(target_address)
        .arg(command);
    // .join();
    println!("\n{:?}\n", handle);
    // let handle = subprocess::Exec::shell(format!(
    //     "ssh -oBatchMode=yes -oStrictHostKeyChecking=no {} \"{}
    // \"",
    //     target_address, command
    // )).join();
    handle_subprocess("ssh", handle.join())?;
    Ok(())
}

const DEPLOY_TARGET_TRIPLE: &'static str = "arm-unknown-linux-gnueabi";

pub fn cargo_build(matches: &ArgMatches, config: &FrcConfig) -> Result<(), String> {
    let mut args = vec![
        "build",
        "--quiet",
        "--target",
        DEPLOY_TARGET_TRIPLE,
        "--bin",
    ];
    args.push(config
        .executable
        .to_str()
        .ok_or("Executable name is not valid Unicode.")?);
    if matches.is_present("release") {
        args.push("--release");
    }
    let exit_code = subprocess::Exec::cmd("cargo")
        .args(&args)
        .join()
        .map_err(str_map("'cargo build' subprocess failed"))?;
    handle_subprocess_exit("cargo build", exit_code)
}
