use std::collections::HashMap;

use super::{impl_day, Part1, Part2};

impl_day!(Day11, 11);

fn blink(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    stones.iter().fold(HashMap::new(), |mut acc, (&key, &val)| {
        if key == 0 {
            acc.entry(1).and_modify(|y| *y += val).or_insert(val);
        } else if ((key.ilog10() + 1) % 2) == 0 {
            let x = key.to_string();
            acc.entry(x[0..x.len() / 2].parse::<i64>().unwrap())
                .and_modify(|y| *y += val)
                .or_insert(val);
            acc.entry(x[(x.len() / 2)..x.len()].parse::<i64>().unwrap())
                .and_modify(|y| *y += val)
                .or_insert(val);
        } else {
            acc.entry(key * 2024)
                .and_modify(|y| *y += val)
                .or_insert(val);
        }
        acc
    })
}

impl Part1 for Day11 {
    fn part1(&self, input: &str) -> usize {
        let mut stones = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .fold(HashMap::new(), |mut acc, x| {
                acc.entry(x).and_modify(|y| *y += 1).or_insert(1);
                acc
            });
        for _ in 0..25 {
            stones = blink(stones);
        }
        stones.into_values().sum::<i64>() as usize
    }
}

impl Part2 for Day11 {
    fn part2(&self, input: &str) -> usize {
        let mut stones = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .fold(HashMap::new(), |mut acc, x| {
                acc.entry(x).and_modify(|y| *y += 1).or_insert(1);
                acc
            });
        for _ in 0..75 {
            stones = blink(stones);
        }
        stones.into_values().sum::<i64>() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use crate::day::DayMeta;
    #[cfg(feature = "nightly")]
    use std::hint::black_box;
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(Day11.part1("125 17"), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day11.part2("125 17"), 65601038650482);
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day11.get_input();
        b.iter(|| black_box(Day11.part1(&input)));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day11.get_input();
        b.iter(|| black_box(Day11.part2(&input)));
    }
}
