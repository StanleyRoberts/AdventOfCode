use std::collections::{HashMap, HashSet};

use super::{impl_day, Part1, Part2};

impl_day!(Day5, 5);

impl Part1 for Day5 {
    fn part1(&self, input: &str) -> usize {
        let (first, second) = match &input.split("\n\n").collect::<Vec<_>>()[..] {
            &[a, b, ..] => (a, b),
            _ => unreachable!(),
        };
        let lookup = first
            .lines()
            .map(|x| x.split('|').map(|x| x.parse::<i32>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .fold(HashMap::new(), |mut acc, (before, after)| {
                acc.entry(after)
                    .and_modify(|x: &mut HashSet<i32>| {
                        x.insert(before);
                    })
                    .or_insert(HashSet::from([before]));
                acc
            });
        second
            .lines()
            .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap()))
            .filter_map(|x| {
                let mut seen = HashSet::new();
                let line = x.collect::<Vec<_>>();
                line.iter()
                    .rev()
                    .all(|&x| {
                        seen.insert(x);
                        lookup
                            .get(&x)
                            .is_none_or(|x| x.intersection(&seen).count() == 0)
                    })
                    .then_some(line)
            })
            .fold(0, |acc, x| acc + x[x.len() / 2]) as usize
    }
}

impl Part2 for Day5 {
    fn part2(&self, input: &str) -> usize {
        let (first, second) = match &input.split("\n\n").collect::<Vec<_>>()[..] {
            &[a, b, ..] => (a, b),
            _ => unreachable!(),
        };
        let lookup = first
            .lines()
            .map(|x| x.split('|').map(|x| x.parse::<i32>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .fold(HashMap::new(), |mut acc, (before, after)| {
                acc.entry(after)
                    .and_modify(|x: &mut HashSet<i32>| {
                        x.insert(before);
                    })
                    .or_insert(HashSet::from([before]));
                acc
            });
        second
            .lines()
            .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap()))
            .filter_map(|x| {
                let mut seen = HashSet::new();
                let line = x.collect::<Vec<_>>();
                (!line.iter().rev().all(|&x| {
                    seen.insert(x);
                    lookup
                        .get(&x)
                        .is_none_or(|x| x.intersection(&seen).count() == 0)
                }))
                .then_some(line)
            })
            .map(|mut x| {
                let mut remaining = HashSet::from_iter(x.clone());
                x.iter_mut().for_each(|x| {
                    let next = *remaining
                        .iter()
                        .find(|x| {
                            lookup
                                .get(x)
                                .is_none_or(|x| x.intersection(&remaining).count() == 0)
                        })
                        .unwrap();
                    remaining.remove(&next);
                    *x = next
                });
                x
            })
            .fold(0, |acc, x| acc + x[x.len() / 2]) as usize
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
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            143
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day5.part2(
                "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            ),
            123
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
