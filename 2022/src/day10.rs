use std::fs;
use std::collections::VecDeque;

macro_rules! pop {
    ($x:expr) => {
        {
        let ret = match $x.pop_front() {
            Some(x) => x,
            None => 0,
        };
        ret
        }
    };
}

fn part1() {
    let content = fs::read_to_string("data/day10.txt").unwrap();
    let lines = content.lines();
    let checks = vec![20, 60, 100, 140, 180, 220];
    let mut x = 1;
    let mut cycle = 0;
    let mut sum = 0;
    for line in lines {
        cycle += 1;
        if checks.contains(&cycle) {
            sum += cycle*x;
        }
        let comm:Vec<&str> = line.split(' ').collect();
        if comm[0] == "addx" {
            cycle += 1;
            if checks.contains(&cycle) {
                sum += cycle*x;
            }
            x += comm[1].parse::<i32>().unwrap();
        }
    }
    println!("part1: {sum}");
}

fn part2() {
    let content = fs::read_to_string("data/day10.txt").unwrap();
    let lines = content.lines();
    let mut x = 1;
    let mut cycle = 0;
    let mut queue = VecDeque::new();
    let mut out = String::new();
    for line in lines {
        let comm:Vec<&str> = line.split(' ').collect();
        if comm[0] == "addx" {
            let val = pop!(queue);
            queue.push_back(comm[1].parse::<i32>().unwrap());
            out.push_str(&printer(x, cycle));
            cycle += 1;
            x += val;
        }
        let val = pop!(queue);
        out.push_str(&printer(x, cycle));
        cycle += 1;
        x += val;
        
    }
    println!("part2:\n{out}")
}

fn printer(x: i32, cycle: i32) -> String {
    let pos = cycle%40;
    let mut conc = String::new();
    if pos==x || pos==x+1 || pos==x-1 {
        conc.push_str("#");
    } else {
        conc.push_str(".")
    }
    if (cycle+1)%40==0{
        conc.push_str("\n");
    }
    return conc;
}

pub fn main() {
    println!("\nDay ");
    part1();
    part2();
}