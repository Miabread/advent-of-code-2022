use std::vec;

use itertools::Itertools;

use crate::{util::lines, Solution, Test};

pub struct Day5;

impl Solution for Day5 {
    type Input = Input;
    fn parse(input: &str) -> Self::Input {
        // let input = input.lines().collect_vec();
        // let (area, commands) = input.split(|line| line.is_empty()).collect_tuple().unwrap();
        // let mut area = area.iter();

        let input = input.lines().collect_vec();
        let mut input = input.split(|line| line.is_empty());

        let mut area = dbg!(input.next().unwrap()).iter();

        let area_length: usize = area
            .next_back()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut stacks = vec![vec![]; area_length + 1usize];

        // for line in area {
        //     for (stack, letter) in line
        //         .split_whitespace()
        //         .map(|item| item.chars().nth(1).unwrap())
        //         .inspect(|f| println!("{f}"))
        //         .enumerate()
        //     {
        //         stacks[stack].insert(0, letter);
        //     }
        // }

        for line in area {
            for (stack, letter) in line.chars().skip(1).step_by(4).enumerate() {
                dbg!((stack, letter));
                stacks[stack].insert(0, letter);
            }
        }

        dbg!(&stacks);

        let commands = input
            .next()
            .unwrap()
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .filter(|part| part.starts_with(|char: char| char.is_numeric()))
                    .map(|part| part.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        Input { stacks, commands }
    }
}

#[derive(Debug)]
pub struct Input {
    stacks: Vec<Vec<char>>,
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
