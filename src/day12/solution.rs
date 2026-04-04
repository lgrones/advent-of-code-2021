use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

use crate::utils::read_lines;

const FILENAME: &str = "src/day12/input.txt";

pub fn solve() -> Result<(), String> {
    let mut mapped: HashMap<&str, CavePtr> = HashMap::new();

    let binding = read_lines(FILENAME);
    binding.iter().for_each(|x| {
        let parts: Vec<&str> = x.split("-").collect();

        let start = mapped
            .entry(parts[0])
            .or_insert(CavePtr(Rc::new(RefCell::new(Cave::create(parts[0])))))
            .clone();

        let end = mapped
            .entry(parts[1])
            .or_insert(CavePtr(Rc::new(RefCell::new(Cave::create(parts[1])))))
            .clone();

        start.borrow_mut().add_neighbor(&end);
        end.borrow_mut().add_neighbor(&start);
    });

    let start_cave = mapped.get("start").unwrap();

    let mut result = part1(start_cave);
    println!("Part 1: {result}");

    result = part2(start_cave);
    println!("Part 2: {result}");

    Ok(())
}

fn part1(start: &CavePtr) -> i32 {
    fn traverse(cave: CavePtr, visited: &mut HashSet<CavePtr>) -> i32 {
        if cave.borrow().id == "end" {
            return 1;
        }

        if cave.borrow().is_small && visited.contains(&cave) {
            return 0;
        }

        let mut paths = 0;
        visited.insert(cave.clone());

        for neighbor in &cave.borrow().neighbors {
            paths = paths + traverse(neighbor.clone(), visited);
        }

        visited.remove(&cave);

        return paths;
    }

    return traverse(start.clone(), &mut HashSet::new());
}

fn part2(start: &CavePtr) -> i32 {
    fn traverse(cave: CavePtr, visited_twice: bool, visited: &mut HashSet<CavePtr>) -> i32 {
        if cave.borrow().id == "end" {
            return 1;
        }

        let mut twice = visited_twice;

        if cave.borrow().is_small && visited.contains(&cave) {
            if visited_twice || cave.borrow().id == "start" {
                return 0;
            }

            twice = true;
        }

        let mut paths = 0;
        let inserted = visited.insert(cave.clone());

        for neighbor in &cave.borrow().neighbors {
            paths = paths + traverse(neighbor.clone(), twice, visited);
        }

        if inserted {
            visited.remove(&cave);
        }

        return paths;
    }

    return traverse(start.clone(), false, &mut HashSet::new());
}

#[derive(Clone)]
struct CavePtr(Rc<RefCell<Cave>>);

impl Deref for CavePtr {
    type Target = Rc<RefCell<Cave>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for CavePtr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl PartialEq for CavePtr {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for CavePtr {}

struct Cave {
    id: String,
    is_small: bool,
    neighbors: HashSet<CavePtr>,
}

impl Cave {
    pub fn create(id: &str) -> Self {
        Cave {
            id: id.to_string(),
            is_small: id == id.to_lowercase(),
            neighbors: HashSet::new(),
        }
    }

    pub fn add_neighbor(&mut self, cave: &CavePtr) {
        self.neighbors.insert(cave.clone());
    }
}
