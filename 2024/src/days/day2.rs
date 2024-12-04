use super::{impl_day, Part1, Part2};

impl_day!(Day2, 2);

impl Part1 for Day2 {
    fn part1(&self, input: &str) -> usize {
        input
            .lines()
            .map(|x| {
                let parsed = x.split(' ').map(|x| x.parse::<i32>().unwrap());
                let mut diff = parsed.clone().zip(parsed.skip(1)).map(|(x, y)| y - x);
                diff.clone().all(|x| (-3..0).contains(&x)) || diff.all(|x| (1..4).contains(&x))
            })
            .filter(|x| *x)
            .count()
    }
}

impl Part2 for Day2 {
    fn part2(&self, input: &str) -> usize {
        fn check_line(iter: impl Iterator<Item = i32> + Clone) -> bool {
            let mut diff = iter.clone().zip(iter.skip(1)).map(|(x, y)| y - x);
            diff.clone().all(|x| (-3..0).contains(&x)) || diff.all(|x| (1..4).contains(&x))
        }
        input
            .lines()
            .map(|x| {
                let parsed = x.split(' ').map(|x| x.parse::<i32>().unwrap());
                let line = parsed.clone().collect::<Vec<_>>();
                !(!check_line(parsed))
                    .then(|| {
                        !(0..line.len()).any(|i| {
                            check_line(
                                line.iter()
                                    .enumerate()
                                    .filter(|(j, _)| i != *j)
                                    .map(|(_, x)| x)
                                    .cloned(),
                            )
                        })
                    })
                    .is_some_and(|x| x)
            })
            .filter(|x| *x)
            .count()
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
            Day2.part1(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day2.part2(
                "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            ),
            4
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day2.get_input();
        b.iter(|| Day2.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day2.get_input();
        b.iter(|| Day2.part2(&input));
    }
}
