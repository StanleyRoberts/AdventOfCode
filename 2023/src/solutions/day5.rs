fn part1<I: Iterator<Item = String>>(mut input: I) -> i64 {
    let seeds = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .zip(vec![false; 20])
        .collect::<Vec<(i64, bool)>>();
    input
        .filter(|x| !x.contains(':'))
        .fold(seeds, |acc: Vec<(i64, bool)>, x| {
            if x.is_empty() {
                acc.iter().map(|(x, _)| (*x, false)).collect()
            } else {
                let map: Vec<i64> = x.split(' ').map(|x| x.parse().unwrap()).collect();
                acc.iter()
                    .map(|(y, set)| {
                        if map[1] <= *y && *y < map[1] + map[2] && !set {
                            (map[0] + (y - map[1]), true)
                        } else {
                            (*y, *set)
                        }
                    })
                    .collect()
            }
        })
        .iter()
        .fold(i64::MAX, |acc, (x, _sum)| if *x < acc { *x } else { acc })
}

fn part2<I: Iterator<Item = String>>(mut input: I) -> i64 {
    let start_ranges: Vec<i64> = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let mut maps: Vec<Vec<Vec<i64>>> = Vec::new();
    let mut cur: Vec<Vec<i64>> = Vec::new();
    for item in input.filter(|x| !x.is_empty()) {
        if item.contains(':') {
            maps.insert(0, cur);
            cur = Vec::new();
        } else {
            cur.push(item.split(' ').map(|x| x.parse().unwrap()).collect());
        }
    }
    maps.insert(0, cur);
    let mut smallest = 0;
    let mut val = 0;
    while !start_ranges
        .chunks(2)
        .any(|range| val > range[0] && val < range[0] + range[1])
    {
        smallest += 1;
        val = smallest;

        for map in &maps {
            val = map
                .iter()
                .find(|line| line[0] <= val && line[0] + line[2] > val)
                .map_or(val, |line| line[1] + (val - line[0]));
        }
    }
    smallest
}

pub fn day5() {
    println!("[Day 5] Part 1: {}", part1(read_input!("day5.txt")));
    println!("[Day 5] Part 2: {}", part2(read_input!("day5.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 35);
    }

    #[test]
    fn sample_part2() {
        let sample = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 46);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day5.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day5.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
