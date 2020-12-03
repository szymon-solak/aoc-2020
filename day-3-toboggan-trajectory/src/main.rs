// https://adventofcode.com/2020/day/3

use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum MapEntry {
    Tree,
    Blank,
}

impl FromStr for MapEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(MapEntry::Blank),
            "#" => Ok(MapEntry::Tree),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct ForestMap {
    rows: Vec<Vec<MapEntry>>,
}

impl ForestMap {
    pub fn parse(raw_map: &str) -> ForestMap {
        let rows = raw_map
            .split('\n')
            .map(|line| line.trim())
            .map(|row| {
                row.chars()
                    .map(|entry| entry.to_string().parse::<MapEntry>().unwrap())
                    .collect::<Vec<MapEntry>>()
            })
            .collect::<Vec<Vec<MapEntry>>>();

        ForestMap { rows }
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn is_tree(&self, x: usize, y: usize) -> bool {
        if let Some(row) = self.rows.get(y) {
            if let Some(entry) = row.get(x % row.len()) {
                return entry == &MapEntry::Tree;
            }
        }

        false
    }
}

fn count_trees_for_slope(map: &ForestMap, delta_x: usize, delta_y: usize) -> usize {
    std::iter::repeat(0)
        .enumerate()
        .map(|(index, _)| index * delta_x)
        .zip((0..map.height()).step_by(delta_y))
        .filter(|(x, y)| map.is_tree(x.to_owned(), y.to_owned()))
        .count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let map = ForestMap::parse(&input_data);
    let trees_part1 = count_trees_for_slope(&map, 3, 1);

    println!("[part 1] Trees: {:?}", trees_part1);

    let trees_part2 = vec![
        count_trees_for_slope(&map, 1, 1),
        count_trees_for_slope(&map, 3, 1),
        count_trees_for_slope(&map, 5, 1),
        count_trees_for_slope(&map, 7, 1),
        count_trees_for_slope(&map, 1, 2),
    ]
    .iter()
    .product::<usize>();

    println!("[part 2] Trees: {:?}", trees_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_trees_for_example_map() {
        // given
        let map = ForestMap::parse(
            "
            ..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#
        "
            .trim(),
        );

        // when
        let trees = count_trees_for_slope(&map, 3, 1);

        // then
        assert_eq!(trees, 7);
    }
}
