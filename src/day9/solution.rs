use crate::utils::read_lines;

const FILENAME: &str = "src/day9/input.txt";

pub fn solve() -> Result<(), String> {
    let height_map: Vec<Vec<i32>> = read_lines(FILENAME)
        .iter()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut result = part1(height_map.clone());
    println!("Part 1: {}", result);

    result = part2(height_map.clone());
    println!("Part 2: {}", result);

    Ok(())
}

fn part1(height_map: Vec<Vec<i32>>) -> i32 {
    let mut low_points = Vec::new();

    for i in 0..height_map.len() {
        for j in 0..height_map[0].len() {
            let row = i as i32;
            let column = j as i32;

            if column - 1 >= 0 && height_map[i][j - 1] <= height_map[i][j] {
                continue;
            }

            if column + 1 < height_map[0].len() as i32 && height_map[i][j + 1] <= height_map[i][j] {
                continue;
            }

            if row - 1 >= 0 && height_map[i - 1][j] <= height_map[i][j] {
                continue;
            }

            if row + 1 < height_map.len() as i32 && height_map[i + 1][j] <= height_map[i][j] {
                continue;
            }

            low_points.push(height_map[i][j])
        }
    }

    low_points.iter().sum::<i32>() + low_points.len() as i32
}

fn part2(height_map: Vec<Vec<i32>>) -> i32 {
    // TODO
    0
}
