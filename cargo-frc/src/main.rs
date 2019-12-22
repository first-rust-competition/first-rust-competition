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
mod config;
mod deploy;
mod util;
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
    let temp_matches = App::new(format!("cargo-{}", COMMAND_NAME))
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
                    SubCommand::with_name("deploy").arg(
                        Arg::with_name("release")
                            .short("r")
                            .long("release")
                            .help("If specified, will target the deployment of a release build"),
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

    let cfg = config::get_config()?;

    match frc_matches.subcommand_name() {
        Some("deploy") => {
            deploy::deploy_command(frc_matches.subcommand_matches("deploy").unwrap(), &cfg)
        }
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
