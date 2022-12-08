use std::fs::read_to_string;

mod day1;
mod day2;
mod day3;
mod day4;
mod day6;
mod util;

trait Solution {
    type Input;
    fn parse(input: &str) -> Self::Input;

    fn part1(_input: &Self::Input) -> u32 {
        0
    }

    fn part2(_input: &Self::Input) -> u32 {
        0
    }
}

trait Test {
    const TEST_OUTPUT1: u32 = 0;
    const TEST_OUTPUT2: u32 = 0;
    const TEST_INPUT: &'static str;
}

trait Run {
    fn solve(&self, input: &str);
    fn test(&self);
}

impl<T: Solution + Test> Run for T {
    fn solve(&self, input: &str) {
        let input = Self::parse(input);
        println!("Part 1: {}", Self::part1(&input));
        println!("Part 2: {}", Self::part2(&input));
    }

    fn test(&self) {
        let input = Self::parse(Self::TEST_INPUT);
        assert_eq!(Self::TEST_OUTPUT1, Self::part1(&input));
        assert_eq!(Self::TEST_OUTPUT2, Self::part2(&input));
        println!("Tests passed");
    }
}

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("no day given")
        .parse()
        .expect("day should be a number");

    let solution: &dyn Run = match day {
        1 => &day1::Day1,
        2 => &day2::Day2,
        3 => &day3::Day3,
        4 => &day4::Day4,
        6 => &day6::Day6,
        _ => panic!("not a valid day"),
    };

    let input = read_to_string(format!("./input/day{day}.txt")).expect("input file should exist");
    println!("Running Day {day}...");
    solution.test();
    solution.solve(&input);
}
