use crate::{Solution, Test};

pub struct Day1;

impl Solution for Day1 {
    type Input = Vec<Vec<u32>>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(|it| it.trim())
            .collect::<Vec<_>>()
            .split(|it| it.is_empty())
            .map(|elf| elf.iter().map(|num| num.parse().unwrap()).collect())
            .collect()
    }

    fn part1(input: &Self::Input) -> u32 {
        input.iter().map(|elf| elf.iter().sum()).max().unwrap()
    }

    fn part2(input: &Self::Input) -> u32 {
        let mut input: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
        input.sort_by(|a, b| a.cmp(b).reverse());
        input.iter().take(3).sum()
    }
}

impl Test for Day1 {
    const TEST_OUTPUT1: u32 = 24000;
    const TEST_OUTPUT2: u32 = 45000;
    const TEST_INPUT: &'static str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    ";
}
