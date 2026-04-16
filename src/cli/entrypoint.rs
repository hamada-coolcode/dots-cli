use super::doctor::DoctorCommand;
use super::version::VersionCommand;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum CliEntrypointSubcommands {
    Doctor(DoctorCommand),
    Version(VersionCommand),
}

#[derive(Parser, Debug)]
#[command(
    name = "dots",
    about = "All of what relates to dotfiles.",
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct CliEntrypoint {
    #[command(subcommand)]
    pub command: CliEntrypointSubcommands,
}
