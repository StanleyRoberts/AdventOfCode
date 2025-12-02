use super::{Part1, Part2, impl_day};

impl_day!(Day2, 2);

impl Part1 for Day2 {
    fn part1(&self, input: &str) -> u64 {
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.split('-'))
            .map(|mut x| {
                x.next().unwrap().parse::<u64>().unwrap()
                    ..=x.next().unwrap().parse::<u64>().unwrap()
            })
            .map(|r| {
                r.filter(|e| {
                    let tens = 10_u64.pow(e.ilog10().div_ceil(2));
                    (e / tens) == (e % tens)
                })
                .sum::<u64>()
            })
            .sum::<u64>()
    }
}

impl Part2 for Day2 {
    fn part2(&self, input: &str) -> u64 {
        input
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.split('-'))
            .map(|mut x| {
                x.next().unwrap().parse::<u64>().unwrap()
                    ..=x.next().unwrap().parse::<u64>().unwrap()
            })
            .map(|r| {
                r.filter(|e| {
                    let len = e.ilog10() as u64 + 1;
                    for chunk_len in 1..((len / 2) + 1) {
                        let tens = 10_u64.pow(chunk_len as u32);
                        let mut splt = (0..(len.div_ceil(chunk_len)))
                            .map(|x| (e / (tens.pow(x as u32))) % tens);
                        let first = splt.next().unwrap();
                        if splt.all(|e| e == first) && len.is_multiple_of(chunk_len) {
                            return true;
                        }
                    }
                    false
                })
                .sum::<u64>()
            })
            .sum::<u64>()
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
        assert_eq!(Day2.part1("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day2.part2("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), 4174379265);
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part1(b: &mut Bencher) {
        let input = Day2.get_input();
        b.iter(|| Day2.part1(&input));
    }

    #[cfg(feature = "nightly")]
    #[cfg_attr(feature = "nightly", bench)]
    fn bench_part2(b: &mut Bencher) {
        let input = Day2.get_input();
        b.iter(|| Day2.part2(&input));
    }
}
