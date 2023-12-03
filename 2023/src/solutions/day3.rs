use std::collections::HashMap;

fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    let mut matrix: Vec<String> = input.map(|x| (".".to_owned() + &x + "..")).collect();
    let dots = String::from_utf8(vec![b'.'; matrix[0].len()]).unwrap();
    matrix.push(dots.clone());
    matrix.insert(0, dots);
    let patt = regex::Regex::new(r"\d+").unwrap();

    matrix
        .iter()
        .enumerate()
        .map(|(row, line)| {
            patt.find_iter(line)
                .filter_map(|num| {
                    let mut part = false;
                    for comp_row in matrix.iter().take(row + 2).skip(row - 1) {
                        for c in (num.start() - 1)..=num.end() {
                            if comp_row.chars().nth(c).unwrap().is_ascii_punctuation()
                                && comp_row.chars().nth(c).unwrap() != '.'
                            {
                                part = true;
                            }
                        }
                    }
                    if part {
                        return Some(num.as_str().parse::<i32>().unwrap());
                    }
                    None
                })
                .sum::<i32>()
        })
        .sum()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    let mut matrix: Vec<String> = input.map(|x| (".".to_owned() + &x + "..")).collect();
    let dots = String::from_utf8(vec![b'.'; matrix[0].len()]).unwrap();
    matrix.push(dots.clone());
    matrix.insert(0, dots);
    let patt = regex::Regex::new(r"\d+").unwrap();
    let mut hm: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    matrix
        .iter()
        .enumerate()
        .map(|(row, line)| {
            for num in patt.find_iter(line) {
                let mut part = false;
                for (r, comp_row) in matrix.iter().take(row + 2).skip(row - 1).enumerate() {
                    for c in (num.start() - 1)..=num.end() {
                        if comp_row.chars().nth(c).unwrap() == '*' {
                            hm.entry(((row + r) as i32, c as i32))
                                .or_insert(vec![])
                                .push(num.as_str().parse().unwrap());
                        }
                    }
                }
            }
        })
        .count();
    let mut count = 0;
    for ((x, y), vec) in hm {
        if vec.len() == 2 {
            count += vec[0] * vec[1];
        }
    }
    count
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
        assert_eq!(part2(sample), 467835);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        b.iter(|| {
            part1(read_input!("day3.txt"));
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        b.iter(|| {
            part2(read_input!("day3.txt"));
        });
    }
}
