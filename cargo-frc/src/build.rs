use crate::config::FrcConfig;
use crate::toolchain::Toolchain;
use crate::util::str_map;
use clap::ArgMatches;
use std::process::Command;

pub const ROBORIO_TARGET_TRIPLE: &str = "arm-unknown-linux-gnueabi";

pub fn roborio_build(toolchain: Toolchain, bin: Option<&str>, release: bool) -> Result<(), String> {
    if !toolchain.installed() {
        return Err(format!(
            "The {} toolchain is not installed",
            toolchain.year()
        ));
    }

    info!("Building with the {} toolchain", toolchain.year());

    let mut args = vec!["build", "--target", ROBORIO_TARGET_TRIPLE];

    if let Some(bin) = bin {
        args.push("--bin");
        args.push(bin);
    }

    if release {
        args.push("--release");
    }

    debug!("Using cargo args {:?}", args);

    let linker = toolchain.linker().to_str().unwrap().to_owned();

    let formatted_triple = ROBORIO_TARGET_TRIPLE.to_uppercase().replace("-", "_");

    let build = Command::new("cargo")
        .args(args)
        .env(format!("CC_{}", ROBORIO_TARGET_TRIPLE), &linker)
        .env(format!("CARGO_TARGET_{}_LINKER", formatted_triple), &linker)
        .env(
            format!("CARGO_TARGET_{}_RUSTFLAGS", formatted_triple),
            "-C target-cpu=cortex-a9",
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

pub fn cargo_build(matches: &ArgMatches, config: &FrcConfig) -> Result<(), String> {
    info!("Building the project...");

    let toolchain = if let Some(y) = matches.value_of("year") {
        Toolchain::from_year(y).ok_or_else(|| "Invalid toolchain year specified".to_owned())?
    } else {
        config
            .toolchain_year
            .ok_or_else(|| "No toolchain specified".to_owned())?
    };

    roborio_build(
        toolchain,
        Some(
            config
                .executable
                .to_str()
                .ok_or("Executable name is not valid Unicode.")?,
        ),
        matches.is_present("release"),
    )
}
