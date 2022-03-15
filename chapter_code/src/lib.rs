use std::io;

pub mod game_objects;
pub mod models;
pub mod shaders;
mod vertex_data;
pub mod vulkano_objects;

pub use vertex_data::{Vertex2d, Vertex3d};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn select_example_to_run(examples: &Vec<&str>, execute: Box<dyn Fn(&str) -> ()>) {
    println!("Select example to run: (default 0)");

    for (i, example) in examples.iter().enumerate() {
        println!("{} {}", i, example);
    }

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    selection = selection.trim().to_string();

    if selection.len() == 0 {
        execute(examples[0]);
        return;
    }

    let is_numeric = selection.chars().all(|c| c.is_numeric());

    if is_numeric {
        let i = selection.parse::<usize>().unwrap();
        if i >= examples.len() {
            println!(
                "The given index \"{}\" doesn't correspond to any known example",
                selection
            );
        } else {
            execute(examples[i]);
        }
        return;
    }

    match examples.iter().position(|&s| s == selection) {
        Some(i) => {
            execute(examples[i]);
        }
        None => {
            println!("\"{}\" doesn't correspond to any known example", selection);
        }
    }
}
