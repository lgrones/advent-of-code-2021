use crate::utils::{read_lines_into_structs, Instantiable};
use regex::Regex;

const FILENAME: &str = "src/day5/input.txt";

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Line {
    start_point: Point,
    end_point: Point,
}

impl Instantiable for Line {
    fn new(values: Vec<&str>) -> Self {
        let start_point: Vec<i32> = values[0]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let end_point: Vec<i32> = values[1]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Line {
            start_point: Point {
                x: start_point[0],
                y: start_point[1],
            },
            end_point: Point {
                x: end_point[0],
                y: end_point[1],
            },
        }
    }
}

impl Line {
    fn is_horizontal_line(&self) -> bool {
        self.start_point.y == self.end_point.y
    }

    fn is_vertical_line(&self) -> bool {
        self.start_point.x == self.end_point.x
    }

    fn is_straight_line(&self) -> bool {
        self.is_horizontal_line() || self.is_vertical_line()
    }

    fn get_points_along_straight_line(&self) -> Vec<Point> {
        let mut result = Vec::new();

        let mut _start = 0;
        let mut _end = 0;

        if self.is_horizontal_line() {
            if self.start_point.x < self.end_point.x {
                _start = self.start_point.x;
                _end = self.end_point.x;
            } else {
                _start = self.end_point.x;
                _end = self.start_point.x;
            }

            for i in _start.._end + 1 {
                result.push(Point {
                    x: i,
                    y: self.start_point.y,
                })
            }
        }

        if self.is_vertical_line() {
            if self.start_point.y < self.end_point.y {
                _start = self.start_point.y;
                _end = self.end_point.y;
            } else {
                _start = self.end_point.y;
                _end = self.start_point.y;
            }

            for i in _start.._end + 1 {
                result.push(Point {
                    x: self.start_point.x,
                    y: i,
                })
            }
        }

        return result;
    }

    fn get_points_along_diagonal_line(&self) -> Vec<Point> {
        let mut result = Vec::new();

        let mut _start = 0;
        let mut _end = 0;

        if self.start_point.x < self.end_point.x {
            _start = self.start_point.x;
            _end = self.end_point.x;
        } else {
            _start = self.end_point.x;
            _end = self.start_point.x;
        }

        for i in 0..(self.start_point.x - self.end_point.x).abs() + 1 {
            if self.start_point.x < self.end_point.x && self.start_point.y < self.end_point.y {
                result.push(Point {
                    x: self.start_point.x + i,
                    y: self.start_point.y + i,
                });
            }

            if self.start_point.x > self.end_point.x && self.start_point.y > self.end_point.y {
                result.push(Point {
                    x: self.start_point.x - i,
                    y: self.start_point.y - i,
                });
            }

            if self.start_point.x < self.end_point.x && self.start_point.y > self.end_point.y {
                result.push(Point {
                    x: self.start_point.x + i,
                    y: self.start_point.y - i,
                });
            }

            if self.start_point.x > self.end_point.x && self.start_point.y < self.end_point.y {
                result.push(Point {
                    x: self.start_point.x - i,
                    y: self.start_point.y + i,
                });
            }
        }

        return result;
    }

    fn get_points_along_line(&self) -> Vec<Point> {
        if self.is_straight_line() {
            return self.get_points_along_straight_line();
        }

        self.get_points_along_diagonal_line()
    }
}

#[allow(dead_code)]
fn print_diagram(points: Vec<Vec<i32>>) {
    for i in 0..points.len() {
        for j in 0..points[0].len() {
            let mut point = String::from(".");

            if points[i][j] > 0 {
                point = points[i][j].to_string();
            }

            print!("{}", point);
        }
        println!("");
    }
}

pub fn solve() -> Result<(), String> {
    let points =
        read_lines_into_structs::<Line, _>(FILENAME, Regex::new(r" -> ").unwrap(), |_, _| true);

    let mut straight_lines = points.clone();
    straight_lines.retain(|x| x.is_straight_line());

    let mut result = part1(straight_lines);
    println!("Part 1: {}", result);

    result = part2(points);
    println!("Part 2: {}", result);

    Ok(())
}

fn part1(lines: Vec<Line>) -> i32 {
    let mut x_max = 0;
    let mut y_max = 0;

    for line in &lines {
        for Point { x, y } in line.get_points_along_line() {
            if x_max < x {
                x_max = x;
            }

            if y_max < y {
                y_max = y;
            }
        }
    }

    let mut diagram =
        vec![vec![0; (x_max + 1).try_into().unwrap()]; (y_max + 1).try_into().unwrap()];

    for line in &lines {
        for Point { x, y } in line.get_points_along_line() {
            diagram[y as usize][x as usize] += 1;
        }
    }

    // print_diagram(diagram.clone());

    diagram
        .iter()
        .flatten()
        .filter(|x| **x >= 2)
        .count()
        .try_into()
        .unwrap()
}

fn part2(lines: Vec<Line>) -> i32 {
    let mut x_max = 0;
    let mut y_max = 0;

    for line in &lines {
        for Point { x, y } in line.get_points_along_line() {
            if x_max < x {
                x_max = x;
            }

            if y_max < y {
                y_max = y;
            }
        }
    }

    let mut diagram =
        vec![vec![0; (x_max + 1).try_into().unwrap()]; (y_max + 1).try_into().unwrap()];

    for line in &lines {
        for Point { x, y } in line.get_points_along_line() {
            diagram[y as usize][x as usize] += 1;
        }
    }

    // print_diagram(diagram.clone());

    diagram
        .iter()
        .flatten()
        .filter(|x| **x >= 2)
        .count()
        .try_into()
        .unwrap()
}
