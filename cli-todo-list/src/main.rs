use std::env::args;
use inline_colorization::{color_green, color_red, color_reset, color_cyan};

fn main() {
    print_header();
    let command_opt = args().nth(1);

    match command_opt {
        Some(command) => {
            match command.as_str() {
                "list" => { println!("Yello"); }
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

fn print_header() {
    println!("Welcome to TODO CLI");
    println!();
    println!("{color_green}Usage:{color_reset} {color_cyan}cli-todo-list [command] [args]{color_reset}");
    println!();
    println!("{color_green}Commands:{color_reset}");
    println!("        {color_cyan}list{color_reset}           Print all embedded todos");
    println!("        {color_cyan}add{color_reset}            Add a new todo list item");
    println!("        {color_cyan}rm{color_reset}             Remove a todo item");
    println!("    {color_cyan}-h, --help{color_reset}        Print help");
    println!();
}
