use super::{impl_day, Part1, Part2};

impl_day!(Day1, 1);

impl Part1 for Day1 {
    fn part1(&self, input: &str) -> usize {
        let (mut left, mut right): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|x| x.split("   ").map(|x| x.parse::<i32>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .unzip();
        left.sort();
        right.sort();
        left.iter()
            .zip(right.iter())
            .fold(0, |acc, (x, y)| acc + (x - y).abs()) as usize
    }
}

impl Part2 for Day1 {
    fn part2(&self, input: &str) -> usize {
        let (left, right): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|x| x.split("   ").map(|x| x.parse::<usize>().unwrap()))
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .unzip();
        left.iter().fold(0, |acc, x| {
            acc + x * (right.iter().filter(|y| *y == x).count())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day1.part1(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            11
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day1.part2(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            ),
            31
        );
    }
}
