macro_rules! parse1 {
    ($iter:expr) => {
        $iter
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .skip(1)
            .map(|x| x.parse::<i32>().unwrap())
    };
}

macro_rules! parse2 {
    ($iter:expr) => {
        $iter
            .next()
            .unwrap()
            .split(' ')
            .filter(|x| !x.is_empty())
            .skip(1)
            .fold(String::new(), |acc, x| acc + x)
            .parse::<f64>()
            .unwrap()
    };
}

fn part1<I: Iterator<Item = String>>(mut input: I) -> i32 {
    parse1!(input)
        .zip(parse1!(input))
        .map(|(time, record)| {
            (0..time)
                .map(|x| x * (time - x))
                .filter(|x| *x > record)
                .count()
        })
        .fold(1, |acc, x| acc * i32::try_from(x).unwrap())
}

fn part2<I: Iterator<Item = String>>(mut input: I) -> i64 {
    let time = parse2!(input);
    let record = parse2!(input);
    #[allow(clippy::cast_possible_truncation)]
    {
        ((0.5 * (time + (time.powf(2.0) - 4.0 * record).sqrt())).floor()
            - (0.5 * (time - (time.powf(2.0) - 4.0 * record).sqrt())).floor()) as i64
    }
}

pub fn day6() {
    println!("[Day 6] Part 1: {}", part1(read_input!("day6.txt")));
    println!("[Day 6] Part 2: {}", part2(read_input!("day6.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "Time:      7  15   30
Distance:  9  40  200"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 288);
    }

    #[test]
    fn sample_part2() {
        let sample = "Time:      7  15   30
Distance:  9  40  200"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 71503);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day6.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day6.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
