use std::{thread, time};
use std::error;
use std::fs;
use reqwest::{blocking, Proxy};
use serde_json::{self, Value};

const YTB_API: &'static str = "https://youtube-search-and-download.p.rapidapi.com/search";


fn get_data(search: String) -> Result<Value, Box<dyn error::Error>>{
    let api_key = fs::read_to_string("data/api.key").unwrap();

    let client = blocking::Client::builder()
        .proxy(Proxy::https("http://127.0.0.1:20172")?)
        .build()?;

    let responce = client
        .get(format!("{}?query={}", YTB_API, search))
        .header("X-RapidAPI-Key", api_key)
        .header("X-RapidAPI-Host", "youtube-search-and-download.p.rapidapi.com")
        .send()?
        .text()?;

    let data: Value = serde_json::from_str(&responce)?;
    Ok(data)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ytb_api_key() {
        println!("{:#?}", get_data(String::from("rick roll")).unwrap());
    }
}
