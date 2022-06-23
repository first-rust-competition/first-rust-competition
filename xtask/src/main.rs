mod clean;
mod init;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;
use tracing::metadata::LevelFilter;

#[derive(Parser, Debug)]
#[clap(name = "wpilib-xtask", version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,

    #[clap(short, long, action)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the workspace.
    Init,
    Clean,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    setup_logging(cli.verbose)?;

    match &cli.commands {
        Commands::Init => init::init(),
        Commands::Clean => clean::clean(),
    }
}

/// Set up tracing and eyre.
fn setup_logging(verbose: bool) -> Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    // Set up tracing.
    let mut layers = Vec::new();

    if verbose {
        let layer = fmt::layer().with_target(false).boxed();
        layers.push(layer);
    } else {
        let layer = fmt::layer().pretty().with_filter(LevelFilter::INFO).boxed();
        layers.push(layer);
    };

    tracing_subscriber::registry()
        .with(layers)
        .with(ErrorLayer::default())
        .init();

    // Set up hooks.
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();

    eyre_hook.install()?;

    std::panic::set_hook(Box::new(move |pi| {
        tracing::error!("{}", panic_hook.panic_report(pi));
    }));

    Ok(())
}
