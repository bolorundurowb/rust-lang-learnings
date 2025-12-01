use std::env::args;
use inline_colorization::{color_green, color_red, color_reset, color_cyan, color_yellow};

fn main() {
    let command_opt = args().nth(1);
    match command_opt {
        None => {
            println!("{color_red}No command provided. Run `cli-password-manager --help` to see the options{color_reset}");
            println!();
        }
        Some(command) => {
            match command.as_str() {
                "list" => list_credentials(),
                "add" => {},
                "get" => {},
                "update" => {},
                "rm" => {},
                "clean" => {},
                "-h" | "--help" => print_header(),
                _ => println!("{color_red}Unknown command: {command}{color_reset}")
            }
        }
    }
}

fn list_credentials() {}

fn add_credentials(service: String, username: String, password: String) {}

fn get_credentials(service: String, username: Option<String>) {}

fn update_credentials(service: String, username: String, password: String) {}

fn remove_credentials(service: String, username: String) {}

fn clean_credentials() {}

fn print_header() {
    println!("Welcome to RustPass CLI");
    println!();
    println!("{color_green}Usage:{color_reset} {color_cyan}cli-password-manager [command] [args]{color_reset}");
    println!();
    println!("{color_green}Commands:{color_reset}");
    println!("     {color_cyan}list{color_reset}           Print all saved passwords");
    println!("     {color_cyan}add{color_reset}            Add a new password");
    println!("     {color_cyan}get{color_reset}            Get the password for a service");
    println!("     {color_cyan}update{color_reset}         Update a password");
    println!("     {color_cyan}rm{color_reset}             Remove a password");
    println!("     {color_red}clean{color_reset}          Clear all passwords");
    println!(" {color_cyan}-h, --help{color_reset}         Print help");
    println!();
}
