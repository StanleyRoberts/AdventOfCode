use std::collections::HashSet;

use super::{Part1, Part2, impl_day};

impl_day!(Day5, 5);

impl Part1 for Day5 {
    fn part1(&self, input: &str) -> u64 {
        let mut split = input.split("\n\n");
        let fresh_ranges = split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut nums = l.split('-');
                nums.next().unwrap().parse::<u64>().unwrap()
                    ..=nums.next().unwrap().parse::<u64>().unwrap()
            })
            .collect::<Vec<_>>();
        split
            .next()
            .unwrap()
            .lines()
            .map(|x| x.parse::<u64>().unwrap())
            .filter(|i| fresh_ranges.iter().any(|range| range.contains(i)))
            .count() as u64
    }
}

impl Part2 for Day5 {
    fn part2(&self, input: &str) -> u64 {
        let mut min_ranges: HashSet<(u64, u64)> = HashSet::new();
        for mut new in input.split("\n\n").next().unwrap().lines().map(|l| {
            let mut nums = l.split('-');
            (
                nums.next().unwrap().parse::<u64>().unwrap(),
                nums.next().unwrap().parse::<u64>().unwrap(),
            )
        }) {
            for existing in min_ranges.clone() {
                if new.0 >= existing.0 && new.0 <= existing.1 {
                    new.0 = existing.1 + 1
                }
                if new.1 >= existing.0 && new.1 <= existing.1 {
                    new.1 = existing.0 - 1
                }
                if new.0 < existing.0 && new.1 > existing.1 {
                    min_ranges.remove(&existing);
                }
            }
            if (new.1 as i64 - new.0 as i64) >= 0 {
                min_ranges.insert((new.0, new.1));
            }
        }
        min_ranges.into_iter().map(|x| x.1 - x.0 + 1).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use crate::day::DayMeta;
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day5.part1(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            3
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day5.part2(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            14
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day5.get_input();
        b.iter(|| Day5.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day5.get_input();
        b.iter(|| Day5.part2(&input));
    }
}
