use itertools::Itertools;

use crate::{util::full_lines, Solution, Test};

pub struct Day4;

impl Solution for Day4 {
    type Input = Vec<((usize, usize), (usize, usize))>;
    fn parse(input: &str) -> Self::Input {
        full_lines(input)
            .map(|line| {
                line.split(',')
                    .map(|range| {
                        range
                            .split('-')
                            .map(|num| num.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap()
            })
            .collect()
    }

    fn part1(input: &Self::Input) -> usize {
        input
            .iter()
            .filter(|(a, b)| {
                let is_a_contained = a.0 >= b.0 && a.1 <= b.1;
                let is_b_contained = b.0 >= a.0 && b.1 <= a.1;
                is_a_contained || is_b_contained
            })
            .count()
    }

    fn part2(input: &Self::Input) -> usize {
        input
            .iter()
            .filter(|(a, b)| {
                let is_a_colliding = (a.1 >= b.0 && b.0 >= a.0) || (a.1 >= b.1 && b.1 >= a.0);
                let is_b_colliding = (b.1 >= a.0 && a.0 >= b.0) || (b.1 >= a.1 && a.1 >= b.0);
                is_a_colliding || is_b_colliding
            })
            .count()
    }
}

impl Test for Day4 {
    const TEST_OUTPUT1: usize = 2;
    const TEST_OUTPUT2: usize = 4;
    const TEST_INPUT: &'static str = "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    ";
}
