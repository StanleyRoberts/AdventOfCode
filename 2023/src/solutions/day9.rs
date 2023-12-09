macro_rules! get_history {
    ($root:expr) => {{
        let mut diffs = vec![$root];
        while diffs[0].iter().any(|&x| x != 0) {
            diffs.insert(
                0,
                diffs[0]
                    .iter()
                    .zip(diffs[0].iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect(),
            );
        }
        diffs
    }};
}

fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .map(|x| x.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .map(|x: Vec<i32>| {
            get_history!(x)
                .iter()
                .fold(0, |acc, x| acc + x[x.len() - 1])
        })
        .sum()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .map(|x| x.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .map(|x: Vec<i32>| get_history!(x).iter().fold(0, |acc, x| x[0] - acc))
        .sum()
}

pub fn day9() {
    println!("[Day 9] Part 1: {}", part1(read_input!("day9.txt")));
    println!("[Day 9] Part 2: {}", part2(read_input!("day9.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 114);
    }

    #[test]
    fn sample_part2() {
        let sample = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 2);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day9.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day9.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
