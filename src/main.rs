// src/main.rs
/*
 * Main executable for IntelligentModelOptimizerDevNext
 */

use clap::Parser;
use intelligentmodeloptimizerdevnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "IntelligentModelOptimizerDevNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
