use std::env::args;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, MAIN_SEPARATOR};
use futures::StreamExt;
use chromiumoxide::{Browser, BrowserConfig};
use inline_colorization::{color_cyan, color_green, color_reset};

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
    let page = browser.new_page(&url).await?;

    let home_page = page.wait_for_navigation()
        .await?
        .content()
        .await
        .expect("Failed to read page content");

    write_content_to_file(home_page.as_str(), "index.html");


    // kill the browser
    browser.close().await?;
    handle.await;

    Ok(())
}

fn write_content_to_file(content: &str, file_name: &str) {
    // write the index file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(format!("{output_dir}{MAIN_SEPARATOR}{file_name}"))
        .unwrap();
    file.write(content.as_bytes()).expect("Failed to write index file");

    println!("{color_green}Url{color_reset} {color_cyan}'{}'{color_reset} {color_green}scraped successfully{color_reset}", url);
}
