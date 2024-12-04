use aoc_2024::{days::day_from_i32, Day};
use chrono::{Datelike, Local};
use clap::Parser;

/// Runs a given AoC day. Defaults to todays date.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    day: Option<i32>,
}

pub fn get_day() -> Box<dyn Day> {
    let args = Args::parse();
    match args.day {
        Some(x) => day_from_i32(x),
        None => day_from_i32(Local::now().day() as i32),
    }
    .expect("Could not determine day. Please indicate it using --day [VAL] where 1 <= VAL <= 25")
}

fn main() {
    let day = get_day();
    day.run_part1();
    day.run_part2();
}
