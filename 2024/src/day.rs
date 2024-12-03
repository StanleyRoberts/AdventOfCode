use crate::input::get_puzzle_input;

pub trait Part1 {
    fn part1(&self, input: &str) -> usize;
}
pub trait Part2 {
    fn part2(&self, input: &str) -> usize;
}

pub trait Day: Part1 + Part2 + DayMeta {
    fn run_part1(&self) {
        println!(
            "{} - Part 1: {}",
            self.get_str(),
            self.part1(&self.get_input())
        )
    }
    fn run_part2(&self) {
        println!(
            "{} - Part 2: {}",
            self.get_str(),
            self.part2(&self.get_input())
        )
    }
}

impl<D: Part1 + Part2 + DayMeta> Day for D {}

pub trait DayMeta {
    fn get_day(&self) -> usize;
    fn get_str(&self) -> String {
        format!("Day {}", self.get_day())
    }
    fn get_input(&self) -> String {
        get_puzzle_input(self.get_day())
    }
}

macro_rules! impl_day {
    ($name:ident, $val:expr) => {
        #[derive(Debug, Default, Clone, Copy)]
        pub(crate) struct $name;
        impl crate::days::DayMeta for $name {
            fn get_day(&self) -> usize {
                $val
            }
        }
    };
}
pub(crate) use impl_day;
