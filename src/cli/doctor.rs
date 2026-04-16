use crate::doctor::checks::run_checks;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum DoctorSubCommands {
    All {
        #[arg(long)]
        json: bool,
    },
}

#[derive(Args, Debug)]
pub struct DoctorCommand {
    #[arg(long)]
    pub json: bool,

    #[command(subcommand)]
    pub commands: Option<DoctorSubCommands>,
}

impl DoctorCommand {
    pub fn run(&self) {
        println!("Running system diagnostics ...");
        let results = run_checks();
        for result in results {
            println!(
                "{}: {}",
                result.name,
                if result.installed {
                    "installed"
                } else {
                    "NOT installed"
                }
            );
        }
    }
}
