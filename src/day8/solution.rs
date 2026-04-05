use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_lines;

const FILENAME: &str = "src/day8/input.txt";

pub fn solve() -> Result<(), String> {
    let lines: Vec<(String, String)> = read_lines(FILENAME)
        .iter()
        .map(|x| {
            let mut parts = x.split(" | ");

            (
                parts.next().unwrap().to_owned(),
                parts.next().unwrap().to_owned(),
            )
        })
        .collect();

    let mut result = part1(&lines.iter().map(|(_, x)| x.to_owned()).collect());
    println!("Part 1: {result}");

    result = part2(&lines);
    println!("Part 2: {result}");

    Ok(())
}

fn part1(outputs: &Vec<String>) -> u32 {
    outputs.iter().fold(0, |acc, output| {
        acc + output
            .split(" ")
            .filter(|x| [2, 3, 4, 7].contains(&x.len()))
            .count() as u32
    })
}

fn part2(lines: &Vec<(String, String)>) -> u32 {
    let mut result = 0;

    fn take_match(v: &mut Vec<String>, pred: impl Fn(&String) -> bool) -> String {
        let i = v.iter().position(pred).unwrap();
        v.remove(i)
    }

    for (signals, outputs) in lines {
        let mut sorted: Vec<String> = signals
            .split(" ")
            .map(|x| x.chars().sorted().collect::<String>())
            .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()))
            .collect();

        let one: Vec<char> = sorted[0].chars().collect();
        let four: Vec<char> = sorted[2].chars().collect();

        let mut map = HashMap::from([
            (sorted.remove(0), 1),
            (sorted.remove(0), 7),
            (sorted.remove(0), 4),
            (sorted.remove(6), 8),
        ]);

        let segments_5 = &mut sorted[0..3].to_vec();
        let segments_6 = &mut sorted[3..].to_vec();

        // 3 must have all segments of 1
        map.insert(
            take_match(segments_5, |x| one.iter().all(|y| x.contains(*y))),
            3,
        );

        // 5 must have 1 missing segment of 4
        map.insert(
            take_match(segments_5, |x| {
                four.iter().filter(|s| x.contains(**s)).count() == 3
            }),
            5,
        );

        // 2 must be the last remaining
        map.insert(segments_5.remove(0), 2);

        // 6 must not have all segments of 1
        map.insert(
            take_match(segments_6, |x| !one.iter().all(|y| x.contains(*y))),
            6,
        );

        // 9 must have all segments of 4
        map.insert(
            take_match(segments_6, |x| four.iter().all(|y| x.contains(*y))),
            9,
        );

        // 0 must be the last remaining
        map.insert(segments_6.remove(0), 0);

        let mut value = vec![];
        for output in outputs.split(" ") {
            value.push(
                map.get(&output.chars().sorted().collect::<String>())
                    .unwrap()
                    .to_string(),
            );
        }

        result = result + value.join("").parse::<u32>().unwrap();
    }

    result
}
