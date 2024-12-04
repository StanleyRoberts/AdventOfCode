pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
use super::day::*;

const LAST_DAY: i32 = 25;

/// Day has not been implemented yet or is not within (inclusive) range 1 to 25
#[derive(Debug)]
pub struct MissingDay;

pub fn day_from_i32(day: i32) -> Result<Box<dyn Day>, MissingDay> {
    match day {
        1 => Ok(Box::new(day1::Day1)),
        2 => Ok(Box::new(day2::Day2)),
        3 => Ok(Box::new(day3::Day3)),
        4 => Ok(Box::new(day4::Day4)),
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
