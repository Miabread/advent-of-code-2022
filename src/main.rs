use std::fs::read_to_string;

mod day1;
mod day2;

trait Solution {
    type Input;
    fn parse(input: &str) -> Self::Input;
    fn part1(input: &Self::Input) -> u32;
    fn part2(input: &Self::Input) -> u32;
}

trait Test {
    const TEST_OUTPUT1: u32;
    const TEST_OUTPUT2: u32;
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
        _ => panic!("not a valid day"),
    };

    let input = read_to_string(format!("./input/day{day}.txt")).expect("input file should exist");

    solution.test();
    solution.solve(&input);
}
