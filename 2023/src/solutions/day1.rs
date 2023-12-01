fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .map(|line| {
            let mut temp = line.chars().filter(char::is_ascii_digit);
            let first = temp.next().unwrap();
            format!("{}{}", first, temp.last().unwrap_or(first))
        })
        .fold(0, |acc, x| acc + x.parse::<i32>().unwrap())
}

fn part2() {}

pub fn day1() {
    println!("[Day 1] Part 1: {}", part1(read_input!("day1.txt")));
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
    fn sample_part2() {}
}
