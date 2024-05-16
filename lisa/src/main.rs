use std::{env, path::PathBuf};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Optional directory
    dir: Option<PathBuf>,

    #[arg(short = 'a', long)]
    show_all: bool,

    #[arg(short = 'l', long)]
    long_listing: bool

}

fn main() -> Result<(), ()>{
    let args = Args::parse();
    // let args = env::args().skip(1).collect::<Vec<_>>();

    println!("{:?}", args);

    Ok(())
}
