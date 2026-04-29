// propkit-cli — scan Rust crates and recommend property tests
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "propkit",
    about = "Scan Rust crates and recommend property tests"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Analyze source files and print property test recommendations
    Scan {
        /// Path to the crate root
        path: PathBuf,
    },
    /// Generate a standalone property test file
    Generate {
        /// Path to the crate root
        path: PathBuf,

        /// Print to stdout instead of writing a file
        #[arg(long)]
        dry_run: bool,

        /// Append to existing test file
        #[arg(long)]
        append: bool,

        /// Minimum confidence level (high, medium, low)
        #[arg(long, default_value = "medium")]
        confidence: String,

        /// Exclude specific types
        #[arg(long)]
        exclude: Vec<String>,

        /// Custom output path
        #[arg(short)]
        o: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan { path } => {
            eprintln!("scan: {} (not yet implemented)", path.display());
        }
        Command::Generate { path, .. } => {
            eprintln!("generate: {} (not yet implemented)", path.display());
        }
    }
}
