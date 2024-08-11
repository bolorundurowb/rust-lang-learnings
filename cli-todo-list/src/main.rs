use std::env::args;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use inline_colorization::{color_green, color_red, color_reset, color_cyan, color_yellow};

const DB_PATH: &str = "./todo.db";

fn main() {
    let command_opt = args().nth(1);

    match command_opt {
        Some(command) => {
            match command.as_str() {
                "list" => { list_todos(); }
                "add" => {
                    let todo_opt = args().nth(2);

                    match todo_opt {
                        Some(todo_entry) => add_todo(todo_entry.as_str()),
                        None => {
                            println!("{color_red}Arguments are required for the 'add' command{color_reset}");
                            println!();
                            print_header();
                        }
                    }
                }
                "clean" => { clean_todos(); }
                "-h" | "--help" => { print_header(); }
                _ => {
                    println!("{color_red}Unknown command '{}' provided{color_reset}", command);
                    println!();
                    print_header();
                }
            }
        }
        None => {
            println!("{color_red}No command provided{color_reset}");
            println!();
            print_header();
        }
    }
}

fn add_todo(input: &str) {
    ensure_db();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(DB_PATH)
        .unwrap();
    writeln!(file, "{}", input).expect("Write failed");
    println!("{color_green}Todo written successfully{color_reset}");
}

fn clean_todos() {
    fs::remove_file(DB_PATH).expect("Failed to clean out todos");
}

fn list_todos() {
    ensure_db();

    let mut file = File::open(DB_PATH).expect("Failed to read database file");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Failed to read database file");

    let lines: Vec<&str> = content.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    if lines.len() == 0 {
        println!("{color_yellow}You have no todos saved{color_reset}");
    } else {
        println!("{color_green}TODOS{color_reset}");
        for line in lines {
            println!(" {color_cyan}-{color_reset} {}", line);
        }
    }
}

fn ensure_db() {
    let file_path = Path::new(DB_PATH);
    let file_exists = file_path.exists();

    if !file_exists {
        File::create(file_path).expect("Failed to create database file");
    }
}

fn print_header() {
    println!("Welcome to TODO CLI");
    println!();
    println!("{color_green}Usage:{color_reset} {color_cyan}cli-todo-list [command] [args]{color_reset}");
    println!();
    println!("{color_green}Commands:{color_reset}");
    println!("     {color_cyan}list{color_reset}           Print all embedded todos");
    println!("     {color_cyan}add{color_reset}            Add a new todo list item");
    println!("     {color_cyan}rm{color_reset}             Remove a todo item");
    println!("     {color_red}clean{color_reset}          Clear all todos");
    println!(" {color_cyan}-h, --help{color_reset}         Print help");
    println!();
}
