use crate::utils::{read_lines_into_structs, Instantiable};
use regex::Regex;

struct Command {
    dir: String,
    value: i32,
}

impl Instantiable for Command {
    fn new(values: Vec<&str>) -> Self {
        Command {
            dir: values[0].to_string(),
            value: values[1].parse::<i32>().unwrap(),
        }
    }
}

pub fn solve() -> Result<(), String> {
    let commands = read_lines_into_structs::<Command, _>(
        "src/day2/input.txt",
        Regex::new(r"\s+").unwrap(),
        |_, _| true,
    );

    let mut result = part1(&commands);
    println!("Part 1: {}", result);

    result = part2(&commands);
    println!("Part 2: {}", result);

    Ok(())
}

fn part1(commands: &Vec<Command>) -> i32 {
    let mut values = [0, 0];

    for command in commands {
        if command.dir == "forward" {
            values[0] += command.value;
        }

        if command.dir == "up" {
            values[1] -= command.value;
        }

        if command.dir == "down" {
            values[1] += command.value;
        }
    }

    values.iter().product()
}

fn part2(commands: &Vec<Command>) -> i32 {
    let mut values = [0, 0, 0];

    for command in commands {
        if command.dir == "forward" {
            values[0] += command.value;
            values[1] += values[2] * command.value;
        }

        if command.dir == "up" {
            values[2] -= command.value;
        }

        if command.dir == "down" {
            values[2] += command.value;
        }
    }

    values[0..2].iter().product()
}
