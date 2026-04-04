use std::{
    collections::{hash_map::RandomState, HashSet},
    i32::MAX,
};

use crate::utils::{read_lines_into_structs, Instantiable};
use regex::Regex;

const FILENAME: &str = "src/day13/input.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Instantiable for Point {
    fn new(values: Vec<&str>) -> Self {
        Point {
            x: values[0].parse::<i32>().unwrap(),
            y: values[1].parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    axis: char,
    value: i32,
}

impl Instantiable for Instruction {
    fn new(values: Vec<&str>) -> Self {
        Instruction {
            axis: values[2].chars().collect::<Vec<char>>()[0],
            value: values[3].parse::<i32>().unwrap(),
        }
    }
}

pub fn solve() -> Result<(), String> {
    let mut end_of_points = MAX;

    let points =
        read_lines_into_structs::<Point, _>(FILENAME, Regex::new(r",").unwrap(), |l: String, i| {
            if l == "".to_string() {
                end_of_points = i;
            }

            end_of_points > i
        });

    let instructions = read_lines_into_structs::<Instruction, _>(
        FILENAME,
        Regex::new(r"\s|=").unwrap(),
        |_, i| end_of_points < i,
    );

    let result = part1(points.clone(), instructions[0]);
    println!("Part 1: {}", result);

    println!("");
    part2(points.clone(), instructions.clone());

    Ok(())
}

fn draw_paper(points: HashSet<Point, RandomState>) {
    let mut size = (0, 0);
    for Point { x, y } in &points {
        if *x > size.0 {
            size = (*x, size.1);
        }

        if *y > size.1 {
            size = (size.0, *y);
        }
    }

    for y in 0..size.1 + 1 {
        for x in 0..size.0 + 1 {
            if points.contains(&Point { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("")
    }

    println!("")
}

fn part1(points: Vec<Point>, instruction: Instruction) -> i32 {
    let mut unique_points: HashSet<Point, RandomState> = HashSet::new();

    for point in points {
        let mut _relevant_axis_value = 0;

        if instruction.axis == 'x' {
            _relevant_axis_value = point.x;
        } else {
            _relevant_axis_value = point.y;
        }

        if _relevant_axis_value < instruction.value {
            unique_points.insert(point);
            continue;
        }

        let new_axis_value = instruction.value - (_relevant_axis_value - instruction.value);

        let new_point: Point;

        if instruction.axis == 'x' {
            new_point = Point {
                x: new_axis_value,
                y: point.y,
            };
        } else {
            new_point = Point {
                x: point.x,
                y: new_axis_value,
            };
        }

        unique_points.insert(new_point);
    }

    unique_points.len() as i32
}

fn part2(points: Vec<Point>, instructions: Vec<Instruction>) {
    let mut unique_points: HashSet<Point> = points.into_iter().collect();

    for instruction in instructions {
        let mut unique_points_copy = HashSet::new();

        for point in &unique_points {
            let mut _relevant_axis_value = 0;

            if instruction.axis == 'x' {
                _relevant_axis_value = point.x;
            } else {
                _relevant_axis_value = point.y;
            }

            if _relevant_axis_value < instruction.value {
                unique_points_copy.insert(*point);
                continue;
            }

            let new_axis_value = instruction.value - (_relevant_axis_value - instruction.value);

            let new_point: Point;

            if instruction.axis == 'x' {
                new_point = Point {
                    x: new_axis_value,
                    y: point.y,
                };
            } else {
                new_point = Point {
                    x: point.x,
                    y: new_axis_value,
                };
            }

            unique_points_copy.insert(new_point);
        }

        unique_points = unique_points_copy.clone();
    }

    draw_paper(unique_points.clone());
    // unique_points.len() as i32
}
