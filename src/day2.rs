use std::{cmp::Ordering, str::FromStr};

use crate::{Solution, Test};

pub struct Day2;

impl Solution for Day2 {
    type Input = Vec<(Move, Move)>;

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.split_whitespace())
            .map(|mut line| {
                (
                    line.next().unwrap().parse().unwrap(),
                    line.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    fn part1(input: &Self::Input) -> u32 {
        input
            .iter()
            .map(|&(theirs, ours)| compute_score(theirs, ours))
            .sum()
    }

    fn part2(input: &Self::Input) -> u32 {
        input
            .iter()
            .map(|&(theirs, ending)| {
                let ending = match ending {
                    Move::Rock => Ordering::Greater,
                    Move::Paper => Ordering::Equal,
                    Move::Scissors => Ordering::Less,
                };

                let ours = compute_ours(theirs, ending);

                compute_score(theirs, ours)
            })
            .sum()
    }
}

fn compute_score(theirs: Move, ours: Move) -> u32 {
    let x = match ours {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    };
    let y = match theirs.cmp(&ours) {
        Ordering::Less => 6,
        Ordering::Equal => 3,
        Ordering::Greater => 0,
    };
    x + y
}

fn compute_ours(theirs: Move, ending: Ordering) -> Move {
    match (theirs, ending) {
        (theirs, Ordering::Equal) => theirs,

        (Move::Rock, Ordering::Less) => Move::Paper,
        (Move::Paper, Ordering::Less) => Move::Scissors,
        (Move::Scissors, Ordering::Less) => Move::Rock,

        (Move::Rock, Ordering::Greater) => Move::Scissors,
        (Move::Paper, Ordering::Greater) => Move::Rock,
        (Move::Scissors, Ordering::Greater) => Move::Paper,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.trim() {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => return Err(()),
        })
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Move::Rock, Move::Paper) => Ordering::Less,
            (Move::Paper, Move::Rock) => Ordering::Greater,

            (Move::Rock, Move::Scissors) => Ordering::Greater,
            (Move::Scissors, Move::Rock) => Ordering::Less,

            (Move::Scissors, Move::Paper) => Ordering::Greater,
            (Move::Paper, Move::Scissors) => Ordering::Less,

            (_, _) => Ordering::Equal,
        }
    }
}

impl Test for Day2 {
    const TEST_OUTPUT1: u32 = 15;
    const TEST_OUTPUT2: u32 = 12;
    const TEST_INPUT: &'static str = "
        A Y
        B X
        C Z
    ";
}
