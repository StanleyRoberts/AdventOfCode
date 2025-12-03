use super::{Part1, Part2, impl_day};

impl_day!(Day3, 3);

impl Part1 for Day3 {
    fn part1(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|x| {
                let (idx, l) = x[..(x.len() - 1)]
                    .chars()
                    .map(|c| c as u32 - 0x30)
                    .enumerate()
                    .fold((0, 0), |mut acc, (idx, e)| {
                        if acc.1 < e {
                            acc = (idx, e)
                        }
                        acc
                    });
                let r = x[(idx + 1)..]
                    .chars()
                    .map(|c| c as u32 - 0x30)
                    .max()
                    .unwrap();
                (l * 10 + r) as u64
            })
            .sum()
    }
}

impl Part2 for Day3 {
    fn part2(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|l| {
                (0..12)
                    .rev()
                    .scan(0, |rightmost, i| {
                        let (idx, v) = l[..(l.len() - (i))]
                            .chars()
                            .map(|x| x as u32 - 0x30)
                            .enumerate()
                            .skip(*rightmost)
                            .fold((0, 0), |mut acc, (idx, e)| {
                                if acc.1 < e {
                                    acc = (idx, e)
                                }
                                acc
                            });
                        *rightmost = idx + 1;
                        Some(v)
                    })
                    .enumerate()
                    .fold(0, |acc, (idx, e)| {
                        acc + (e as u64 * 10_u64.pow(11 - idx as u32))
                    })
            })
            .sum()
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
            Day3.part1(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            357
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day3.part2(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            3121910778619
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day3.get_input();
        b.iter(|| Day3.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day3.get_input();
        b.iter(|| Day3.part2(&input));
    }
}
