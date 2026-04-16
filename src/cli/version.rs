use clap::Args;

#[derive(Args, Debug)]
pub struct VersionCommand;

impl VersionCommand {
    pub fn run(&self) {
        println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
