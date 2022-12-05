use std::fs::read_to_string;

mod day1;
mod day2;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("no day given")
        .parse()
        .expect("to be a number");

    let main = match day {
        1 => day1::main,
        2 => day2::main,
        _ => panic!("not a valid day"),
    };

    let input = read_to_string(format!("./input/day{day}.txt")).expect("input file should exist");

    main(&input);
}
