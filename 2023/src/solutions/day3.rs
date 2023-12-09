use std::collections::HashMap;

fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    let mut matrix: Vec<String> = input.map(|x| (".".to_owned() + &x + "..")).collect();
    let dots = ".".repeat(matrix[0].len());
    matrix.push(dots.clone());
    matrix.insert(0, dots);
    let patt = regex::Regex::new(r"\d+").unwrap();

    matrix
        .iter()
        .enumerate()
        .map(|(row, line)| {
            patt.find_iter(line)
                .filter_map(|num| {
                    for search_line in matrix.iter().take(row + 2).skip(row - 1) {
                        for char in (num.start() - 1)..=num.end() {
                            if search_line
                                .chars()
                                .nth(char)
                                .unwrap()
                                .is_ascii_punctuation()
                                && search_line.chars().nth(char).unwrap() != '.'
                            {
                                return Some(num.as_str().parse::<i32>().unwrap());
                            }
                        }
                    }
                    None
                })
                .sum::<i32>()
        })
        .sum()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    let mut matrix: Vec<String> = input.map(|x| (".".to_owned() + &x + "..")).collect();
    let dots = ".".repeat(matrix[0].len());
    matrix.push(dots.clone());
    matrix.insert(0, dots);
    let patt = regex::Regex::new(r"\d+").unwrap();
    let mut hm: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    for (row, line) in matrix.iter().enumerate() {
        for num in patt.find_iter(line) {
            for (r, search_row) in matrix.iter().take(row + 2).skip(row - 1).enumerate() {
                for c in (num.start() - 1)..=num.end() {
                    if search_row.chars().nth(c).unwrap() == '*' {
                        hm.entry(((row + r).try_into().unwrap(), c.try_into().unwrap()))
                            .or_default()
                            .push(num.as_str().parse().unwrap());
                    }
                }
            }
        }
    }
    hm.values()
        .filter(|vec| vec.len() == 2)
        .fold(0, |acc, x| acc + (x[0] * x[1]))
}

pub fn day3() {
    println!("[Day 3] Part 1: {}", part1(read_input!("day3.txt")));
    println!("[Day 3] Part 2: {}", part2(read_input!("day3.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 4361);
    }

    #[test]
    fn sample_part2() {
        let sample = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .lines()
            .map(&str::to_owned);
        assert_eq!(part2(sample), 467_835);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day3.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day3.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
