use std::env::args;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};
use inline_colorization::{color_cyan, color_green, color_reset};
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main() {
    let url = args().nth(1).expect("A source URL is required");
    let output_dir = args().nth(2).expect("An output dir is required");

    let path = Path::new(output_dir.as_str());

    if !path.exists() {
        fs::create_dir(path).expect("Failed to create the output directory");
    }

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header(USER_AGENT, "Winner's Bot")
        .send()
        .await
        .unwrap();

    let index_text = response.text().await.unwrap();

    // write the index file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{output_dir}{MAIN_SEPARATOR}index.html"))
        .unwrap();
    file.write(index_text.as_bytes()).expect("Failed to write index file");

    println!("{color_green}Url{color_reset} {color_cyan}'{}'{color_reset} {color_green}scraped successfully{color_reset}", url);
}
