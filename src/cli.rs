use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // File
    pub file: Vec<String>,

    /// Package to include, (use general toml syntax ['package = "0.1"', 'package = { version = "0.1" }'])
    #[arg(short, long)]
    pub include: Vec<String>,
}

pub fn parse() -> Args {
    Args::parse()
}