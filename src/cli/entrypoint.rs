/// CLI Entrypoint module defining the main CLI structure.
///
/// This module defines the root CLI structure `CliEntrypoint` and its
/// subcommands `CliEntrypointSubcommands`. It uses clap for argument
/// parsing with help and version flags disabled (handled manually).
use super::doctor::DoctorCommand;
use super::version::VersionCommand;
use clap::{Parser, Subcommand};

/// Enumeration of all top-level subcommands available in the CLI.
///
/// Each variant corresponds to a command that can be invoked by the user.
/// The variants hold their respective command structs for argument parsing.
#[derive(Subcommand, Debug)]
pub enum CliEntrypointSubcommands {
    /// Run system diagnostics and health checks.
    ///
    /// Checks for required tools like git and mise.
    ///
    /// # Example
    ///
    /// ```bash
    /// $ dots doctor
    /// Running system diagnostics ...
    /// git: installed
    /// mise: installed
    /// ```
    Doctor(DoctorCommand),

    /// Display the application version.
    ///
    /// # Example
    ///
    /// ```bash
    /// $ dots version
    /// dots-cli v0.0.0
    /// ```
    Version(VersionCommand),
}

/// Main CLI struct that serves as the entry point for the application.
///
/// This struct uses clap's derive macros to automatically generate
/// command-line argument parsing. Help and version flags are disabled
/// as they are handled by custom subcommands.
#[derive(Parser, Debug)]
#[command(
    name = "dots",
    about = "All of what relates to dotfiles.",
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct CliEntrypoint {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: CliEntrypointSubcommands,
}
