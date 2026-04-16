mod cli;

use clap::Parser;
use cli::Cli;

pub fn main() {
    let args = Cli::parse();

    println!("Name: {}", args.name);

    if args.verbose {
        println!("Verbose mode enabled.");
    }
}
