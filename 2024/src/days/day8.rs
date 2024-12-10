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

/// I got into a competition with a friend about optimising this specific part. This is the result of that.
/// I don't recommend using this at all as I have not rigourously checked its soundness
#[allow(dead_code)]
pub fn part1_opt(input: &str) -> usize {
    /*use std::arch::{
        asm,
        x86_64::{__m128, __m128i, _mm_extract_ps, _mm_set_ps, _mm_shuffle_ps},
    };*/
    use std::{
        mem::{self, MaybeUninit},
        ptr::drop_in_place,
    };
    let mut hor = 0_i8;
    let mut ver = 0_i8;
    let mut map: [MaybeUninit<Vec<(i8, i8)>>; 128] = [const { MaybeUninit::uninit() }; 128];
    let mut keys = HashSet::with_capacity(128);
    input.lines().for_each(|line| {
        hor = 0;
        line.chars().for_each(|ch| {
            if ch != '.' {
                if keys.contains(&ch) {
                    unsafe {
                        map.get_unchecked_mut(ch as u8 as usize)
                            .assume_init_mut()
                            .push((ver, hor))
                    };
                } else {
                    unsafe {
                        map.get_unchecked_mut(ch as u8 as usize)
                            .write(vec![(ver, hor)])
                    };
                    keys.insert(ch);
                }
            }
            hor += 1;
        });
        ver += 1;
    });
    let ret = keys
        .iter()
        .map(|&ch| unsafe { mem::transmute(map.get_unchecked(ch as usize)) })
        .fold(
            HashSet::new(),
            |mut acc: HashSet<(i8, i8)>, mut vec: &Vec<(i8, i8)>| unsafe {
                /*for (i, &(a, b)) in vec.iter().enumerate() {
                    let outadd: __m128 = _mm_set_ps(a as f32, b as f32, 0.0, 0.0);
                    for &(x, y) in vec.iter().skip(i + 1) {
                        let mut inadd: __m128 = _mm_set_ps(0.0, 0.0, x as f32, y as f32);
                        asm! {
                            "blendps {0}, {1}, 0xC",
                            inout(xmm_reg) inadd,
                            in(xmm_reg) outadd,
                            options(pure, nomem, nostack)
                        }
                        let insub: __m128 = _mm_shuffle_ps::<0b01001110>(inadd, inadd);
                        asm! {
                            "addps {0}, {0}",
                            "subps {0}, {1}",
                            inout(xmm_reg) inadd,
                            in(xmm_reg) insub,
                            options(pure, nomem, nostack)
                        }
                        acc.insert((
                            f32::from_bits(_mm_extract_ps::<0>(inadd) as u32).to_int_unchecked(),
                            f32::from_bits(_mm_extract_ps::<1>(inadd) as u32).to_int_unchecked(),
                        ));
                        acc.insert((
                            f32::from_bits(_mm_extract_ps::<2>(inadd) as u32).to_int_unchecked(),
                            f32::from_bits(_mm_extract_ps::<3>(inadd) as u32).to_int_unchecked(),
                        ));
                    }
                }*/

                for (i, (a, b)) in vec.iter().enumerate() {
                    for (x, y) in vec.iter().skip(i + 1) {
                        let diff = (x - a, y - b);
                        acc.insert((a - diff.0, b - diff.1));
                        acc.insert((x + diff.0, y + diff.1));
                    }
                }
                drop_in_place(&mut vec);
                acc
            },
        )
        .iter()
        .map(|(x, y)| (*x as u8, *y as u8))
        .filter(|(x, y)| *x < ver as u8 && *y < hor as u8)
        .count();
    ret
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

    #[test]
    fn test_part1_opt() {
        use crate::day::DayMeta;
        let input = Day8.get_input();
        assert_eq!(Day8.part1(&input), part1_opt(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1_opt(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| black_box(part1_opt(&input)));
    }
}
