use crate::utils::read_lines;

const FILENAME: &str = "src/day25/input.txt";

pub fn solve() -> Result<(), String> {
    let sea_cucumbers: Vec<Vec<char>> = read_lines(FILENAME)
        .iter()
        .map(|x| x.chars().collect())
        .collect();

    let result = part1(sea_cucumbers.clone());
    println!("Part 1: {}", result);

    // result = part2(height_map.clone());
    // println!("Part 2: {}", result);

    Ok(())
}

fn draw_map(sea_cucumbers: Vec<Vec<char>>) {
    for i in 0..sea_cucumbers.len() {
        for j in 0..sea_cucumbers[0].len() {
            print!("{}", sea_cucumbers[i][j]);
        }
        println!();
    }
}

fn part1(mut sea_cucumbers: Vec<Vec<char>>) -> i32 {
    println!("initial state:");
    draw_map(sea_cucumbers.clone());
    println!("");

    let mut step = 0;

    // Loop until we've found the solution
    loop {
        // were we able to move this step?
        let mut move_possible = false;
        // all indexes of sea cucumbers that can move this step
        let mut cucumbers_able_to_move = Vec::new();

        // find all east-facing sea cucumbers that can move this step and save the index of each
        // we can't move immediately, since the ones on the far right would be able to move when the ones at index 0 have already moved
        for row in 0..sea_cucumbers.len() {
            for col in 0..sea_cucumbers[0].len() {
                if sea_cucumbers[row][col] == '>' {
                    // the last index wraps around to 0
                    let mut _next_space = 0;
                    if col + 1 < sea_cucumbers[0].len() {
                        _next_space = col + 1;
                    } else {
                        _next_space = 0;
                    }

                    // check if the next space is empty
                    if sea_cucumbers[row][_next_space] == '.' {
                        cucumbers_able_to_move.push((row, col, _next_space));
                    }
                }
            }
        }

        // move each sea-cucumber that is able to
        for east_cucumber in &cucumbers_able_to_move {
            sea_cucumbers[east_cucumber.0][east_cucumber.2] = '>';
            sea_cucumbers[east_cucumber.0][east_cucumber.1] = '.';
        }

        // remember if we were able to move this step
        if cucumbers_able_to_move.len() > 0 {
            move_possible = true;
        }

        cucumbers_able_to_move.clear();

        // do the same for the vertical direction
        // find all south-facing sea cucumbers that can move this step and save the index of each
        for col in 0..sea_cucumbers[0].len() {
            for row in 0..sea_cucumbers.len() {
                if sea_cucumbers[row][col] == 'v' {
                    // the last index wraps around to 0
                    let mut _next_space = 0;
                    if row + 1 < sea_cucumbers.len() {
                        _next_space = row + 1;
                    } else {
                        _next_space = 0;
                    }

                    // check if the next space is empty
                    if sea_cucumbers[_next_space][col] == '.' {
                        cucumbers_able_to_move.push((row, col, _next_space));
                    }
                }
            }
        }

        // move each sea-cucumber that is able to
        for south_cucumber in &cucumbers_able_to_move {
            sea_cucumbers[south_cucumber.2][south_cucumber.1] = 'v';
            sea_cucumbers[south_cucumber.0][south_cucumber.1] = '.';
        }

        // remember if we were able to move this step
        if cucumbers_able_to_move.len() > 0 {
            move_possible = true;
        }

        step += 1;

        println!("After {} steps:", step);
        draw_map(sea_cucumbers.clone());
        println!("");

        if !move_possible {
            break;
        }
    }

    return step;
}
