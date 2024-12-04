use super::{impl_day, Part1, Part2};

impl_day!(Day4, 4);

fn check_dirs(mat: &[Vec<char>], pos: (usize, usize)) -> i32 {
    let mut found = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            let mut cur = pos;
            found += ['M', 'A', 'S'].into_iter().all(|x| {
                cur.0 = (cur.0 as i32 + i) as usize;
                cur.1 = (cur.1 as i32 + j) as usize;
                Some(Some(&x)) == mat.get(cur.0).map(|y| y.get(cur.1))
            }) as i32;
        }
    }
    found
}

impl Part1 for Day4 {
    fn part1(&self, input: &str) -> usize {
        let mat: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let mut count = 0;
        for i in 0..mat.len() {
            for j in 0..mat.first().unwrap().len() {
                if mat[i][j] == 'X' {
                    count += check_dirs(&mat, (i, j))
                }
            }
        }
        count as usize
    }
}

fn check_dia(mat: &[Vec<char>], pos: (i32, i32), offset: (i32, i32)) -> bool {
    let c = mat[(pos.0 - offset.0) as usize][(pos.1 - offset.1) as usize];
    match mat[(pos.0 + offset.0) as usize][(pos.1 + offset.1) as usize] {
        'M' => c == 'S',
        'S' => c == 'M',
        _ => false,
    }
}

impl Part2 for Day4 {
    fn part2(&self, input: &str) -> usize {
        let mat: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let mut count = 0;
        for i in 1..mat.len() - 1 {
            for j in 1..mat[0].len() - 1 {
                if mat[i][j] == 'A' {
                    count += (check_dia(&mat, (i as i32, j as i32), (-1, -1))
                        && check_dia(&mat, (i as i32, j as i32), (-1, 1)))
                        as i32;
                }
            }
        }
        count as usize
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
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            18
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day4.part2(
                "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            ),
            9
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
