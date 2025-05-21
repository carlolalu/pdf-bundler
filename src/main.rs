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
mod test {
    use anyhow::Result;
    use lopdf::{
        self, Document, Object, Stream,
        content::{Content, Operation},
        dictionary,
    };

    pub fn craft_random_text_of_len(char_length: usize) -> String {
        use rand::distr::{SampleString, StandardUniform};

        let random_string: String = StandardUniform.sample_string(&mut rand::rng(), char_length);
        //println!("random_valid_text: [[[{random_valid_text}]]]");

        random_string
    }

    /// Generate a PDF file with minimal features
    fn generate_basic_pdf_with(doc_name: &str, pages: u8) -> Result<()> {
        let mut doc = Document::with_version("1.5");

        let pages_id = doc.new_object_id();

        let font_id = doc.add_object(dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Courier",
        });

        let resources_id = doc.add_object(dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                },

        });

        let pages_ids: Vec<_> = (1..pages)
            .map(|page_number| {
                let page_title = format!("Page {}", page_number);
                // this length should be adapted at the page size
                let random_text = craft_random_text_of_len(20);
                //let page_random_text: Vec<String>= page_random_text.chars().collect::<Vec<_>>().chunks(20).collect()

                // Wrapper for a vector of operands and operations in PDF
                let content = Content {
                    operations: vec![
                        Operation::new("BT", vec![]),
                        Operation::new("Td", vec![50.into(), 600.into()]),
                        Operation::new("TL", vec![50.into()]),
                        Operation::new("Tf", vec!["F1".into(), 46.into()]),
                        Operation::new("Tj", vec![lopdf::Object::string_literal(doc_name)]),
                        Operation::new("Tf", vec!["F1".into(), 36.into()]),
                        Operation::new("'", vec![lopdf::Object::string_literal(page_title)]),
                        Operation::new("Tf", vec!["F1".into(), 20.into()]),
                        Operation::new("'", vec![lopdf::Object::string_literal(random_text)]),
                        Operation::new("ET", vec![]),
                    ],
                };

                let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode()?));

                let page_id = doc.add_object(dictionary! {
                    "Type" => "Page",
                    "Parent" => pages_id,
                    "Contents" => content_id,
                });

                Ok(page_id)
            })
            .collect::<Result<_>>()?;

        let pages = dictionary! {
            "Type" => "Pages",
            "Kids" => pages_ids.iter().map(|&page_id| page_id.into()).collect::<Vec<_>>(),
            "Count" => pages,
            "Resources" => resources_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        };

        // Using `insert()` here, instead of `add_object()` since the ID is already known.
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });

        // The "Root" key in trailer is set to the ID of the document catalog,
        // the remainder of the trailer is set during `doc.save()`.
        doc.trailer.set("Root", catalog_id);
        doc.compress();

        doc.save("test/pdfs/example.pdf")?;

        Ok(())
    }

    #[test]
    fn generate_basic_pdf() -> Result<()> {
        generate_basic_pdf_with("Basic Dok", 30)
    }
}
