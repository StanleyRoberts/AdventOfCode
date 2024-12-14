use super::{impl_day, Part1, Part2};

impl_day!(Day14, 14);

impl Part1 for Day14 {
    fn part1(&self, input: &str) -> usize {
        const SECONDS: i32 = 100;
        const HEIGHT: i32 = if cfg!(test) { 7 } else { 103 };
        const WIDTH: i32 = if cfg!(test) { 11 } else { 101 };
        let quads = input
            .lines()
            .map(|x| x.split_ascii_whitespace())
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .map(|(x, y)| {
                let mut pos = x
                    .strip_prefix("p=")
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap());
                let mut vel = y
                    .strip_prefix("v=")
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap());
                (
                    (pos.next().unwrap(), pos.next().unwrap()),
                    (vel.next().unwrap(), vel.next().unwrap()),
                )
            })
            .map(|(pos, vel)| {
                (
                    (pos.0 + (vel.0 * SECONDS)).rem_euclid(WIDTH),
                    (pos.1 + (vel.1 * SECONDS)).rem_euclid(HEIGHT),
                )
            })
            .fold((0, 0, 0, 0), |mut acc, x| {
                match x.0.cmp(&(WIDTH / 2)) {
                    std::cmp::Ordering::Less => match x.1.cmp(&(HEIGHT / 2)) {
                        std::cmp::Ordering::Less => acc.0 += 1,
                        std::cmp::Ordering::Greater => acc.1 += 1,
                        std::cmp::Ordering::Equal => (),
                    },
                    std::cmp::Ordering::Greater => match x.1.cmp(&(HEIGHT / 2)) {
                        std::cmp::Ordering::Less => acc.2 += 1,
                        std::cmp::Ordering::Greater => acc.3 += 1,
                        std::cmp::Ordering::Equal => (),
                    },
                    std::cmp::Ordering::Equal => (),
                }
                acc
            });
        quads.0 * quads.1 * quads.2 * quads.3
    }
}

impl Part2 for Day14 {
    fn part2(&self, input: &str) -> usize {
        const HEIGHT: i32 = if cfg!(test) { 7 } else { 103 };
        const WIDTH: i32 = if cfg!(test) { 11 } else { 101 };
        const MAX_VAR: i32 = 500;
        type Robots = [((i32, i32), (i32, i32))];

        fn step(robots: &mut Robots) {
            for robot in robots {
                robot.0 .0 = (robot.0 .0 + robot.1 .0).rem_euclid(WIDTH);
                robot.0 .1 = (robot.0 .1 + robot.1 .1).rem_euclid(HEIGHT);
            }
        }

        let mut robots = input
            .lines()
            .map(|x| x.split_ascii_whitespace())
            .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
            .map(|(x, y)| {
                let mut pos = x
                    .strip_prefix("p=")
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap());
                let mut vel = y
                    .strip_prefix("v=")
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<i32>().unwrap());
                (
                    (pos.next().unwrap(), pos.next().unwrap()),
                    (vel.next().unwrap(), vel.next().unwrap()),
                )
            })
            .collect::<Vec<_>>();
        let mut i = 0;
        loop {
            i += 1;
            step(&mut robots);
            let (x_sum, y_sum, x_sum_sq, y_sum_sq) =
                robots
                    .iter()
                    .fold((0, 0, 0, 0), |(x, y, x_sq, y_sq), (pos, _)| {
                        (
                            x + pos.0,
                            y + pos.1,
                            x_sq + (pos.0 * pos.0),
                            y_sq + (pos.1 * pos.1),
                        )
                    });
            let (x_var, y_var) = (
                (x_sum_sq - (x_sum * x_sum) / robots.len() as i32) / robots.len() as i32,
                (y_sum_sq - (y_sum * y_sum) / robots.len() as i32) / robots.len() as i32,
            );
            if x_var < MAX_VAR && y_var < MAX_VAR {
                return i;
            }
        }
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
        assert_eq!(
            Day14.part1(
                "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"
            ),
            12
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day14.get_input();
        b.iter(|| black_box(Day14.part1(&input)));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day14.get_input();
        b.iter(|| black_box(Day14.part2(&input)));
    }
}
