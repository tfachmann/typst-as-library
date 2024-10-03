# Typst as Library

As of March 15th, 2024 (typst version `0.11.0`) it has finally been published to [crates.io](https://crates.io/crates/typst), so there is no more need for git dependencies.

This repository shows how to use [typst](https://github.com/typst/typst) as a library in Rust.

```rust
fn main() {
    let content = "= Hello, World!";

    // All the abstraction needed is here (!)
    let world = TypstWrapperWorld::new("./".to_owned(), content.to_owned());

    // Render document
    let mut tracer = Tracer::default();
    let document = typst::compile(&world, &mut tracer).expect("Error compiling typst");

    // Output to pdf
    let pdf = typst_pdf::pdf(&document, Smart::Auto, None);
    fs::write("./output.pdf", pdf).expect("Error writing PDF.");
}
```

You can run the example above via `cargo run --example readme`

---

## Acknowledgment

Code has been inspired by
- [https://github.com/fenjalien/obsidian-typst](https://github.com/fenjalien/obsidian-typst)
- [https://github.com/mattfbacon/typst-bot](https://github.com/mattfbacon/typst-bot)
