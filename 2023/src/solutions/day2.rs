fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    input
        .map(|line| {
            let binding = line.replace([';', ',', ':'], "");
            let mut string = binding.split(' ');
            string.next();
            let id = string.next().unwrap();
            let mut invalid = false;
            while let Some(x) = string.next() {
                match x.parse::<i32>() {
                    Ok(13) => {
                        invalid =
                            invalid || ![Some("green"), Some("blue")].contains(&string.next());
                    }
                    Ok(14) => {
                        invalid = invalid || Some("blue") != string.next();
                    }
                    Ok(15..) => invalid = true,
                    _ => (),
                }
            }
            if invalid {
                0
            } else {
                id.parse::<i32>().unwrap()
            }
        })
        .sum()
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    input.fold(0, |acc, line| {
        let mut top = [0, 0, 0];
        let binding = line.replace([';', ','], "");
        let mut slow = "";
        for x in binding.split(' ') {
            match x {
                "red" => top[0] = std::cmp::max(top[0], slow.parse::<i32>().unwrap()),
                "green" => top[1] = std::cmp::max(top[1], slow.parse::<i32>().unwrap()),
                "blue" => top[2] = std::cmp::max(top[2], slow.parse::<i32>().unwrap()),
                _ => (),
            }
            slow = x;
        }
        let sum = top[0] * top[1] * top[2];
        acc + sum
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
