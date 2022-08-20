use std::{thread, time};
use std::error;
use reqwest::blocking;
use serde_json::{self, Value};

const BILI_API: &'static str = "http://api.bilibili.com/x/space/arc/search/";
const PS: u8 = 30;

pub struct VideoInfo {
    pub bvid: String,
    pub title: String,
}

#[derive(Debug)]
pub enum BiliError {
    ParseError,
    RequestError,
    Interception,
}

pub fn get_and_save_data(mid: u32, pn: u8) {
    let line_vec: Vec<String> = (1..pn)
        .map(|i| {
            let data_json = get_data(mid, i).unwrap();
            match parse_data(&data_json) {
                Ok(vinfo_list) => {
                    thread::sleep(time::Duration::from_millis(500));
                    vinfo_list
                },
                Err(BiliError::Interception) => {
                    thread::sleep(time::Duration::from_millis(1000));
                    parse_data(&data_json).unwrap()
                },
                Err(_) => todo!()
            }
        })
        .flatten()
        .map(|i| {
            format!("{} {}", i.bvid, i.title)
        })
        .collect();
    println!("{:#?}", line_vec)
}

fn get_data(mid: u32, pn: u8) -> Result<Value, Box<dyn error::Error>> {
    let request_url = format!("{}?mid={}&ps={}&pn={}", BILI_API, mid, PS, pn);

    let body = blocking::get(request_url)?
        .text()?;
    let data: Value = serde_json::from_str(&body)?;

    Ok(data)
}

fn parse_data(data: &Value) -> Result<Vec<VideoInfo>, BiliError> {
    let json_map = data.as_object().unwrap();
    match json_map["code"].as_i64().unwrap() {
        0 => {
            match &json_map["data"]["list"]["vlist"] {
                Value::Array(vlist) => {
                    let res: Vec<VideoInfo> = vlist
                        .iter()
                        .map(|s| {
                            let vinfo = s.as_object().unwrap();
                            VideoInfo {
                                bvid: vinfo["bvid"].as_str().unwrap().to_owned(),
                                title: vinfo["title"].as_str().unwrap().to_owned(),
                            }
                        })
                        .collect();
                    Ok(res)
                },
                _ => Err(BiliError::ParseError),
            }
        },
        -400 => Err(BiliError::RequestError),
        -412 => Err(BiliError::Interception),
        _ => Err(BiliError::ParseError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_json() {
        // my bilibili id
        println!("{:#?}", get_data(489732092, 1).unwrap());
    }

    #[test]
    fn get_and_join() {
        get_and_save_data(489732092, 3)
    }
}
