use super::{Part1, Part2, impl_day};

impl_day!(Day1, 1);

impl Part1 for Day1 {
    fn part1(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|x| (x.chars().nth(0).unwrap(), x[1..].parse::<i32>().unwrap()))
            .scan(50, |pos, (dir, mag)| {
                *pos = if dir == 'L' { *pos - mag } else { *pos + mag }.rem_euclid(100);
                Some(*pos)
            })
            .filter(|x| *x == 0)
            .count() as u64
    }
}

impl Part2 for Day1 {
    fn part2(&self, input: &str) -> u64 {
        input
            .lines()
            .map(|x| (x.chars().nth(0).unwrap(), x[1..].parse::<i32>().unwrap()))
            .fold((0, 50), |(count, pos), (dir, mag)| {
                let rem = mag.rem_euclid(100);
                let mut count = count + (mag / 100) as u64;
                if pos != 0 && ((dir == 'L' && rem > pos) || (dir == 'R' && rem > 100 - pos)) {
                    count += 1;
                }
                let pos = if dir == 'L' { pos - rem } else { pos + rem }.rem_euclid(100);
                if pos == 0 {
                    count += 1;
                }
                (count, pos)
            })
            .0
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
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(Day1.part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(Day1.part2(input), 6);
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day1.get_input();
        b.iter(|| Day1.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day1.get_input();
        b.iter(|| Day1.part2(&input));
    }
}
