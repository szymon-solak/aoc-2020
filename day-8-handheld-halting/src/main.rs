// https://adventofcode.com/2020/day/8

use std::collections::HashSet;

#[derive(Debug)]
pub struct State {
    pub accumulator: i16,
    pub instruction: i16,
}

impl State {
    pub fn new() -> State {
        State {
            accumulator: 0,
            instruction: 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation {
    Acc(i16),
    Nop(i16),
    Jmp(i16),
}

impl Operation {
    pub fn apply(&self, state: State) -> State {
        match self {
            Operation::Acc(v) => State {
                accumulator: state.accumulator + v,
                instruction: state.instruction + 1,
            },
            Operation::Nop(_) => State {
                accumulator: state.accumulator,
                instruction: state.instruction + 1,
            },
            Operation::Jmp(v) => State {
                accumulator: state.accumulator,
                instruction: state.instruction + v,
            },
        }
    }
}

fn parse_instruction(s: &str) -> Operation {
    let mut parts = s.split_whitespace();
    let instruction = parts.next().unwrap();
    let val = parts.next().unwrap().parse::<i16>().unwrap();

    match instruction {
        "acc" => Operation::Acc(val),
        "nop" => Operation::Nop(val),
        "jmp" => Operation::Jmp(val),
        _ => unreachable!(),
    }
}

fn main() {
    let input_data = std::fs::read_to_string("./data/data.txt").unwrap();

    let instructions = input_data
        .lines()
        .map(|r| parse_instruction(r))
        .collect::<Vec<Operation>>();

    println!("[part 1] {:?}", run(&instructions));

    let mut mods = instructions.iter().enumerate().filter(|(_, op)| {
        if let Operation::Jmp(_) = op {
            return true;
        }
        if let Operation::Nop(_) = op {
            return true;
        }
        false
    });

    while let Some((mod_index, mod_op)) = mods.next() {
        let mut mod_instructions = instructions
            .iter()
            .clone()
            .map(|f| f.to_owned())
            .collect::<Vec<Operation>>();

        mod_instructions[mod_index] = match mod_op {
            Operation::Jmp(v) => Operation::Nop(v.to_owned()),
            Operation::Nop(v) => Operation::Jmp(v.to_owned()),
            _ => unreachable!(),
        };

        let (res, last_state) = run(&mod_instructions);

        if res {
            println!("[part 2] {:?}", last_state);
            break;
        }
    }
}

fn run(instructions: &[Operation]) -> (bool, State) {
    let mut state = State::new();
    let mut seen = HashSet::<usize>::new();

    loop {
        let index = state.instruction as usize;

        if index == instructions.len() {
            return (true, state);
        }

        if index > instructions.len() {
            return (false, state);
        }

        if seen.contains(&index) {
            return (false, state);
        }

        seen.insert(index);

        let instruction = instructions.get(index).unwrap();
        state = instruction.apply(state);
    }
}
