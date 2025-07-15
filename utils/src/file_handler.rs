/**

Lets understand:
    - Error propagation operator i.e "?"   
 */
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub fn create_file(dir_path: &str, file_name: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = format!("{}/{}", dir_path, file_name);
    
    if !Path::new(dir_path).exists() {
        std::fs::create_dir_all(dir_path)?;
        println!("Directory {} created", dir_path);
    } else {
        println!("Directory {} already exists", dir_path);
    }

    let mut file = File::create(&file_path)?;

    file.write_all(content.as_bytes())?;

    println!("File {} created", file_path);

    Ok(())
}

pub fn read_small_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn read_large_file(file_path: &str) -> Result<BufReader<File>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}


