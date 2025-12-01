//! Provides puzzle input and writes it to a file
//!
//! Requires two environment variables to be set:
//! - SESSION: session token for AoC, retrieved by logging in to the AoC site and inspecting any requests cookie field and extracting the value in the "session" field
//! - CONTACT: your contact info, in the case requests spam the AoC site

use std::{env, fs, path::Path};

use dotenv::dotenv;
use reqwest::{
    blocking::Client,
    header::{COOKIE, USER_AGENT},
};

const DATA_DIR: &str = "data/";

pub fn get_puzzle_input(day: usize) -> String {
    let path = Path::new(&(env!("CARGO_MANIFEST_DIR").to_owned()))
        .join(DATA_DIR)
        .join(day.to_string() + ".txt");
    let _ = fs::create_dir_all(path.parent().unwrap());
    if let Ok(content) = fs::read_to_string(&path) {
        return content;
    }
    let _ = dotenv();
    let input = Client::new()
        .get(format!("https://adventofcode.com/2025/day/{day}/input"))
        .header(
            COOKIE,
            "session=".to_owned()
                + &env::var("SESSION").expect("No session token found. Please set SESSION env var"),
        )
        .header(
            USER_AGENT,
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"), " ").to_owned()
                + &env::var("CONTACT").expect("No contact info found. Please set CONTACT env var"),
        )
        .send()
        .unwrap()
        .text()
        .unwrap();
    fs::write(path, &input).unwrap();
    input
}
