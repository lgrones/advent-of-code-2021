use std::{
    fs::File,
    i32::MAX,
    io::{self, BufReader, Lines},
};

use crate::utils::{read_lines, read_lines_iterable};

pub fn solve() -> Result<(), io::Error> {
    let lines = read_lines_iterable("src/day1/input.txt")?;

    let mut count = part1(lines);
    println!("Part 1: {}", count);

    let all_lines = read_lines("src/day1/input.txt");
    count = part2(all_lines);
    println!("Part 2: {}", count);

    Ok(())
}

fn part1(lines: Lines<BufReader<File>>) -> i32 {
    // initialized with MAX to prevent the first comparison from generating a count
    let mut prev_depth = MAX;
    let mut count = 0;

    for line in lines {
        if let Ok(l) = line {
            let depth = l.parse::<i32>().unwrap();

            // compare the curretn value with the last
            if depth > prev_depth {
                count += 1;
            }

            prev_depth = depth;
        }
    }

    count
}

fn part2(lines: Vec<String>) -> i32 {
    let mut count = 0;
    let mut window: Vec<i32> = Vec::new();
    let mut next_window: Vec<i32> = Vec::new();

    'outer: for i in 0..lines.len() {
        // create windows of 3
        for j in 0..3 {
            // if both windows are full, compare the two
            if window.len() == 3 && next_window.len() == 3 {
                let prev_depth: i32 = window.iter().sum();
                let depth: i32 = next_window.iter().sum();

                if depth > prev_depth {
                    count += 1;
                }

                // the last window is now the next window
                window.clear();
                window.append(&mut next_window);

                // condition to prevent out of bounds errors
                if i + j >= lines.len() {
                    break 'outer;
                }

                // the next window can already be populated with the current value
                next_window.push(lines[i + j].parse::<i32>().unwrap());

                continue;
            }

            // condition to prevent out of bounds errors
            if i + j >= lines.len() {
                break 'outer;
            }

            // if the windows aren't full, add the value to the proper one
            if window.len() != 3 {
                window.push(lines[i + j].parse::<i32>().unwrap());
            } else {
                next_window.push(lines[i + j].parse::<i32>().unwrap());
            }
        }
    }

    count
}
