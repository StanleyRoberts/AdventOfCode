#![cfg_attr(feature = "nightly", feature(test))]
mod day;
pub mod days;
mod input;
pub use day::Day;

#[cfg(feature = "nightly")]
extern crate test;
