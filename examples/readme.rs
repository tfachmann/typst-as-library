use std::fs;

use typst::{eval::Tracer, foundations::Smart};
use typst_as_library::TypstWrapperWorld;

fn main() {
    let content = "= Hello, World!";

    // All the abstraction needed is here (!)
    let world = TypstWrapperWorld::new("./examples".to_owned(), content.to_owned());

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst");

    // Output to pdf
    let pdf = typst_pdf::pdf(&document, Smart::Auto, None);
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf: `./output.pdf`");
}
