use std::fs;

use typst_as_library::TypstWrapperWorld;
use typst_pdf::PdfOptions;

fn main() {
    let content = "= Hello, World!";

    // All the abstraction needed is here (!)
    let world = TypstWrapperWorld::new("./examples".to_owned(), content.to_owned());

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Output to pdf
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf: `./output.pdf`");
}
