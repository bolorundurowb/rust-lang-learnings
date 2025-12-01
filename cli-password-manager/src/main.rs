mod utils;

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
            let command = command.as_str();
let service_opt = args().nth(2);
            let username_opt = args().nth(3);
            let password_opt = args().nth(4);

            // if command == "add" || command == "update" || command == "rm" {
            //     let service = service_opt.expect("Service is required");
            //     &username_opt.expect("Username is required");
            //
            //     if command != "rm" {
            //         &password_opt.expect("Password is required");
            //     }
            // }

            match command {
                "add" => {
                    let service = service_opt.expect("Service is required");
                    let username = username_opt.expect("Username is required");
                    let password = password_opt.expect("Password is required");
                    add_credentials(service, username, password);
                },
                "get" => {
                    get_credentials(service_opt, username_opt);
                },
                "update" => {
                    let service = service_opt.expect("Service is required");
                    let username = username_opt.expect("Username is required");
                    let password = password_opt.expect("Password is required");
                    update_credentials(service, username, password);
                },
                "rm" => {
                    let service = service_opt.expect("Service is required");
                    let username = username_opt.expect("Username is required");
                    remove_credentials(service, username);
                },
                "clean" => {
                    clean_credentials();
                },
                "-h" | "--help" => print_header(),
                _ => println!("{color_red}Unknown command: {command}{color_reset}")
            }
        }
    }
}


fn add_credentials(service: String, username: String, password: String) {}

fn get_credentials(service: Option<String>, username: Option<String>) {}

fn update_credentials(service: String, username: String, password: String) {}

fn remove_credentials(service: String, username: String) {}

fn clean_credentials() {}

fn print_header() {
    println!("Welcome to RustPass CLI");
    println!();
    println!("{color_green}Usage:{color_reset} {color_cyan}cli-password-manager [command] [args]{color_reset}");
    println!();
    println!("{color_green}Commands:{color_reset}");
    println!("     {color_cyan}add     <service> <username> <password>{color_reset}   Add a new password");
    println!("     {color_cyan}get     [service] [username]{color_reset}              Get the saved credentials either for a specific service or all of them");
    println!("     {color_cyan}update  <service> <username> <password>{color_reset}   Update a password");
    println!("     {color_cyan}rm      <service> <username>{color_reset}              Remove a password");
    println!("     {color_red}clean{color_reset}                                     Clear all passwords");
    println!(" {color_cyan}-h, --help{color_reset}                                    Print help");
    println!();
}
