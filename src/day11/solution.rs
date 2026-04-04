use std::{collections::HashSet, collections::VecDeque};

use crate::utils::read_lines;

const FILENAME: &str = "src/day11/input.txt";

pub fn solve() -> Result<(), String> {
    let mut octopi: Vec<Octopus> = read_lines(FILENAME)
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, energy_level)| Octopus {
                    x: x as isize,
                    y: y as isize,
                    energy_level: energy_level.to_digit(10).unwrap() as u8,
                })
        })
        .flatten()
        .collect();

    let (result_1, result_2) = part1_and_2(&mut octopi);
    println!("Part 1: {result_1}");
    println!("Part 2: {result_2}");

    Ok(())
}

fn part1_and_2(octopi: &mut Vec<Octopus>) -> (i32, i32) {
    let mut result_1 = 0;
    let mut result_2 = 0;
    let mut step = 0;

    loop {
        step += 1;

        let mut flashed = HashSet::new();
        let mut queue = VecDeque::new();

        // First, the energy level of each octopus increases by 1
        octopi.iter_mut().for_each(|x| x.increment());

        // Then, any octopus with an energy level greater than 9...
        queue.extend(
            octopi
                .iter_mut()
                .enumerate()
                .filter(|(_, x)| x.is_maxed())
                .map(|(i, _)| i),
        );

        while !queue.is_empty() {
            let indices: Vec<usize> = queue.drain(..).collect();

            // ...flashes
            indices.iter().copied().for_each(|i| {
                octopi[i].flash();
                flashed.insert(i);
            });

            // This increases the energy level of all adjacent octopuses by 1
            for index in indices {
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }

                        let i = octopi
                            .iter()
                            .enumerate()
                            .find(|(i, octopus)| {
                                !flashed.contains(i)
                                    && !queue.contains(i)
                                    && octopus.x == octopi[index].x + x
                                    && octopus.y == octopi[index].y + y
                            })
                            .and_then(|(i, _)| Some(i));

                        if let Some(i) = i {
                            octopi[i].increment();

                            // If this causes an octopus to have an energy level greater than 9, it also flashes
                            if octopi[i].is_maxed() {
                                queue.push_back(i);
                            }
                        }
                    }
                }
            }
        }

        if step <= 100 {
            result_1 = result_1 + flashed.len();
        }

        if result_2 == 0 && flashed.len() == octopi.len() {
            result_2 = step;
        }

        if step >= 100 && result_2 != 0 {
            return (result_1 as i32, result_2);
        }
    }
}

struct Octopus {
    x: isize,
    y: isize,
    energy_level: u8,
}

impl Octopus {
    pub fn increment(&mut self) {
        self.energy_level = self.energy_level + 1;
    }

    pub fn flash(&mut self) {
        self.energy_level = 0;
    }

    pub fn is_maxed(&self) -> bool {
        self.energy_level > 9
    }
}
