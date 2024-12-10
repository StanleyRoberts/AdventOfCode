use std::collections::{HashMap, HashSet};

use super::{impl_day, Part1, Part2};

impl_day!(Day8, 8);

impl Part1 for Day8 {
    fn part1(&self, input: &str) -> usize {
        let city = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        city.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, line)| {
                line.iter().enumerate().for_each(|(j, ch)| {
                    if *ch != '.' {
                        acc.entry(ch)
                            .and_modify(|vec: &mut Vec<(i32, i32)>| vec.push((i as i32, j as i32)))
                            .or_insert(vec![(i as i32, j as i32)]);
                    }
                });
                acc
            })
            .iter()
            .fold(HashSet::new(), |mut acc, (_ch, vec)| {
                for (i, (a, b)) in vec.iter().enumerate() {
                    for (x, y) in vec.iter().skip(i + 1) {
                        let diff = (x - a, y - b);
                        acc.insert((a - diff.0, b - diff.1));
                        acc.insert((x + diff.0, y + diff.1));
                    }
                }
                acc
            })
            .iter()
            .filter(|(x, y)| {
                *x < city.len() as i32 && *y < city[0].len() as i32 && *x >= 0 && *y >= 0
            })
            .count()
    }
}

impl Part2 for Day8 {
    fn part2(&self, input: &str) -> usize {
        let city = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        city.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, line)| {
                line.iter().enumerate().for_each(|(j, ch)| {
                    if *ch != '.' {
                        acc.entry(ch)
                            .and_modify(|vec: &mut Vec<(i32, i32)>| vec.push((i as i32, j as i32)))
                            .or_insert(vec![(i as i32, j as i32)]);
                    }
                });
                acc
            })
            .iter()
            .fold(HashSet::new(), |mut acc, (_ch, vec)| {
                for i in 0..vec.len() {
                    for j in i + 1..vec.len() {
                        let (mut a, mut b) = vec[i];
                        let (mut x, mut y) = vec[j];
                        let diff = (x - a, y - b);
                        while city
                            .get(a as usize)
                            .and_then(|x| x.get(b as usize))
                            .is_some()
                        {
                            acc.insert((a, b));
                            a -= diff.0;
                            b -= diff.1;
                        }
                        while city
                            .get(x as usize)
                            .and_then(|x| x.get(y as usize))
                            .is_some()
                        {
                            acc.insert((x, y));
                            x += diff.0;
                            y += diff.1;
                        }
                    }
                }
                acc
            })
            .len()
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
            Day8.part1(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            14
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day8.part2(
                "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            ),
            34
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| black_box(Day8.part1(&input)));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| black_box(Day8.part2(&input)));
    }
}
