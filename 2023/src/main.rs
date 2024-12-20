#![warn(clippy::all, clippy::pedantic)]
#![cfg_attr(feature = "bench", feature(test))]
#[cfg(feature = "bench")]
extern crate test;
#[macro_use]
pub mod helpers;

pub mod solutions;

fn main() {
    solutions::day1();
    solutions::day2();
    solutions::day3();
    solutions::day4();
    //solutions::day5(); // takes a sec
    solutions::day6();
    solutions::day7();
    solutions::day8();
    solutions::day9();
    //solutions::day10();
    solutions::day11();
}
