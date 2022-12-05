use std::{cmp::Ordering, str::FromStr};

pub fn main(input: &str) {
    let input = parse(input);
    dbg!(part1(&input));
    dbg!(part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
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

type Input = Vec<(Move, Move)>;

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

fn parse(input: &str) -> Input {
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

fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|&(theirs, ours)| compute_score(theirs, ours))
        .sum()
}

fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|&(theirs, ending)| {
            let ending = match ending {
                Move::Rock => Ordering::Greater,
                Move::Paper => Ordering::Equal,
                Move::Scissors => Ordering::Less,
            };

            let ours = match (theirs, ending) {
                (theirs, Ordering::Equal) => theirs,

                (Move::Rock, Ordering::Less) => Move::Paper,
                (Move::Paper, Ordering::Less) => Move::Scissors,
                (Move::Scissors, Ordering::Less) => Move::Rock,

                (Move::Rock, Ordering::Greater) => Move::Scissors,
                (Move::Paper, Ordering::Greater) => Move::Rock,
                (Move::Scissors, Ordering::Greater) => Move::Paper,
            };

            compute_score(theirs, ours)
        })
        .sum()
}

#[test]
fn test() {
    let input = parse(
        "
        A Y
        B X
        C Z
    ",
    );
    assert_eq!(15, part1(&input));
    assert_eq!(12, part2(&input));
}
