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

    use pdfium_render::prelude::*;

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
            .or_else(|_| Pdfium::bind_to_system_library())
            .unwrap() // Or use the ? unwrapping operator to pass any error up to the caller
    );

    Ok(())
}
