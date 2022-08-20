use std::{thread, time};
use std::error;
use reqwest::blocking;
use serde_json::{self, Value};

const BILI_API: &'static str = "http://api.bilibili.com/x/space/arc/search/";
const PS: u8 = 15;  // pn should double

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
                    thread::sleep(time::Duration::from_millis(3000));
                    vinfo_list
                },
                Err(BiliError::Interception) => {
                    thread::sleep(time::Duration::from_millis(10000));
                    parse_data(&data_json).unwrap()
                },
                Err(_) => todo!()
            }
        })
        .flatten()
        .map(|i| {
            format!("{} :|: {}", i.bvid, i.title)
        })
        .collect();
    println!("{:#?}", line_vec)
}

fn get_data(mid: u32, pn: u8) -> Result<Value, Box<dyn error::Error>> {
    let request_url = format!("{}?mid={}&ps={}&pn={}", BILI_API, mid, PS, pn);
    //let cookie = std::fs::read_to_string("data/cookie").unwrap();

    let client = blocking::Client::builder()
        .proxy(reqwest::Proxy::https("socks5://127.0.0.1:9050")?)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_13_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.186 Safari/537.36")
        .build()?;
    let body = client
        .get(request_url)
        .header("Host", "api.bilibili.com")
        .header("Accept-Language", "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")
        .header("Referer", format!("https://space.bilibili.com/{}/video", mid))
        .header("Origin", "https://space.bilibili.com")
        .header("Connection", "keep-alive")
        //.header("Cookie", cookie)
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "cors")
        .header("Sec-Fetch-Site", "same-site")
        .header("TE" ,"trailers")
        .send()?
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
                        .filter_map(|s| {
                            let vinfo = s.as_object().unwrap();
                            if vinfo["copyright"].as_str().unwrap() == "1" {
                            //if true {
                                Some(
                                    VideoInfo {
                                        bvid: vinfo["bvid"].as_str().unwrap().to_owned(),
                                        title: vinfo["title"].as_str().unwrap().to_owned(),
                                    }
                                )
                            } else {
                                None
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
