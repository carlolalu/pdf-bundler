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

#[cfg(test)]
mod tests {
    use std::io::Write;

    use anyhow::Result;
    use printpdf::*;
    use rand;

    pub fn craft_random_text_of_len(char_length: usize) -> String {
        use rand::distr::{SampleString, StandardUniform};

        let random_string: String = StandardUniform.sample_string(&mut rand::rng(), char_length);
        //println!("random_valid_text: [[[{random_valid_text}]]]");

        random_string
    }

    fn create_basic_pdf(name: &str, n_pages: u8) -> Result<()> {
        let mut doc = PdfDocument::new("example.pdf");

        // I think the catalog is implicit

        let page1_title = "Page 1";
        let mut page1_lines: Vec<Op> = craft_random_text_of_len(600)
            .chars()
            .collect::<Vec<char>>()
            .chunks_mut(20)
            .map(|chunk| {
                let line: String = chunk.iter().collect();
                Op::MoveToNextLineShowText { text: line }
            })
            .collect();

        let mut page1_contents = vec![
            Op::StartTextSection,
            Op::SetFontSizeBuiltinFont {
                size: Pt(40_f32),
                font: BuiltinFont::CourierBold,
            },
            Op::MoveTextCursorAndSetLeading {
                tx: 2_f32,
                ty: 6_f32,
            },
        ];

        page1_contents.append(&mut page1_lines);
        page1_contents.push(Op::EndTextSection);

        let page1 = PdfPage::new(Mm(210.0), Mm(297.0), page1_contents);

        let pdf_bytes: Vec<u8> = doc.with_pages(vec![page1]).save(
            &PdfSaveOptions::default(),
            &mut vec![PdfWarnMsg::info(
                1,
                1,
                "I do not know what does this struct do, it is totally undocumented".to_string(),
            )],
        );

        let mut file = std::fs::File::create(std::path::Path::new("./test/pdfs/basic.pdf"))?;
        file.write(&pdf_bytes)?;

        Ok(())
    }

    #[test]
    fn basic_pdf() -> Result<()> {
        create_basic_pdf("random document", 10)?;
        Ok(())
    }
}
