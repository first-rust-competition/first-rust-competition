use crate::toolchain::Toolchain;
use crate::util::str_map;
use clap::ArgMatches;
use std::process::Command;

const DEPLOY_TARGET_TRIPLE: &str = "arm-unknown-linux-gnueabi";

pub fn roborio_build(toolchain: Toolchain, bin: Option<&str>, release: bool) -> Result<(), String> {
    if !toolchain.installed() {
        return Err(format!(
            "The {} toolchain is not installed",
            toolchain.year()
        ));
    }

    info!("Building with the {} toolchain", toolchain.year());

    let mut args = vec!["build", "--target", DEPLOY_TARGET_TRIPLE];

    if let Some(bin) = bin {
        args.push("--bin");
        args.push(bin);
    }

    if release {
        args.push("--release");
    }

    debug!("Using cargo args {:?}", args);

    let build = Command::new("cargo")
        .args(args)
        .env(
            "RUSTFLAGS",
            format!(
                "-C target-cpu=cortex-a9 -C linker={}",
                toolchain.linker().to_str().unwrap()
            ),
        )
        .status()
        .map_err(str_map("Failed to execute cargo build"))?;

    trace!("Build process completed");

    if !build.success() {
        return Err("Build failed".to_owned());
    }

    trace!("Build succeeded");

    Ok(())
}

pub fn build_command(matches: &ArgMatches) -> Result<(), String> {
    if let Some(toolchain) = matches.value_of("year").and_then(Toolchain::from_year) {
        roborio_build(
            toolchain,
            matches.value_of("bin"),
            matches.is_present("release"),
        )
    } else {
        Err("Invalid toolchain year specified".to_owned())
    }
}
