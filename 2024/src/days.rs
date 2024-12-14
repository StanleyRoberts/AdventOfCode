pub mod day1;
pub mod day10;
pub mod day11;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

use super::day::*;

const LAST_DAY: i32 = 25;

/// Day has not been implemented yet or is not within (inclusive) range 1 to 25
#[derive(Debug)]
pub struct MissingDay;

pub fn day_from_i32(day: i32) -> Result<Box<dyn Day>, MissingDay> {
    match day {
        1 => Ok(Box::new(day1::Day1)),
        10 => Ok(Box::new(day10::Day10)),
        11 => Ok(Box::new(day11::Day11)),
        14 => Ok(Box::new(day14::Day14)),
        2 => Ok(Box::new(day2::Day2)),
        3 => Ok(Box::new(day3::Day3)),
        4 => Ok(Box::new(day4::Day4)),
        5 => Ok(Box::new(day5::Day5)),
        6 => Ok(Box::new(day6::Day6)),
        7 => Ok(Box::new(day7::Day7)),
        8 => Ok(Box::new(day8::Day8)),
        9 => Ok(Box::new(day9::Day9)),
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
