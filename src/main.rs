mod file_ops;

use clap::Parser;
use colored::*;
use file_ops::{combine_srt_files_to_markdown, generate_srt_files};
use std::path::PathBuf;

/// A simple program to process and combine .srt files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input directory containing .srt files
    #[arg(short, long, default_value = "input")]
    input: PathBuf,

    /// Output directory for processed and combined files
    #[arg(short, long, default_value = "output")]
    output: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    generate_srt_files(&args.input, &args.output)?;
    combine_srt_files_to_markdown(&args.output);
    println!("{}", "✨✨ All operations completed successfully.".green());
    Ok(())
}
