use aoc_2025::days::{AllDays, day_from_i32};
use chrono::{Datelike, Local};
use clap::Parser;

/// Runs a given AoC day. Defaults to todays date.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    /// Which day to run
    day: Option<i32>,
    #[arg(short, long, default_value_t = false)]
    /// Run all days
    all: bool,
}

fn main() {
    let args = Args::parse();
    if args.all {
        for i in AllDays::default() {
            i.run_part1();
            i.run_part2();
        }
    } else {
        let day = match args.day {
            Some(x) => day_from_i32(x),
            None => day_from_i32(Local::now().day() as i32),
        }
        .expect(
            "Could not determine day. Please indicate it using --day [VAL] where 1 <= VAL <= 25",
        );
        day.run_part1();
        day.run_part2();
    }
}
