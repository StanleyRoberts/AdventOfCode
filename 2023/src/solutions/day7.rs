use std::cmp::{min, Ordering};

#[inline]
fn get_hand_val(hand: &str, ptwo: bool) -> i32 {
    let mut sort = hand.chars().collect::<Vec<char>>();
    let jokers = if ptwo {
        i32::try_from(sort.iter().filter(|x| *x == &'J').count()).unwrap()
    } else {
        0
    };
    sort.sort_unstable();
    if sort[0] == sort[4] {
        return 6;
    } else if sort[0] == sort[3] || sort[1] == sort[4] {
        return 5 + min(1, jokers);
    } else if (sort[0] == sort[1] && sort[2] == sort[4])
        || (sort[3] == sort[4] && sort[0] == sort[2])
    {
        return 4 + if jokers == 0 { 0 } else { 2 };
    } else if sort[0] == sort[2] || sort[1] == sort[3] || sort[2] == sort[4] {
        return 3 + if jokers == 0 { 0 } else { 2 };
    }
    sort.dedup();
    match sort.len() {
        5 => jokers,
        4 => 1 + if jokers == 0 { 0 } else { 2 },
        3 => {
            2 + if jokers == 0 {
                0
            } else if jokers == 1 {
                2
            } else {
                3
            }
        }
        _ => unreachable!(),
    }
}

#[inline]
fn score_sorted(mut sorted: Vec<(i32, String, i32)>) -> i32 {
    sorted.sort_by(|a, b| {
        if a.0.partial_cmp(&b.0) == Some(Ordering::Equal) {
            a.1.partial_cmp(&b.1).unwrap()
        } else {
            a.0.partial_cmp(&b.0).unwrap()
        }
    });
    sorted.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + ((i32::try_from(i + 1).unwrap()) * x.2)
    })
}

#[inline]
fn parse<I: Iterator<Item = String>>(
    input: I,
    map: [(char, char); 5],
    ptwo: bool,
) -> Vec<(i32, String, i32)> {
    input
        .map(|x| x.split(' ').map(std::string::ToString::to_string).collect())
        .fold(Vec::new(), |mut acc, x: Vec<String>| {
            acc.push((
                get_hand_val(&x[0], ptwo),
                map.iter()
                    .fold(x[0].clone(), |acc, y| acc.replace(y.0, &y.1.to_string())),
                x[1].parse::<i32>().unwrap(),
            ));
            acc
        })
}

fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    let map = [('A', 'e'), ('K', 'd'), ('Q', 'c'), ('J', 'b'), ('T', 'a')];
    score_sorted(parse(input, map, false))
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    let map = [('A', 'e'), ('K', 'd'), ('Q', 'c'), ('J', '!'), ('T', 'a')];
    score_sorted(parse(input, map, true))
}

pub fn day7() {
    println!("[Day 7] Part 1: {}", part1(read_input!("day7.txt")));
    println!("[Day 7] Part 2: {}", part2(read_input!("day7.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 6440);
    }

    #[test]
    fn sample_part2() {
        let sample = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 5905);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day7.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day7.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
