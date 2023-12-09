use std::collections::HashMap;

fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    (a * b) / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn part1<I: Iterator<Item = String>>(mut input: I) -> i32 {
    let instrs = input.next().unwrap();
    let map = input
        .skip(1)
        .map(|x| {
            let binding = x.replace(['=', ',', '(', ')'], "");
            let tup: Vec<&str> = binding.split(' ').collect();
            (tup[0].to_owned(), (tup[2].to_owned(), tup[3].to_owned()))
        })
        .collect::<HashMap<String, (String, String)>>();
    let mut count = 0;
    let mut curr = "AAA";
    loop {
        for char in instrs.chars() {
            if curr == "ZZZ" {
                return count;
            }
            if char == 'L' {
                curr = &map[curr].0;
            } else {
                curr = &map[curr].1;
            }
            count += 1;
        }
    }
}

fn part2<I: Iterator<Item = String>>(mut input: I) -> i64 {
    let instrs = input.next().unwrap();
    let map = input
        .skip(1)
        .map(|x| {
            let binding = x.replace(['=', ',', '(', ')'], "");
            let tup: Vec<&str> = binding.split(' ').collect();
            (tup[0].to_owned(), (tup[2].to_owned(), tup[3].to_owned()))
        })
        .collect::<HashMap<String, (String, String)>>();
    let mut curr: Vec<&String> = map
        .keys()
        .filter(|x| x.chars().nth(2).unwrap() == 'A')
        .collect();
    let mut counts = vec![0; curr.len()];
    for i in 0..curr.len() {
        while curr[i].chars().nth(2).unwrap() != 'Z' {
            for char in instrs.chars() {
                if curr[i].chars().nth(2).unwrap() == 'Z' {
                    break;
                }
                if char == 'L' {
                    curr[i] = &map[curr[i]].0;
                } else {
                    curr[i] = &map[curr[i]].1;
                }
                counts[i] += 1;
            }
        }
    }
    lcm(&counts)
}

pub fn day8() {
    println!("[Day 8] Part 1: {}", part1(read_input!("day8.txt")));
    println!("[Day 8] Part 2: {}", part2(read_input!("day8.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 2);

        let sample = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 6);
    }

    #[test]
    fn sample_part2() {
        let sample = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 6);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day8.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day8.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
