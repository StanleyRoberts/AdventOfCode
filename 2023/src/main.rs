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
}
