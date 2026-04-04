use crate::utils::read_lines;

const FILENAME: &str = "src/day7/input.txt";

pub fn solve() -> Result<(), String> {
    let horizontal_positions: Vec<i32> = read_lines(FILENAME)[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut result = part1(horizontal_positions.clone());
    println!("Part 1: {}", result);

    result = part2(horizontal_positions.clone());
    println!("Part 2: {}", result);

    Ok(())
}

// Only looked that up after realizing it myself to make sure ;)
// "It would be more correct to say that a point that minimizes the sum of distances is the median." - copper.hat
// https://math.stackexchange.com/questions/318381/on-a-1-d-line-the-point-that-minimizes-the-sum-of-the-distances-is-the-median#:~:text=It%20would%20be%20more%20correct,of%20distances%20is%20the%20median.
fn part1(mut horizontal_positions: Vec<i32>) -> i32 {
    horizontal_positions.sort_unstable();
    let median = horizontal_positions[horizontal_positions.len() / 2];

    let mut fuel = 0;
    for pos in horizontal_positions {
        fuel += (pos - median).abs();
    }

    fuel
}

// Needed a "factorial but with addition". Thank god there's google
// https://en.wikipedia.org/wiki/Triangular_number
fn part2(horizontal_positions: Vec<i32>) -> i32 {
    // It's not actually the average, as the number is not rounded, but simply truncated
    // The solution is apparently not the average at all though: https://www.reddit.com/r/adventofcode/comments/rars4g/2021_day_7_why_do_these_values_work_spoilers/
    let average = horizontal_positions.iter().sum::<i32>() / (horizontal_positions.len() as i32);

    let mut fuel = 0;
    for pos in horizontal_positions {
        let steps = (pos - average).abs();
        fuel += steps * (steps + 1) / 2;
    }

    fuel
}
