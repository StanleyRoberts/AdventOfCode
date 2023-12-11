#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

fn transpose<T>(v: &[Vec<T>]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    let universe = transpose(&input.fold(Vec::new(), |mut acc, x| {
        acc.push(x.chars().collect());
        if x.chars().all(|x| x == '.') {
            acc.push(x.chars().collect());
        }
        acc
    }))
    .iter()
    .fold(Vec::new(), |mut acc, x| {
        acc.push(x.clone());
        if x.iter().all(|&x| x == '.') {
            acc.push(x.clone());
        }
        acc
    });
    let galaxies = universe
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, x)| {
            x.iter().enumerate().for_each(|(j, &y)| {
                if y == '#' {
                    acc.push((i as i32, j as i32));
                }
            });
            acc
        });
    galaxies
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, x)| {
            acc + galaxies[(i + 1)..].iter().fold(0usize, |acc, y| {
                acc + ((x.0 - y.0).abs() + (x.1 - y.1).abs()) as usize
            })
        })
        .try_into()
        .unwrap()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i64 {
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();
    let universe = transpose(&input.enumerate().fold(Vec::new(), |mut acc, (i, x)| {
        acc.push(x.chars().collect());
        if x.chars().all(|x| x == '.') {
            empty_rows.push(i as i32);
        }
        acc
    }))
    .iter()
    .enumerate()
    .fold(Vec::new(), |mut acc, (i, x)| {
        acc.push(x.clone());
        if x.iter().all(|&x| x == '.') {
            empty_cols.push(i as i32);
        }
        acc
    });
    let galaxies = universe
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, x)| {
            x.iter().enumerate().for_each(|(j, &y)| {
                if y == '#' {
                    acc.push((i as i32, j as i32));
                }
            });
            acc
        });
    galaxies
        .iter()
        .enumerate()
        .fold(0usize, |acc, (i, x)| {
            acc + galaxies[(i + 1)..].iter().fold(0usize, |acc, y| {
                let inter = ((x.0 - y.0).abs() + (x.1 - y.1).abs()) as usize;
                let modifier = empty_cols.iter().fold(0, |acc, &a| {
                    if (a > x.0 && a < y.0) || (a < x.0 && a > y.0) {
                        acc + 999_999
                    } else {
                        acc
                    }
                }) + empty_rows.iter().fold(0, |acc, &b| {
                    if (b > x.1 && b < y.1) || (b < x.1 && b > y.1) {
                        acc + 999_999
                    } else {
                        acc
                    }
                });
                acc + inter + modifier
            })
        })
        .try_into()
        .unwrap()
}

pub fn day11() {
    println!("[Day 11] Part 1: {}", part1(read_input!("day11.txt")));
    println!("[Day 11] Part 2: {}", part2(read_input!("day11.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 374);
    }

    #[test]
    fn sample_part2() {
        let sample = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 82_000_210);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day11.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day11.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
