use std::fs;
use std::collections::HashSet;
use scan_fmt::scan_fmt;

macro_rules! cmp {
    ($x:expr, $y:expr) => {
        {
        let mut ret = 0;
        if $x > $y { ret = 1 }
        else if $y > $x { ret = -1 }
        ret
        }
    };
}

fn part1() {
    let content = fs::read_to_string("data/day9.txt").unwrap();
    let lines = content.lines();
    let (mut i, mut j) = (0, 0); //head
    let (mut x, mut y) = (0, 0); //tail
    let mut visited = HashSet::new();
    visited.insert((x, y));

    for line in lines {
        let (instr, num) = scan_fmt!(line, "{} {d}", char, i32).unwrap();
        for _ in 0..num {
            match instr {
                'R' => i+=1,
                'L' => i-=1,
                'U' => j+=1,
                'D' => j-=1,
                 _ => (),
            }
            let col = x+cmp!(i, x)==i && y+cmp!(j, y)==j;
            (x, y) = (if col {x} else {x+cmp!(i, x)}, if col {y} else {y+cmp!(j, y)});
            visited.insert((x, y));
        }
    }
    println!("part1: {}", visited.len());
}

fn part2() {
    let content = fs::read_to_string("data/day9.txt").unwrap();
    let lines = content.lines();
    let mut snake = vec![(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    for line in lines {
        let (instr, num) = scan_fmt!(line, "{} {d}", char, i32).unwrap();
        for _ in 0..num {
            for k in 0..snake.len() {
                if k==0 { 
                    match instr {
                        'R' => snake[0].0+=1,
                        'L' => snake[0].0-=1,
                        'U' => snake[0].1+=1,
                        'D' => snake[0].1-=1,
                         _ => (),
                    }
                } else {
                    let (x, y) = snake[k];
                    let (i, j) = snake[k-1];
                    let col = x+cmp!(i, x)==i && y+cmp!(j, y)==j;
                    snake[k] = (if col {x} else {x+cmp!(i, x)}, if col {y} else {y+cmp!(j, y)});
                }
            }
            visited.insert(snake[9]);
        }
    }
    println!("part1: {}", visited.len());
}

pub fn main() {
    println!("\nDay ");
    part1();
    part2();
}