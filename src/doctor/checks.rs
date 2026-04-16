/// Health checks module for system diagnostics.
///
/// Provides functions to check for the presence of required binaries
/// on the system.
use serde::Serialize;
use std::process::Command;

/// Represents the result of a health check for a binary.
///
/// # Fields
///
/// - `name`: The name of the binary being checked (e.g., "git", "mise")
/// - `installed`: Whether the binary is installed and available in PATH
///
/// # Example
///
/// ```rust
/// let result = HealthResult {
///     name: "git",
///     installed: true,
/// };
/// println!("{} is {}", result.name, if result.installed { "installed" } else { "not installed" });
/// ```
#[derive(Debug, Serialize)]
pub struct HealthResult {
    /// The name of the binary being checked.
    pub name: &'static str,
    /// Whether the binary is installed and available in PATH.
    pub installed: bool,
}

/// Run all health checks.
///
/// Executes checks for all required binaries and returns a vector of
/// results indicating their installation status.
///
/// # Returns
///
/// A `Vec<HealthResult>` containing the check results for each binary.
///
/// # Example
///
/// ```rust
/// let results = run_checks();
/// for result in results {
///     println!("{}: {}", result.name, if result.installed { "installed" } else { "NOT installed" });
/// }
/// ```
pub fn run_checks() -> Vec<HealthResult> {
    vec![check_binary("git"), check_binary("mise")]
}

/// Check if a binary is installed.
///
/// Uses the `which` command to determine if the specified binary exists
/// in the system PATH.
///
/// # Arguments
///
/// - `name`: The name of the binary to check (e.g., "git", "mise")
///
/// # Returns
///
/// A `HealthResult` indicating whether the binary is installed.
///
/// # Example
///
/// ```rust
/// let git_check = check_binary("git");
/// if git_check.installed {
///     println!("git is installed!");
/// }
/// ```
fn check_binary(name: &'static str) -> HealthResult {
    let installed = Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    HealthResult { name, installed }
}
