mod init;

use clap::{Parser, Subcommand};
use color_eyre::eyre::Result;

#[derive(Parser, Debug)]
#[clap(name = "wpiilib-xtask", version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the workspace.
    Init,
}

fn main() -> Result<()> {
    setup_logging()?;
    let cli = Cli::parse();

    match &cli.commands {
        Commands::Init => init::init(),
    }
}

/// Set up tracing and eyre.
fn setup_logging() -> Result<()> {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    // Set up tracing.
    let fmt_layer = fmt::layer().with_target(false);

    tracing_subscriber::registry()
        .with(fmt_layer)
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
