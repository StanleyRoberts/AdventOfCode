#![warn(clippy::all, clippy::pedantic)]
#[macro_use]
pub mod helpers;

pub mod solutions;

fn main() {
    solutions::day1();
}
