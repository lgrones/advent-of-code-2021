use crate::utils::read_lines;

pub fn solve() -> Result<(), String> {
    let values = read_lines("src/day3/input.txt");

    let mut result = part1(&values);
    println!("Part 1: {}", result);

    result = part2(&values);
    println!("Part 2: {}", result);

    Ok(())
}

// this can definitely be solved much more elegantly by bit shifting

fn part1(values: &Vec<String>) -> i32 {
    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");

    for i in 0..values[0].chars().count() {
        let mut ones = 0;
        let mut zeroes = 0;

        // we should probably use binary numbers here and bit shift them
        // instead of using strings
        for value in values {
            if value.chars().nth(i).unwrap() == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }

        // we could also improve things here
        // the rates are always opposites of each other
        // so by simply flipping each bit of one answer, we get the other
        if ones > zeroes {
            gamma_rate.push_str("1");
            epsilon_rate.push_str("0");
        } else {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        }
    }

    i32::from_str_radix(&gamma_rate, 2).unwrap() * i32::from_str_radix(&epsilon_rate, 2).unwrap()
}

fn part2(v: &Vec<String>) -> i32 {
    let mut values = v.to_owned();
    let mut values2 = v.to_owned();
    let mut oxygen_rating = String::from("");
    let mut co2_rating = String::from("");

    for i in 0..values[0].chars().count() {
        if values.len() == 1 && values2.len() == 1 {
            break;
        }

        let mut ones = 0;
        let mut zeroes = 0;

        for value in &values[0..values.len()] {
            if value.chars().nth(i).unwrap() == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }

        let mut ones2 = 0;
        let mut zeroes2 = 0;

        for value in &values2[0..values2.len()] {
            if value.chars().nth(i).unwrap() == '1' {
                ones2 += 1;
            } else {
                zeroes2 += 1;
            }
        }

        if ones >= zeroes {
            if values.len() > 1 {
                values.retain(|x| x.chars().nth(i).unwrap() == '1');
            }
        } else {
            if values.len() > 1 {
                values.retain(|x| x.chars().nth(i).unwrap() == '0');
            }
        }

        if zeroes2 <= ones2 {
            if values2.len() > 1 {
                values2.retain(|x| x.chars().nth(i).unwrap() == '0');
            }
        } else {
            if values2.len() > 1 {
                values2.retain(|x| x.chars().nth(i).unwrap() == '1');
            }
        }
    }

    oxygen_rating = values[0].to_owned();
    co2_rating = values2[0].to_owned();

    i32::from_str_radix(&oxygen_rating, 2).unwrap() * i32::from_str_radix(&co2_rating, 2).unwrap()
}
