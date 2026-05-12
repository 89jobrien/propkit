// propkit-cli — scan Rust crates and recommend property tests
//
// Copyright 2026 Joseph O'Brien
// SPDX-License-Identifier: MIT OR Apache-2.0

mod analyzer;
mod generators;

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
        Command::Generate {
            path,
            dry_run,
            confidence,
            o,
            ..
        } => {
            let min_confidence = match confidence.as_str() {
                "high" => analyzer::Confidence::High,
                "low" => analyzer::Confidence::Low,
                _ => analyzer::Confidence::Medium,
            };
            let analyses = scan_crate(&path);
            let output = generators::property::generate_tests(&analyses, min_confidence);

            eprintln!("note: add proptest to dev-dependencies: cargo add --dev proptest");

            if dry_run {
                print!("{output}");
            } else {
                let out_path = o.unwrap_or_else(|| path.join("tests/propkit_properties.rs"));
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent).ok();
                }
                std::fs::write(&out_path, &output)
                    .unwrap_or_else(|e| eprintln!("error writing {}: {e}", out_path.display()));
                eprintln!("wrote {}", out_path.display());
            }
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
