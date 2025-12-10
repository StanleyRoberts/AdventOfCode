use std::collections::HashSet;

use super::{Part1, Part2, impl_day};

impl_day!(Day8, 8);

fn euclidean_distance(x: (u64, u64, u64), y: (u64, u64, u64)) -> u64 {
    let x = (x.0 as i64, x.1 as i64, x.2 as i64);
    let y = (y.0 as i64, y.1 as i64, y.2 as i64);
    ((x.0 - y.0).pow(2) + (x.1 - y.1).pow(2) + (x.2 - y.2).pow(2)).isqrt() as u64
}

impl Part1 for Day8 {
    fn part1(&self, input: &str) -> u64 {
        let junctions = input
            .lines()
            .map(|x| {
                let mut iter = x.split(',').map(|x| x.parse::<u64>().unwrap());
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let mut distances = junctions
            .iter()
            .enumerate()
            .flat_map(|(i, &x)| {
                junctions
                    .iter()
                    .enumerate()
                    .filter_map(move |(j, &y)| {
                        if j < i {
                            None
                        } else {
                            Some(((x, y), euclidean_distance(x, y)))
                        }
                    })
                    .filter(|(_, e)| *e > 0)
            })
            .collect::<Vec<_>>();
        distances.sort_by_key(|x| x.1);
        let mut ordered_distances = distances.into_iter().map(|(x, _)| x);
        let mut circuits: Vec<HashSet<(u64, u64, u64)>> = junctions
            .iter()
            .map(|&x| HashSet::from([x]))
            .collect::<Vec<_>>();
        let mut remaining_pairs = if cfg!(test) { 10 } else { 1000 };
        while remaining_pairs > 0 {
            let (left_junction, right_junction) = ordered_distances.next().unwrap();
            let left_circuit = circuits
                .iter()
                .position(|x| x.contains(&left_junction))
                .unwrap();
            let right_circuit = circuits
                .iter()
                .position(|x| x.contains(&right_junction))
                .unwrap();
            remaining_pairs -= 1;
            if left_circuit != right_circuit {
                let right = circuits[right_circuit].clone();
                circuits[left_circuit].extend(right);
                circuits.remove(right_circuit);
            }
        }
        circuits.sort_by_key(|x| x.len());
        circuits
            .into_iter()
            .rev()
            .take(3)
            .map(|x| x.len() as u64)
            .product()
    }
}

impl Part2 for Day8 {
    fn part2(&self, input: &str) -> u64 {
        let junctions = input
            .lines()
            .map(|x| {
                let mut iter = x.split(',').map(|x| x.parse::<u64>().unwrap());
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let mut distances = junctions
            .iter()
            .enumerate()
            .flat_map(|(i, &x)| {
                junctions
                    .iter()
                    .enumerate()
                    .filter_map(move |(j, &y)| {
                        if j < i {
                            None
                        } else {
                            Some(((x, y), euclidean_distance(x, y)))
                        }
                    })
                    .filter(|(_, e)| *e > 0)
            })
            .collect::<Vec<_>>();
        distances.sort_by_key(|x| x.1);
        let mut ordered_distances = distances.into_iter().map(|(x, _)| x);
        let mut circuits: Vec<HashSet<(u64, u64, u64)>> = junctions
            .iter()
            .map(|&x| HashSet::from([x]))
            .collect::<Vec<_>>();
        loop {
            let (left_junction, right_junction) = ordered_distances.next().unwrap();
            let left_circuit = circuits
                .iter()
                .position(|x| x.contains(&left_junction))
                .unwrap();
            let right_circuit = circuits
                .iter()
                .position(|x| x.contains(&right_junction))
                .unwrap();
            if left_circuit != right_circuit {
                if circuits.len() == 2 {
                    return left_junction.0 * right_junction.0;
                }
                let right = circuits[right_circuit].clone();
                circuits[left_circuit].extend(right);
                circuits.remove(right_circuit);
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
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day8.part1(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            ),
            40
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day8.part2(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
            ),
            25272
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| Day8.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day8.get_input();
        b.iter(|| Day8.part2(&input));
    }
}
