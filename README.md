# Typst as Library

This repository shows how to use [typst](https://github.com/typst/typst) as a library in Rust.

```rust
fn main() {
    let content = "= Hello, World!";

    // All the abstraction needed is here (!)
    let world = TypstWrapperWorld::new("./".to_owned(), content);

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst");

    // Output to pdf
    let pdf = typst_pdf::pdf(&document, None, None);
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
}
```

---

## Acknowledgment

Code has been inspired by
- [https://github.com/fenjalien/obsidian-typst](https://github.com/fenjalien/obsidian-typst)
- [https://github.com/mattfbacon/typst-bot](https://github.com/mattfbacon/typst-bot)
