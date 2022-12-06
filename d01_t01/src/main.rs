use clap::Parser;
use std::path::PathBuf;

use d01_t01::{compute};

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

    // read file into calorie vector
    let mut calories = compute(std::fs::read(args.file).unwrap());
    // sort in descending order
    calories.sort_by(|a, b| b.cmp(a));

    println!("part one: {}", calories[0]);

    println!("part two: {}", calories[0..3].iter().sum::<u64>())
}
