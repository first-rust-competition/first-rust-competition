extern crate serde_json;
extern crate subprocess;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate serde;
mod config;
mod deploy;
use clap::{App, AppSettings, Arg, SubCommand};
use std::env;

const COMMAND_NAME: &'static str = "frc";
const COMMAND_DESCRIPTION: &'static str = "The unufficial cargo extension for FRC.";

fn main() {
    match cli_app() {
        Ok(_) => println!("Finished Successfully."),
        Err(x) => error!("Fatal: {}", x),
    }
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
        .subcommand(SubCommand::with_name(COMMAND_NAME)
            // the real entry point
            .about(COMMAND_DESCRIPTION)
            .subcommand(SubCommand::with_name("deploy")
                .arg(Arg::with_name("release")
                    .short("r")
                    .long("release")
                    .help("If specified, will target the deployment of a release build")
                )
            )
            .setting(AppSettings::SubcommandRequired)
        )
        .setting(AppSettings::SubcommandRequired)
        .get_matches();

    let frc_matches = temp_matches
        .subcommand_matches(COMMAND_NAME)
        .ok_or("frc subcommand not specified")?;

    // TODO: add verbosity option
    env::set_var("RUST_LOG", "error");
    pretty_env_logger::init();

    let cfg = config::get_config()?;

    match frc_matches.subcommand_name() {
        Some("deploy") => {
            deploy::deploy_command(frc_matches.subcommand_matches("deploy").unwrap(), &cfg)
        }
        _ => Err(String::from("No subcommand specified (!UNREACHABLE!)")),
    }
}
