use reqwest::{self, Error};
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() {
    let artist = "Platon-Karataev";
    let song = "Ocean";
    let url = format!("https://genius.com/{artist}-{song}-lyrics");
    let lyrics = extract_lyrics(url.as_str())
        .await
        .expect("Error during extracting the lyrics");

    println!("{}", lyrics);
}

async fn extract_lyrics(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);

    let selector = Selector::parse("div.Lyrics__Container-sc-1ynbvzw-1.kUgSbL").unwrap();
    let div = document.select(&selector).next();
    let lyrics = div
        .expect("Element was not found")
        .inner_html()
        .replace("<br>", "\n");

    Ok(lyrics)
}
