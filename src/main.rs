use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::path::PathBuf;
use std::fs;

/// A program to execute `pdfunite` on a folder with maximum 3 levels and obtain a PDF-file with a ToC of all the tree.
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


fn validate_file(file:) -> Result<()> {

    // file ending is `pdf`
    
}