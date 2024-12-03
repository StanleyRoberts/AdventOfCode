use std::{env, fs, path::Path};

use dotenv::dotenv;
use reqwest::{blocking::Client, header::COOKIE};

const DATA_DIR: &str = "data/";

pub fn get_puzzle_input(day: usize) -> String {
    let path = Path::new(DATA_DIR).join(day.to_string() + ".txt");
    if let Ok(content) = fs::read_to_string(&path) {
        return content;
    }
    let _ = dotenv();
    let input = Client::new()
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .header(
            COOKIE,
            "session=".to_owned()
                + &env::var("SESSION").expect("No session token found. Please set SESSION env var"),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();
    let _ = fs::write(path, &input);
    input
}
