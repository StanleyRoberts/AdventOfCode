use std::{iter, ops::Range};

use super::{Part1, Part2, impl_day};

impl_day!(Day9, 9);

impl Part1 for Day9 {
    fn part1(&self, input: &str) -> u64 {
        let corners = input
            .lines()
            .map(|x| x.split(','))
            .map(|mut x| {
                (
                    x.next().unwrap().parse::<i64>().unwrap(),
                    x.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let mut max = 0;
        for (i, x) in corners.iter().enumerate() {
            for y in corners.iter().skip(i) {
                max = max.max(((x.0 - y.0).abs() + 1) * ((x.1 - y.1).abs() + 1))
            }
        }
        max as u64
    }
}

fn range_from_points(a: i64, b: i64) -> Range<i64> {
    let mut arr = [a, b];
    arr.sort();
    (arr[0] + 1)..arr[1]
}

fn line_intersects_rectangle(
    line: ((i64, i64), (i64, i64)),
    rectangle: ((i64, i64), (i64, i64)),
) -> bool {
    let rect_y = range_from_points(rectangle.0.0, rectangle.1.0);
    let rect_x = range_from_points(rectangle.0.1, rectangle.1.1);
    if rect_y.contains(&line.0.0) {
        let line_x = range_from_points(line.0.1, line.1.1);
        rect_x.contains(&line_x.start)
            || rect_x.contains(&line_x.end)
            || line_x.contains(&rect_x.start)
            || line_x.contains(&rect_x.end)
    } else if rect_x.contains(&line.0.1) {
        let line_y = range_from_points(line.0.0, line.1.0);
        rect_y.contains(&line_y.start)
            || rect_y.contains(&line_y.end)
            || line_y.contains(&rect_y.start)
            || line_y.contains(&rect_y.end)
    } else {
        false
    }
}

impl Part2 for Day9 {
    fn part2(&self, input: &str) -> u64 {
        let corners = input
            .lines()
            .map(|x| x.split(','))
            .map(|mut x| {
                (
                    x.next().unwrap().parse::<i64>().unwrap(),
                    x.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let lines = corners
            .iter()
            .zip(corners.iter().skip(1))
            .chain(iter::once((&corners[0], &corners[corners.len() - 1])))
            .collect::<Vec<_>>();
        let mut max = 0;
        for (i, a) in corners.iter().enumerate() {
            for b in corners.iter().skip(i) {
                if !lines
                    .iter()
                    .any(|(first, second)| line_intersects_rectangle((**first, **second), (*a, *b)))
                {
                    max = max.max(((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
                }
            }
        }
        max as u64
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
            Day9.part1(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            ),
            50
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day9.part2(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            ),
            24
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day9.get_input();
        b.iter(|| Day9.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day9.get_input();
        b.iter(|| Day9.part2(&input));
    }
}
