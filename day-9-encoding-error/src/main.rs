// https://adventofcode.com/2020/day/9

use itertools::Itertools;

fn main() {
    let input_data = std::fs::read_to_string("./data/data.txt").unwrap();
    let numbers = input_data
        .lines()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let invalid_number = find_invalid_number(&numbers).unwrap();
    println!("[part 1] {:?}", invalid_number);

    let set = find_contiguous_set(&numbers, invalid_number);
    println!(
        "[part 2] {:?}",
        set.iter().min().unwrap() + set.iter().max().unwrap()
    );
}

fn find_invalid_number(numbers: &[i64]) -> Option<i64> {
    for number_index in 25..numbers.len() {
        let number = numbers.get(number_index).unwrap();
        let prev_batch = &numbers[number_index - 25..number_index];

        if prev_batch
            .iter()
            .combinations(2)
            .map(|p| p.iter().map(|v| v.to_owned()).sum::<i64>())
            .find(|s| s == number)
            .is_none()
        {
            return Some(number.to_owned());
        }
    }

    None
}

fn find_contiguous_set(numbers: &[i64], target: i64) -> &[i64] {
    let mut start = 0;
    let mut end = 1;

    loop {
        let s = &numbers[start..end];
        let sum = s.iter().sum::<i64>();

        if sum < target {
            end += 1
        }
        if sum > target {
            start += 1;
            end = start + 1;
        }

        if sum == target {
            return s;
        }
    }
}
