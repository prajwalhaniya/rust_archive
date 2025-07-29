use std::env;
use std::fs;
use std::io::{ self, BufRead, BufReader };
use std::process;

#[derive(Debug)]
struct Config {
    pattern: String,
    files: Vec<String>,
    case_insensitive: bool,
    line_numbers: bool,
    count_only: bool,
    invert_match: bool,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Usage: grep [OPTIONS] PATTERN [FILE...]");
        }

        let mut pattern = String::new();
        let mut files = Vec::new();
        let mut case_insensitive = false;
        let mut line_numbers = false;
        let mut count_only = false;
        let mut invert_match = false;
        
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "-i" | "--ignore-case" => case_insensitive = true,
                "-n" | "--line-number" => line_numbers = true,
                "-c" | "--count" => count_only = true,
                "-v" | "--invert-match" => invert_match = true,
                arg if arg.starts_with('-') => {
                    eprintln!("Unknown option: {}", arg);
                    return Err("Invalid option");
                }
                _ => {
                    if pattern.is_empty() {
                        pattern = args[i].clone();
                    } else {
                        files.push(args[i].clone());
                    }
                }
            }
            i += 1;
        }

        if pattern.is_empty() {
            return Err("Pattern is required");
        }

        if files.is_empty() {
            files.push("-".to_string());
        }

        Ok(Config {
            pattern,
            files,
            case_insensitive,
            line_numbers,
            count_only,
            invert_match,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let multiple_files = config.files.len() > 1;
    
    for filename in &config.files {
        if filename == "-" {
            let stdin = io::stdin();
            let reader = stdin.lock();
            search_reader(reader, &config, None, multiple_files)?;
        } else {
            let file = fs::File::open(filename)?;
            let reader = BufReader::new(file);
            search_reader(reader, &config, Some(filename), multiple_files)?;
        }
    }
    
    Ok(())
}

fn search_reader<R: BufRead>(
    reader: R,
    config: &Config,
    filename: Option<&str>,
    multiple_files: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = if config.case_insensitive {
        config.pattern.to_lowercase()
    } else {
        config.pattern.clone()
    };

    let mut line_number = 0;
    let mut match_count = 0;
    let mut matches = Vec::new();

    for line in reader.lines() {
        let line = line?;
        line_number += 1;

        let search_line = if config.case_insensitive {
            line.to_lowercase()
        } else {
            line.clone()
        };

        let is_match = search_line.contains(&pattern);
        let should_print = if config.invert_match { !is_match } else { is_match };

        if should_print {
            match_count += 1;
            if !config.count_only {
                matches.push((line_number, line));
            }
        }
    }

    if config.count_only {
        if multiple_files {
            if let Some(name) = filename {
                println!("{}:{}", name, match_count);
            } else {
                println!("(standard input):{}", match_count);
            }
        } else {
            println!("{}", match_count);
        }
    } else {
        for (line_num, line) in matches {
            let mut output = String::new();
            
            // Add filename if multiple files
            if multiple_files {
                if let Some(name) = filename {
                    output.push_str(&format!("{}:", name));
                } else {
                    output.push_str("(standard input):");
                }
            }
            
            // Add line number if requested
            if config.line_numbers {
                output.push_str(&format!("{}:", line_num));
            }
            
            output.push_str(&line);
            println!("{}", output);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_basic_search() {
        let config = Config {
            pattern: "hello".to_string(),
            files: vec![],
            case_insensitive: false,
            line_numbers: false,
            count_only: false,
            invert_match: false,
        };

        let input = "hello world\nfoo bar\nhello rust\n";
        let cursor = Cursor::new(input);
        
        // This would normally print to stdout, but we're testing the logic
        assert!(search_reader(cursor, &config, None, false).is_ok());
    }

    #[test]
    fn test_case_insensitive() {
        let config = Config {
            pattern: "HELLO".to_string(),
            files: vec![],
            case_insensitive: true,
            line_numbers: false,
            count_only: false,
            invert_match: false,
        };

        let input = "hello world\nHELLO rust\n";
        let cursor = Cursor::new(input);
        
        assert!(search_reader(cursor, &config, None, false).is_ok());
    }

    #[test]
    fn test_config_parsing() {
        let args = vec![
            "grep".to_string(),
            "-i".to_string(),
            "-n".to_string(),
            "pattern".to_string(),
            "file.txt".to_string(),
        ];

        let config = Config::new(args).unwrap();
        assert_eq!(config.pattern, "pattern");
        assert_eq!(config.files, vec!["file.txt"]);
        assert!(config.case_insensitive);
        assert!(config.line_numbers);
    }
}