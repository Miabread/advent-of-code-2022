use itertools::Itertools;

use crate::{
    util::{lines, SelfExt},
    Solution, Test,
};

pub struct Day5;

impl Solution for Day5 {
    type Input = Input;
    fn parse(input: &str) -> Self::Input {
        let input = lines(input).collect_vec();
        let (area, commands) = input.split(|line| line.is_empty()).collect_tuple().unwrap();

        let area_length: u32 = area
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let commands = commands
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .filter(|part| part.starts_with(|char: char| char.is_numeric()))
                    .map(|part| part.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        Input { commands }
    }
}

pub struct Input {
    commands: Vec<(u32, u32, u32)>,
}

impl Test for Day5 {
    const TEST_INPUT: &'static str = "
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    ";
}
