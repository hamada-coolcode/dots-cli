mod cli;
mod doctor;

use crate::cli::entrypoint::{CliEntrypoint, CliEntrypointSubcommands};
use clap::Parser;

/// Main entry point for the dots CLI application.
///
/// Parses command-line arguments using clap and dispatches to the
/// appropriate command handler based on the user's input.
///
/// # Example
///
/// ```bash
/// $ dots doctor
/// Running system diagnostics ...
/// git: installed
/// mise: installed
///
/// $ dots version
/// dots-cli v0.0.0
/// ```
pub fn main() {
    let args = CliEntrypoint::parse();

    match args.command {
        CliEntrypointSubcommands::Doctor(cmd) => cmd.run(),
        CliEntrypointSubcommands::Version(cmd) => cmd.run(),
    }
}
