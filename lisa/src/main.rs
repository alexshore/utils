use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Optional directory
    dir: Option<PathBuf>,

    /// Show hidden files
    #[arg(short = 'a', long)]
    show_all: bool,

    /// Show extended information
    #[arg(short = 'l', long)]
    long_listing: bool,
}

fn main() -> Result<()> {
    let args = dbg!(Args::parse());

    Ok(())
}
