use std::fs;

fn part1() {
    let content = fs::read_to_string("data/day1.txt").expect("Error");
    let branches = content.lines();
    let mut var = 0;
    let mut max = 0;

    for s in branches {
        if s == "" {
            var = 0;
        } else {
            var += s.trim().parse::<i32>().unwrap();
        }
        if var > max {
            max = var;
        }
    }
    println!("part1: {max}");
}

fn set_vec(vec: &mut Vec<i32>, var: i32) {
    let min = *vec.iter().min().unwrap();
    for i in 0..vec.len() {
        if var>vec[i] && vec[i]==min{
            vec[i] = var;
            break;
        }
    }
}

fn part2() {
    let content = fs::read_to_string("data/day1.txt").expect("Error");
    let branches = content.lines();
    let mut var = 0;
    let mut max:Vec<i32> = vec![0, 0, 0];

    for s in branches {
        if s == "" {
            set_vec(&mut max, var);
            var = 0;
        } else {
            var += s.trim().parse::<i32>().unwrap();
        }
    }
    set_vec(&mut max, var);
    for i in 1..max.len() {
        max[0] += max[i];
    }
    println!("part2: {}", max[0])
}

pub fn main() {
    println!("\nDay 1");
    part1();
    part2();
}