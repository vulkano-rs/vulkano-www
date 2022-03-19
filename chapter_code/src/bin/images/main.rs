mod image_clear;
mod mandelbrot;

use chapter_code::select_example_to_run;

const EXAMPLES: [&str; 2] = ["image_clear", "mandelbrot"];

fn execute_example(selection: &str) {
    println!("Running '{}'", selection);
    match selection {
        "image_clear" => {
            image_clear::main();
        }
        "mandelbrot" => {
            mandelbrot::main();
        }
        _ => panic!(),
    }
}

fn main() {
    select_example_to_run(&EXAMPLES.to_vec(), execute_example);
}
