use std::{collections::HashMap, vec};

use crate::utils::read_lines;

const FILENAME: &str = "src/day10/input.txt";

pub fn solve() -> Result<(), String> {
    let mut parsers: Vec<Parser> = read_lines(FILENAME)
        .iter()
        .map(|x| Parser::create(x))
        .collect();

    let mut result = part1(&mut parsers);
    println!("Part 1: {result}");

    result = part2(
        &parsers
            .iter()
            .filter(|x| x.incomplete)
            .map(|x| x.to_owned())
            .collect(),
    );
    println!("Part 2: {result}");

    Ok(())
}

fn part1(parsers: &mut Vec<Parser>) -> i64 {
    parsers.iter_mut().fold(0, |acc, curr| {
        acc + match curr.parse() {
            Some(')') => 3,
            Some(']') => 57,
            Some('}') => 1197,
            Some('>') => 25137,
            _ => 0,
        }
    })
}

fn part2(parsers: &Vec<Parser>) -> i64 {
    let mut scores = vec![];

    for parser in parsers {
        scores.push(parser.stack.iter().rev().fold(0, |acc, c| {
            acc * 5
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                }
        }))
    }

    scores.sort();
    scores[scores.len() / 2]
}

#[derive(Debug, Clone)]
struct Parser {
    pub incomplete: bool,
    pub stack: Vec<char>,
    syntax: HashMap<char, char>,
    line: String,
}

impl Parser {
    pub fn create(line: &str) -> Self {
        Parser {
            syntax: HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]),
            stack: vec![],
            line: line.to_string(),
            incomplete: false,
        }
    }

    pub fn parse(&mut self) -> Option<char> {
        for c in self.line.chars() {
            if !self.syntax.contains_key(&c) {
                self.stack.push(c);
                continue;
            }

            if self
                .stack
                .last()
                .is_some_and(|x| x == self.syntax.get(&c).unwrap())
            {
                self.stack.pop();
                continue;
            }

            return Some(c);
        }

        self.incomplete = !self.stack.is_empty();

        None
    }
}
