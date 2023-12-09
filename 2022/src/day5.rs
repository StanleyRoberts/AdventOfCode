use std::fs;
use itertools::Itertools;
use scan_fmt::scan_fmt;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<&str>) {
    let (crates, instrs) = match input.split("\n\n").into_iter().collect_tuple() {
        Some(x) => x,
        None => input.split("\n\r\n").into_iter().collect_tuple().unwrap(),
    };
    let lines = crates.split('\n').collect::<Vec<&str>>().into_iter().rev().collect::<Vec<&str>>();
    let mut stacklist: Vec<Vec<char>> = vec![Vec::new();1+(lines[0].len()/4)];

    for row in lines[1..].into_iter() {
        for (i, item) in row.chars().skip(1).step_by(4).enumerate() {
            if item.is_alphabetic() {
                stacklist[i].push(item);
            }
        }
    }
    return (stacklist, instrs.split('\n').collect::<Vec<&str>>());
}

fn part1() {
    let content: &str = &fs::read_to_string("data/day5.txt").unwrap();
    let (mut stacklist, instrs) = parse(content);

    for instr in instrs {
        let (num, src, dest) = scan_fmt!(instr, "move {d} from {d} to {d}", i32, usize, usize).unwrap();
        for _ in 0..num {
            let val = stacklist[src-1].pop().unwrap();
            stacklist[dest-1].push(val);
        }
    }
    print!("part1: ");
    for v in stacklist{ print!("{}", v[v.len()-1]); }
    println!();
}

fn part2() {
    let content: &str = &fs::read_to_string("data/day5.txt").unwrap();
    let (mut stacklist, instrs) = parse(content);

    for instr in instrs {
        let (num, src, dest) = scan_fmt!(instr, "move {d} from {d} to {d}", i32, usize, usize).unwrap();
        let val: Vec<char> = (0..num).map(|_| stacklist[src-1].pop().unwrap()).collect();
        for v in val.into_iter().rev() {
            stacklist[dest-1].push(v);
        }
    }
    print!("part2: ");
    for v in stacklist{ print!("{}", v[v.len()-1]); }
    println!();
}

pub fn main() {
    println!("\nDay 5");
    part1();
    part2();
}