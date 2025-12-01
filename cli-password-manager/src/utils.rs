use std::fs::File;
use std::io::Read;
use std::path::Path;
use inline_colorization::{color_bright_green, color_reset};
use serde::{Deserialize, Serialize};

pub const DB_FILE: &str = "passwords.db";

#[derive(Debug, Deserialize, Serialize)]
struct Credential {
    service: String,
    username: String,
    enc_password: String,
}

fn initialize_db() {
    let db_path = Path::new(DB_FILE);
    if !db_path.exists() {
        File::create(db_path).expect("Unable to create database file");
    }
}

fn read_db() -> Vec<Credential> {
    initialize_db();
    let mut file = File::open(DB_FILE).expect("Unable to open database file");
    let mut content = String::new();
    let file_size_read = file.read_to_string(&mut content).expect("Unable to read database file");

    println!("{color_bright_green}File size read: {} {color_reset}", file_size_read);

    serde_json::from_str(&content).expect("Unable to parse database file")
}