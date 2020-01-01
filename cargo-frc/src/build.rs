use crate::toolchain::Toolchain;
use crate::util::str_map;
use clap::ArgMatches;
use std::process::Command;

const DEPLOY_TARGET_TRIPLE: &str = "arm-unknown-linux-gnueabi";

fn roborio_build(toolchain: Toolchain) -> Result<(), String> {
    if !toolchain.installed() {
        return Err(format!(
            "The {} toolchain is not installed",
            toolchain.year()
        ));
    }

    info!("Building with the {} toolchain", toolchain.year());

    let build = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg(DEPLOY_TARGET_TRIPLE)
        //        .arg("--quiet")
        //        .arg("--bin")
        .env(
            "RUSTFLAGS",
            format!(
                "-C target-cpu=cortex-a9 -C linker={}",
                toolchain.linker().to_str().unwrap()
            ),
        )
        .status()
        .map_err(str_map("Failed to execute cargo build"))?;

    if !build.success() {
        return Err("Build failed".to_owned());
    }

    Ok(())
}

pub fn build_command(matches: &ArgMatches) -> Result<(), String> {
    if let Some(toolchain) = matches.value_of("year").and_then(Toolchain::from_year) {
        roborio_build(toolchain)
    } else {
        Err("Invalid toolchain year specified".to_owned())
    }
}
