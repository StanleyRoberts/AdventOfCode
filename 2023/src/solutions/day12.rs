fn part1<I: Iterator<Item = String>>(input: I) -> i32 {
    let nsplit = input
        .map(|x| x.split(' ').map(str::to_owned).collect())
        .map(|y: Vec<String>| {
            (
                y[0].clone(),
                y[1].split(',')
                    .map(|z| z.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect::<Vec<(String, Vec<i32>)>>();
    println!("{nsplit:?}");
    0
}

fn part2<I: Iterator<Item = String>>(input: I) -> i32 {
    0
}

pub fn day12() {
    println!("[Day 12] Part 1: {}", part1(read_input!("day12.txt")));
    println!("[Day 12] Part 2: {}", part2(read_input!("day12.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let sample = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            .lines()
            .map(&str::to_owned);
        assert_eq!(part1(sample), 1);
    }

    #[test]
    fn sample_part2() {
        let sample = "".lines().map(&str::to_owned);
        assert_eq!(part2(sample), 1);
    }
}

#[cfg(feature = "bench")]
mod bench {
    use super::*;

    #[bench]
    fn run_part1(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day12.txt").collect();
        b.iter(|| {
            part1(file.iter().cloned());
        });
    }

    #[bench]
    fn run_part2(b: &mut test::Bencher) {
        let file: Vec<String> = read_input!("day12.txt").collect();
        b.iter(|| {
            part2(file.iter().cloned());
        });
    }
}
