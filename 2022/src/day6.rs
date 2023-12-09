use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn part1() {
    let content = fs::read_to_string("data/day6.txt").unwrap();
    let line = content.trim();

    for i in 0..line.chars().collect::<Vec<char>>().len() {
        if i>3 && HashSet::<char>::from_iter((&line[i-4..i]).chars()).len() == 4 {
            println!("part1: {i}");
            break;
        }
    }
}

fn part2() {
    let content = fs::read_to_string("data/day6.txt").unwrap();
    let line = content.trim();

    for i in 0..line.chars().collect::<Vec<char>>().len() {
        if i>13 && HashSet::<char>::from_iter((&line[i-14..i]).chars()).len() == 14 {
            println!("part2: {i}");
            break;
        }
    }
}

pub fn main() {
    println!("\nDay 6");
    part1();
    part2();
}