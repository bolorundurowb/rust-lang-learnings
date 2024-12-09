use inline_colorization::{color_blue, color_green, color_red, color_reset};
use std::env::args;

const VALID_COMMANDS: [&str; 4] = ["to-c", "to-f", "-h", "--help"];

fn main() {
    let input = args().skip(1).collect::<Vec<String>>();

    if input.is_empty() {
        println!("{color_red}No input arguments supplied.{color_reset}");
        return;
    }

    let command = &input[0].trim().to_lowercase();

    if !VALID_COMMANDS.contains(&&**command) {
        print_help_prompt();
        return;
    }

    if input.len() == 1 {
        print_header();
    } else if input.len() == 2 {
        let raw_value = &input[1];
        let value_parse_result = raw_value.parse::<f32>();

        match value_parse_result {
            Ok(parsed_value) => {
                if command == "to-c" {
                    let result = (parsed_value - 32f32) / 1.8;
                    println!("{color_green}{}°C{color_reset}", result);
                } else if command == "to-f" {
                    let result = (parsed_value * 1.8) + 32f32;
                    println!("{color_green}{}°F{color_reset}", result);
                } else {
                    panic!();
                }
            }
            Err(_) => {
                print_help_prompt();
            }
        }
    } else {
        print_help_prompt();
    }
}

fn print_help_prompt() {
    println!("{color_red}Invalid input. Invoke 'cli-temp-conv -h' to see usages.{color_reset}");
    println!();
}

fn print_header() {
    println!("Welcome to the CLI Temperature Converter!");
    println!();
    println!(
        "{color_green}Usage:{color_reset} {color_blue}cli-temp-conv [command] [value]{color_reset}"
    );
    println!();
    println!("{color_green}Commands:{color_reset}");
    println!(
        "      {color_blue}to-C{color_reset}       Convert the value to Celsius from Fahrenheit"
    );
    println!(
        "      {color_blue}to-F{color_reset}       Convert the value to Fahrenheit from Celsius"
    );
    println!("      {color_blue}-h, --help{color_reset}     Print help information");
}
