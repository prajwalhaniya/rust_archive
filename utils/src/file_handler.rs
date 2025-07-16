/**

Lets understand:
    - Error propagation operator i.e "?"   
 */
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use tokio::sync::Mutex;
use std::sync::Arc;

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

pub async fn write_line_to_a_file(file_path: &Arc<Mutex<File>>, message: &str) {
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}", timestamp, message);

    let mut file = file_path.lock().await;

    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("Failed to write to log file: {}", e);
    }
    
    if let Err(e) = file.flush() {
        eprintln!("Failed to flush log file: {}", e);
    }
}
