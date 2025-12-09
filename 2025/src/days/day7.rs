use super::{Part1, Part2, impl_day};

impl_day!(Day7, 7);

impl Part1 for Day7 {
    fn part1(&self, input: &str) -> u64 {
        let mat = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut beams = vec![false; mat[0].len()];
        let mut count = 0;
        beams[mat[0].iter().position(|&ch| ch == 'S').unwrap()] = true;
        for row in mat {
            for (column, &element) in row.iter().enumerate() {
                if element == '^' && beams[column] {
                    beams[column] = false;
                    beams[column - 1] = true;
                    beams[column + 1] = true;
                    count += 1;
                }
            }
        }
        count
    }
}

impl Part2 for Day7 {
    fn part2(&self, input: &str) -> u64 {
        let mat = input
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut beams = vec![0; mat[0].len()];
        beams[mat[0].iter().position(|&ch| ch == 'S').unwrap()] = 1;
        for row in mat {
            for (column, &element) in row.iter().enumerate() {
                if element == '^' && beams[column] > 0 {
                    beams[column - 1] += beams[column];
                    beams[column + 1] += beams[column];
                    beams[column] = 0;
                }
            }
        }
        beams.iter().sum()
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
            Day7.part1(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            ),
            21
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day7.part2(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            ),
            40
        );
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day7.get_input();
        b.iter(|| Day7.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day7.get_input();
        b.iter(|| Day7.part2(&input));
    }
}
