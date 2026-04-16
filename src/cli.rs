use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A CLI that helps to manage and save dotfiles."
)]
pub struct Cli {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub verbose: bool,
}
