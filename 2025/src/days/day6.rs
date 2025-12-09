use super::{Part1, Part2, impl_day};

impl_day!(Day6, 6);

impl Part1 for Day6 {
    fn part1(&self, input: &str) -> u64 {
        let mut iter = input.lines().rev().map(|x| x.split(' '));
        let op = iter
            .next()
            .unwrap()
            .filter_map(|x| x.chars().next())
            .collect::<Vec<_>>();
        let mut iter = iter.map(|x| x.filter_map(|y| y.parse::<u64>().ok()));
        let first = iter.next().unwrap().collect::<Vec<_>>();
        iter.fold(first, |mut acc, x| {
            for (i, y) in x.enumerate() {
                if op[i] == '*' {
                    acc[i] *= y
                } else {
                    acc[i] += y
                }
            }
            acc
        })
        .iter()
        .sum()
    }
}

impl Part2 for Day6 {
    fn part2(&self, input: &str) -> u64 {
        let mut iter = input.lines().rev();
        let ops = iter
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|x| x.chars().next())
            .collect::<Vec<_>>();
        let mat = iter
            .rev()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut new = Vec::new();
        let mut line = Vec::new();
        for i in 0..mat[0].len() {
            let mut step = true;
            let mut num = String::new();
            for row in &mat {
                if row[i] != ' ' {
                    step = false;
                    num.push(row[i])
                }
            }
            if !num.is_empty() {
                line.push(num);
            }
            if step {
                new.push(line.clone());
                line = Vec::new();
            }
        }
        new.push(line.clone());
        new.into_iter()
            .zip(ops)
            .map(|(ls, op)| {
                let ls = ls.into_iter().map(|x| x.parse::<u64>().unwrap());
                if op == '*' {
                    ls.product::<u64>()
                } else {
                    ls.sum()
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "nightly")]
    use crate::day::DayMeta;
    #[cfg(feature = "nightly")]
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(
            Day6.part1(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + "
            ),
            4277556
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day6.part2(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + "
            ),
            3263827
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day6.get_input();
        b.iter(|| Day6.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day6.get_input();
        b.iter(|| Day6.part2(&input));
    }
}
