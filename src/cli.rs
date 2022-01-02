use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    pub operation: String,
}

pub fn parse() -> Cli {
    Cli::parse()
}
