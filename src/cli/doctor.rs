/// Doctor command module for system diagnostics.
///
/// Provides the `doctor` subcommand that checks for required tools
/// and their installation status.
use crate::doctor::checks::run_checks;
use clap::Args;
use owo_colors::OwoColorize;
use serde_json::to_string;

/// Command struct for the `doctor` subcommand.
///
/// Runs system diagnostics to check for required tools and their
/// installation status.
#[derive(Args, Debug)]
pub struct DoctorCommand {
    /// Output results in JSON format instead of human-readable text.
    ///
    /// When set to true, results are printed as a JSON array.
    ///
    /// # Example
    ///
    /// ```bash
    /// $ dots doctor --json
    /// [{"name":"git","installed":true},{"name":"mise","installed":true}]
    /// ```
    #[arg(long)]
    pub json: bool,
}

impl DoctorCommand {
    /// Execute the doctor command.
    ///
    /// Runs all health checks and displays the results. When `json` is
    /// true, outputs results in JSON format. Otherwise displays results
    /// in a formatted table.
    ///
    /// # Example
    ///
    /// ```bash
    /// $ dots doctor
    /// ┌─────────── dots doctor ───────────┐
    /// │ git            ✓ installed        │
    /// │ mise            ✓ installed        │
    /// └────────────────────────────────────┘
    /// ```
    pub fn run(&self) {
        let results = run_checks();

        if self.json {
            if let Ok(json) = to_string(&results) {
                println!("{}", json);
            }
        } else {
            print_results(&results);
        }
    }
}

/// Print health check results in a formatted table with colors.
fn print_results(results: &[crate::doctor::checks::HealthResult]) {
    println!("┌─ dots doctor ─┐");

    for r in results {
        let status = if r.installed {
            format!("{} ✓", r.name).green().to_string()
        } else {
            format!("{} ✗", r.name).red().to_string()
        };

        println!("│ {}", status);
    }

    println!("└───────────────┘");
}
