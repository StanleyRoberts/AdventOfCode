use std::{fs, path::Path};

use chrono::{Datelike, Local};
use clap::Parser;

const DAY_TEMPLATE: &str = "use super::{impl_day, Part1, Part2};

impl_day!(Day£, £);

impl Part1 for Day£ {
    fn part1(&self, input: &str) -> usize {
        todo!()
    }
}

impl Part2 for Day£ {
    fn part2(&self, input: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Day£.part1(todo!()), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day£.part2(todo!()), todo!());
    }
}";

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

/// Sets up a given AoC day. Defaults to todays date.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    day: Option<i32>,
}

fn main() {
    let args = Args::parse();
    let day = args.day.unwrap_or_else(|| Local::now().day() as i32);

    let path = format!("src/days/day{day}.rs");
    let file = Path::new(&path);
    assert!(!file.exists(), "Day already exists");
    let contents = DAY_TEMPLATE.replace("£", &day.to_string());
    let _ = fs::write(file, contents);

    let module = Path::new("src/days/");
    let _ = fs::create_dir_all(module);
    let mod_rs = Path::new("src/days.rs");
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
    let mod_conts = all_days
        .iter()
        .fold(String::new(), |acc, x| acc + &format!("pub mod day{x};\n"))
        + MODULE_PRE
        + &all_days.iter().fold(String::new(), |acc, x| {
            acc + &format!("\n        {x} => Ok(Box::new(day{x}::Day{x})),")
        })
        + MODULE_POST;
    let _ = fs::write(mod_rs, mod_conts);
}
