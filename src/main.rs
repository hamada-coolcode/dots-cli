mod cli;
mod doctor;

use crate::cli::entrypoint::{CliEntrypoint, CliEntrypointSubcommands};
use clap::Parser;

pub fn main() {
    let args = CliEntrypoint::parse();

    match args.command {
        CliEntrypointSubcommands::Doctor(cmd) => cmd.run(),
        CliEntrypointSubcommands::Version(cmd) => cmd.run(),
    }
}
