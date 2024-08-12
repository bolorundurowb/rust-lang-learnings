use std::env::args;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};
use futures::StreamExt;
use chromiumoxide::{Browser, BrowserConfig};
use inline_colorization::{color_cyan, color_green, color_reset};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = args().nth(1).expect("A source URL is required");
    let output_dir = args().nth(2).expect("An output dir is required");

    let path = Path::new(output_dir.as_str());

    if !path.exists() {
        fs::create_dir(path).expect("Failed to create the output directory");
    }

    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;
    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let home_page = load_page_and_read_html(&browser, &url).await;
    write_content_to_file(&output_dir, &url, home_page.as_str(), "index.html");

    for relative_url in find_relative_urls(&home_page) {
        let next_url = format!("{}{}", &url, &relative_url);
        let page_content = load_page_and_read_html(&browser, &next_url).await;
        write_content_to_file(&output_dir, &next_url, &page_content, &format!("{}.html", relative_url.replace("/", "_")));
    }

    // kill the browser
    browser.close().await?;
    handle.await;

    Ok(())
}

async fn load_page_and_read_html(browser: &Browser, url: &str) -> String {
    let page = browser.new_page(&*url).await.unwrap();

    let result = page.wait_for_navigation()
        .await.unwrap();
    result.content().await.unwrap()
}

fn write_content_to_file(output_dir: &str, url: &str, content: &str, file_name: &str) {
    // write the index file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{output_dir}{MAIN_SEPARATOR}{file_name}"))
        .unwrap();
    file.write(content.as_bytes()).expect("Failed to write output file");

    println!("{color_green}Url{color_reset} {color_cyan}'{}'{color_reset} {color_green}scraped successfully{color_reset}", url);
}

fn find_relative_urls(content_to_search: &str) -> Vec<String> {
    let reg = Regex::new(r#"href="((?:\./|\.\.|/)[\w/\.-]+)""#).unwrap();
    let mut result: Vec<String> = Vec::new();

    for capture in reg.captures_iter(&content_to_search) {
        let mut copy = String::new();
        let _ = &capture[1].clone_into(&mut copy);
        result.push(remove_leading_dot(copy.as_str()));
    }

    result
}

fn remove_leading_dot(s: &str) -> String {
    if s.starts_with('.') {
        &s[1..]
    } else {
        s
    }.to_string()
}

