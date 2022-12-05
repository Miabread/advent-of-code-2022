pub fn main(input: &str) {
    let input = parse(input);
    dbg!(part1(&input));
    dbg!(part2(&input));
}

type Input = Vec<Vec<u32>>;

fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|it| it.trim())
        .collect::<Vec<_>>()
        .split(|it| it.is_empty())
        .map(|elf| elf.iter().map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &Input) -> u32 {
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn part2(input: &Input) -> u32 {
    let mut input: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    input.sort_by(|a, b| a.cmp(b).reverse());
    input.iter().take(3).sum()
}

#[test]
fn test() {
    let input = parse(
        "
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
    ",
    );
    assert_eq!(24000, part1(&input));
    assert_eq!(45000, part2(&input));
}
