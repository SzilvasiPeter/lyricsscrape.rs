use reqwest::{self, StatusCode};
use scraper::{Html, Selector};
use tokio;

#[tokio::main]
async fn main() {
    let artist = read_artist().await;
    let body = read_lyrics(&artist).await;
    let lyrics = extract_lyrics(&body).await;

    println!("{}", lyrics);
}

async fn read_artist() -> String {
    let mut artist = String::new();

    println!("Give the name of the artist: ");
    std::io::stdin()
        .read_line(&mut artist)
        .expect("Error during reading the artist");

    let artist = artist.to_lowercase().replace(" ", "-");
    let artist_page = format!("https://genius.com/artist/{artist}");

    let status = reqwest::get(artist_page).await.unwrap().status();
    if status != StatusCode::OK {
        eprintln!("Unable to find the given artist!");
    }

    return artist;
}

async fn read_lyrics(artist: &str) -> String {
    let mut song = String::new();

    println!("Give a song from the artist: ");
    std::io::stdin()
        .read_line(&mut song)
        .expect("Error during reading the song");

    let lyrics_page = format!("https://genius.com/{artist}-{song}-lyrics");

    let response = reqwest::get(lyrics_page).await.unwrap();
    if response.status() != StatusCode::OK {
        eprintln!("The lyrics for the song could not be found!")
    }

    return response.text().await.unwrap();
}

async fn extract_lyrics(body: &str) -> String {
    let document = Html::parse_document(body);

    let selector = Selector::parse("div.Lyrics__Container-sc-1ynbvzw-1.kUgSbL").unwrap();
    let div = document.select(&selector).next();
    let lyrics = div
        .expect("Element was not found")
        .inner_html()
        .replace("<br>", "\n");

    return lyrics;
}
