use std::fs;
use std::collections::HashSet;

fn to_int(char: char) -> i32 {
    let mut val = char as i32;
    if val > 96 {
        val -= 96;
    } else {
        val -= 38;
    }
    return val;
}

fn part1() {
    let content = fs::read_to_string("data/day3.txt").expect("Error");
    let lines = content.lines();
    let mut total = 0;
    for line in lines {
        let mut set: HashSet<i32> = HashSet::new();
        let comp2 = &line[line.len()/2 .. line.len()];
        for char in (&line[0 .. line.len()/2]).chars() {
            if comp2.contains(char) {
                set.insert(to_int(char));
            }
        }
        for val in set{
            total += val;
        }
    }
    println!("total: {total}");
}

fn part2() {
    let content = fs::read_to_string("data/day3.txt").expect("Error");
    let lines: Vec<&str> = content.split('\n').collect();
    let mut i = 0;
    let mut total = 0;
    loop {
        if i >= lines.len() {
            break;
        }

        let elf1: HashSet<char> = lines[i+0].trim().chars().collect();
        let elf2: HashSet<char> = lines[i+1].trim().chars().collect();
        let elf3: HashSet<char> = lines[i+2].trim().chars().collect();
        for x in elf1.iter().filter(|k| elf2.contains(k)).filter(|k| elf3.contains(k)) {
            total += to_int(*x);
        }

        i += 3;
    }
    println!("total: {total}")
}

pub fn main() {
    println!("\nDay 3");
    part1();
    part2();
}
