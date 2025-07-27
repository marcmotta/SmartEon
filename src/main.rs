// src/main.rs
/*
 * Main executable for SmartEon
 */

use clap::Parser;
use smarteon::{Result, run};

#[derive(Parser)]
#[command(version, about = "SmartEon - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
