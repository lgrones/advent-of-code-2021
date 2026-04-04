use crate::utils::read_lines;

const FILENAME: &str = "src/day6/input.txt";

pub fn solve() -> Result<(), String> {
    let lantern_fish: Vec<i32> = read_lines(FILENAME)[0]
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut result: i64 = part1(lantern_fish.clone());
    println!("Part 1: {}", result);

    result = part2(lantern_fish.clone());
    println!("Part 2: {}", result);

    Ok(())
}

fn part1(mut lantern_fish: Vec<i32>) -> i64 {
    for _i in 0..80 {
        let mut new_fish = 0;
        for fish in lantern_fish.iter_mut() {
            if *fish == 0 {
                *fish = 7;
                new_fish += 1;
            }

            *fish -= 1;
        }

        for _ in 0..new_fish {
            lantern_fish.push(8);
        }
    }

    lantern_fish.len() as i64
}

fn part2(lantern_fish: Vec<i32>) -> i64 {
    // Create an array that holds the amount of fish that need a particular amount of days to spawn a new one
    // Each index of the array is equal to the amount of days needed
    // For example: array[2] == 3, that means that 3 fish need 2 more days until they spawn new ones
    let mut lantern_fish_map: [i64; 10] = [
        0,
        lantern_fish.iter().filter(|x| **x == 1).count() as i64,
        lantern_fish.iter().filter(|x| **x == 2).count() as i64,
        lantern_fish.iter().filter(|x| **x == 3).count() as i64,
        lantern_fish.iter().filter(|x| **x == 4).count() as i64,
        lantern_fish.iter().filter(|x| **x == 5).count() as i64,
        0,
        0,
        0,
        0,
    ];

    for _i in 0..256 {
        // Each day the fish at index 0 are added to the fish at index 6 (here 7 as we left shift afterwards)
        // and also added to the fish at index 8 (here 9)
        if lantern_fish_map[0] > 0 {
            lantern_fish_map[7] += lantern_fish_map[0];
            lantern_fish_map[9] += lantern_fish_map[0];
        }

        // Shift everything to the left (simulating a day)
        for j in 1..lantern_fish_map.len() {
            lantern_fish_map[j - 1] = lantern_fish_map[j];
        }

        // Lastly delete the fish at index 9, as this index is only used to temporarily store the index 8 fish
        lantern_fish_map[9] = 0;
    }

    lantern_fish_map.iter().sum()
}
