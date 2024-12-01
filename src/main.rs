use std::env;
use chrono::offset::Utc;
use chrono::Datelike;
use reqwest::{blocking::Client, header::{HeaderMap, HeaderValue, COOKIE}};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = env::var("ADVENT_OF_CODE_SESSION")?;
    let cookie_str = format!("session={session}");
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(&cookie_str)?);
    let client = Client::builder().default_headers(headers).build()?;
    
    let day = Utc::now().day();
    let url = format!("https://adventofcode.com/2024/day/{day}/input");
    let res = client.get(url).send()?;
    let filepath = format!("inputs/{day}.txt");
    fs::write(&filepath, res.text()?)?;

    Ok(())
}