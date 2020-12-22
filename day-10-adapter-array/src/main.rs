// https://adventofcode.com/2020/day/10

#![feature(is_sorted)]

use itertools::Itertools;

fn main() {
    let input_data = std::fs::read_to_string("./data/data.txt").unwrap();

    let mut adapter_chain = input_data
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    adapter_chain.sort_unstable();

    let device_jolts = adapter_chain.last().unwrap() + 3;

    let mut chain = vec![0];
    chain.append(&mut adapter_chain);
    chain.push(device_jolts);

    let one_diffs = chain
        .windows(2)
        .filter(|pair| pair[1] - pair[0] == 1)
        .count();
    let three_diffs = chain
        .windows(2)
        .filter(|pair| pair[1] - pair[0] == 3)
        .count();

    println!("[part 1] {:?}", one_diffs * three_diffs);
    println!(
        "[part 2] {:?}",
        count_chunk_combinations(into_chunks(&chain))
    )
}

fn into_chunks(adapters: &[i32]) -> Vec<Vec<i32>> {
    let mut chunks = Vec::<Vec<i32>>::new();
    let mut chunk = Vec::<i32>::new();

    let head = &adapters[0];
    let body = &adapters[1..adapters.len() - 1];
    let tail = &adapters[adapters.len() - 1];

    chunks.push(vec![head.to_owned()]);

    for adapter in body.iter() {
        let last_item = chunk.last().cloned();
        if last_item.is_some() && adapter.to_owned() == last_item.unwrap() + 3 {
            chunks.push(chunk.to_owned());
            chunk.clear();
        }

        chunk.push(adapter.to_owned());
    }

    if chunk.len() > 0 { chunks.push(chunk.to_owned()) }

    chunks.push(vec![tail.to_owned()]);

    chunks
}

fn count_chunk_combinations(adapters: Vec<Vec<i32>>) -> u128 {
    adapters
        .iter()
        .enumerate()
        .filter(|(_, chunk)| chunk.len() > 1)
        .map(|(index, c)| {
            (1..=c.len())
                .map(|l| {
                    let perms = c.iter()
                        .permutations(l)
                        .filter(|p| p.is_sorted())
                        .filter(|p| {
                            if index > 0 {
                                if let Some(prev_adapter) = adapters.get(index - 1) {
                                    if p.first().unwrap().to_owned()
                                        - prev_adapter.last().unwrap().to_owned()
                                        > 3
                                    {
                                        return false;
                                    }
                                }
                            }

                            if let Some(next_adapter) = adapters.get(index + 1) {
                                if next_adapter.first().unwrap().to_owned()
                                    - p.last().unwrap().to_owned()
                                    > 3
                                {
                                    return false;
                                }
                            } else {
                                let max_val = adapters.last().unwrap().last().unwrap() + 3;
                                if p.last().unwrap().to_owned() > &max_val { return false }
                            }

                            true
                        })
                        .filter(|p| {
                            p
                                .windows(2)
                                .map(|pair| { pair[1] - pair[0] })
                                .filter(|p| p <= &&3)
                                .collect_vec()
                                .len() + 1 == p.len()
                        })
                        .collect_vec();

                    perms.len() as u128
                })
                .sum::<u128>()
        })
        .product::<u128>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_example() {
        // given
        let input = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];

        // when
        let arrangments = count_chunk_combinations(into_chunks(&input));

        // then
        assert_eq!(arrangments, 8);
    }

    #[test]
    fn part_2_example_2() {
        // given
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 52
        ];
        input.sort_unstable();

        // when
        let arrangements = count_chunk_combinations(into_chunks(&input));

        // then
        assert_eq!(arrangements, 19208);
    }

    #[test]
    fn additional_example_1() {
        // given
        let mut input = vec![10, 6, 4, 7, 1, 5, 0, 13];
        input.sort_unstable();

        // when
        let arrangements = count_chunk_combinations(into_chunks(&input));

        // then
        assert_eq!(arrangements, 4);
    }

    #[test]
    fn additional_example_2() {
        // given
        let mut input = vec![4, 11, 7, 8, 1, 6, 5, 0, 14];
        input.sort_unstable();

        // when
        let arrangements = count_chunk_combinations(into_chunks(&input));

        // then
        assert_eq!(arrangements, 7);
    }

    #[test]
    fn additional_example_3() {
        // given
        let mut input = vec![3, 1, 6, 2, 0, 9];
        input.sort_unstable();

        // when
        let arrangements = count_chunk_combinations(into_chunks(&input));

        // then
        assert_eq!(arrangements, 4);
    }

    #[test]
    fn additional_example_4() {
        // given
        let mut input = vec![17, 6, 10, 5, 13, 7, 1, 4, 12, 11, 14, 0, 20];
        input.sort_unstable();

        // when
        let arrangements = count_chunk_combinations(into_chunks(&input));

        // then
        assert_eq!(arrangements, 28);
    }
}
