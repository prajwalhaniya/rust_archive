mod file_handler;

use std::io::BufRead;

use file_handler::{create_file, read_large_file, read_small_file};

fn main() {
    println!("Hello, world!");
    let result = create_file("test", "test.txt", "Hello, world!");
    match result {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error creating file: {}", e),
    }

    let content = read_large_file("test/test.txt");
    match content {
        Ok(content) => {
            let mut lines = content.lines();
            while let Some(line) = lines.next() {
                println!("{:?}", line);
            }
        },
        Err(e) => println!("Error reading file: {}", e),
    }
}

