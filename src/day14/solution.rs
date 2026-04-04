use std::collections::HashMap;

use crate::utils::read_lines;

const FILENAME: &str = "src/day14/input.txt";

pub fn solve() -> Result<(), String> {
    let polymer_template: Vec<char> = read_lines(FILENAME)[0].chars().collect();

    let lines = read_lines(FILENAME);
    let mut insertion_rules = HashMap::new();
    for line in &lines[2..lines.len()] {
        let key_value_pair: Vec<&str> = line.split(" -> ").collect();
        insertion_rules.insert(
            key_value_pair[0].to_string(),
            key_value_pair[1].chars().collect::<Vec<char>>()[0],
        );
    }

    let result = part1(polymer_template.clone(), insertion_rules);
    println!("Part 1: {}", result);

    // result = part2(points.clone(), instructions.clone());
    // println!("Part 2: {}", result);

    Ok(())
}

fn part1(mut polymer_template: Vec<char>, insertion_rules: HashMap<String, char>) -> i32 {
    let mut new_polymer_template = polymer_template.clone();

    for _i in 0..10 {
        let mut inserted_count = 0;

        // look at each pair in the template
        for i in 0..polymer_template.len() - 1 {
            let pair: String = polymer_template[i..i + 2].iter().collect();

            // insert the new element between the pair
            new_polymer_template
                .insert(i + 1 + inserted_count, *insertion_rules.get(&pair).unwrap());
            inserted_count += 1;
        }

        // the template now is the new template
        polymer_template = new_polymer_template.clone();
    }

    let mut occurrences = HashMap::new();

    for element in polymer_template {
        *occurrences.entry(element).or_insert(0) += 1;
    }

    let mut sorted_occurrences = occurrences.iter().collect::<Vec<(&char, &i32)>>();
    sorted_occurrences.sort_by(|a, b| b.1.cmp(a.1));

    sorted_occurrences[0].1 - sorted_occurrences[sorted_occurrences.len() - 1].1
}
