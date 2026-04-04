use crate::utils::{read_lines, read_lines_into_lists_of_structs, Instantiable};
use regex::Regex;

const FILENAME: &str = "src/day4/input.txt";

#[derive(Debug, Clone)]
struct Field {
    value: i32,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Row {
    fields: Vec<Field>,
}

impl Instantiable for Row {
    fn new(values: Vec<&str>) -> Self {
        Row {
            fields: values
                .iter()
                .map(|x| Field {
                    marked: false,
                    value: x.parse::<i32>().unwrap(),
                })
                .collect(),
        }
    }
}

fn find_winning_board(boards: &Vec<Vec<Row>>) -> Option<Vec<Row>> {
    // Iterate over every board
    for board in boards {
        let result = is_winning_board(board);
        if result.is_some() {
            return result;
        }
    }

    // Otherwise return nothing
    None
}

fn is_winning_board(board: &Vec<Row>) -> Option<Vec<Row>> {
    // Check if any row is full
    for Row { fields } in board {
        if fields.iter().all(|x| x.marked) {
            return Some(board.to_vec());
        }
    }

    // Or if any Column is full
    for col in 0..board[0].fields.len() {
        if board.iter().all(|x| x.fields[col].marked) {
            return Some(board.to_vec());
        }
    }

    None
}

pub fn solve() -> Result<(), String> {
    let numbers: Vec<i32> = read_lines(FILENAME)[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let boards = read_lines_into_lists_of_structs::<Row, _>(
        FILENAME,
        Regex::new(r"\s+").unwrap(),
        "",
        |_, i| i > 1,
    );

    let mut result = part1(numbers.clone(), boards.clone());
    println!("Part 1: {}", result);

    result = part2(numbers, boards);
    println!("Part 2: {}", result);

    Ok(())
}

fn part1(numbers: Vec<i32>, mut boards: Vec<Vec<Row>>) -> i32 {
    let mut winning_board: Vec<Row> = Vec::new();
    let mut winning_number = 0;

    for number in numbers {
        for board in boards.iter_mut() {
            for Row { fields } in board {
                for field in fields {
                    if field.value == number {
                        field.marked = true;
                    }
                }
            }
        }

        let winning_board_option = find_winning_board(&boards);
        if winning_board_option.is_some() {
            winning_board = winning_board_option.unwrap();
            winning_number = number;
            break;
        }
    }

    let mut score = 0;

    for row in winning_board {
        for Field { value, marked } in row.fields {
            if !marked {
                score += value;
            }
        }
    }

    score * winning_number
}

fn part2(numbers: Vec<i32>, mut boards: Vec<Vec<Row>>) -> i32 {
    let mut losing_board: Vec<Row> = Vec::new();
    let mut winning_number = 0;

    for number in numbers {
        for board in boards.iter_mut() {
            for Row { fields } in board {
                for field in fields {
                    if field.value == number {
                        field.marked = true;
                    }
                }
            }
        }

        if boards.len() > 1 {
            boards.retain(|x| is_winning_board(x).is_none());
        }

        if boards.len() == 1 && is_winning_board(&boards[0]).is_some() {
            losing_board = boards.get(0).unwrap().to_vec();
            winning_number = number;
            break;
        }
    }

    let mut score = 0;

    for row in losing_board {
        for Field { value, marked } in row.fields {
            if !marked {
                score += value;
            }
        }
    }

    score * winning_number
}
