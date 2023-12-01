fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .map(|line| {
            let mut temp = line.chars().filter(char::is_ascii_digit);
            let first = temp.next().unwrap();
            format!("{first}{}", temp.last().unwrap_or(first))
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    let numbers = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    part1(input.map(|line| {
        numbers.iter().fold(line, |acc, x| {
            acc.replace(x.0, &(x.0.to_owned() + x.1 + x.0))
        })
    }))
}

pub fn day1() {
    println!("[Day 1] Part 1: {}", part1(read_input!("day1.txt")));
    println!("[Day 1] Part 2: {}", part2(read_input!("day1.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 142);
    }

    #[test]
    fn sample_part2() {
        let sample = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 281);
    }
}
