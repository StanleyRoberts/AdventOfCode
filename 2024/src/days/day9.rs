use std::collections::VecDeque;

use super::{impl_day, Part1, Part2};

impl_day!(Day9, 9);

impl Part1 for Day9 {
    fn part1(&self, input: &str) -> usize {
        let mut disk = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .fold(
                (VecDeque::new(), false, 0),
                |(mut acc, empty, count), ch| {
                    (0..ch.to_string().parse::<i32>().unwrap()).for_each(|_| {
                        if !empty {
                            acc.push_back(count.to_string());
                        } else {
                            acc.push_back(".".to_owned());
                        }
                    });
                    (acc, !empty, if !empty { count + 1 } else { count })
                },
            )
            .0;
        let mut fin = Vec::new();
        while let Some(i) = disk.pop_front() {
            if i != "." {
                fin.push(i);
            } else {
                let Some(start) = (loop {
                    let Some(x) = disk.pop_back() else {
                        break None;
                    };
                    if x != "." {
                        break Some(x);
                    }
                }) else {
                    break;
                };
                fin.push(start);
            }
        }
        fin.iter().enumerate().fold(0_i64, |acc, (i, x)| {
            acc + ((i as i64) * x.to_string().parse::<i64>().unwrap())
        }) as usize
    }
}

fn fill_space(size: i32, files: &[(String, i32)]) -> Option<((String, i32), i32)> {
    files
        .iter()
        .rev()
        .find(|&x| x.1 <= size && x.0 != ".")
        .map(|x| (x.clone(), size - x.1))
}

impl Part2 for Day9 {
    fn part2(&self, input: &str) -> usize {
        let mut disk = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .fold((Vec::new(), false, 0), |(mut acc, empty, count), ch| {
                if !empty {
                    acc.push((count.to_string(), ch.to_string().parse::<i32>().unwrap()));
                } else {
                    acc.push((".".to_owned(), ch.to_string().parse::<i32>().unwrap()));
                }
                (acc, !empty, if !empty { count + 1 } else { count })
            })
            .0;
        let mut fin = Vec::new();
        for (id, size) in disk.clone() {
            if id != "." && disk.iter().any(|x| *x == (id.clone(), size)) {
                let idx = disk.iter().position(|x| *x == (id.clone(), size)).unwrap();
                disk.remove(idx);
                fin.push((id, size))
            } else {
                let mut space_to_fill = size;
                while let Some((fill, rem)) = fill_space(space_to_fill, &disk) {
                    space_to_fill = rem;
                    let idx = disk.iter().position(|x| *x == fill).unwrap();
                    disk.remove(idx);
                    fin.push(fill);
                }
                fin.push((".".to_owned(), space_to_fill))
            }
        }
        fin.into_iter()
            .flat_map(|x| vec![x.0; x.1 as usize])
            .enumerate()
            .filter(|x| x.1 != ".")
            .fold(0_i64, |acc, (i, x)| {
                acc + ((i as i64) * x.to_string().parse::<i64>().unwrap())
            }) as usize
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
        assert_eq!(Day9.part1("2333133121414131402"), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day9.part2("2333133121414131402"), 2858);
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day9.get_input();
        b.iter(|| black_box(Day9.part1(&input)));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    #[ignore]
    fn bench_part2(b: &mut Bencher) {
        let input = Day9.get_input();
        b.iter(|| black_box(Day9.part2(&input)));
    }
}
