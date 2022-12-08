use std::collections::HashSet;

use crate::{Solution, Test};

pub struct Day3;

impl Solution for Day3 {
    type Input = Vec<(HashSet<usize>, HashSet<usize>)>;
    fn parse(input: &str) -> Self::Input {
        fn parse_pocket(pocket: &str) -> HashSet<usize> {
            let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
            pocket
                .chars()
                .map(|item| 1 + chars.find(item).unwrap())
                .collect()
        }

        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|bag| bag.split_at(bag.len() / 2))
            .map(|(a, b)| (parse_pocket(a), parse_pocket(b)))
            .collect()
    }

    fn part1(input: &Self::Input) -> usize {
        input
            .iter()
            .map(|(a, b)| a.intersection(b).next().unwrap())
            .sum()
    }

    fn part2(input: &Self::Input) -> usize {
        input
            .chunks_exact(3)
            .map(|group| {
                group
                    .iter()
                    .map(|(a, b)| a.union(b).collect::<HashSet<_>>())
                    .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<_>>())
                    .unwrap()
                    .iter()
                    .next()
                    .copied()
                    .unwrap()
            })
            .sum()
    }
}

impl Test for Day3 {
    const TEST_OUTPUT1: usize = 157;
    const TEST_OUTPUT2: usize = 70;
    const TEST_INPUT: &'static str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";
}
