use std::{
    env, fs,
    path::{Path, PathBuf},
};

use rustc_version::{Channel, version_meta};

const MODULE_PRE: &str = "
use super::day::*;

const LAST_DAY: i32 = 25;

/// Day has not been implemented yet or is not within (inclusive) range 1 to 25
#[derive(Debug)]
pub struct MissingDay;

pub fn day_from_i32(day: i32) -> Result<Box<dyn Day>, MissingDay> {
    match day {";
const MODULE_POST: &str = "
        _ => Err(MissingDay),
    }
}

#[derive(Default)]
pub struct AllDays {
    cur: i32,
}

impl Iterator for AllDays {
    type Item = Box<dyn Day>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;
        if self.cur >= LAST_DAY {
            None
        } else {
            match day_from_i32(self.cur) {
                Ok(a) => Some(a),
                Err(_) => self.next(),
            }
        }
    }
}
";

fn make_days_rs() {
    println!("cargo::rerun-if-changed=src/days");
    let module = Path::new("src/days/");
    let _ = fs::create_dir_all(module);

    let out_dir = env::var("OUT_DIR").unwrap();
    let mod_rs = Path::new(&out_dir).join("days.rs");
    let all_days = fs::read_dir(module)
        .unwrap()
        .map(|x| {
            x.unwrap()
                .file_name()
                .into_string()
                .unwrap()
                .replace("day", "")
                .replace(".rs", "")
        })
        .collect::<Vec<_>>();
    let mut parents_to_crate_root = 0;
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut current = mod_rs.as_path();
    while current != crate_root {
        current = current.parent().unwrap();
        parents_to_crate_root += 1;
    }
    let relative_days_dir = PathBuf::from("../".repeat(parents_to_crate_root - 1))
        .join("src")
        .join("days");
    let mod_conts = all_days.iter().fold(String::new(), |acc: String, x| {
        acc + &format!(
            "#[path = \"{path}\"]\npub mod day{x};\n",
            path = relative_days_dir
                .join(format!("day{x}.rs"))
                .to_string_lossy() //"pub mod day{x} {{ include!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/days/day{x}.rs\")); }}\n"
        )
    }) + MODULE_PRE
        + &all_days.iter().fold(String::new(), |acc, x| {
            acc + &format!("\n        {x} => Ok(Box::new(day{x}::Day{x})),")
        })
        + MODULE_POST;
    let _ = fs::write(mod_rs, mod_conts);
}

fn main() {
    if matches!(version_meta().unwrap().channel, Channel::Nightly) {
        println!("cargo::rustc-cfg=feature=\"nightly\"");
    }
    make_days_rs();
}
