use std::fs;

fn get_val(num: &str) -> i32 {
    return match num {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

fn part1() {
    let content = fs::read_to_string("data/day2.txt").expect("Error");
    let lines = content.lines();
    let mut score = 0;
    for line in lines{
        let pair: Vec<&str> = line.split(" ").collect();
        let opp = get_val(pair[0].trim());
        let us = get_val(pair[1].trim());
        if opp == us {
            score += 3
        } else if ((opp+1)%3)+1 == us {
            score += 0
        } else {
            score += 6
        }
        score += us;
    }
    println!("part1: {score}")
}

fn part2() {
    let content = fs::read_to_string("data/day2.txt").expect("Error");
    let lines = content.lines();
    let mut score = 0;
    for line in lines{
        let pair: Vec<&str> = line.split(" ").collect();
        let opp = get_val(pair[0].trim());
        if pair[1] == "X" {
            score += ((opp+1)%3)+1;
        } else if pair[1] == "Y" {
            score += 3;
            score += opp;
        } else {
            score += 6;
            score += ((opp)%3)+1;
        }
    }
    println!("part2: {score}")

}

pub fn main() {
    println!("\nDay 2");
    part1();
    part2();
}
