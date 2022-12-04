use std::fs;

fn part1() {
    let content = fs::read_to_string("data/day4.txt").expect("Error");
    let lines = content.lines();
    let mut overlap = 0;
    for line in lines {
        let pair = line.split(&[',', '-']);
        let list: Vec<i32> = pair.map(|x| x.parse::<i32>().unwrap()).collect();
        if list[0] <= list[2] && list[1] >= list[3] ||
           list[2] <= list[0] && list[3] >= list[1] {
            overlap+= 1;
        }
    }
    println!("part1: {overlap}");
}

fn part2() {
    let content = fs::read_to_string("data/day4.txt").expect("Error");
    let lines = content.lines();
    let mut overlap = 0;
    for line in lines {
        let pair = line.split(&[',', '-']);
        let list: Vec<i32> = pair.map(|x| x.parse::<i32>().unwrap()).collect();
        if list[1] >= list[2] && list[0] <= list[3]
        || list[3] >= list[0] && list[2] <= list[1] {
            overlap+= 1;
        }
    }
    println!("part2: {overlap}");
}

pub fn main() {
    println!("\nDay 4");
    part1();
    part2();
}
