# Chapter source code

This folder contains the source code used in the guide.

## Viewing the source code

To view the source code from each chapter, navigate to `chapter_code/src/bin/`. Each file / folder
corresponds to a specific chapter.

Some chapters contain multiple examples, in which case the source code will be subdivided inside the chapter folder.
For example, in case of `images`, there will be two examples: `chapter_code/src/bin/image_clear.rs` and `chapter_code/src/bin/mandelbrot.rs`.

## Running the source code

If you want to run the source code and experiment for yourself, run `cargo run --bin <chapter>` inside this folder.
For example:

```bash
cargo run --bin windowing
```
