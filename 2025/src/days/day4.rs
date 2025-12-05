use super::{Part1, Part2, impl_day};

impl_day!(Day4, 4);

impl Part1 for Day4 {
    fn part1(&self, input: &str) -> u64 {
        let mut grid = input
            .lines()
            .map(|l| ".".to_owned() + l + ".")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        grid.insert(0, vec!['.'; grid[0].len()]);
        grid.push(vec!['.'; grid[0].len()]);
        let mut total_count = 0;
        for i in 1..(grid.len() - 1) {
            let line = &grid[i];
            for j in 1..(line.len() - 1) {
                if grid[i][j] == '@'
                    && (-1..=1)
                        .flat_map(|y| (-1..=1).map(move |x| (x, y)))
                        .filter(|(x, y)| {
                            grid[(i as isize + x) as usize][(j as isize + y) as usize] == '@'
                        })
                        .count()
                        <= 4
                {
                    total_count += 1;
                }
            }
        }
        total_count
    }
}

impl Part2 for Day4 {
    fn part2(&self, input: &str) -> u64 {
        let mut last_count = 1;
        let mut total_count = 0;
        let mut grid = input
            .lines()
            .map(|l| ".".to_owned() + l + ".")
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        grid.insert(0, vec!['.'; grid[0].len()]);
        grid.push(vec!['.'; grid[0].len()]);
        while last_count != total_count {
            last_count = total_count;
            for i in 1..(grid.len() - 1) {
                let line = &grid[i];
                for j in 1..(line.len() - 1) {
                    if grid[i][j] == '@'
                        && (-1..=1)
                            .flat_map(|y| (-1..=1).map(move |x| (x, y)))
                            .filter(|(x, y)| {
                                grid[(i as isize + x) as usize][(j as isize + y) as usize] == '@'
                            })
                            .count()
                            <= 4
                    {
                        grid[i][j] = '.';
                        total_count += 1;
                    }
                }
            }
        }
        total_count
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
            Day4.part1(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            13
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day4.part2(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            43
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day4.get_input();
        b.iter(|| Day4.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day4.get_input();
        b.iter(|| Day4.part2(&input));
    }
}
