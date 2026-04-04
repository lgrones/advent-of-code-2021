use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::utils::read_lines;

const FILENAME: &str = "src/day15/input.txt";

pub fn solve() -> Result<(), String> {
    let mut map = Map::create();

    read_lines(FILENAME)
        .iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .for_each(|risk| map.add_node(y, risk.to_digit(10).unwrap() as usize))
        });

    let mut result = part1(&map);
    println!("Part 1: {result}");

    result = part2(&mut map);
    println!("Part 2: {result}");

    Ok(())
}

fn part1(map: &Map) -> usize {
    let mut least_risks = HashMap::from([((0, 0), 0)]);
    let mut heap = BinaryHeap::from([Node {
        risk: 0,
        position: (0, 0),
    }]);

    while let Some(Node { risk, position }) = heap.pop() {
        if map.is_end(position) {
            return risk;
        }

        if risk > *least_risks.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        for (position, risk_level) in map.get_neighbors(position) {
            let next = Node {
                risk: risk + risk_level,
                position,
            };

            if next.risk >= *least_risks.get(&next.position).unwrap_or(&usize::MAX) {
                continue;
            }

            heap.push(next.clone());
            least_risks.insert(next.position, next.risk);
        }
    }

    usize::MAX
}

fn part2(map: &mut Map) -> usize {
    map.expand();

    part1(map)
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    risk: usize,
    position: (usize, usize),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Map {
    map: Vec<Vec<usize>>,
}

impl Map {
    fn create() -> Self {
        Map { map: vec![] }
    }

    fn is_end(&self, position: (usize, usize)) -> bool {
        position == (self.map.len() - 1, self.map[0].len() - 1)
    }

    fn add_node(&mut self, y: usize, risk: usize) {
        if self.map.get(y).is_none() {
            self.map.push(vec![]);
        }

        self.map[y].push(risk);
    }

    fn get_neighbors(&self, (y, x): (usize, usize)) -> Vec<((usize, usize), usize)> {
        let rows = self.map.len() as isize;
        let cols = self.map[0].len() as isize;

        [
            (y as isize - 1, x as isize),
            (y as isize + 1, x as isize),
            (y as isize, x as isize - 1),
            (y as isize, x as isize + 1),
        ]
        .into_iter()
        .filter(|&(ny, nx)| ny >= 0 && ny < rows && nx >= 0 && nx < cols)
        .map(|(ny, nx)| {
            let pos = (ny as usize, nx as usize);
            (pos, self.map[pos.0][pos.1])
        })
        .collect()
    }

    fn expand(&mut self) {
        for y in 0..self.map.len() {
            let mut partial_row = self.map[y].clone();

            for _ in 0..4 {
                partial_row = partial_row
                    .iter()
                    .map(|x| match x + 1 {
                        1..=9 => x + 1,
                        _ => 1,
                    })
                    .collect();

                self.map[y].extend(partial_row.iter().copied());
            }
        }

        let mut chunk = self.map.clone();

        for _ in 0..4 {
            chunk = chunk
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|x| match x + 1 {
                            1..=9 => x + 1,
                            _ => 1,
                        })
                        .collect()
                })
                .collect();

            self.map.extend(chunk.iter().cloned());
        }
    }
}
