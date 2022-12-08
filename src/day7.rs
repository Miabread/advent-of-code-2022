use std::collections::HashMap;

use itertools::Itertools;

use crate::{util::full_lines, Solution, Test};

pub struct Day7;

impl Solution for Day7 {
    type Input = HashMap<String, Vec<Entry>>;

    fn parse(input: &str) -> Self::Input {
        let lines = full_lines(input)
            .map(|line| match line {
                "$ ls" => Line::Ls,
                "$ cd .." => Line::CdParent,
                "$ cd /" => Line::CdRoot,
                line if line.starts_with("$ cd ") => {
                    Line::Cd(line.trim_start_matches("$ cd ").to_owned())
                }
                line if line.starts_with("dir ") => {
                    Line::Entry(Entry::Dir(line.trim_start_matches("dir ").to_owned()))
                }
                line => {
                    let (size, name) = line.split_whitespace().collect_tuple().unwrap();
                    Line::Entry(Entry::File(size.parse().unwrap(), name.to_owned()))
                }
            })
            .collect_vec();

        let mut dirs = HashMap::<String, Vec<Entry>>::new();
        let mut cwd = Vec::new();

        for line in lines {
            match line {
                Line::Ls => {}
                Line::Cd(path) => cwd.push(path),
                Line::CdRoot => cwd.clear(),
                Line::CdParent => {
                    cwd.pop();
                }
                Line::Entry(entry) => {
                    dirs.entry(cwd.last().cloned().unwrap_or_default())
                        .or_default()
                        .push(entry);
                }
            }
        }

        dirs
    }

    fn part1(input: &Self::Input) -> usize {
        fn recurse(
            name: &str,
            dirs: &HashMap<String, Vec<Entry>>,
            sizes: &mut HashMap<String, usize>,
        ) -> usize {
            let size = dirs[name]
                .iter()
                .map(|entry| match entry {
                    Entry::Dir(name) => recurse(name, dirs, sizes),
                    Entry::File(size, _) => *size,
                })
                .sum();
            sizes.insert(name.to_owned(), size);
            size
        }

        let mut sizes = HashMap::new();

        recurse("", input, &mut sizes);

        sizes.into_values().filter(|&size| size <= 100000).sum()
    }
}

#[derive(Debug, Clone)]
pub enum Line {
    Cd(String),
    CdParent,
    CdRoot,
    Ls,
    Entry(Entry),
}

#[derive(Debug, Clone)]
pub enum Entry {
    Dir(String),
    File(usize, String),
}

impl Test for Day7 {
    const TEST_OUTPUT1: usize = 95437;
    const TEST_INPUT: &'static str = "
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    ";
}
