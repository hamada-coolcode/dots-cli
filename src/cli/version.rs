/// Version command module for displaying application information.
///
/// Provides the `version` subcommand that shows the application name
/// and version from Cargo.toml.
use clap::Args;

/// Command struct for the `version` subcommand.
///
/// Displays the application name and version using environment variables
/// from Cargo.toml.
#[derive(Args, Debug)]
pub struct VersionCommand;

impl VersionCommand {
    /// Execute the version command.
    ///
    /// Prints the application name and version in the format `<name> v<version>`.
    ///
    /// # Example
    ///
    /// ```bash
    /// $ dots version
    /// dots-cli v0.0.0
    /// ```
    pub fn run(&self) {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
