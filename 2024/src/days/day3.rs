use regex::Regex;

use super::{impl_day, Part1, Part2};

impl_day!(Day3, 3);

impl Part1 for Day3 {
    fn part1(&self, input: &str) -> usize {
        let regexp = Regex::new(r"mul\((\d){1,3},(\d){1,3}\)").unwrap();
        regexp
            .find_iter(input)
            .map(|x| {
                x.as_str()
                    .replace("mul(", "")
                    .replace(")", "")
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .fold(0, |acc, x| acc + (x[0] * x[1])) as usize
    }
}

impl Part2 for Day3 {
    fn part2(&self, input: &str) -> usize {
        let regexp = Regex::new(r"mul\((\d){1,3},(\d){1,3}\)|do\(\)|don't\(\)").unwrap();
        let mut enabled = true;
        regexp
            .find_iter(input)
            .filter_map(|x| match &x.as_str()[0..3] {
                "mul" => enabled.then(|| {
                    x.as_str()
                        .replace("mul(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                }),
                x => {
                    enabled = x == "do(";
                    None
                }
            })
            .fold(0, |acc, x| acc + (x[0] * x[1])) as usize
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
            Day3.part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day3.part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
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
