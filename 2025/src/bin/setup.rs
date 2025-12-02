use std::{fs, path::Path};

use chrono::{Datelike, Local};
use clap::Parser;

const DAY_TEMPLATE: &str = "use super::{impl_day, Part1, Part2};

impl_day!(Day£, £);

impl Part1 for Day£ {
    fn part1(&self, input: &str) -> u64 {
        todo!()
    }
}

impl Part2 for Day£ {
    fn part2(&self, input: &str) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = \"nightly\")]
    use crate::day::DayMeta;
    #[cfg(feature = \"nightly\")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(Day£.part1(todo!()), todo!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day£.part2(todo!()), todo!());
    }

    #[cfg(feature = \"nightly\")]
    #[cfg_attr(feature = \"nightly\", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day£.get_input();
        b.iter(|| Day£.part1(&input));
    }

    #[cfg(feature = \"nightly\")]
    #[cfg_attr(feature = \"nightly\", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day£.get_input();
        b.iter(|| Day£.part2(&input));
    }
}";

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
}
