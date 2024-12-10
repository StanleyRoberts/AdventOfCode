use std::collections::HashSet;

use super::{impl_day, Part1, Part2};

impl_day!(Day10, 10);

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn trails_from(map: &[Vec<char>], (x, y): (i32, i32), ends: &mut HashSet<(i32, i32)>) -> usize {
    let val = map[x as usize][y as usize];
    if val == '9' && !ends.contains(&(x, y)) {
        ends.insert((x, y));
        return 1;
    }
    DIRS.iter()
        .map(|z| {
            let next = (x - z.0, y - z.1);
            let Some(next_val) = map
                .get(next.0 as usize)
                .and_then(|x| x.get(next.1 as usize))
            else {
                return 0;
            };
            if val as u8 + 1 == *next_val as u8 {
                trails_from(map, next, ends)
            } else {
                0
            }
        })
        .sum()
}

impl Part1 for Day10 {
    fn part1(&self, input: &str) -> usize {
        let map = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let heads = map.iter().enumerate().fold(Vec::new(), |acc, (i, x)| {
            x.iter().enumerate().fold(acc, |mut acc, (j, y)| {
                if *y == '0' {
                    acc.push((i, j));
                }
                acc
            })
        });
        heads
            .iter()
            .map(|x| trails_from(&map, (x.0 as i32, x.1 as i32), &mut HashSet::new()))
            .sum()
    }
}

fn trails_from2(map: &[Vec<char>], (x, y): (i32, i32)) -> usize {
    let val = map[x as usize][y as usize];
    if val == '9' {
        return 1;
    }
    DIRS.iter()
        .map(|z| {
            let next = (x - z.0, y - z.1);
            let Some(next_val) = map
                .get(next.0 as usize)
                .and_then(|x| x.get(next.1 as usize))
            else {
                return 0;
            };
            if val as u8 + 1 == *next_val as u8 {
                trails_from2(map, next)
            } else {
                0
            }
        })
        .sum()
}

impl Part2 for Day10 {
    fn part2(&self, input: &str) -> usize {
        let map = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let heads = map.iter().enumerate().fold(Vec::new(), |acc, (i, x)| {
            x.iter().enumerate().fold(acc, |mut acc, (j, y)| {
                if *y == '0' {
                    acc.push((i, j));
                }
                acc
            })
        });
        heads
            .iter()
            .map(|x| trails_from2(&map, (x.0 as i32, x.1 as i32)))
            .sum()
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
            Day10.part1(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            36
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day10.part2(
                "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            ),
            81
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day10.get_input();
        b.iter(|| black_box(Day10.part1(&input)));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day10.get_input();
        b.iter(|| black_box(Day10.part2(&input)));
    }
}
