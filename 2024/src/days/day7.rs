use std::ops;

use super::{impl_day, Part1, Part2};

impl_day!(Day7, 7);

impl Part1 for Day7 {
    fn part1(&self, input: &str) -> usize {
        input
            .lines()
            .map(|x| x.split(':'))
            .map(|mut x| {
                (
                    x.next().unwrap().parse::<i64>().unwrap(),
                    x.next()
                        .unwrap()
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(total, eq)| {
                for check in 0..(2_i32.pow(eq.len() as u32)) {
                    if *total
                        == eq
                            .iter()
                            .copied()
                            .enumerate()
                            .reduce(|(_, acc), (i, x)| match check >> i & 1 {
                                0 => (0, (acc + x)),
                                1 => (0, (acc * x)),
                                _ => unreachable!(),
                            })
                            .unwrap()
                            .1
                    {
                        return true;
                    }
                }
                false
            })
            .map(|x| x.0)
            .reduce(|acc, x| (acc + x))
            .unwrap() as usize
    }
}

struct TernaryIncrementor(pub Vec<u8>);

impl ops::Add<i32> for TernaryIncrementor {
    type Output = Option<Self>;

    fn add(self, _rhs: i32) -> Self::Output {
        let mut ctr = (self.0.len() - 1) as i32;
        let mut new = self.0.clone();
        while ctr >= 0 {
            match self.0[ctr as usize] {
                0 => {
                    new[ctr as usize] = 1;
                    return Some(Self(new));
                }
                1 => {
                    new[ctr as usize] = 2;
                    return Some(Self(new));
                }
                2 => {
                    let mut rev = ctr;
                    while (rev as usize) < new.len() {
                        new[rev as usize] = 0;
                        rev += 1;
                    }
                }
                _ => unreachable!(),
            }
            ctr -= 1;
        }
        None
    }
}

impl Part2 for Day7 {
    fn part2(&self, input: &str) -> usize {
        input
            .lines()
            .map(|x| x.split(':'))
            .map(|mut x| {
                (
                    x.next().unwrap().parse::<i64>().unwrap(),
                    x.next()
                        .unwrap()
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(total, eq)| {
                let mut orders = Some(TernaryIncrementor(vec![0; eq.len()]));
                while let Some(check) = orders {
                    if *total
                        == eq
                            .iter()
                            .copied()
                            .enumerate()
                            .reduce(|(_, acc), (i, x)| match check.0[i] {
                                0 => (0, (acc + x)),
                                1 => (0, (acc * x)),
                                2 => (0, x + acc * 10_i32.pow(1 + x.ilog10()) as i64),
                                _ => unreachable!(),
                            })
                            .unwrap()
                            .1
                    {
                        return true;
                    }
                    orders = check + 1;
                }
                false
            })
            .map(|x| x.0)
            .reduce(|acc, x| (acc + x))
            .unwrap() as usize
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
            Day7.part1(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            3749
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day7.part2(
                "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            ),
            11387
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day7.get_input();
        b.iter(|| Day7.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    #[ignore]
    fn bench_part2(b: &mut Bencher) {
        let input = Day7.get_input();
        b.iter(|| Day7.part2(&input));
    }
}
