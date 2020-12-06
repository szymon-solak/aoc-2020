// https://adventofcode.com/2020/day/6

#![feature(iterator_fold_self)]
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let groups = input_data.split("\n\n").collect::<Vec<&str>>();

    let uniq_sum = groups
        .iter()
        .map(|g| {
            let mut chars = g
                .lines()
                .collect::<Vec<&str>>()
                .join("")
                .chars()
                .collect::<Vec<char>>();
            chars.sort_unstable();
            chars.dedup();
            chars
        })
        .map(|g| g.iter().count())
        .collect::<Vec<usize>>();

    println!("[part 1] {:?}", uniq_sum.iter().sum::<usize>());

    let all_sum = groups
        .iter()
        .map(|g| {
            g.lines()
                .collect::<Vec<&str>>()
                .iter()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold_first(|acc, set| acc.intersection(&set).copied().collect())
                .into_iter()
                .map(|s| s.len())
                .sum()
        })
        .collect::<Vec<usize>>();

    println!("[part 2] {:?}", all_sum.iter().sum::<usize>())
}
