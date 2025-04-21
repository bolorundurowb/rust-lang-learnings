use std::env::args;
use std::fs;
use inline_colorization::{color_blue, color_bright_red, color_cyan, color_green, color_magenta, color_red, color_reset, color_yellow};

fn main() {
    let file_path_opt = args().nth(1);

    match file_path_opt {
        None => { println!("{color_red}Please provide a file path.{color_reset}") }
        Some(file_path) => {
            let metadata = fs::metadata(&file_path).unwrap();
            let content = fs::read_to_string(&file_path).unwrap();
            let lines: Vec<&str> = content.split('\r').collect();
            let line_count = lines.len();

            let mut word_count = 0;
            for line in lines {
                let words: Vec<&str> = line.split(' ').collect();
                word_count += words.len();
            }

            let file_size = metadata.len();
            let file_created = metadata.created().unwrap();
            let file_modified = metadata.modified().unwrap();

            println!("FILE STATS TOOL");
            println!("-------------------------------------------------");
            println!("  File path: {color_blue}\"{file_path}\"{color_reset}");
            println!("  File size: {color_yellow}{file_size} bytes{color_reset}");
            println!("  File created at: {color_magenta}{file_created:?}{color_reset}");
            println!("  File modified at: {color_bright_red}{file_modified:?}{color_reset}");
            println!("  Line count: {color_green}{line_count}{color_reset}");
            println!("  Word count: {color_cyan}{word_count}{color_reset}");
        }
    }
}
