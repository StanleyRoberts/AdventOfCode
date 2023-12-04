fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .map(|line| {
            let mut split = line.split('|');
            let winners: Vec<&str> = split.next().unwrap().split(' ').skip(2).collect();
            split
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| winners.contains(x) && !x.is_empty())
                .count()
        })
        .filter(|x| *x != 0)
        .fold(0, |acc, x| acc + 2_i32.pow((x - 1).try_into().unwrap()))
}

fn part2<I: Iterator<Item = String>>(input: I) -> usize {
    let mut counts: Vec<usize> = vec![1; 205];
    input
        .enumerate()
        .map(|(i, line)| {
            let mut split = line.split('|');
            let winners: Vec<&str> = split.next().unwrap().split(' ').skip(2).collect();
            for j in 0..split
                .next()
                .unwrap()
                .split(' ')
                .filter(|x| winners.contains(x) && !x.is_empty())
                .count()
            {
                counts[j + i + 1] += counts[i];
            }
            counts[i]
        })
        .sum()
}

pub fn day4() {
    println!("[Day 4] Part 1: {}", part1(read_input!("day4.txt")));
    println!("[Day 4] Part 2: {}", part2(read_input!("day4.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 13);
    }

    #[test]
    fn sample_part2() {
        let sample = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .lines()
        .map(&str::to_owned);
        assert_eq!(part2(sample.peekable()), 30);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day4.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day4.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
