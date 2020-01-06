// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate serde_json;
extern crate subprocess;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate fern;
extern crate ref_slice;
extern crate serde;
extern crate tempfile;
mod build;
mod config;
mod deploy;
mod init;
mod toolchain;
mod util;
use crate::toolchain::Toolchain;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use fern::colors::{Color, ColoredLevelConfig};
use util::*;

const COMMAND_NAME: &str = "frc";
const COMMAND_DESCRIPTION: &str = "The unofficial cargo extension for FRC.";

fn main() {
    std::process::exit(match cli_app() {
        Ok(_) => {
            println!("Finished Successfully.");
            0
        }
        Err(x) => {
            error!("Fatal: {}\nRun with -vv for more info.", x);
            1
        }
    });
}

fn cli_app() -> Result<(), String> {
    let valid_toolchains = Toolchain::available()
        .iter()
        .map(Toolchain::year)
        .collect::<Vec<_>>();

    let temp_matches =
        App::new(format!("cargo-{}", COMMAND_NAME))
            .about("This is meant to be run as 'cargo frc', try running it like that.")
            .version(&crate_version!()[..])
            // We have to lie about our binary name since this will be a third party
            // subcommand for cargo, this trick stolen from cargo-cook who stole from cargo-outdated
            .bin_name("cargo")
            // We use a subcommand because parsed after `cargo` is sent to the third party plugin
            // which will be interpreted as a subcommand/positional arg by clap
            .subcommand(
                SubCommand::with_name(COMMAND_NAME)
                    // the real entry point
                    .about(COMMAND_DESCRIPTION)
                    .arg(
                        Arg::with_name("verbose")
                            .long("verbose")
                            .short("v")
                            .multiple(true)
                            .global(true),
                    )
                    .subcommand(
                        SubCommand::with_name("deploy")
                            .arg(Arg::with_name("release").short("r").long("release").help(
                                "If specified, will target the deployment of a release build",
                            ))
                            .arg(
                                Arg::with_name("year")
                                    .short("y")
                                    .long("year")
                                    .default_value("2020")
                                    .possible_values(&valid_toolchains)
                                    .takes_value(true)
                                    .help("The toolchain year to use for linking"),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("init")
                            .about("Create a new basic robot project in the current directory")
                            .arg(
                                Arg::with_name("NUMBER")
                                    .help("The team number to be used when deploying")
                                    .takes_value(true)
                                    .required(true)
                                    .index(1),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("new")
                            .about("Create a new basic robot project")
                            .arg(
                                Arg::with_name("NAME")
                                    .required(true)
                                    .index(1)
                                    .help("The name for the new robot project"),
                            )
                            .arg(
                                Arg::with_name("NUMBER")
                                    .help("The team number to be used when deploying")
                                    .takes_value(true)
                                    .required(true)
                                    .index(2),
                            ),
                    )
                    .subcommand(
                        SubCommand::with_name("toolchain")
                            .about("Manage FRC toolchains")
                            .subcommand(
                                SubCommand::with_name("install")
                                    .alias("i")
                                    .about("Install FRC toolchains")
                                    .arg(
                                        Arg::with_name("YEAR")
                                            .required(true)
                                            .possible_values(&valid_toolchains)
                                            .index(1)
                                            .help("The year of the toolchain to install"),
                                    ),
                            )
                            .subcommand(
                                SubCommand::with_name("list")
                                    .alias("l")
                                    .about("List available and installed toolchains"),
                            )
                            .setting(AppSettings::SubcommandRequiredElseHelp),
                    )
                    .subcommand(
                        SubCommand::with_name("build")
                            .alias("b")
                            .about("Cross-compile for the roborio using FRC toolchains")
                            .arg(
                                Arg::with_name("year")
                                    .short("y")
                                    .long("year")
                                    .default_value("2020")
                                    .possible_values(&valid_toolchains)
                                    .takes_value(true)
                                    .help("The toolchain year to use for linking"),
                            )
                            .arg(
                                Arg::with_name("release")
                                    .short("r")
                                    .long("release")
                                    .takes_value(false)
                                    .help("Build in release mode"),
                            )
                            .arg(
                                Arg::with_name("bin")
                                    .long("bin")
                                    .takes_value(true)
                                    .help("Specify which binary to build. (Optional)"),
                            ),
                    )
                    .setting(AppSettings::SubcommandRequired),
            )
            .setting(AppSettings::SubcommandRequired)
            .get_matches();

    let frc_matches = temp_matches
        .subcommand_matches(COMMAND_NAME)
        .ok_or("frc subcommand not specified")?;

    let level = setup_logger(frc_matches).map_err(str_map("Could not initialize logging"))?;
    info!("Using log level {}", level);

    match frc_matches.subcommand_name() {
        Some("deploy") => {
            let cfg = config::get_config()?;
            deploy::deploy_command(frc_matches.subcommand_matches("deploy").unwrap(), &cfg)
        }
        Some("init") => init::init_command(frc_matches.subcommand_matches("init").unwrap()),
        Some("new") => init::new_command(frc_matches.subcommand_matches("new").unwrap()),
        Some("toolchain") => {
            toolchain::handle_cmd(frc_matches.subcommand_matches("toolchain").unwrap())
        }
        Some("build") => build::build_command(frc_matches.subcommand_matches("build").unwrap()),
        _ => Err(String::from("No subcommand specified (!UNREACHABLE!)")),
    }
}

fn setup_logger(matches: &ArgMatches) -> Result<log::LevelFilter, fern::InitError> {
    let level = match 2 + matches.occurrences_of("verbose") {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::White);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!("{} {}", colors.color(record.level()), message,))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(level)
}
