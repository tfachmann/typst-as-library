# Typst as Library

This repository shows how to use [typst](https://github.com/typst/typst) as a library in Rust.
Any code presented in this repository is meant to help you understand how to interface with `typst`.
Please use the code as you like.

```rust
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
}
```

Check the [example](https://github.com/tfachmann/typst-as-library/tree/main/examples/native) for more information.

---

## Acknowledgment

Code has been inspired by
- [https://github.com/fenjalien/obsidian-typst](https://github.com/fenjalien/obsidian-typst)
- [https://github.com/mattfbacon/typst-bot](https://github.com/mattfbacon/typst-bot)
