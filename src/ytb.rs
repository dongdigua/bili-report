use std::{thread, time};
use std::error;
use std::fs;
use std::io::prelude::*;
use reqwest::blocking;
use serde_json::{self, Value};

const YTB_API: &'static str = "https://youtube-search-and-download.p.rapidapi.com/search";

pub struct VideoInfo {
    pub link: String,
    pub title: String,
}

pub fn search_and_save_evidence() {
    let bili_videos = fs::read_to_string("data/bili.list").unwrap();
    let mut evidence_file = fs::OpenOptions::new()
        .write(true)
        .open("evidence.txt")
        .unwrap();

    bili_videos
        .lines()
        .map(|s| {
            s.split(" :|: ")  // bvid :|: title
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .map(|v| {
            let search_result = get_data(v[1].clone()).unwrap();
            let vinfo = parse_data(search_result);

            writeln!(evidence_file, "{} | {} | {} | {}", v[0], v[1], vinfo.link, vinfo.title)
        });
}

fn get_data(search: String) -> Result<Value, Box<dyn error::Error>>{
    let api_key = fs::read_to_string("data/api.key").unwrap();

    let client = blocking::Client::builder()
        .proxy(reqwest::Proxy::https("socks5://127.0.0.1:9050")?)
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

fn parse_data(data: Value) -> VideoInfo {
    let json_list = data.as_object().unwrap()["contents"].as_array().unwrap();

    match &json_list[0] {
        Value::Object(video) => {
            VideoInfo {
                link: format!("youtu.be/{}", video["videoId"].as_str().unwrap()),
                title: video["title"].as_str().unwrap().to_owned(),
            }
        }
        _ => {
            thread::sleep(time::Duration::from_millis(1000));
            VideoInfo {
                link: String::from("error"),
                title: String::from("error"),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ytb_api_key() {
        println!("{:#?}", get_data(String::from("rick roll")).unwrap());
    }
}
