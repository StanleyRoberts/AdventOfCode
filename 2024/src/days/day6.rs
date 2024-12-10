use std::collections::HashSet;

use super::{impl_day, Part1, Part2};

impl_day!(Day6, 6);

impl Part1 for Day6 {
    fn part1(&self, input: &str) -> usize {
        let map = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut current = map
            .iter()
            .enumerate()
            .find_map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .find_map(|(j, &x)| (x == '^').then_some((i as i32, j as i32)))
            })
            .unwrap();
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut idx = 0;
        let mut visited = HashSet::new();
        visited.insert(current);
        while let Some(new) = map
            .get((current.0 + dirs[idx].0) as usize)
            .and_then(|x| x.get((current.1 + dirs[idx].1) as usize))
        {
            if *new == '#' {
                idx = (idx + 1) % 4;
            } else {
                let cur = ((current.0 + dirs[idx].0), (current.1 + dirs[idx].1));
                current = cur;
                visited.insert(cur);
            }
        }
        visited.len()
    }
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_looper(map: &[Vec<char>], start: (i32, i32), idx: usize) -> bool {
    let mut current = start;
    let mut idx = idx;
    let mut visited = HashSet::new();
    visited.insert((current, DIRS[idx]));
    while let Some(new) = map
        .get((current.0 + DIRS[idx].0) as usize)
        .and_then(|x| x.get((current.1 + DIRS[idx].1) as usize))
    {
        let cur = ((current.0 + DIRS[idx].0), (current.1 + DIRS[idx].1));
        if visited.contains(&(cur, DIRS[idx])) {
            return true;
        }
        if *new == '#' {
            idx = (idx + 1) % 4;
        } else {
            current = cur;
            visited.insert((cur, DIRS[idx]));
        }
    }
    false
}

impl Part2 for Day6 {
    fn part2(&self, input: &str) -> usize {
        let mut map = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut cur = map
            .iter()
            .enumerate()
            .find_map(|(i, x)| {
                x.iter()
                    .enumerate()
                    .find_map(|(j, &x)| (x == '^').then_some((i as i32, j as i32)))
            })
            .unwrap();
        let mut idx = 0;
        let mut visited = HashSet::new();
        visited.insert(cur);
        let mut count = 0;
        while let Some(new) = map
            .get((cur.0 + DIRS[idx].0) as usize)
            .and_then(|x| x.get((cur.1 + DIRS[idx].1) as usize))
        {
            if *new == '#' {
                idx = (idx + 1) % 4;
            } else {
                cur = ((cur.0 + DIRS[idx].0), (cur.1 + DIRS[idx].1));
                if visited.insert(cur) {
                    map[cur.0 as usize][cur.1 as usize] = '#';
                    count += is_looper(&map, ((cur.0 - DIRS[idx].0), (cur.1 - DIRS[idx].1)), idx)
                        as usize;
                    map[cur.0 as usize][cur.1 as usize] = '.';
                }
            }
        }
        count
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
            Day6.part1(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            41
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day6.part2(
                "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            ),
            6
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day6.get_input();
        b.iter(|| black_box(Day6.part1(&input)));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    #[ignore]
    fn bench_part2(b: &mut Bencher) {
        let input = Day6.get_input();
        b.iter(|| black_box(Day6.part2(&input)));
    }
}
