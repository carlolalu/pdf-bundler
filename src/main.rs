use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

/// Merge together all the PDFs in a folder and its subfolder (max 4 levels) into a single document
/// provided with a ToC (Table fo Contents) reflecting the structure of tree of the folder and its descendants.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder containing the pdfs
    #[arg(short, long)]
    root_pdfs: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let entries: Vec<_> = std::fs::read_dir(&args.root_pdfs)
        .with_context(|| format!("Could not read file `{}`", args.root_pdfs.display()))?
        .collect();

    println!("Your entries are: {:?}", entries);

    Ok(())
}
