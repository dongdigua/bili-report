pub mod bili;
pub mod ytb;

use std::fs;

fn main() {
    match std::env::args().collect::<Vec<String>>()[1].as_str() {
        "bili" => {
            let uid = fs::read_to_string("data/bili.uid").unwrap().parse::<u32>().unwrap();
            bili::get_and_save_data(uid, 3)
        }
        "ytb" => {
            ()
        }
        _ => todo!()
    }
}
