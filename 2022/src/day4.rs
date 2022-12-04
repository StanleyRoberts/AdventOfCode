use std::fs;

fn part1() {
    let content = fs::read_to_string("data/day4.txt").expect("Error");
    let lines = content.lines();
    let mut overlap = 0;
    for line in lines {
        let pair: Vec<&str> = line.split(&[',', '-'][..]).collect();
        let elf1: Vec<i32> = vec![pair[0].parse::<i32>().unwrap(), pair[1].parse::<i32>().unwrap()];
        let elf2: Vec<i32> = vec![pair[2].parse::<i32>().unwrap(), pair[3].parse::<i32>().unwrap()];
        if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] ||
           elf2[0] <= elf1[0] && elf2[1] >= elf1[1] {
            overlap+= 1;
        }
    }
    println!("overlap: {overlap}");
}

fn part2() {
    let content = fs::read_to_string("data/day4.txt").expect("Error");
    let lines = content.lines();
    let mut overlap = 0;
    for line in lines {
        let pair: Vec<&str> = line.split(&[',', '-'][..]).collect();
        let elf1: Vec<i32> = vec![pair[0].parse::<i32>().unwrap(), pair[1].parse::<i32>().unwrap()];
        let elf2: Vec<i32> = vec![pair[2].parse::<i32>().unwrap(), pair[3].parse::<i32>().unwrap()];
        if elf1[1] >= elf2[0] && elf1[0] <= elf2[1]
        || elf2[1] >= elf1[0] && elf2[0] <= elf1[1] {
            overlap+= 1;
        }
    }
    println!("overlap: {overlap}");
}

pub fn main() {
    println!("\nDay 4");
    part1();
    part2();
}
