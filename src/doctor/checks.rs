use std::process::Command;

pub struct HealthResult {
    pub name: &'static str,
    pub installed: bool,
}

pub fn run_checks() -> Vec<HealthResult> {
    vec![check_binary("git"), check_binary("mise")]
}

fn check_binary(name: &'static str) -> HealthResult {
    let installed = Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    HealthResult { name, installed }
}
