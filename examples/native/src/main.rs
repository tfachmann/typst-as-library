use std::fs;

use typst::layout::Abs;
use typst_as_library::TypstWrapperWorld;
use typst_pdf::PdfOptions;

fn main() {
    let content = r#"
#import "@preview/polylux:0.4.0": *

#set page(paper: "presentation-16-9")


#set text(
  font: "Lato",
  size: 23pt,
)

#slide[
  #set page(footer: none)
  #set align(horizon + center)

= Hello, World!
A document (+ `polylux` library) rendered with `Typst`!
$ y = m x + n $
]"#
    .to_owned();

    // Create world with content.
    let world = TypstWrapperWorld::new("../".to_owned(), content);

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Output to pdf and svg
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default()).expect("Error exporting PDF");
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
    println!("Created pdf: `./output.pdf`");

    let svg = typst_svg::svg_merged(&document, Abs::pt(2.0));
    fs::write("./output.svg", svg).expect("Error writing SVG.");
    println!("Created svg: `./output.svg`");
}
