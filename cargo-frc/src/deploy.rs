use super::config::FrcConfig;
use clap::ArgMatches;
use std::path::Path;
use std::time::Duration;
use subprocess;
use subprocess::ExitStatus;

const DEPLOY_TARGET_TRIPLE: &'static str = "arm-unknown-linux-gnueabi";

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
    println!("{:?}", executable_path);
    for addr in addresses.iter() {
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
    println!(
        "ssh -q {} exit\nYOU MAY NEED TO ACCEPT KEYS OR ENTER A PASSWORD",
        address
    );
    let mut process = subprocess::Exec::cmd("ssh")
        .arg("-q")
        .arg(address)
        .arg("exit")
        .popen()
        .map_err(|e| {
            format!(
                "'ssh' subprocess failed testing address '{}': {}.",
                address,
                e.to_string()
            )
        })?;
    let ret = match process.wait_timeout(Duration::from_secs(2)).map_err(|e| {
        format!(
            "'ssh' subprocess testing address '{}' failed to wait: {}.",
            address,
            e.to_string()
        )
    })? {
        Some(ExitStatus::Exited(0)) => Ok(true),
        _ => Ok(false),
    };
    process.kill().map_err(|e| {
        format!(
            "'ssh' subprocess testing address '{}' timed out and could not be killed: {}.",
            address,
            e.to_string()
        )
    })?;
    ret
}

fn do_deploy(rio_address: &str, executable_path: &Path) -> Result<(), String> {
    Ok(())
}

pub fn cargo_build(matches: &ArgMatches, config: &FrcConfig) -> Result<(), String> {
    let mut args = vec!["build", "--target", DEPLOY_TARGET_TRIPLE, "--bin"];
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
        .map_err(|e| format!("'cargo build' subprocess failed: {}.", e.to_string()))?;
    match exit_code {
        ExitStatus::Exited(0) => (),
        ExitStatus::Signaled(code) => {
            return Err(format!(
                "'cargo build' exited from Signal or Other, code {}",
                code
            ));
        }
        // duplicate because above code is u8 and this one is i32
        ExitStatus::Other(code) => {
            return Err(format!(
                "'cargo build' exited from Signal or Other, code {}",
                code
            ));
        }
        _ => {
            return Err(String::from(
                "'cargo build' exited Undetermined. Did your executable build fail?",
            ));
        }
    }
    Ok(())
}
