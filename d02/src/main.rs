use clap::Parser;
use d02::{compute, strategy_1, strategy_2};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file path to read
    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    // parse arguments
    let args = Args::parse();

    let data = std::fs::read(args.file).unwrap();

    println!("part one: {}", compute(&data, strategy_1));
    println!("part two: {}", compute(&data, strategy_2));
}
