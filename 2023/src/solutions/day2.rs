fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .filter_map(|line| {
            let binding = line.replace([';', ',', ':'], "");
            let mut string = binding.split(' ');
            string.clone().skip(2).zip(string.clone().skip(3)).try_fold(
                string.nth(1).unwrap().parse::<i32>().unwrap(),
                |acc, x| {
                    if match x.0.parse::<i32>() {
                        Ok(13) => !["green", "blue"].contains(&x.1),
                        Ok(14) => "blue" != x.1,
                        Ok(15..) => true,
                        _ => false,
                    } {
                        return None;
                    }
                    Some(acc)
                },
            )
        })
        .sum()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    input.fold(0, |acc, line| {
        let binding = line.replace([';', ','], "");
        binding
            .split(' ')
            .zip(binding.split(' ').skip(1))
            .fold([0, 0, 0], |mut acc, x| {
                match x.1 {
                    "red" => acc[0] = std::cmp::max(acc[0], x.0.parse::<i32>().unwrap()),
                    "green" => acc[1] = std::cmp::max(acc[1], x.0.parse::<i32>().unwrap()),
                    "blue" => acc[2] = std::cmp::max(acc[2], x.0.parse::<i32>().unwrap()),
                    _ => (),
                }
                acc
            })
            .iter()
            .product::<i32>()
            + acc
    })
}

pub fn day2() {
    println!("[Day 2] Part 1: {}", part1(read_input!("day2.txt")));
    println!("[Day 2] Part 2: {}", part2(read_input!("day2.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines()
            .map(str::to_owned);
        assert_eq!(part1(sample), 8);
    }

    #[test]
    fn sample_part2() {
        let sample = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            .lines()
            .map(str::to_owned);
        assert_eq!(part2(sample), 2286);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day2.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day2.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
