// propkit-cli — scan Rust crates and recommend property tests
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

mod analyzer;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

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
            let analyses = scan_crate(&path);
            print!("{}", analyzer::format_analysis(&analyses));
        }
        Command::Generate { path, .. } => {
            eprintln!("generate: {} (not yet implemented)", path.display());
        }
    }
}

fn scan_crate(path: &Path) -> Vec<analyzer::FileAnalysis> {
    let mut analyses = Vec::new();
    for entry in walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let p = entry.path();
        if p.extension().is_some_and(|ext| ext == "rs")
            && let Some(analysis) = analyzer::analyze_file(p)
        {
            analyses.push(analysis);
        }
    }
    analyses
}
