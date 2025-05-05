use serde::{Serialize, Deserialize};
use std::fs::{File};
use std::io::{BufReader};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    text: String,
    done: bool
}

const FILE: &str = "todo.json";

fn load_todos() -> Vec<Todo> {
    if !Path::new(FILE).exists() {
        return Vec::new();
    }

    let file: File = File::open(FILE).expect("Failed to open todo file");
    let reader: BufReader<File> = BufReader::new(file);

    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

fn save_todos(todos: &Vec<Todo>) {
    let file: File = File::create(FILE).expect("Failed to create the file");
    serde_json::to_writer_pretty(file, todos).expect("Failed to write todos");
}

fn list_todos() {
    let todos = load_todos();

    for (i, todo) in todos.iter().enumerate() {
        let status = if todo.done { "[x]" } else { "[ ]" };
        println!("{} {} {}", i + 1, status, todo.text);
    }
}

fn add_todo(text: &str) {
   let mut todo = load_todos();
   todo.push(Todo { text: text.to_string(), done: false });
   save_todos(&todo);
   println!("Todo added: {}", text)
}

fn mark_done(index: usize) {
    let mut todos = load_todos();
    if index == 0 || index > todos.len() {
        println!("Invalid todo number");
        return;
    }

    todos[index - 1].done = true;
    save_todos(&todos);
    println!("Todo marked as done!");
}

fn print_help() {
    println!("Todo App Commands");
    println!("add <todo text>");
    println!("list");
    println!("done <number>");
    println!("help");
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: todo add <todo text>");
            } else {
                add_todo(&args[2..].join(" "));
            }
        }
        "list" => list_todos(),
        "done" => {
            if args.len() < 3 {
                println!("Please provide the todo number.");
            } else  if let Ok(num) = args[2].parse::<usize>() {
                mark_done(num);
            } else {
                println!("Invalid number");
            }
        }
        "help" => print_help(),
        _ => print_help(),
    }
}
