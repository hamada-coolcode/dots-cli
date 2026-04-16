/// CLI module containing all command-line interface components.
///
/// This module organizes the command handlers and the main CLI entry point.
/// Each command has its own file with a corresponding `<CommandName>Command` struct
/// that implements the `run()` method.
pub mod doctor;
pub mod entrypoint;
pub mod version;
