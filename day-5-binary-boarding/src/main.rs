// https://adventofcode.com/2020/day/5

use std::str::FromStr;

#[derive(Debug)]
struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    pub fn parse_from_binary_spec(spec: Vec<char>) -> Seat {
        let (row, cols) = spec
            .iter()
            .fold(((0..127), (0..7)), |(rows, cols), &m| match m {
                'F' => ((rows.start..rows.end - (rows.len() / 2 + 1)), cols),
                'B' => ((rows.start + (rows.len() / 2 + 1)..rows.end), cols),
                'L' => (rows, (cols.start..cols.end - (cols.len() / 2 + 1))),
                'R' => (rows, (cols.start + ((cols.len() / 2) + 1)..cols.end)),
                _ => (rows, cols),
            });

        Seat {
            row: row.start,
            column: cols.start,
        }
    }

    pub fn get_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

impl FromStr for Seat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spec = s.chars().collect::<Vec<char>>();

        Ok(Seat::parse_from_binary_spec(spec))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .expect("The first argument should be the data file path");
    let input_data = std::fs::read_to_string(path).unwrap();

    let seats = input_data
        .lines()
        .map(|s| s.parse::<Seat>().unwrap())
        .collect::<Vec<Seat>>();

    let seat_ids = seats.iter().map(|s| s.get_id()).collect::<Vec<usize>>();
    let min_id = seat_ids.iter().min().unwrap().to_owned();
    let max_id = seat_ids.iter().max().unwrap().to_owned();

    println!("[part 1] {:?}", max_id);
    println!(
        "[part 2] {:?}",
        (min_id..max_id)
            .filter(|id| !seat_ids.contains(id))
            .collect::<Vec<usize>>()
    );
}
